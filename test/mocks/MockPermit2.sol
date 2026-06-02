// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {ISignatureTransfer} from "../../src/interfaces/permit2/ISignatureTransfer.sol";

/// @notice Minimal Permit2 stand-in for unit tests: performs the authorized pull (owner -> to) and
///         records the witness so tests can assert the adapter bound the right intent. Real EIP-712
///         signature verification is exercised against the REAL Permit2 in the fork test; here the
///         signature is opaque and `setReject(true)` simulates Permit2 rejecting it.
contract MockPermit2 is ISignatureTransfer {
    bytes32 public lastWitness;
    string public lastWitnessTypeString;
    address public lastOwner;
    uint256 public lastRequestedAmount;
    bool public reject;

    error Rejected();
    error ExceedsPermitted();

    function setReject(bool v) external {
        reject = v;
    }

    function permitWitnessTransferFrom(
        PermitTransferFrom memory permit,
        SignatureTransferDetails calldata transferDetails,
        address owner,
        bytes32 witness,
        string calldata witnessTypeString,
        bytes calldata /*signature*/
    ) external {
        if (reject) revert Rejected();
        if (transferDetails.requestedAmount > permit.permitted.amount) revert ExceedsPermitted();
        lastWitness = witness;
        lastWitnessTypeString = witnessTypeString;
        lastOwner = owner;
        lastRequestedAmount = transferDetails.requestedAmount;
        SafeTransferLib.safeTransferFrom(
            permit.permitted.token, owner, transferDetails.to, transferDetails.requestedAmount
        );
    }
}
