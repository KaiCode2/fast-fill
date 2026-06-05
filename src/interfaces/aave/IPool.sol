// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title  IPool
/// @notice Minimal, hand-written interface for the Aave V3 `Pool.supply` entrypoint.
/// @dev    Hand-written to match this repo's convention (only `forge-std` + `solady` are vendored).
///         `supply` mints the corresponding aToken to `onBehalfOf`, so a fast-fill hook can have the
///         interest-bearing position land directly with the end user. Pool addresses (per chain) live
///         in `script/config/Addresses.sol`.
interface IPool {
    /// @notice Supplies `amount` of `asset` into the protocol, minting aTokens to `onBehalfOf`.
    /// @dev Reverts if the reserve is frozen/paused, the asset is not listed, or supply caps are hit —
    ///      the failure a fast-fill hook converts into a `RedirectFunds(user)` refund.
    /// @param asset        The ERC20 to supply.
    /// @param amount       The amount to supply.
    /// @param onBehalfOf   The address that receives the aTokens (the end user).
    /// @param referralCode Aave referral code (0 if unused).
    function supply(address asset, uint256 amount, address onBehalfOf, uint16 referralCode) external;
}
