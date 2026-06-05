// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {FastFillBase} from "../FastFillBase.sol";
import {Order, OrderLib, Execution} from "../libraries/OrderLib.sol";
import {ExecHook, ExecHookLib} from "../libraries/ExecHookLib.sol";
import {AddressCast} from "../libraries/AddressCast.sol";
import {BurnMessageV2Lib} from "../libraries/BurnMessageV2Lib.sol";
import {PermitLib} from "../libraries/PermitLib.sol";
import {ITokenMessengerV2} from "../interfaces/cctp/ITokenMessengerV2.sol";
import {IMessageTransmitterV2} from "../interfaces/cctp/IMessageTransmitterV2.sol";
import {ICctpExecReceiver} from "../interfaces/ICctpExecReceiver.sol";
import {IFastFillConfig, ChainConfig} from "../interfaces/IFastFillConfig.sol";

/// @title  CctpAdapter
/// @notice Fast-fill adapter for Circle CCTP v2 (USDC). Bidirectional: `initiateCCTP` starts an
///         outbound transfer; settlement finalizes an inbound one.
///
///         There are two settlement paths, chosen at burn time by the order's `mintFee`:
///         • `mintFee == 0` (direct): the burn names this adapter as both `mintRecipient` and
///           `destinationCaller`, and anyone calls `settle(message, attestation)` — `receiveMessage`
///           mints to us and we run `_settle` atomically.
///         • `mintFee > 0` (routed): the burn names the canonical `CctpExecutor` as `mintRecipient` and
///           `destinationCaller`. A relayer calls `CctpExecutor.execute`, which mints, pays the relayer
///           `mintFee`, forwards the rest here, and calls `onCctpExecute` — the routed analog of
///           `settle`. This gives permissionless, incentivized destination delivery without a keeper.
///
///         This chain's bridge addresses (USDC, the CCTP TokenMessenger + MessageTransmitter) are read
///         from the immutable `config` registry ONCE at construction and cached as immutables; remote-
///         chain data (the destination burn domain, the source-chain auth) still resolves per call.
///         Because the locals are read from `config` (not constructor args), the init-code — and thus
///         the CREATE2 address — is identical on every chain, so the counterpart adapter is always
///         `address(this)`: a burn carries `messageSender == address(this)`, the anti-forgery check.
contract CctpAdapter is FastFillBase, ICctpExecReceiver {
    using OrderLib for Order;
    using AddressCast for bytes32;
    using AddressCast for address;

    /// @notice The immutable chain registry. Same address on every chain (CREATE2).
    IFastFillConfig public immutable config;

    /// @notice The canonical `CctpExecutor` (same CREATE2 address on every chain). Routed orders
    ///         (`mintFee > 0`) name it as `mintRecipient`/`destinationCaller`; it calls `onCctpExecute`.
    address public immutable cctpExecutor;

    /// @notice This chain's CCTP locals, resolved from the registry ONCE at construction and cached, so
    ///         the hot paths (`fill`/`settle`) don't pay a cold registry view call (and a TokenMessenger
    ///         call) each time. The registry is immutable and this adapter is pinned to one chain at
    ///         deploy; reading them in the constructor (not from args) keeps the CREATE2 init-code — and
    ///         thus the deterministic address — identical across chains. REMOTE-chain config (the dst
    ///         burn domain, src-chain auth) still resolves dynamically from `config`, as it varies per order.
    address public immutable usdc; // local USDC (the CCTP burn/mint token)
    address public immutable cctpTokenMessenger; // local CCTP v2 TokenMessenger
    address public immutable cctpTransmitter; // local CCTP v2 MessageTransmitter (derived from the messenger)

    /// @notice CCTP v2 `minFinalityThreshold` presets the user picks to opt into a bridging speed.
    ///         `<= FINALITY_FAST` makes the burn eligible for a fast (soft-finality) transfer, which
    ///         charges up to `maxFee`; `FINALITY_FINALIZED` waits for hard finality (set `maxFee = 0`).
    ///         The chosen value is the user's: in the self-submitted path it is their own tx, and in
    ///         the sponsored path it is bound into the Permit2 witness so a relayer cannot change it.
    uint32 public constant FINALITY_FAST = 1000;
    uint32 public constant FINALITY_FINALIZED = 2000;

    /// @dev Extra gas added to the order's `callbackGasLimit` when sizing the executor's hook budget for
    ///      a routed order, covering `onCctpExecute`'s authentication + `_settle` + the nested
    ///      delivery's own 63/64 buffer. The executor enforces the relayer actually forwards this.
    uint64 internal constant SETTLE_GAS_OVERHEAD = 350_000;

    error ReceiveMessageFailed();
    error MintRecipientMismatch(bytes32 mintRecipient);
    error UntrustedSourceDomain(uint32 sourceDomain);
    error UntrustedSender(bytes32 messageSender);
    error UntrustedExecutor(address caller);
    error WrongOutputToken(bytes32 outputToken);
    error MaxFeeTooHigh(uint256 maxFee, uint256 inputAmount);
    error MintFeeTooHigh(uint256 mintFee, uint256 maxOutput);
    error ZeroCctpExecutor();
    error DomainMismatch(uint32 configured, uint32 onchain);

    constructor(address config_, address owner_, uint256 maxFeeRate_, address cctpExecutor_)
        FastFillBase(owner_, maxFeeRate_)
    {
        if (cctpExecutor_ == address(0)) revert ZeroCctpExecutor();
        IFastFillConfig cfg = IFastFillConfig(config_);
        config = cfg;
        cctpExecutor = cctpExecutor_;

        // Resolve + cache this chain's CCTP locals once, and cross-check the registry's domain against
        // the live MessageTransmitter — a wrong constant now reverts at DEPLOY, not on first use.
        ChainConfig memory lc = cfg.chainConfig(block.chainid);
        if (lc.usdc == address(0)) revert UnsupportedChain(uint32(block.chainid));
        address transmitter = ITokenMessengerV2(lc.cctpTokenMessenger).localMessageTransmitter();
        uint32 onchainDomain = IMessageTransmitterV2(transmitter).localDomain();
        if (onchainDomain != lc.cctpDomain) revert DomainMismatch(lc.cctpDomain, onchainDomain);
        usdc = lc.usdc;
        cctpTokenMessenger = lc.cctpTokenMessenger;
        cctpTransmitter = transmitter;
    }

    function _bridgeType() internal pure override returns (uint8) {
        return OrderLib.BRIDGE_CCTP;
    }

    function _resolveOutputToken(Order memory order) internal view override returns (address) {
        if (order.outputToken != usdc.toBytes32()) revert WrongOutputToken(order.outputToken);
        return usdc;
    }

    // ---------------------------------------------------------------------------------------------
    // Source
    // ---------------------------------------------------------------------------------------------

    /// @notice Burn USDC on this chain and create an optimistically-fillable order on `dstChainId`.
    ///         `msg.sender` is the user and the payer (approve this adapter, or batch a `selfPermit`
    ///         via `multicall` for a single transaction).
    /// @param recipient Destination EVM address encoded as bytes32 (upper 12 bytes zero).
    /// @param maxFee The max CCTP fast-transfer fee (Circle's transport cost).
    /// @param mintFee USDC paid to whoever relays the destination mint. `0` keeps the order on the
    ///               direct `settle()` path (no relay incentive); `> 0` routes it through the
    ///               CctpExecutor so any relayer can earn `mintFee` for delivering it.
    ///               `outputAmount = inputAmount - maxFee - mintFee` is the deterministic worst-case
    ///               amount the filler is owed (feeExecuted <= maxFee, so the arrived amount net of
    ///               mintFee is always >= outputAmount).
    /// @param minFinalityThreshold The bridging speed the user opts into (FINALITY_FAST / FINALITY_FINALIZED).
    /// @param deliveryWindow Seconds until the time premium decays to 0; `expectedDeliveryTime` is
    ///                       derived on-chain as `block.timestamp + deliveryWindow`.
    /// @param discountRate Time-premium accrual per second (WAD); `baseFee` is a flat fee on any fill.
    /// @param exec Optional destination execution: `gasLimit` (max 5,000,000) forwarded to the
    ///             recipient's `onFastFill` hook + the `data` payload. Empty `data` = deliver funds only, no callback.
    function initiateCCTP(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 maxFee,
        uint256 mintFee,
        uint32 minFinalityThreshold,
        uint64 deliveryWindow,
        uint256 discountRate,
        uint256 baseFee,
        Execution calldata exec
    ) external whenNotPaused returns (bytes32 orderId, uint64 nonce) {
        if (maxFee >= inputAmount) revert MaxFeeTooHigh(maxFee, inputAmount);
        if (mintFee >= inputAmount - maxFee) revert MintFeeTooHigh(mintFee, inputAmount - maxFee);
        nonce = _nextNonce();
        ChainConfig memory dc = config.chainConfig(dstChainId); // single destination-config read
        if (!dc.supported || dc.usdc == address(0)) revert UnsupportedChain(dstChainId);
        Order memory order = _buildOrder(
            msg.sender,
            dstChainId,
            recipient,
            inputAmount,
            maxFee,
            mintFee,
            deliveryWindow,
            discountRate,
            baseFee,
            nonce,
            dc.usdc
        );
        order.callbackGasLimit = exec.gasLimit;
        order.hookData = exec.data;
        _assertCreatable(order);
        SafeTransferLib.safeTransferFrom(usdc, msg.sender, address(this), inputAmount);
        orderId = _finishInitiate(order, maxFee, mintFee, minFinalityThreshold, dc.cctpDomain);
    }

    /// @notice Sponsored initiate: a third party submits, but the USDC is pulled from `from` against
    ///         its Permit2 signature, and `from` is recorded as the order's sender. The signature
    ///         commits to `orderWitness(...)`, so the submitter cannot alter the recipient, amounts,
    ///         timing, pricing, or bridge mode `from` agreed to — `maxFee`, `minFinalityThreshold`, and
    ///         `mintFee` are bound via `bridgeParams`. This is the on-chain half of a signed bridge
    ///         intent. `deliveryWindow` is relative, so the signed window holds regardless of when the
    ///         relayer submits.
    function initiateCCTPFor(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 maxFee,
        uint256 mintFee,
        uint32 minFinalityThreshold,
        uint64 deliveryWindow,
        uint256 discountRate,
        uint256 baseFee,
        Execution calldata exec,
        address from,
        PermitLib.Permit2Data calldata permit
    ) external whenNotPaused returns (bytes32 orderId, uint64 nonce) {
        if (maxFee >= inputAmount) revert MaxFeeTooHigh(maxFee, inputAmount);
        if (mintFee >= inputAmount - maxFee) revert MintFeeTooHigh(mintFee, inputAmount - maxFee);
        nonce = _nextNonce();
        ChainConfig memory dc = config.chainConfig(dstChainId); // single destination-config read
        if (!dc.supported || dc.usdc == address(0)) revert UnsupportedChain(dstChainId);
        Order memory order = _buildOrder(
            from,
            dstChainId,
            recipient,
            inputAmount,
            maxFee,
            mintFee,
            deliveryWindow,
            discountRate,
            baseFee,
            nonce,
            dc.usdc
        );
        order.callbackGasLimit = exec.gasLimit;
        order.hookData = exec.data;
        _assertCreatable(order);
        // Pull AFTER building so the witness binds the order (incl. hookData + gas); bind the bridge mode too.
        _pullOrderViaPermit2(order, from, permit, keccak256(abi.encode(maxFee, minFinalityThreshold, mintFee)));
        orderId = _finishInitiate(order, maxFee, mintFee, minFinalityThreshold, dc.cctpDomain);
    }

    /// @dev Hash, burn, and emit, once the order is built and the USDC is held by this adapter.
    function _finishInitiate(
        Order memory order,
        uint256 maxFee,
        uint256 mintFee,
        uint32 minFinalityThreshold,
        uint32 dstDomain
    ) private returns (bytes32 orderId) {
        orderId = order.hash();
        _dispatchBurn(order, maxFee, mintFee, minFinalityThreshold, dstDomain);
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
    ///      deliveryWindow` (checked add reverts on overflow). `outputAmount` reserves both the CCTP
    ///      `maxFee` and the `mintFee` up front, so the filler's worst-case payout is what arrives net
    ///      of every transport cost.
    function _buildOrder(
        address from,
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 maxFee,
        uint256 mintFee,
        uint64 deliveryWindow,
        uint256 discountRate,
        uint256 baseFee,
        uint64 nonce,
        address dstUsdc
    ) private view returns (Order memory order) {
        order = Order({
            bridgeType: OrderLib.BRIDGE_CCTP,
            srcChainId: uint32(block.chainid),
            dstChainId: dstChainId,
            sender: from.toBytes32(),
            recipient: recipient,
            inputToken: usdc.toBytes32(), // local USDC, cached at construction
            outputToken: dstUsdc.toBytes32(), // destination USDC, from the single dst-config read
            inputAmount: inputAmount,
            outputAmount: inputAmount - maxFee - mintFee,
            nonce: nonce,
            startTime: uint64(block.timestamp),
            expectedDeliveryTime: uint64(block.timestamp) + deliveryWindow,
            discountRate: discountRate,
            baseFee: baseFee,
            callbackGasLimit: 0, // set by the entrypoint after build (keeps this frame's stack shallow)
            hookData: ""
        });
    }

    /// @dev Approve the TokenMessenger and burn-with-hook. `mintFee == 0` targets this adapter directly
    ///      (legacy `settle` path); `mintFee > 0` targets the CctpExecutor with an `ExecHook` envelope.
    function _dispatchBurn(
        Order memory order,
        uint256 maxFee,
        uint256 mintFee,
        uint32 minFinalityThreshold,
        uint32 dstDomain
    ) private {
        if (mintFee == 0) {
            // Direct path: this adapter is mintRecipient + destinationCaller; `settle` consumes the
            // message and runs `_settle` atomically.
            _burn(
                cctpTokenMessenger,
                usdc,
                order.inputAmount,
                dstDomain,
                address(this).toBytes32(),
                maxFee,
                minFinalityThreshold,
                OrderLib.encode(order)
            );
        } else {
            // Routed path: the CctpExecutor is mintRecipient + destinationCaller. It pays the relayer
            // `mintFee` and calls `onCctpExecute(target = this)`, which settles. `gasLimit` covers the
            // settle work plus the recipient's own hook; `refundTo` parks funds with the recipient if
            // `onCctpExecute` ever reverts.
            uint64 routedGasLimit = order.callbackGasLimit + SETTLE_GAS_OVERHEAD;
            if (routedGasLimit > MAX_CALLBACK_GAS_LIMIT) {
                revert InvalidCallbackGasLimit(routedGasLimit, MAX_CALLBACK_GAS_LIMIT);
            }
            ExecHook memory h = ExecHook({
                mintFee: mintFee,
                target: address(this).toBytes32(),
                gasLimit: routedGasLimit,
                refundTo: order.recipient,
                payload: OrderLib.encode(order)
            });
            _burn(
                cctpTokenMessenger,
                usdc,
                order.inputAmount,
                dstDomain,
                cctpExecutor.toBytes32(),
                maxFee,
                minFinalityThreshold,
                ExecHookLib.encode(h)
            );
        }
    }

    /// @dev The burn itself, in its own frame to keep the stack shallow. `target` is used for BOTH
    ///      `mintRecipient` and `destinationCaller` on the dst chain (this adapter, or the executor).
    function _burn(
        address messenger,
        address token,
        uint256 amount,
        uint32 dstDomain,
        bytes32 target,
        uint256 maxFee,
        uint32 minFinalityThreshold,
        bytes memory hookData
    ) private {
        SafeTransferLib.safeApproveWithRetry(token, messenger, amount);
        ITokenMessengerV2(messenger)
            .depositForBurnWithHook(amount, dstDomain, target, token, target, maxFee, minFinalityThreshold, hookData);
    }

    // ---------------------------------------------------------------------------------------------
    // Destination settle
    // ---------------------------------------------------------------------------------------------

    /// @notice Finalize an inbound DIRECT transfer (`mintFee == 0`). Anyone may relay; `receiveMessage`
    ///         enforces that this contract is the `destinationCaller`, so the mint and settlement are
    ///         atomic. (Routed `mintFee > 0` transfers arrive via `onCctpExecute` instead.)
    function settle(bytes calldata message, bytes calldata attestation) external nonReentrant whenNotPaused {
        // Mint + authenticate. Reverts (and rolls back) on a bad attestation or a used nonce. The
        // MessageTransmitter was resolved + cross-checked once at construction.
        if (!IMessageTransmitterV2(cctpTransmitter).receiveMessage(message, attestation)) {
            revert ReceiveMessageFailed();
        }

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
        _authenticateOrder(sourceDomain, messageSender, order);

        uint256 arrived = amount - feeExecuted;
        _settle(order, order.hash(), arrived);
    }

    /// @notice Finalize an inbound ROUTED transfer (`mintFee > 0`). Called by the CctpExecutor after it
    ///         has minted, taken the relayer's `mintFee`, and transferred `amount` USDC to this adapter.
    ///         This is the routed analog of `settle`, authenticated identically: only the executor may
    ///         call, and the order must carry our own address as the CCTP `sender` (anti-forgery).
    /// @dev Deliberately NOT `whenNotPaused`. The executor has already consumed the CCTP message and
    ///      sent us the USDC; a revert here would make the executor re-route the funds to the envelope's
    ///      `refundTo` and strand an optimistic filler. Settlement of an already-bridged routed order
    ///      must always complete. Pausing still blocks new initiates/fills and the direct `settle`.
    function onCctpExecute(uint32 sourceDomain, bytes32 sender, address token, uint256 amount, bytes calldata payload)
        external
        override
        nonReentrant
    {
        if (msg.sender != cctpExecutor) revert UntrustedExecutor(msg.sender);
        if (token != usdc) revert WrongOutputToken(token.toBytes32());
        Order memory order = OrderLib.decode(payload);
        _authenticateOrder(sourceDomain, sender, order);
        _settle(order, order.hash(), amount);
    }

    /// @dev Authenticate a settling order against the CCTP message's authenticated fields: the source
    ///      domain must match the order's claimed source chain (per the registry), and the burn must
    ///      have been initiated by our adapter on that chain (same CREATE2 address). The latter is the
    ///      anti-forgery check — only we can produce a message whose CCTP `messageSender` is our
    ///      address. Shared by `settle` and `onCctpExecute`.
    function _authenticateOrder(uint32 sourceDomain, bytes32 messageSender, Order memory order) private view {
        ChainConfig memory sc = config.chainConfig(order.srcChainId);
        if (!sc.supported || sc.cctpDomain != sourceDomain) revert UntrustedSourceDomain(sourceDomain);
        if (messageSender != address(this).toBytes32()) revert UntrustedSender(messageSender);
    }
}
