// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Minimal ^0.8 interface for Circle's CCTP v2 MessageTransmitter.
/// @dev Mirrors `receiveMessage` in circlefin/evm-cctp-contracts
///      `src/v2/MessageTransmitterV2.sol` (master branch). Validates Circle's attestation over the
///      entire message (including hookData), mints `amount - feeExecuted` to `mintRecipient`,
///      consumes the message nonce (replay protection), and — when the source set a non-zero
///      `destinationCaller` — requires `msg.sender == destinationCaller`.
interface IMessageTransmitterV2 {
    function receiveMessage(bytes calldata message, bytes calldata attestation) external returns (bool success);
}
