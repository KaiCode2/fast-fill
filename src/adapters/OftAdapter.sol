// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {FastFillBase} from "../FastFillBase.sol";
import {Order, OrderLib} from "../libraries/OrderLib.sol";
import {AddressCast} from "../libraries/AddressCast.sol";
import {OFTComposeMsgCodec} from "../libraries/OFTComposeMsgCodec.sol";
import {ILayerZeroComposer} from "../interfaces/layerzero/ILayerZeroComposer.sol";
import {IOFT, SendParam, MessagingFee} from "../interfaces/layerzero/IOFT.sol";

/// @title  OftAdapter
/// @notice Fast-fill adapter for a LayerZero v2 OFT. Bidirectional: `initiateOFT` starts an
///         outbound transfer; `lzCompose` finalizes an inbound one.
///
///         On the destination, the OFT credits the bridged tokens to this contract during
///         `_lzReceive`, then the Endpoint invokes `lzCompose` with the order payload. Three gates
///         authenticate it: caller is the Endpoint, the local `from` OFT is ours, and the embedded
///         `composeFrom` is our registered source adapter. This is the OFT analogue of CCTP's
///         destinationCaller + messageSender checks.
///
///         Note: this is distinct from LayerZero's lockbox `OFTAdapter`; here `oft` is a configured
///         OFT instance that this contract uses to send/receive a single token.
contract OftAdapter is FastFillBase, ILayerZeroComposer {
    using OrderLib for Order;
    using AddressCast for bytes32;
    using AddressCast for address;

    address public immutable endpoint;
    IOFT public immutable oft;
    address public immutable oftToken;

    /// @dev chainId <-> LayerZero endpoint id. LZ eids are never 0, so eid==0 means "unconfigured".
    mapping(uint32 chainId => uint32 eid) public chainIdToEid;
    mapping(uint32 eid => uint32 chainId) public eidToChainId;

    /// @dev The OFT token address on each remote chain. A LayerZero OFT token has a DIFFERENT
    ///      address per chain (e.g. USDT0 is 0x01bF... on Optimism but 0xFd08... on Arbitrum), so
    ///      the source must stamp the order's `outputToken` with the DESTINATION's token (what is
    ///      actually delivered there) — not its own local `oftToken`. Must equal the destination
    ///      adapter's `oftToken`.
    mapping(uint32 chainId => address token) public remoteOftToken;

    error NotEndpoint(address caller);
    error UntrustedLocalOFT(address from);
    error UntrustedPeer(bytes32 composeFrom);
    error UnknownEid(uint32 chainId);
    error UnknownRemoteOftToken(uint32 chainId);
    error UntrustedSourceEid(uint32 srcEid);
    error WrongOutputToken(bytes32 outputToken);

    event EidConfigured(uint32 indexed chainId, uint32 indexed eid);
    event RemoteOftTokenConfigured(uint32 indexed chainId, address indexed token);

    constructor(address owner_, uint256 maxFeeRate_, address endpoint_, address oft_)
        FastFillBase(owner_, maxFeeRate_)
    {
        endpoint = endpoint_;
        oft = IOFT(oft_);
        oftToken = IOFT(oft_).token();
    }

    function _bridgeType() internal pure override returns (uint8) {
        return OrderLib.BRIDGE_OFT;
    }

    function _resolveOutputToken(Order memory order) internal view override returns (address) {
        if (order.outputToken != oftToken.toBytes32()) revert WrongOutputToken(order.outputToken);
        return oftToken;
    }

    // ---------------------------------------------------------------------------------------------
    // Source
    // ---------------------------------------------------------------------------------------------

    /// @notice Send the OFT token to `dstChainId` and create an optimistically-fillable order.
    /// @dev `msg.value` pays the LayerZero native messaging fee (size it off-chain via quoteSend).
    ///      `extraOptions` must include a compose-gas allowance (OptionsBuilder.addExecutorLzComposeOption).
    /// @param minAmountLD The slippage floor, used as the deterministic `outputAmount` the filler is owed.
    function initiateOFT(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 minAmountLD,
        bytes calldata extraOptions,
        uint64 expectedDeliveryTime,
        uint256 discountRate
    ) external payable whenNotPaused returns (bytes32 orderId, uint64 nonce) {
        uint32 dstEid = chainIdToEid[dstChainId];
        if (eidToChainId[dstEid] != dstChainId) revert UnknownEid(dstChainId);

        nonce = _nextNonce();
        Order memory order =
            _buildOrder(dstChainId, recipient, inputAmount, minAmountLD, expectedDeliveryTime, discountRate, nonce);
        _assertCreatable(order);
        orderId = order.hash();

        _pullAndSend(order, dstEid, extraOptions);

        emit OrderCreated(
            orderId, OrderLib.BRIDGE_OFT, msg.sender, dstChainId, order.outputToken, order.outputAmount, nonce
        );
    }

    /// @dev Split out of `initiateOFT` to keep each stack frame shallow (avoids stack-too-deep).
    function _buildOrder(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 minAmountLD,
        uint64 expectedDeliveryTime,
        uint256 discountRate,
        uint64 nonce
    ) private view returns (Order memory order) {
        address outToken = remoteOftToken[dstChainId];
        if (outToken == address(0)) revert UnknownRemoteOftToken(dstChainId);
        order = Order({
            bridgeType: OrderLib.BRIDGE_OFT,
            srcChainId: uint32(block.chainid),
            dstChainId: dstChainId,
            sender: msg.sender.toBytes32(),
            recipient: recipient,
            inputToken: oftToken.toBytes32(),
            outputToken: outToken.toBytes32(),
            inputAmount: inputAmount,
            outputAmount: minAmountLD,
            nonce: nonce,
            startTime: uint64(block.timestamp),
            expectedDeliveryTime: expectedDeliveryTime,
            discountRate: discountRate
        });
    }

    /// @dev Pull the token from the user, approve the OFT, and dispatch the cross-chain send.
    function _pullAndSend(Order memory order, uint32 dstEid, bytes calldata extraOptions) private {
        SafeTransferLib.safeTransferFrom(oftToken, msg.sender, address(this), order.inputAmount);
        SafeTransferLib.safeApproveWithRetry(oftToken, address(oft), order.inputAmount);

        SendParam memory sendParam = SendParam({
            dstEid: dstEid,
            to: remoteAdapter[order.dstChainId],
            amountLD: order.inputAmount,
            minAmountLD: order.outputAmount,
            extraOptions: extraOptions,
            composeMsg: OrderLib.encode(order),
            oftCmd: ""
        });
        oft.send{value: msg.value}(sendParam, MessagingFee({nativeFee: msg.value, lzTokenFee: 0}), msg.sender);
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
        if (msg.sender != endpoint) revert NotEndpoint(msg.sender);
        if (from != address(oft)) revert UntrustedLocalOFT(from);

        uint32 srcEid = OFTComposeMsgCodec.srcEid(message);
        bytes32 composeFrom = OFTComposeMsgCodec.composeFrom(message);
        uint256 arrived = OFTComposeMsgCodec.amountLD(message);
        Order memory order = OrderLib.decode(OFTComposeMsgCodec.composeMsg(message));

        uint32 mappedSrc = eidToChainId[srcEid];
        if (mappedSrc == 0 || mappedSrc != order.srcChainId) revert UntrustedSourceEid(srcEid);
        if (composeFrom != remoteAdapter[order.srcChainId]) revert UntrustedPeer(composeFrom);
        if (block.chainid != order.dstChainId) revert WrongDestinationChain(order.dstChainId);

        // The OFT already credited `arrived` tokens to this contract during _lzReceive.
        _settle(order, order.hash(), arrived);
    }

    // ---------------------------------------------------------------------------------------------
    // Admin
    // ---------------------------------------------------------------------------------------------

    /// @notice Configure the chainId <-> LayerZero eid mapping in both directions.
    function setEid(uint32 chainId, uint32 eid) external onlyOwner {
        chainIdToEid[chainId] = eid;
        eidToChainId[eid] = chainId;
        emit EidConfigured(chainId, eid);
    }

    /// @notice Configure the OFT token address on a remote chain (used to stamp an order's outputToken).
    function setRemoteOftToken(uint32 chainId, address token) external onlyOwner {
        remoteOftToken[chainId] = token;
        emit RemoteOftTokenConfigured(chainId, token);
    }
}
