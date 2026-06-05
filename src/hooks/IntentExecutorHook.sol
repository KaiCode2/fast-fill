// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {BaseFastFillHook} from "./BaseFastFillHook.sol";
import {IIntentExecutor} from "../interfaces/intent/IIntentExecutor.sol";

/// @title  IntentExecutorHook
/// @notice Demo fast-fill hook: on delivery, run a set of user-signed executions on the user's ERC-7579
///         smart account via the Rhinestone `IntentExecutor` module. The user signs an `IntentExecutor`
///         payload (its destination executions); that payload travels in `order.hookData`, and the order's
///         `hookDataHash` is in turn committed by the origin claim (the sponsored Permit2 `OrderIntent`
///         witness, or by directly submitting on the origin chain) — so one origin authorization covers
///         both the claim and the destination executions.
///
///         On fill this hook forwards the delivered funds to the account and relays the signed ops to the
///         executor's permissionless entrypoint, which verifies the account's signature, consumes the
///         nonce, and runs the executions. If anything fails (bad signature, used nonce, reverting ops),
///         the base contract redirects the original funds to the account — the user keeps their funds,
///         just without the executions running.
///
///         `hookData = abi.encode(uint8 variant, bytes signedOps)` where `variant` selects the single- vs
///         multi-chain entrypoint and `signedOps` is the abi-encoded `SingleChainOps`/`MultiChainOps`.
contract IntentExecutorHook is BaseFastFillHook {
    using SafeTransferLib for address;

    /// @notice `variant` selecting `executeSinglechainOps` (the default).
    uint8 internal constant VARIANT_SINGLE = 0;
    /// @notice `variant` selecting `executeMultichainOps`.
    uint8 internal constant VARIANT_MULTI = 1;

    /// @notice The deployed Rhinestone `IntentExecutor` module this hook relays signed ops to.
    IIntentExecutor public immutable executor;

    constructor(address executor_) {
        executor = IIntentExecutor(executor_);
    }

    /// @dev The beneficiary is the `account` carried inside the signed ops.
    function _userOf(bytes calldata hookData) internal pure override returns (address account) {
        (uint8 variant, bytes memory signedOps) = abi.decode(hookData, (uint8, bytes));
        account = _accountOf(variant, signedOps);
    }

    /// @dev Move the delivered funds into the account, then relay the signed ops to the executor.
    function _doAction(address token, uint256 amount, address account, bytes calldata hookData) internal override {
        (uint8 variant, bytes memory signedOps) = abi.decode(hookData, (uint8, bytes));

        // Funds must be in the account before the user's ops run on it.
        token.safeTransfer(account, amount);

        if (variant == VARIANT_MULTI) {
            executor.executeMultichainOps(abi.decode(signedOps, (IIntentExecutor.MultiChainOps)));
        } else {
            executor.executeSinglechainOps(abi.decode(signedOps, (IIntentExecutor.SingleChainOps)));
        }
    }

    /// @dev Extract the smart account from the encoded ops (its first field in either struct).
    function _accountOf(uint8 variant, bytes memory signedOps) private pure returns (address account) {
        if (variant == VARIANT_MULTI) {
            account = abi.decode(signedOps, (IIntentExecutor.MultiChainOps)).account;
        } else {
            account = abi.decode(signedOps, (IIntentExecutor.SingleChainOps)).account;
        }
    }
}
