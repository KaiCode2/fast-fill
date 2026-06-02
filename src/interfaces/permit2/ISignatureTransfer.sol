// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice The slice of Uniswap's Permit2 SignatureTransfer we use. Permit2 is deployed at the same
///         address (0x000000000022D473030F116dDEE9F6B43aC78BA3) on every chain. `permitWitnessTransferFrom`
///         pulls tokens from `owner` against an off-chain signature that also commits to a `witness`
///         — we set the witness to the order intent (or fill authorization), so a third party that
///         submits the transaction cannot alter what the signer agreed to.
/// @dev Mirrors Uniswap/permit2 `ISignatureTransfer`.
interface ISignatureTransfer {
    struct TokenPermissions {
        address token;
        uint256 amount;
    }

    struct PermitTransferFrom {
        TokenPermissions permitted;
        uint256 nonce;
        uint256 deadline;
    }

    struct SignatureTransferDetails {
        address to;
        uint256 requestedAmount;
    }

    function permitWitnessTransferFrom(
        PermitTransferFrom memory permit,
        SignatureTransferDetails calldata transferDetails,
        address owner,
        bytes32 witness,
        string calldata witnessTypeString,
        bytes calldata signature
    ) external;
}
