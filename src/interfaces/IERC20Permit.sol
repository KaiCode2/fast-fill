// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice EIP-2612 permit (supported by USDC and USD₮0). Lets a holder authorize an allowance with
///         an off-chain signature, so an approval and an action can land in a single transaction
///         (via `selfPermit` + `multicall`).
interface IERC20Permit {
    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s)
        external;
}
