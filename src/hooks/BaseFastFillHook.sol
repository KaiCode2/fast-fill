// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IFastFillReceiver} from "../interfaces/IFastFillReceiver.sol";

/// @title  BaseFastFillHook
/// @notice Shared substrate for demo destination-execution hooks. It standardizes the one behaviour the
///         hooks exist to showcase: do something productive with the funds the instant they arrive, and
///         if that fails, hand the *original* funds straight back to the user — with no stuck balances.
///
///         The fast-fill adapter transfers `amount` of `token` to this contract and then calls
///         {onFastFill} in the same atomic frame. A concrete hook implements `_doAction` (the swap /
///         deposit / execution) and `_userOf` (who the funds belong to). `_doAction` runs inside an
///         external self-call so that ANY failure — a reverting swap, a frozen Aave reserve, a bad
///         signature — rolls back every partial side effect and is re-thrown as the adapter's
///         `RedirectFunds(user)` sentinel. The adapter then delivers the original `token`/`amount` to
///         `user`. The user is therefore never worse off than a plain transfer, even when the action
///         fails. (`src/CallbackExecutor.sol` performs the redirect; `src/interfaces/IFastFillReceiver.sol`
///         documents the revert protocol.)
///
///         Hooks are intended to be deployed as stateless, per-chain singletons: `order.recipient` is the
///         hook, and the final user + per-action parameters travel in `order.hookData`. The contracts hold
///         no balances between calls and authenticate nothing on `onFastFill` (matching the reference
///         receiver model): a direct call by anyone simply finds no funds to act on and reverts harmlessly.
abstract contract BaseFastFillHook is IFastFillReceiver {
    /// @notice `execute` is an internal trampoline; only this contract may call it.
    error OnlySelf();

    /// @notice Emitted when the hook's action ran successfully and proceeds were sent to `user`.
    event HookExecuted(bytes32 indexed orderId, address indexed user, address token, uint256 amount);

    /// @inheritdoc IFastFillReceiver
    /// @dev Decodes the beneficiary, then runs `_doAction` in an external self-call. On any failure the
    ///      self-call reverts (rolling back partial work) and we re-throw `RedirectFunds(user)` so the
    ///      adapter refunds the user the original funds. The 1/64 gas the EIP-150 rule reserves for this
    ///      frame is enough to perform the redirect revert.
    function onFastFill(bytes32 orderId, address token, uint256 amount, bytes calldata hookData) external {
        address user = _userOf(hookData);
        try this.execute(token, amount, user, hookData) {
            emit HookExecuted(orderId, user, token, amount);
        } catch {
            revert IFastFillReceiver.RedirectFunds(user);
        }
    }

    /// @notice Self-only trampoline that wraps the concrete action in its own call frame so a revert
    ///         atomically undoes every side effect (approvals, transfers, partial executor state).
    function execute(address token, uint256 amount, address user, bytes calldata hookData) external {
        if (msg.sender != address(this)) revert OnlySelf();
        _doAction(token, amount, user, hookData);
    }

    /// @dev Decode the final beneficiary from `hookData`. Used both before the action (to capture the
    ///      redirect target up front) and is the address proceeds are routed to on success.
    function _userOf(bytes calldata hookData) internal pure virtual returns (address user);

    /// @dev Perform the hook's action with `amount` of `token` already held by this contract, sending the
    ///      resulting position/tokens to `user`. MUST revert on any failure so funds redirect to `user`.
    function _doAction(address token, uint256 amount, address user, bytes calldata hookData) internal virtual;
}
