// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title  IFastFillReceiver
/// @notice Destination-execution callback. A recipient contract implements this to be notified — and
///         to act — the moment its bridged funds arrive, whether delivered by an optimistic fill or by
///         the bridge settling. The same interface is used by both the CCTP and OFT adapters.
///
///         The adapter transfers `amount` of `token` to the recipient and then calls `onFastFill` in
///         the SAME atomic frame. If `onFastFill` reverts, that transfer is rolled back and the funds
///         are re-routed by the adapter according to the revert data:
///           • revert with `RedirectFunds(dest)` → the funds are delivered to `dest` instead;
///           • revert with anything else (incl. empty) → the funds are credited to the recipient's
///             claimable ledger, recoverable via `claim(token)`.
///         The call is made with a fixed, user-signed gas limit and is only made when the recipient
///         has code (an EOA / undeployed address just receives the funds, with no callback).
interface IFastFillReceiver {
    /// @notice Revert with this from `onFastFill` to have the adapter deliver the funds to `dest`
    ///         (e.g. a fallback wallet, or the receiver itself) instead of crediting the claim ledger.
    /// @dev `dest == address(0)` or the adapter's own address is treated as "credit the claim ledger".
    error RedirectFunds(address dest);

    /// @notice Called by the fast-fill adapter after `amount` of `token` has been transferred to this
    ///         contract. Reverting rolls the transfer back and triggers the routing described above.
    /// @param orderId  The order being delivered (`keccak256(abi.encode(order))`).
    /// @param token    The ERC20 just transferred to this contract.
    /// @param amount   The amount transferred (the fill payout, or the full arrived amount at settle).
    /// @param hookData The arbitrary payload the user attached to the order.
    function onFastFill(bytes32 orderId, address token, uint256 amount, bytes calldata hookData) external;
}
