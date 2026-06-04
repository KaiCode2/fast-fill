// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Outcome of a destination-execution callback.
enum CallbackResult {
    Executed, // the callback succeeded; funds delivered to the target and the hook ran
    Redirected, // the callback reverted with RedirectFunds(dest); funds delivered to dest instead
    Claimable // the callback reverted otherwise; funds credited to the fallback claimant's ledger
}

/// @notice Shared pull-payment and callback-event surface for contracts using CallbackExecutor.
interface ICallbackExecutor {
    /// @notice A push payout failed and was credited to the pull-payment ledger instead.
    event PayoutDeferred(bytes32 indexed id, address indexed to, address indexed token, uint256 amount);

    /// @notice A previously-deferred payout was claimed.
    event Claimed(address indexed account, address indexed token, uint256 amount);

    /// @notice A destination-execution hook ran for a delivered transfer; `result` records the outcome
    ///         and `fundsTo` the address that ultimately received the funds.
    event DestinationCallback(bytes32 indexed id, address indexed fundsTo, CallbackResult result);

    /// @notice Withdraw funds credited to the caller after a failed push payout.
    function claim(address token) external returns (uint256 amount);

    /// @notice Funds owed to `account` in `token` from a deferred payout.
    function claimable(address account, address token) external view returns (uint256);
}
