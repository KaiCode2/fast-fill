// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Safe conversions between `address` and the `bytes32` form used by CCTP
///         (`mintRecipient`, `destinationCaller`) and LayerZero (`to`, `composeFrom`).
library AddressCast {
    /// @dev `b` has non-zero bytes above the low 20, so it is not a valid EVM address.
    error InvalidAddress(bytes32 b);

    function toAddress(bytes32 b) internal pure returns (address) {
        if (uint256(b) >> 160 != 0) revert InvalidAddress(b);
        return address(uint160(uint256(b)));
    }

    function toBytes32(address a) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(a)));
    }
}
