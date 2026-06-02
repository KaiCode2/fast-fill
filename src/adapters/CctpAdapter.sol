// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {FastFillBase} from "../FastFillBase.sol";
import {Order, OrderLib} from "../libraries/OrderLib.sol";
import {AddressCast} from "../libraries/AddressCast.sol";
import {BurnMessageV2Lib} from "../libraries/BurnMessageV2Lib.sol";
import {ITokenMessengerV2} from "../interfaces/cctp/ITokenMessengerV2.sol";
import {IMessageTransmitterV2} from "../interfaces/cctp/IMessageTransmitterV2.sol";

/// @title  CctpAdapter
/// @notice Fast-fill adapter for Circle CCTP v2 (USDC). Bidirectional: `initiateCCTP` starts an
///         outbound transfer; `settle` finalizes an inbound one by wrapping `receiveMessage`.
///
///         Settlement is atomic and authenticated: the source sets `mintRecipient` and
///         `destinationCaller` to the destination adapter, so only this contract can call
///         `receiveMessage`. After it mints `amount - feeExecuted` USDC here and consumes the CCTP
///         nonce, the message bytes (incl. hookData) are proven authentic, and we additionally
///         require the burn's `messageSender` to be our registered source adapter — so a burn
///         crafted by anyone else (with a forged order) can never be settled here.
contract CctpAdapter is FastFillBase {
    using OrderLib for Order;
    using AddressCast for bytes32;
    using AddressCast for address;

    ITokenMessengerV2 public immutable tokenMessenger;
    IMessageTransmitterV2 public immutable messageTransmitter;
    address public immutable usdc;

    /// @dev chainId <-> CCTP domain. Domain 0 (Ethereum) is valid, so "configured" is determined by
    ///      round-trip consistency (forward) and by chainId != 0 (reverse), never by domain == 0.
    mapping(uint32 chainId => uint32 domain) public chainIdToDomain;
    mapping(uint32 domain => uint32 chainId) public domainToChainId;

    /// @dev USDC address on each remote chain. USDC is a different address per chain, so the source
    ///      must stamp the order's `outputToken` with the DESTINATION's USDC (what is actually
    ///      minted/delivered there) — not its own. Must equal the destination adapter's `usdc`.
    mapping(uint32 chainId => address usdc) public remoteUsdc;

    error ReceiveMessageFailed();
    error MintRecipientMismatch(bytes32 mintRecipient);
    error UnknownDomain(uint32 chainId);
    error UnknownRemoteUsdc(uint32 chainId);
    error UntrustedSourceDomain(uint32 sourceDomain);
    error UntrustedSender(bytes32 messageSender);
    error WrongOutputToken(bytes32 outputToken);
    error MaxFeeTooHigh(uint256 maxFee, uint256 inputAmount);

    event DomainConfigured(uint32 indexed chainId, uint32 indexed domain);
    event RemoteUsdcConfigured(uint32 indexed chainId, address indexed usdc);

    constructor(
        address owner_,
        uint256 maxFeeRate_,
        address tokenMessenger_,
        address messageTransmitter_,
        address usdc_
    ) FastFillBase(owner_, maxFeeRate_) {
        tokenMessenger = ITokenMessengerV2(tokenMessenger_);
        messageTransmitter = IMessageTransmitterV2(messageTransmitter_);
        usdc = usdc_;
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
    /// @param maxFee The max fast-transfer fee. `outputAmount = inputAmount - maxFee` is the
    ///               deterministic worst-case amount the filler is owed (feeExecuted <= maxFee).
    function initiateCCTP(
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 maxFee,
        uint32 minFinalityThreshold,
        uint64 expectedDeliveryTime,
        uint256 discountRate
    ) external whenNotPaused returns (bytes32 orderId, uint64 nonce) {
        uint32 destinationDomain = chainIdToDomain[dstChainId];
        if (domainToChainId[destinationDomain] != dstChainId) revert UnknownDomain(dstChainId);
        if (maxFee >= inputAmount) revert MaxFeeTooHigh(maxFee, inputAmount);

        nonce = _nextNonce();
        Order memory order =
            _buildOrder(dstChainId, recipient, inputAmount, maxFee, expectedDeliveryTime, discountRate, nonce);
        _assertCreatable(order);
        orderId = order.hash();

        _pullAndBurn(order, destinationDomain, maxFee, minFinalityThreshold);

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
        address outUsdc = remoteUsdc[dstChainId];
        if (outUsdc == address(0)) revert UnknownRemoteUsdc(dstChainId);
        order = Order({
            bridgeType: OrderLib.BRIDGE_CCTP,
            srcChainId: uint32(block.chainid),
            dstChainId: dstChainId,
            sender: msg.sender.toBytes32(),
            recipient: recipient,
            inputToken: usdc.toBytes32(),
            outputToken: outUsdc.toBytes32(),
            inputAmount: inputAmount,
            outputAmount: inputAmount - maxFee,
            nonce: nonce,
            startTime: uint64(block.timestamp),
            expectedDeliveryTime: expectedDeliveryTime,
            discountRate: discountRate
        });
    }

    /// @dev Pull USDC from the user, approve the TokenMessenger, and burn-with-hook to the dst adapter.
    function _pullAndBurn(Order memory order, uint32 destinationDomain, uint256 maxFee, uint32 minFinalityThreshold)
        private
    {
        bytes32 dstAdapter = remoteAdapter[order.dstChainId];
        SafeTransferLib.safeTransferFrom(usdc, msg.sender, address(this), order.inputAmount);
        SafeTransferLib.safeApproveWithRetry(usdc, address(tokenMessenger), order.inputAmount);
        tokenMessenger.depositForBurnWithHook(
            order.inputAmount,
            destinationDomain,
            dstAdapter, // mintRecipient = destination adapter
            usdc,
            dstAdapter, // destinationCaller = destination adapter (only it can receiveMessage)
            maxFee,
            minFinalityThreshold,
            OrderLib.encode(order)
        );
    }

    // ---------------------------------------------------------------------------------------------
    // Destination settle
    // ---------------------------------------------------------------------------------------------

    /// @notice Finalize an inbound transfer. Anyone may relay; `receiveMessage` enforces that this
    ///         contract is the `destinationCaller`, so the mint and settlement are atomic.
    function settle(bytes calldata message, bytes calldata attestation) external nonReentrant whenNotPaused {
        // Mint + authenticate. Reverts (and rolls back) on a bad attestation or a used nonce.
        if (!messageTransmitter.receiveMessage(message, attestation)) revert ReceiveMessageFailed();

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

        // The source domain must map to the order's source chain...
        uint32 mappedSrc = domainToChainId[sourceDomain];
        if (mappedSrc == 0 || mappedSrc != order.srcChainId) revert UntrustedSourceDomain(sourceDomain);
        // ...and the burn must have been initiated by our registered source adapter (anti-forgery).
        if (messageSender != remoteAdapter[order.srcChainId]) revert UntrustedSender(messageSender);

        uint256 arrived = amount - feeExecuted;
        _settle(order, order.hash(), arrived);
    }

    // ---------------------------------------------------------------------------------------------
    // Admin
    // ---------------------------------------------------------------------------------------------

    /// @notice Configure the chainId <-> CCTP domain mapping in both directions.
    function setDomain(uint32 chainId, uint32 domain) external onlyOwner {
        chainIdToDomain[chainId] = domain;
        domainToChainId[domain] = chainId;
        emit DomainConfigured(chainId, domain);
    }

    /// @notice Configure the USDC address on a remote chain (used to stamp an order's outputToken).
    function setRemoteUsdc(uint32 chainId, address token) external onlyOwner {
        remoteUsdc[chainId] = token;
        emit RemoteUsdcConfigured(chainId, token);
    }
}
