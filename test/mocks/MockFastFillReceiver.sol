// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IFastFillReceiver} from "../../src/interfaces/IFastFillReceiver.sol";

/// @notice Configurable destination-execution receiver for tests. Records the callback args and, per
///         the selected mode, exercises every branch the adapter must handle: success, a plain revert
///         (→ claimable), a `RedirectFunds` revert (→ redirect), gas exhaustion, a return-bomb, and an
///         attempt to re-enter the adapter (which the reentrancy guard must block → claimable).
contract MockFastFillReceiver is IFastFillReceiver {
    enum Mode {
        Succeed, // return normally → funds delivered + hook ran
        RevertEmpty, // revert with no data → claimable
        RevertString, // revert with a non-redirect reason → claimable
        RevertRedirect, // revert RedirectFunds(redirectDest) → funds sent to redirectDest
        BurnGas, // consume all forwarded gas (INVALID) → claimable
        ReturnBomb, // revert with a large payload → bounded copy, still claimable
        Reenter, // call back into the adapter (guard must block) → claimable
        StealThenRevert // move the received funds out, then revert → claw-back must undo the move
    }

    Mode public mode;
    address public redirectDest;
    address public stash;
    address public reenterTarget;
    bytes public reenterCalldata;

    // Recorded on a successful call (rolled back if the callback reverts).
    bytes32 public lastOrderId;
    address public lastToken;
    uint256 public lastAmount;
    bytes public lastHookData;
    uint256 public callCount;

    function setMode(Mode m) external {
        mode = m;
    }

    function setRedirect(address dest) external {
        redirectDest = dest;
    }

    function setStash(address s) external {
        stash = s;
    }

    function setReenter(address target, bytes calldata cd) external {
        reenterTarget = target;
        reenterCalldata = cd;
    }

    function onFastFill(bytes32 orderId, address token, uint256 amount, bytes calldata hookData) external {
        lastOrderId = orderId;
        lastToken = token;
        lastAmount = amount;
        lastHookData = hookData;
        callCount++;

        Mode m = mode;
        if (m == Mode.Succeed) return;
        if (m == Mode.RevertEmpty) revert();
        if (m == Mode.RevertString) revert("rejected");
        if (m == Mode.RevertRedirect) revert IFastFillReceiver.RedirectFunds(redirectDest);
        if (m == Mode.BurnGas) {
            assembly {
                invalid()
            }
        }
        if (m == Mode.ReturnBomb) {
            assembly {
                revert(0, 0x800)
            } // 2KB of revert data; the adapter must bound its copy
        }
        if (m == Mode.Reenter) {
            (bool ok,) = reenterTarget.call(reenterCalldata);
            require(ok, "reentry blocked"); // re-entry reverts (guard) → this reverts → claimable
        }
        if (m == Mode.StealThenRevert) {
            (bool moved,) = token.call(abi.encodeWithSelector(0xa9059cbb, stash, amount)); // transfer(stash, amount)
            moved; // referenced only to silence the unused-return warning
            revert(); // the revert must roll the transfer above back too → adapter reclaims the funds
        }
    }
}
