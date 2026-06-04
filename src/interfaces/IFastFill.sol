// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ICallbackExecutor} from "./ICallbackExecutor.sol";
import {Order} from "../libraries/OrderLib.sol";

/// @notice Lifecycle status of an order, keyed by orderId on the destination chain.
enum FillStatus {
    None, // 0: never seen (default)
    Filled, // a relayer optimistically paid the recipient
    Settled // the bridge message arrived and funds were disbursed (terminal)
}

/// @notice Destination-chain record for an order. Packs into a single storage slot.
struct OrderRecord {
    address filler; // 20 bytes — relayer that filled (0 if unfilled)
    FillStatus status; // 1 byte
    uint40 fillTime; // 5 bytes — block.timestamp of the fill (audit)
}

/// @notice Shared external surface implemented by every fast-fill adapter.
interface IFastFill is ICallbackExecutor {
    /// @notice An order was initiated on the source chain.
    event OrderCreated(
        bytes32 indexed orderId,
        uint8 bridgeType,
        address indexed sender,
        uint32 dstChainId,
        bytes32 outputToken,
        uint256 outputAmount,
        uint64 nonce
    );

    /// @notice A relayer optimistically filled an in-flight order.
    event OrderFilled(
        bytes32 indexed orderId, address indexed filler, uint256 payoutToRecipient, uint256 feeToFiller, uint40 fillTime
    );

    /// @notice The bridge message arrived and the order was disbursed.
    event OrderSettled(
        bytes32 indexed orderId, address indexed filler, uint256 arrivedAmount, uint256 surplusToRecipient
    );

    /// @notice Optimistically fill an in-flight order; pays the recipient now, records the filler.
    function fill(Order calldata order) external returns (bytes32 orderId);

    /// @notice Preview the fill outcome at a given time (e.g. block.timestamp).
    function quoteFill(Order calldata order, uint256 fillTime)
        external
        view
        returns (uint256 payoutToRecipient, uint256 feeToFiller);

    /// @notice Read the destination-chain record for an order.
    function getOrder(bytes32 orderId) external view returns (OrderRecord memory);
}
