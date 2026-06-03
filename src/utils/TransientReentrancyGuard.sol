// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Minimal transient-storage reentrancy guard for chains that support EIP-1153.
abstract contract TransientReentrancyGuard {
    error Reentrancy();

    /// @dev Same compact slot as Solady's ReentrancyGuardTransient to avoid low-slot collisions.
    uint256 private constant _REENTRANCY_GUARD_SLOT = 0x8000000000ab143c06;

    modifier nonReentrant() virtual {
        /// @solidity memory-safe-assembly
        assembly {
            if tload(_REENTRANCY_GUARD_SLOT) {
                mstore(0x00, 0xab143c06) // `Reentrancy()`.
                revert(0x1c, 0x04)
            }
            tstore(_REENTRANCY_GUARD_SLOT, 1)
        }
        _;
        /// @solidity memory-safe-assembly
        assembly {
            tstore(_REENTRANCY_GUARD_SLOT, 0)
        }
    }
}
