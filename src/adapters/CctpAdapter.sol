// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {FastFillBase} from "../FastFillBase.sol";
import {Order, OrderLib} from "../libraries/OrderLib.sol";
import {AddressCast} from "../libraries/AddressCast.sol";
import {BurnMessageV2Lib} from "../libraries/BurnMessageV2Lib.sol";
import {PermitLib} from "../libraries/PermitLib.sol";
import {ITokenMessengerV2} from "../interfaces/cctp/ITokenMessengerV2.sol";
import {IMessageTransmitterV2} from "../interfaces/cctp/IMessageTransmitterV2.sol";
import {IFastFillConfig, ChainConfig} from "../interfaces/IFastFillConfig.sol";

/// @title  CctpAdapter
/// @notice Fast-fill adapter for Circle CCTP v2 (USDC). Bidirectional: `initiateCCTP` starts an
///         outbound transfer; `settle` finalizes an inbound one by wrapping `receiveMessage`.
///
///         All chain-specific data (TokenMessenger, USDC, CCTP domains) is resolved at call time
///         from the immutable `config` registry keyed by `block.chainid`, so this contract holds no
///         per-chain configuration and — given identical constructor args — deploys to the same
///         CREATE2 address on every chain. The counterpart adapter is therefore always
///         `address(this)`: a burn names us as both `mintRecipient`/`destinationCaller` and, on
///         settle, must carry `messageSender == address(this)`, which is the anti-forgery check.
contract CctpAdapter is FastFillBase {
    using OrderLib for Order;
    using AddressCast for bytes32;
    using AddressCast for address;

    /// @notice The immutable chain registry. Same address on every chain (CREATE2).
    IFastFillConfig public immutable config;

    /// @notice CCTP v2 `minFinalityThreshold` presets the user picks to opt into a bridging speed.
    ///         `<= FINALITY_FAST` makes the burn eligible for a fast (soft-finality) transfer, which
    ///         charges up to `maxFee`; `FINALITY_FINALIZED` waits for hard finality (set `maxFee = 0`).
    ///         The chosen value is the user's: in the self-submitted path it is their own tx, and in
    ///         the sponsored path it is bound into the Permit2 witness so a relayer cannot change it.
    uint32 public constant FINALITY_FAST = 1000;
    uint32 public constant FINALITY_FINALIZED = 2000;

    error ReceiveMessageFailed();
    error MintRecipientMismatch(bytes32 mintRecipient);
    error UntrustedSourceDomain(uint32 sourceDomain);
    error UntrustedSender(bytes32 messageSender);
    error WrongOutputToken(bytes32 outputToken);
    error MaxFeeTooHigh(uint256 maxFee, uint256 inputAmount);
    error DomainMismatch(uint32 configured, uint32 onchain);

    constructor(address config_, address owner_, uint256 maxFeeRate_) FastFillBase(owner_, maxFeeRate_) {
        config = IFastFillConfig(config_);
    }

    function _bridgeType() internal pure override returns (uint8) {
        return OrderLib.BRIDGE_CCTP;
    }

    function _resolveOutputToken(Order memory order) internal view override returns (address) {
        address local = config.chainConfig(block.chainid).usdc;
        if (order.outputToken != local.toBytes32()) revert WrongOutputToken(order.outputToken);
        return local;
    }

    function _requireSupportedRemote(uint32 chainId) internal view override {
        ChainConfig memory c = config.chainConfig(chainId);
        if (!c.supported || c.usdc == address(0)) revert UnsupportedChain(chainId);
    }

    // ---------------------------------------------------------------------------------------------
    // Source
    // ---------------------------------------------------------------------------------------------

    /// @notice Burn USDC on this chain and create an optimistically-fillable order on `dstChainId`.
    ///         `msg.sender` is the user and the payer (approve this adapter, or batch a `selfPermit`
    ///         via `multicall` for a single transaction).
    /// @param maxFee The max fast-transfer fee. `outputAmount = inputAmount - maxFee` is the
    ///               deterministic worst-case amount the filler is owed (feeExecuted <= maxFee).
    /// @param minFinalityThreshold The bridging speed the user opts into (FINALITY_FAST / FINALITY_FINALIZED).
    /// @param deliveryWindow Seconds until the time premium decays to 0; `expectedDeliveryTime` is
    ///                       derived on-chain as `block.timestamp + deliveryWindow`.
    /// @param discountRate Time-premium accrual per second (WAD); `baseFee` is a flat fee on any fill.
    function initiateCCTP(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 maxFee,
        uint32 minFinalityThreshold,
        uint64 deliveryWindow,
        uint256 discountRate,
        uint256 baseFee
    ) external whenNotPaused returns (bytes32 orderId, uint64 nonce) {
        if (maxFee >= inputAmount) revert MaxFeeTooHigh(maxFee, inputAmount);
        SafeTransferLib.safeTransferFrom(config.chainConfig(block.chainid).usdc, msg.sender, address(this), inputAmount);
        nonce = _nextNonce();
        Order memory order = _buildOrder(
            msg.sender, dstChainId, recipient, inputAmount, maxFee, deliveryWindow, discountRate, baseFee, nonce
        );
        _assertCreatable(order);
        orderId = _finishInitiate(order, maxFee, minFinalityThreshold);
    }

    /// @notice Sponsored initiate: a third party submits, but the USDC is pulled from `from` against
    ///         its Permit2 signature, and `from` is recorded as the order's sender. The signature
    ///         commits to `orderWitness(...)`, so the submitter cannot alter the recipient, amounts,
    ///         timing, pricing, or bridge mode `from` agreed to — the `maxFee` + `minFinalityThreshold`
    ///         the user opted into are bound via `bridgeParams`. This is the on-chain half of a signed
    ///         bridge intent. `deliveryWindow` is relative, so the signed window holds regardless of
    ///         when the relayer submits.
    function initiateCCTPFor(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 maxFee,
        uint32 minFinalityThreshold,
        uint64 deliveryWindow,
        uint256 discountRate,
        uint256 baseFee,
        address from,
        PermitLib.Permit2Data calldata permit
    ) external whenNotPaused returns (bytes32 orderId, uint64 nonce) {
        if (maxFee >= inputAmount) revert MaxFeeTooHigh(maxFee, inputAmount);
        nonce = _nextNonce();
        Order memory order =
            _buildOrder(from, dstChainId, recipient, inputAmount, maxFee, deliveryWindow, discountRate, baseFee, nonce);
        _assertCreatable(order);
        // Pull AFTER building so the witness binds the order; bind the opted-into bridge mode too.
        _pullOrderViaPermit2(order, from, permit, keccak256(abi.encode(maxFee, minFinalityThreshold)));
        orderId = _finishInitiate(order, maxFee, minFinalityThreshold);
    }

    /// @dev Hash, burn, and emit, once the order is built and the USDC is held by this adapter.
    function _finishInitiate(Order memory order, uint256 maxFee, uint32 minFinalityThreshold)
        private
        returns (bytes32 orderId)
    {
        orderId = order.hash();
        _dispatchBurn(order, maxFee, minFinalityThreshold);
        emit OrderCreated(
            orderId,
            OrderLib.BRIDGE_CCTP,
            order.sender.toAddress(),
            order.dstChainId,
            order.outputToken,
            order.outputAmount,
            order.nonce
        );
    }

    /// @dev Split out to keep each stack frame shallow (avoids stack-too-deep). Timing is derived
    ///      fully on-chain: `startTime = block.timestamp`, `expectedDeliveryTime = startTime +
    ///      deliveryWindow` (checked add reverts on overflow).
    function _buildOrder(
        address from,
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 maxFee,
        uint64 deliveryWindow,
        uint256 discountRate,
        uint256 baseFee,
        uint64 nonce
    ) private view returns (Order memory order) {
        address inUsdc = config.chainConfig(block.chainid).usdc;
        address outUsdc = config.chainConfig(dstChainId).usdc;
        if (inUsdc == address(0)) revert UnsupportedChain(uint32(block.chainid));
        if (outUsdc == address(0)) revert UnsupportedChain(dstChainId);
        order = Order({
            bridgeType: OrderLib.BRIDGE_CCTP,
            srcChainId: uint32(block.chainid),
            dstChainId: dstChainId,
            sender: from.toBytes32(),
            recipient: recipient,
            inputToken: inUsdc.toBytes32(),
            outputToken: outUsdc.toBytes32(),
            inputAmount: inputAmount,
            outputAmount: inputAmount - maxFee,
            nonce: nonce,
            startTime: uint64(block.timestamp),
            expectedDeliveryTime: uint64(block.timestamp) + deliveryWindow,
            discountRate: discountRate,
            baseFee: baseFee
        });
    }

    /// @dev Approve the TokenMessenger and burn-with-hook to ourselves on the dst chain. The USDC is
    ///      already held by this adapter (pulled by the entrypoint).
    function _dispatchBurn(Order memory order, uint256 maxFee, uint32 minFinalityThreshold) private {
        ChainConfig memory lc = config.chainConfig(block.chainid);
        _assertLocalDomain(lc.cctpTokenMessenger, lc.cctpDomain);
        uint32 dstDomain = config.chainConfig(order.dstChainId).cctpDomain;
        _burn(
            lc.cctpTokenMessenger,
            lc.usdc,
            order.inputAmount,
            dstDomain,
            maxFee,
            minFinalityThreshold,
            OrderLib.encode(order)
        );
    }

    /// @dev "No room to screw up": the registry's local domain must match the live MessageTransmitter.
    function _assertLocalDomain(address messenger, uint32 expected) private view {
        address transmitter = ITokenMessengerV2(messenger).localMessageTransmitter();
        uint32 onchain = IMessageTransmitterV2(transmitter).localDomain();
        if (onchain != expected) revert DomainMismatch(expected, onchain);
    }

    /// @dev The burn itself, in its own frame to keep the stack shallow. `mintRecipient` and
    ///      `destinationCaller` are both our adapter on the dst chain (the same CREATE2 address).
    function _burn(
        address messenger,
        address usdc,
        uint256 amount,
        uint32 dstDomain,
        uint256 maxFee,
        uint32 minFinalityThreshold,
        bytes memory hookData
    ) private {
        bytes32 self = address(this).toBytes32();
        SafeTransferLib.safeApproveWithRetry(usdc, messenger, amount);
        ITokenMessengerV2(messenger)
            .depositForBurnWithHook(amount, dstDomain, self, usdc, self, maxFee, minFinalityThreshold, hookData);
    }

    // ---------------------------------------------------------------------------------------------
    // Destination settle
    // ---------------------------------------------------------------------------------------------

    /// @notice Finalize an inbound transfer. Anyone may relay; `receiveMessage` enforces that this
    ///         contract is the `destinationCaller`, so the mint and settlement are atomic.
    function settle(bytes calldata message, bytes calldata attestation) external nonReentrant whenNotPaused {
        address messenger = config.chainConfig(block.chainid).cctpTokenMessenger;
        address transmitter = ITokenMessengerV2(messenger).localMessageTransmitter();

        // Mint + authenticate. Reverts (and rolls back) on a bad attestation or a used nonce.
        if (!IMessageTransmitterV2(transmitter).receiveMessage(message, attestation)) revert ReceiveMessageFailed();

        (
            uint32 sourceDomain,
            bytes32 messageSender,
            bytes32 mintRecipient,
            uint256 amount,
            uint256 feeExecuted,
            bytes calldata hookData
        ) = BurnMessageV2Lib.parse(message);

        if (mintRecipient != address(this).toBytes32()) revert MintRecipientMismatch(mintRecipient);

        Order memory order = OrderLib.decode(hookData);

        // The source domain must match the order's claimed source chain (per the registry)...
        ChainConfig memory sc = config.chainConfig(order.srcChainId);
        if (!sc.supported || sc.cctpDomain != sourceDomain) revert UntrustedSourceDomain(sourceDomain);
        // ...and the burn must have been initiated by our adapter on that chain (same address).
        if (messageSender != address(this).toBytes32()) revert UntrustedSender(messageSender);

        uint256 arrived = amount - feeExecuted;
        _settle(order, order.hash(), arrived);
    }
}
