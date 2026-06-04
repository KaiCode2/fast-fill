// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice The CctpExecutor envelope. Carried verbatim as a CCTP v2 message's `hookData` (and thus
///         attested by Circle), it tells the executor how much to pay the relayer and where to route
///         the rest of the minted USDC.
/// @dev Deliberately decoupled from the bridge-agnostic fast-fill `Order`: `mintFee` is a CCTP-only
///      cost, so it lives here rather than polluting the `Order` struct shared with OFT. For a
///      fast-fill order, `payload == OrderLib.encode(order)` and `target` is the CctpAdapter.
struct ExecHook {
    uint256 mintFee; // USDC paid to whoever relays the destination mint (0 = no relay incentive)
    bytes32 target; // forward-only recipient OR a receiver contract (hook mode), as bytes32
    uint64 gasLimit; // gas budget for the hook; 0 => forward-only (deliver USDC, no call)
    bytes32 refundTo; // claimant on the executor's claim ledger if the hook reverts (funds-never-stuck)
    bytes payload; // integrator data passed to the hook; fast-fill = abi.encode(Order)
}

library ExecHookLib {
    /// @notice Encode an envelope for transport in a CCTP `hookData` field.
    function encode(ExecHook memory h) internal pure returns (bytes memory) {
        return abi.encode(h);
    }

    /// @notice Decode an envelope from a CCTP `hookData` field. Reverts on malformed data, which (since
    ///         it happens after `receiveMessage`) just rolls the whole relay back, leaving the message
    ///         redeemable — no funds are stranded.
    function decode(bytes memory data) internal pure returns (ExecHook memory) {
        return abi.decode(data, (ExecHook));
    }
}
