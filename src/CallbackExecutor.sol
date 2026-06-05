// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {LibCall} from "solady/utils/LibCall.sol";

import {ICallbackExecutor, CallbackResult} from "./interfaces/ICallbackExecutor.sol";
import {IFastFillReceiver} from "./interfaces/IFastFillReceiver.sol";
import {TransientReentrancyGuard} from "./utils/TransientReentrancyGuard.sol";

/// @title  CallbackExecutor
/// @notice Shared substrate for "deliver an ERC20 and, in the SAME atomic frame, optionally run a
///         recipient callback — with funds-never-stuck guarantees" plus the pull-payment claim ledger.
///
///         Extracted from FastFillBase so the fast-fill adapters and the standalone CctpExecutor share
///         ONE audited implementation of the transfer-then-call clawback. The single load-bearing
///         property: the transfer and the callback happen in one external self-call, so a callback that
///         reverts (or a target that steals then reverts) rolls the transfer back and the funds are
///         re-routed by the bounded revert data — `RedirectFunds(dest)` → dest, anything else → the
///         `fallbackClaimant`'s claim ledger. A codeless target or empty callback just receives funds.
///
///         The whole flow runs under the caller's `nonReentrant` guard, and the callback is gas-capped
///         and return-bomb-safe.
abstract contract CallbackExecutor is ICallbackExecutor, TransientReentrancyGuard {
    /// @notice Maximum gas an order may request for destination execution (a create-time bound used by
    ///         fast-fill adapters).
    uint64 public constant MAX_CALLBACK_GAS_LIMIT = 5_000_000;

    /// @dev `RedirectFunds(address)` revert data: selector (4 bytes) + encoded address (32 bytes).
    uint16 private constant CALLBACK_RETURNDATA_LIMIT = 0x24;

    /// @dev ABI-encoded length of `InsufficientCallbackGas(uint256,uint256)` revert data: selector (4) +
    ///      two words (64) = 68. Strictly greater than `CALLBACK_RETURNDATA_LIMIT`, so a callback whose
    ///      bubbled revert is capped at 36 bytes can never forge this — see the catch in `_deliverWithHook`.
    uint256 private constant INSUFFICIENT_GAS_REVERT_LEN = 0x44;

    /// @dev Headroom reserved (on top of the callback budget) before the delivery self-call so the ERC20
    ///      transfer cannot run out of gas and the in-frame budget guard in `_executeDelivery` is always
    ///      reached. A conservative upper bound on the most expensive supported-token transfer scaled for
    ///      the two nested EIP-150 63/64 deductions; validated against real transfer costs by the fork
    ///      gas tests. The exact "callback got its full budget" guarantee is the in-frame guard, not this.
    uint256 private constant TRANSFER_HEADROOM = 120_000;

    /// @dev Slack added to the in-frame budget guard to cover the `LibCall.tryCall` setup (the calldata→
    ///      memory copy of `callbackData` + CALL base cost) and the bounded-returndata epilogue, so that
    ///      passing the guard means the callback receives its FULL signed budget after the final 63/64
    ///      deduction. Fails closed (reverts the whole fill) if exceeded — never a silent route to the
    ///      claim ledger.
    uint256 private constant INNER_CALLBACK_BUFFER = 10_000;

    error NothingToClaim();
    error InsufficientCallbackGas(uint256 available, uint256 callbackGasLimit);
    error OnlySelf();

    /// @notice Funds that failed to push (e.g. reverting/blacklisted recipient), claimable later.
    mapping(address account => mapping(address token => uint256)) internal _claimable;

    // ---------------------------------------------------------------------------------------------
    // Payout & claim (pull-payment fallback)
    // ---------------------------------------------------------------------------------------------

    /// @dev Try to push `amount` of `token` to `to`; on failure credit the claim ledger instead.
    function _payout(bytes32 id, address token, address to, uint256 amount) internal {
        if (amount == 0) return;
        if (_tryTransfer(token, to, amount)) return;
        _claimable[to][token] += amount;
        emit PayoutDeferred(id, to, token, amount);
    }

    /// @dev A return-value-checked ERC20 transfer that never reverts (returns success instead).
    function _tryTransfer(address token, address to, uint256 amount) private returns (bool) {
        (bool ok, bytes memory ret) = token.call(abi.encodeWithSelector(0xa9059cbb, to, amount)); // transfer(address,uint256)
        return ok && (ret.length == 0 || abi.decode(ret, (bool)));
    }

    /// @notice Withdraw funds credited to the caller after a failed push payout.
    function claim(address token) public virtual override nonReentrant returns (uint256 amount) {
        amount = _claimable[msg.sender][token];
        if (amount == 0) revert NothingToClaim();
        _claimable[msg.sender][token] = 0;
        SafeTransferLib.safeTransfer(token, msg.sender, amount);
        emit Claimed(msg.sender, token, amount);
    }

    /// @notice Funds owed to `account` in `token` from a deferred payout.
    function claimable(address account, address token) public view virtual override returns (uint256) {
        return _claimable[account][token];
    }

    // ---------------------------------------------------------------------------------------------
    // Destination execution (deliver + optional callback in one atomic frame)
    // ---------------------------------------------------------------------------------------------

    /// @dev Deliver `amount` of `token` to `recipient` and, if `callbackData` is non-empty and the
    ///      recipient is a contract, call it with `callbackData` in the SAME atomic frame. A reverting
    ///      callback rolls the transfer back and routes the funds by the receiver's bounded revert
    ///      data: `RedirectFunds(dest)` -> dest; anything else -> `fallbackClaimant`'s claim ledger.
    ///      Funds are never stuck. The caller MUST hold its `nonReentrant` guard so the callback cannot
    ///      re-enter; the call is gas-capped + return-bomb-safe.
    /// @param id              An identifier for events (orderId for fast-fill; a delivery id otherwise).
    /// @param callbackData    Pre-encoded calldata for the recipient callback; empty = deliver only.
    /// @param fallbackClaimant Who is credited if the callback reverts without a valid RedirectFunds.
    function _deliverWithHook(
        bytes32 id,
        address token,
        address recipient,
        uint256 amount,
        bytes memory callbackData,
        uint64 gasLimit,
        address fallbackClaimant
    ) internal {
        if (amount == 0) return;
        // No callback requested, or a codeless address (EOA / undeployed) that cannot run one: just
        // deliver. A low-level call to a codeless address would otherwise "succeed" with no effect.
        if (callbackData.length == 0 || recipient.code.length == 0) {
            _payout(id, token, recipient, amount);
            return;
        }
        // Fast-fail guard: reserve enough gas that the delivery self-call can run the ERC20 transfer and
        // reach the in-frame budget guard inside `_executeDelivery`. `gasLimit/31` (> gasLimit*((64/63)^2-1))
        // covers the two nested EIP-150 63/64 deductions; `TRANSFER_HEADROOM` covers the transfer itself.
        // Under-funding here reverts the whole fill (forcing a retry) instead of silently routing to the
        // claim ledger; the exact "callback received its full budget" guarantee is the guard in
        // `_executeDelivery`, which fails closed even if `TRANSFER_HEADROOM` under-estimates the transfer.
        if (gasleft() < gasLimit + gasLimit / 31 + TRANSFER_HEADROOM) {
            revert InsufficientCallbackGas(gasleft(), gasLimit);
        }

        try this._executeDelivery(token, recipient, amount, gasLimit, callbackData) {
            emit DestinationCallback(id, recipient, CallbackResult.Executed);
        } catch (bytes memory reason) {
            // The in-frame budget guard tripped: the relayer under-funded destination gas, so the callback
            // would not have received its full signed budget. Abort the whole fill (force a retry) instead
            // of crediting the fallback claim ledger. The guard reverts with the full
            // `InsufficientCallbackGas(uint256,uint256)` (68 bytes); a callback's bubbled revert is capped
            // at `CALLBACK_RETURNDATA_LIMIT` (36 bytes) by `_executeDelivery`, so it can never forge this —
            // the length check is the guarantee, the selector is a discriminator.
            if (reason.length == INSUFFICIENT_GAS_REVERT_LEN) {
                bytes32 selWord;
                /// @solidity memory-safe-assembly
                assembly {
                    selWord := mload(add(reason, 0x20))
                }
                if (bytes4(selWord) == InsufficientCallbackGas.selector) {
                    /// @solidity memory-safe-assembly
                    assembly {
                        revert(add(reason, 0x20), mload(reason))
                    }
                }
            }
            address dest = _parseRedirect(reason);
            if (dest != address(0) && dest != address(this)) {
                _payout(id, token, dest, amount);
                emit DestinationCallback(id, dest, CallbackResult.Redirected);
            } else {
                _claimable[fallbackClaimant][token] += amount;
                emit PayoutDeferred(id, fallbackClaimant, token, amount);
                emit DestinationCallback(id, fallbackClaimant, CallbackResult.Claimable);
            }
        }
    }

    /// @dev Self-only delivery+callback frame. Transfers funds then calls `callbackData` with a fixed
    ///      gas budget, copying at most 36 bytes of returndata (return-bomb-safe). On callback failure
    ///      it re-reverts with those bounded bytes so the caller's `try` rolls the transfer back and can
    ///      parse a redirect. Not `nonReentrant`, so the self-call passes the caller's guard.
    function _executeDelivery(
        address token,
        address recipient,
        uint256 amount,
        uint256 gasLimit,
        bytes calldata callbackData
    ) external {
        if (msg.sender != address(this)) revert OnlySelf();
        SafeTransferLib.safeTransfer(token, recipient, amount);
        // Exact budget guarantee: after the transfer only the final this->recipient CALL remains, so a
        // single EIP-150 63/64 deduction applies. Require gasleft >= ceil(gasLimit*64/63) + slack so the
        // CALL forwards the FULL signed `gasLimit`; otherwise the relayer under-funded the tx — revert
        // (caught and re-thrown by `_deliverWithHook`) rather than letting an under-funded callback OOG
        // and route to the claim ledger. `gasLimit*64/63 == gasLimit + ceil(gasLimit/63)`; `+ 1` lifts the
        // floored division to the ceiling.
        if (gasleft() < gasLimit + gasLimit / 63 + 1 + INNER_CALLBACK_BUFFER) {
            revert InsufficientCallbackGas(gasleft(), gasLimit);
        }
        (bool ok,, bytes memory ret) = LibCall.tryCall(recipient, 0, gasLimit, CALLBACK_RETURNDATA_LIMIT, callbackData);
        if (!ok) LibCall.bubbleUpRevert(ret);
    }

    /// @dev Decode a `RedirectFunds(address)` revert (selector + 32-byte address = 36 bytes); else 0.
    ///      The selector is signature-derived, so it matches whether a receiver reverts with
    ///      `IFastFillReceiver.RedirectFunds` or `ICctpExecReceiver.RedirectFunds` — they are identical.
    function _parseRedirect(bytes memory reason) private pure returns (address dest) {
        if (reason.length == CALLBACK_RETURNDATA_LIMIT) {
            bytes32 selWord;
            bytes32 addrWord;
            /// @solidity memory-safe-assembly
            assembly {
                selWord := mload(add(reason, 0x20))
                addrWord := mload(add(reason, 0x24))
            }
            if (bytes4(selWord) == IFastFillReceiver.RedirectFunds.selector) {
                dest = address(uint160(uint256(addrWord)));
            }
        }
    }
}
