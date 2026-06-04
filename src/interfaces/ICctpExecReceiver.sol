// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title  ICctpExecReceiver
/// @notice Hook-mode callback for `CctpExecutor`. A contract implements this to receive bridged USDC —
///         and to act on it — for any CCTP transfer routed through the executor with `gasLimit > 0`.
///         The executor transfers `amount` USDC to the receiver and then calls `onCctpExecute` in the
///         SAME atomic frame (transfer-then-call), so reverting rolls the transfer back.
///
///         SECURITY / TRUST MODEL — READ BEFORE IMPLEMENTING:
///         • This is permissionlessly reachable. ANYONE can craft a real CCTP burn that names the
///           executor as mintRecipient and sets `target = your contract` with an arbitrary `payload`.
///           Treat every argument as untrusted.
///         • Authenticate the caller: `require(msg.sender == <the canonical CctpExecutor>)`.
///         • The ONLY trustworthy provenance is `sourceDomain` and `sender` — these are the CCTP
///           message's *authenticated* fields, passed through verbatim by the executor (never taken
///           from `payload`). If you only trust your own cross-chain deployments, verify
///           `sender == <your contract's bytes32 address>` (a burn's `sender` is whoever called
///           `depositForBurnWithHook`, so only you can produce a message that passes this check).
///         • To reject/redirect: revert with `RedirectFunds(dest)` to have the executor deliver the
///           funds to `dest` instead; any other revert credits the envelope's `refundTo` on the
///           executor's claim ledger. Funds are never stuck.
interface ICctpExecReceiver {
    /// @notice Revert with this from `onCctpExecute` to redirect the delivered funds to `dest` instead
    ///         of crediting the claim ledger. Selector-identical to `IFastFillReceiver.RedirectFunds`.
    error RedirectFunds(address dest);

    /// @notice Called by the executor after `amount` of `usdc` has been transferred to this contract.
    /// @param sourceDomain The authenticated CCTP source domain of the burn.
    /// @param sender       The authenticated burner (the address that called depositForBurnWithHook).
    /// @param usdc         The USDC token delivered on this chain.
    /// @param amount       The USDC transferred (minted amount minus the relayer's mintFee).
    /// @param payload      The integrator data from the envelope (untrusted unless `sender` is checked).
    function onCctpExecute(uint32 sourceDomain, bytes32 sender, address usdc, uint256 amount, bytes calldata payload)
        external;
}
