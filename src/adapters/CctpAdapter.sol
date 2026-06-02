// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {FastFillBase} from "../FastFillBase.sol";
import {Order, OrderLib} from "../libraries/OrderLib.sol";
import {AddressCast} from "../libraries/AddressCast.sol";
import {BurnMessageV2Lib} from "../libraries/BurnMessageV2Lib.sol";
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
    /// @param maxFee The max fast-transfer fee. `outputAmount = inputAmount - maxFee` is the
    ///               deterministic worst-case amount the filler is owed (feeExecuted <= maxFee).
    /// @param minFinalityThreshold <= 1000 selects a fast (soft-finality) transfer.
    function initiateCCTP(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 maxFee,
        uint32 minFinalityThreshold,
        uint64 expectedDeliveryTime,
        uint256 discountRate
    ) external whenNotPaused returns (bytes32 orderId, uint64 nonce) {
        if (maxFee >= inputAmount) revert MaxFeeTooHigh(maxFee, inputAmount);

        nonce = _nextNonce();
        Order memory order =
            _buildOrder(dstChainId, recipient, inputAmount, maxFee, expectedDeliveryTime, discountRate, nonce);
        _assertCreatable(order);
        orderId = order.hash();

        _pullAndBurn(order, maxFee, minFinalityThreshold);

        emit OrderCreated(
            orderId, OrderLib.BRIDGE_CCTP, msg.sender, dstChainId, order.outputToken, order.outputAmount, nonce
        );
    }

    /// @dev Split out of `initiateCCTP` to keep each stack frame shallow (avoids stack-too-deep).
    function _buildOrder(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 maxFee,
        uint64 expectedDeliveryTime,
        uint256 discountRate,
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
            sender: msg.sender.toBytes32(),
            recipient: recipient,
            inputToken: inUsdc.toBytes32(),
            outputToken: outUsdc.toBytes32(),
            inputAmount: inputAmount,
            outputAmount: inputAmount - maxFee,
            nonce: nonce,
            startTime: uint64(block.timestamp),
            expectedDeliveryTime: expectedDeliveryTime,
            discountRate: discountRate
        });
    }

    /// @dev Pull USDC, approve the TokenMessenger, and burn-with-hook to ourselves on the dst chain.
    function _pullAndBurn(Order memory order, uint256 maxFee, uint32 minFinalityThreshold) private {
        ChainConfig memory lc = config.chainConfig(block.chainid);
        _assertLocalDomain(lc.cctpTokenMessenger, lc.cctpDomain);

        uint32 dstDomain = config.chainConfig(order.dstChainId).cctpDomain;
        SafeTransferLib.safeTransferFrom(lc.usdc, msg.sender, address(this), order.inputAmount);
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
