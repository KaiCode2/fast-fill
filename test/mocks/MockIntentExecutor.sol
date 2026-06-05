// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IIntentExecutor} from "../../src/interfaces/intent/IIntentExecutor.sol";
import {MockERC7579Account} from "./MockERC7579Account.sol";

/// @notice Minimal Rhinestone-`IntentExecutor` stand-in. Mirrors the real module's externally-observable
///         contract: it checks the signature is present, enforces single-use nonces, and runs the ops on
///         the account. `setRevert(true)` simulates a failed signature/authorization — the failure the
///         IntentExecutorHook must convert into a `RedirectFunds(account)` refund. The real module performs
///         full EIP-712 verification; the mock keeps just enough to exercise the hook's success + redirect.
contract MockIntentExecutor is IIntentExecutor {
    bool public shouldRevert;
    mapping(address account => mapping(uint256 nonce => bool)) public nonceUsed;

    address public lastAccount;
    uint256 public lastNonce;
    uint256 public callCount;

    function setRevert(bool v) external {
        shouldRevert = v;
    }

    function executeSinglechainOps(SingleChainOps calldata signedOps) external {
        _run(signedOps.account, signedOps.nonce, signedOps.signature, signedOps.ops.data);
    }

    function executeMultichainOps(MultiChainOps calldata signedOps) external {
        _run(signedOps.account, signedOps.nonce, signedOps.signature, signedOps.ops.data);
    }

    function _run(address account, uint256 nonce, bytes calldata signature, bytes calldata opData) internal {
        require(!shouldRevert, "INVALID_SIGNATURE");
        require(signature.length != 0, "EMPTY_SIGNATURE");
        require(!nonceUsed[account][nonce], "NONCE_USED");
        nonceUsed[account][nonce] = true;
        lastAccount = account;
        lastNonce = nonce;
        ++callCount;
        MockERC7579Account(payable(account)).executeFromExecutor(bytes32(0), opData);
    }
}
