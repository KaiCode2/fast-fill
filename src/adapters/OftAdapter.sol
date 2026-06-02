// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {FastFillBase} from "../FastFillBase.sol";
import {Order, OrderLib} from "../libraries/OrderLib.sol";
import {AddressCast} from "../libraries/AddressCast.sol";
import {OFTComposeMsgCodec} from "../libraries/OFTComposeMsgCodec.sol";
import {PermitLib} from "../libraries/PermitLib.sol";
import {ILayerZeroComposer} from "../interfaces/layerzero/ILayerZeroComposer.sol";
import {ILayerZeroEndpointV2} from "../interfaces/layerzero/ILayerZeroEndpointV2.sol";
import {IOFT, SendParam, MessagingFee} from "../interfaces/layerzero/IOFT.sol";
import {IFastFillConfig, ChainConfig} from "../interfaces/IFastFillConfig.sol";

/// @title  OftAdapter
/// @notice Fast-fill adapter for a LayerZero v2 OFT (USD₮0). Bidirectional: `initiateOFT` starts an
///         outbound transfer; `lzCompose` finalizes an inbound one.
///
///         All chain-specific data (the OFT, its token, the LZ eid) is resolved at call time from
///         the immutable `config` registry keyed by `block.chainid` — the endpoint and token are in
///         turn read live from the OFT and cross-checked against the registry. The contract holds no
///         per-chain configuration and deploys to one CREATE2 address on every chain, so the
///         counterpart is always `address(this)`: `lzCompose` requires `composeFrom == address(this)`.
contract OftAdapter is FastFillBase, ILayerZeroComposer {
    using OrderLib for Order;
    using AddressCast for bytes32;
    using AddressCast for address;

    /// @notice The immutable chain registry. Same address on every chain (CREATE2).
    IFastFillConfig public immutable config;

    error NotEndpoint(address caller);
    error UntrustedLocalOFT(address from);
    error UntrustedPeer(bytes32 composeFrom);
    error UntrustedSourceEid(uint32 srcEid);
    error WrongOutputToken(bytes32 outputToken);
    error TokenMismatch(address onchain, address configured);
    error EidMismatch(uint32 configured, uint32 onchain);

    constructor(address config_, address owner_, uint256 maxFeeRate_) FastFillBase(owner_, maxFeeRate_) {
        config = IFastFillConfig(config_);
    }

    function _bridgeType() internal pure override returns (uint8) {
        return OrderLib.BRIDGE_OFT;
    }

    function _resolveOutputToken(Order memory order) internal view override returns (address) {
        address local = config.chainConfig(block.chainid).usdt0Token;
        if (order.outputToken != local.toBytes32()) revert WrongOutputToken(order.outputToken);
        return local;
    }

    function _requireSupportedRemote(uint32 chainId) internal view override {
        ChainConfig memory c = config.chainConfig(chainId);
        if (!c.supported || c.usdt0Oft == address(0) || c.usdt0Token == address(0)) revert UnsupportedChain(chainId);
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
    /// @param minAmountLD The slippage floor, used as the deterministic `outputAmount` the filler is owed.
    /// @param deliveryWindow Seconds until the time premium decays to 0; `expectedDeliveryTime` is
    ///                       derived on-chain as `block.timestamp + deliveryWindow`.
    /// @param discountRate Time-premium accrual per second (WAD); `baseFee` is a flat fee on any fill.
    function initiateOFT(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 minAmountLD,
        bytes calldata extraOptions,
        uint64 deliveryWindow,
        uint256 discountRate,
        uint256 baseFee
    ) external payable whenNotPaused returns (bytes32 orderId, uint64 nonce) {
        SafeTransferLib.safeTransferFrom(
            config.chainConfig(block.chainid).usdt0Token, msg.sender, address(this), inputAmount
        );
        nonce = _nextNonce();
        Order memory order = _buildOrder(
            msg.sender, dstChainId, recipient, inputAmount, minAmountLD, deliveryWindow, discountRate, baseFee, nonce
        );
        _assertCreatable(order);
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
        address from,
        PermitLib.Permit2Data calldata permit
    ) external payable whenNotPaused returns (bytes32 orderId, uint64 nonce) {
        nonce = _nextNonce();
        Order memory order = _buildOrder(
            from, dstChainId, recipient, inputAmount, minAmountLD, deliveryWindow, discountRate, baseFee, nonce
        );
        _assertCreatable(order);
        // Pull AFTER building so the witness binds the order; bind the opted-into executor config too.
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
        address inToken = config.chainConfig(block.chainid).usdt0Token;
        address outToken = config.chainConfig(dstChainId).usdt0Token;
        if (inToken == address(0)) revert UnsupportedChain(uint32(block.chainid));
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
            baseFee: baseFee
        });
    }

    /// @dev Approve the OFT and dispatch the cross-chain send to ourselves on dst. The token is
    ///      already held by this adapter (pulled by the entrypoint).
    function _dispatchSend(Order memory order, bytes calldata extraOptions) private {
        ChainConfig memory lc = config.chainConfig(block.chainid);
        IOFT oftLocal = IOFT(lc.usdt0Oft);

        // "No room to screw up": the live OFT's token + endpoint eid must match the registry.
        address inToken = oftLocal.token();
        if (inToken != lc.usdt0Token) revert TokenMismatch(inToken, lc.usdt0Token);
        uint32 onchainEid = ILayerZeroEndpointV2(oftLocal.endpoint()).eid();
        if (onchainEid != lc.lzEid) revert EidMismatch(lc.lzEid, onchainEid);

        SafeTransferLib.safeApproveWithRetry(inToken, address(oftLocal), order.inputAmount);

        SendParam memory sendParam = SendParam({
            dstEid: config.chainConfig(order.dstChainId).lzEid,
            to: address(this).toBytes32(), // our adapter on the dst chain (same address)
            amountLD: order.inputAmount,
            minAmountLD: order.outputAmount,
            extraOptions: extraOptions,
            composeMsg: OrderLib.encode(order),
            oftCmd: ""
        });
        oftLocal.send{value: msg.value}(sendParam, MessagingFee({nativeFee: msg.value, lzTokenFee: 0}), msg.sender);
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
        address oftAddr = config.chainConfig(block.chainid).usdt0Oft;
        if (oftAddr == address(0)) revert UnsupportedChain(uint32(block.chainid));
        if (msg.sender != IOFT(oftAddr).endpoint()) revert NotEndpoint(msg.sender);
        if (from != oftAddr) revert UntrustedLocalOFT(from);

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
