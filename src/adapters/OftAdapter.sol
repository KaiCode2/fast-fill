// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {FastFillBase} from "../FastFillBase.sol";
import {Order, OrderLib, Execution} from "../libraries/OrderLib.sol";
import {AddressCast} from "../libraries/AddressCast.sol";
import {OFTComposeMsgCodec} from "../libraries/OFTComposeMsgCodec.sol";
import {PermitLib} from "../libraries/PermitLib.sol";
import {ILayerZeroComposer} from "../interfaces/layerzero/ILayerZeroComposer.sol";
import {ILayerZeroEndpointV2} from "../interfaces/layerzero/ILayerZeroEndpointV2.sol";
import {IOFT, SendParam, MessagingFee} from "../interfaces/layerzero/IOFT.sol";
import {IFastFillConfig, ChainConfig, OftDeployment} from "../interfaces/IFastFillConfig.sol";

/// @title  OftAdapter
/// @notice Fast-fill adapter for a single LayerZero v2 OFT, selected at deploy time by `oftId` (see
///         `OftId`: USD₮0, USDe, sUSDe, ENA, USDtb, …). Bidirectional: `initiateOFT` starts an
///         outbound transfer; `lzCompose` finalizes an inbound one.
///
///         This chain's OFT entrypoint + token + LZ endpoint are read from the immutable `config`
///         registry keyed by `(block.chainid, oftId)` ONCE at construction — cross-checked there against
///         the live OFT (token + endpoint eid) — and cached as immutables; remote-chain data (the dst
///         send eid, the src-chain auth) still resolves per call. Reading the locals from `config` (not
///         constructor args) keeps the per-`oftId` CREATE2 address identical on every chain, so the
///         counterpart is always `address(this)`: `lzCompose` requires `composeFrom == address(this)`.
///         Each `oftId` is a distinct deployment with its own address and its own isolated reimbursement pool.
contract OftAdapter is FastFillBase, ILayerZeroComposer {
    using OrderLib for Order;
    using AddressCast for bytes32;
    using AddressCast for address;

    /// @notice The immutable chain registry. Same address on every chain (CREATE2).
    IFastFillConfig public immutable config;

    /// @notice The OFT this adapter handles (see `OftId`). Immutable: it keys every `oftConfig`
    ///         lookup and is baked into the adapter's CREATE2 salt, so one adapter == one OFT.
    uint8 public immutable oftId;

    /// @notice This chain's OFT locals (entrypoint, ERC20, LayerZero endpoint), resolved from the
    ///         registry ONCE at construction and cached, so `fill`/`lzCompose` don't pay a cold registry
    ///         view call (and OFT `token()`/`endpoint()` calls) each time. The registry is immutable and
    ///         this adapter is pinned to one chain at deploy; reading them in the constructor (not from
    ///         args) keeps the per-`oftId` CREATE2 address identical across chains. REMOTE-chain config
    ///         (the dst send eid, src-chain auth) still resolves dynamically from `config`, per order.
    address public immutable localOft;
    address public immutable localToken;
    address public immutable localEndpoint;

    error NotEndpoint(address caller);
    error UntrustedLocalOFT(address from);
    error UntrustedPeer(bytes32 composeFrom);
    error UntrustedSourceEid(uint32 srcEid);
    error WrongOutputToken(bytes32 outputToken);
    error TokenMismatch(address onchain, address configured);
    error EidMismatch(uint32 configured, uint32 onchain);

    constructor(address config_, address owner_, uint256 maxFeeRate_, uint8 oftId_) FastFillBase(owner_, maxFeeRate_) {
        IFastFillConfig cfg = IFastFillConfig(config_);
        config = cfg;
        oftId = oftId_;

        // Resolve + cache this chain's OFT locals once, and cross-check the live OFT (token + endpoint
        // eid) against the registry — wrong constants now revert at DEPLOY, not on first use.
        OftDeployment memory d = cfg.oftConfig(block.chainid, oftId_);
        if (d.oft == address(0) || d.token == address(0)) revert UnsupportedChain(uint32(block.chainid));
        address onchainToken = IOFT(d.oft).token();
        if (onchainToken != d.token) revert TokenMismatch(onchainToken, d.token);
        address endpoint = IOFT(d.oft).endpoint();
        uint32 cfgEid = cfg.chainConfig(block.chainid).lzEid;
        uint32 onchainEid = ILayerZeroEndpointV2(endpoint).eid();
        if (onchainEid != cfgEid) revert EidMismatch(cfgEid, onchainEid);
        localOft = d.oft;
        localToken = d.token;
        localEndpoint = endpoint;
    }

    function _bridgeType() internal pure override returns (uint8) {
        return OrderLib.BRIDGE_OFT;
    }

    function _resolveOutputToken(Order memory order) internal view override returns (address) {
        if (order.outputToken != localToken.toBytes32()) revert WrongOutputToken(order.outputToken);
        return localToken;
    }

    function _requireSupportedRemote(uint32 chainId) internal view override {
        ChainConfig memory c = config.chainConfig(chainId);
        OftDeployment memory d = config.oftConfig(chainId, oftId);
        if (!c.supported || d.oft == address(0) || d.token == address(0)) revert UnsupportedChain(chainId);
    }

    // ---------------------------------------------------------------------------------------------
    // Source
    // ---------------------------------------------------------------------------------------------

    /// @notice Send the OFT token to `dstChainId` and create an optimistically-fillable order.
    ///         `msg.sender` is the user and the payer (approve this adapter for the token).
    /// @dev `msg.value` pays the LayerZero native messaging fee (size it off-chain via quoteSend).
    ///      `extraOptions` is the user's chosen executor/DVN configuration — typically it must include
    ///      a compose-gas allowance (OptionsBuilder.addExecutorLzComposeOption) so the LayerZero
    ///      executor auto-delivers `lzCompose` (settlement). It is the user's per-tx opt-in to executor
    ///      behaviour; in the sponsored path it is bound into the Permit2 witness (relayer can't change it).
    /// @param recipient Destination EVM address encoded as bytes32 (upper 12 bytes zero).
    /// @param minAmountLD The slippage floor, used as the deterministic `outputAmount` the filler is owed.
    /// @param deliveryWindow Seconds until the time premium decays to 0; `expectedDeliveryTime` is
    ///                       derived on-chain as `block.timestamp + deliveryWindow`.
    /// @param discountRate Time-premium accrual per second (WAD); `baseFee` is a flat fee on any fill.
    /// @param exec Optional destination execution: `gasLimit` (max 5,000,000) forwarded to the
    ///             recipient's `onFastFill` hook + the `data` payload. Empty `data` = deliver funds only, no callback.
    function initiateOFT(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 minAmountLD,
        bytes calldata extraOptions,
        uint64 deliveryWindow,
        uint256 discountRate,
        uint256 baseFee,
        Execution calldata exec
    ) external payable whenNotPaused returns (bytes32 orderId, uint64 nonce) {
        nonce = _nextNonce();
        Order memory order = _buildOrder(
            msg.sender, dstChainId, recipient, inputAmount, minAmountLD, deliveryWindow, discountRate, baseFee, nonce
        );
        order.callbackGasLimit = exec.gasLimit;
        order.hookData = exec.data;
        _assertCreatable(order);
        SafeTransferLib.safeTransferFrom(localToken, msg.sender, address(this), inputAmount);
        orderId = _finishInitiate(order, extraOptions);
    }

    /// @notice Sponsored initiate: a third party submits and pays the LayerZero fee (`msg.value`),
    ///         while the token is pulled from `from` against its Permit2 signature and `from` is
    ///         recorded as the order's sender. The signature commits to `orderWitness(...)`, so the
    ///         submitter cannot alter what `from` agreed to — including the executor configuration,
    ///         which is bound via `bridgeParams = keccak256(extraOptions)`. `deliveryWindow` is
    ///         relative, so the signed window holds regardless of when the relayer submits.
    function initiateOFTFor(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 minAmountLD,
        bytes calldata extraOptions,
        uint64 deliveryWindow,
        uint256 discountRate,
        uint256 baseFee,
        Execution calldata exec,
        address from,
        PermitLib.Permit2Data calldata permit
    ) external payable whenNotPaused returns (bytes32 orderId, uint64 nonce) {
        nonce = _nextNonce();
        Order memory order = _buildOrder(
            from, dstChainId, recipient, inputAmount, minAmountLD, deliveryWindow, discountRate, baseFee, nonce
        );
        order.callbackGasLimit = exec.gasLimit;
        order.hookData = exec.data;
        _assertCreatable(order);
        // Pull AFTER building so the witness binds the order (incl. hookData + gas); bind the executor config too.
        _pullOrderViaPermit2(order, from, permit, keccak256(extraOptions));
        orderId = _finishInitiate(order, extraOptions);
    }

    /// @dev Hash, send, and emit, once the order is built and the token is held by this adapter.
    function _finishInitiate(Order memory order, bytes calldata extraOptions) private returns (bytes32 orderId) {
        orderId = order.hash();
        _dispatchSend(order, extraOptions);
        emit OrderCreated(
            orderId,
            OrderLib.BRIDGE_OFT,
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
        uint256 minAmountLD,
        uint64 deliveryWindow,
        uint256 discountRate,
        uint256 baseFee,
        uint64 nonce
    ) private view returns (Order memory order) {
        address inToken = localToken; // local, cached at construction (guaranteed non-zero)
        address outToken = config.oftConfig(dstChainId, oftId).token; // remote, per-order
        if (outToken == address(0)) revert UnsupportedChain(dstChainId);
        order = Order({
            bridgeType: OrderLib.BRIDGE_OFT,
            srcChainId: uint32(block.chainid),
            dstChainId: dstChainId,
            sender: from.toBytes32(),
            recipient: recipient,
            inputToken: inToken.toBytes32(),
            outputToken: outToken.toBytes32(),
            inputAmount: inputAmount,
            outputAmount: minAmountLD,
            nonce: nonce,
            startTime: uint64(block.timestamp),
            expectedDeliveryTime: uint64(block.timestamp) + deliveryWindow,
            discountRate: discountRate,
            baseFee: baseFee,
            callbackGasLimit: 0, // set by the entrypoint after build (keeps this frame's stack shallow)
            hookData: ""
        });
    }

    /// @dev Approve the OFT and dispatch the cross-chain send to ourselves on dst. The token is
    ///      already held by this adapter (pulled by the entrypoint).
    function _dispatchSend(Order memory order, bytes calldata extraOptions) private {
        // Local OFT + token are cached at construction (where the live token/eid were cross-checked).
        SafeTransferLib.safeApproveWithRetry(localToken, localOft, order.inputAmount);

        SendParam memory sendParam = SendParam({
            dstEid: config.chainConfig(order.dstChainId).lzEid, // remote, per-order
            to: address(this).toBytes32(), // our adapter on the dst chain (same address)
            amountLD: order.inputAmount,
            minAmountLD: order.outputAmount,
            extraOptions: extraOptions,
            composeMsg: OrderLib.encode(order),
            oftCmd: ""
        });
        IOFT(localOft).send{value: msg.value}(
            sendParam, MessagingFee({nativeFee: msg.value, lzTokenFee: 0}), msg.sender
        );
    }

    // ---------------------------------------------------------------------------------------------
    // Destination settle (LayerZero compose callback)
    // ---------------------------------------------------------------------------------------------

    /// @inheritdoc ILayerZeroComposer
    function lzCompose(address from, bytes32, bytes calldata message, address, bytes calldata)
        external
        payable
        nonReentrant
        whenNotPaused
    {
        // Local OFT + endpoint are cached at construction.
        if (msg.sender != localEndpoint) revert NotEndpoint(msg.sender);
        if (from != localOft) revert UntrustedLocalOFT(from);

        uint32 srcEid = OFTComposeMsgCodec.srcEid(message);
        bytes32 composeFrom = OFTComposeMsgCodec.composeFrom(message);
        uint256 arrived = OFTComposeMsgCodec.amountLD(message);
        Order memory order = OrderLib.decode(OFTComposeMsgCodec.composeMsg(message));

        // The source eid must match the order's claimed source chain (per the registry)...
        ChainConfig memory sc = config.chainConfig(order.srcChainId);
        if (!sc.supported || sc.lzEid != srcEid) revert UntrustedSourceEid(srcEid);
        // ...and the compose must originate from our adapter on that chain (same address).
        if (composeFrom != address(this).toBytes32()) revert UntrustedPeer(composeFrom);
        if (block.chainid != order.dstChainId) revert WrongDestinationChain(order.dstChainId);

        // The OFT already credited `arrived` tokens to this contract during _lzReceive.
        _settle(order, order.hash(), arrived);
    }
}
