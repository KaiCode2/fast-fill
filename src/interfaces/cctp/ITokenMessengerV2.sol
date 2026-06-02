// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Minimal ^0.8 interface for Circle's CCTP v2 TokenMessenger.
/// @dev Mirrors `depositForBurnWithHook` in circlefin/evm-cctp-contracts
///      `src/v2/TokenMessengerV2.sol` (master branch). Hand-written because the upstream contract
///      is pinned to `solidity 0.7.6` and cannot be imported into this ^0.8 build.
interface ITokenMessengerV2 {
    /// @param amount               Amount of `burnToken` to burn on the source chain.
    /// @param destinationDomain    CCTP domain of the destination chain.
    /// @param mintRecipient        Recipient of the minted token on the destination (bytes32).
    /// @param burnToken            Address of the token to burn (USDC).
    /// @param destinationCaller    If non-zero, the only address allowed to call `receiveMessage`.
    /// @param maxFee               Max fee, in `burnToken` units, paid for a fast transfer.
    /// @param minFinalityThreshold <= 1000 makes the burn eligible for a fast (soft-finality) transfer.
    /// @param hookData             Arbitrary application payload, attested with the message.
    function depositForBurnWithHook(
        uint256 amount,
        uint32 destinationDomain,
        bytes32 mintRecipient,
        address burnToken,
        bytes32 destinationCaller,
        uint256 maxFee,
        uint32 minFinalityThreshold,
        bytes calldata hookData
    ) external;
}
