// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Parses the fields fast-fill needs out of a CCTP v2 message blob.
/// @dev Field offsets mirror Circle's reference contracts (circlefin/evm-cctp-contracts,
///      master branch): src/messages/v2/MessageV2.sol and src/messages/v2/BurnMessageV2.sol.
//
// MessageV2 header layout  (byteOffset:field(size)):
//   0:version(4) 4:sourceDomain(4) 8:destinationDomain(4) 12:nonce(32)
//   44:sender(32) 76:recipient(32) 108:destinationCaller(32)
//   140:minFinalityThreshold(4) 144:finalityThresholdExecuted(4) 148:messageBody(dynamic)
// BurnMessageV2 body layout (offset within messageBody):
//   0:version(4) 4:burnToken(32) 36:mintRecipient(32) 68:amount(32)
//   100:messageSender(32) 132:maxFee(32) 164:feeExecuted(32) 196:expirationBlock(32)
//   228:hookData(dynamic)
library BurnMessageV2Lib {
    uint256 internal constant SOURCE_DOMAIN_OFFSET = 4;
    uint256 internal constant MESSAGE_BODY_OFFSET = 148;

    uint256 internal constant BODY_MINT_RECIPIENT_OFFSET = 36;
    uint256 internal constant BODY_AMOUNT_OFFSET = 68;
    uint256 internal constant BODY_MESSAGE_SENDER_OFFSET = 100;
    uint256 internal constant BODY_FEE_EXECUTED_OFFSET = 164;
    uint256 internal constant BODY_HOOK_DATA_OFFSET = 228;

    /// @dev Smallest valid message: full header + full burn body, with empty hookData.
    uint256 internal constant MIN_MESSAGE_LENGTH = MESSAGE_BODY_OFFSET + BODY_HOOK_DATA_OFFSET; // 376

    error MessageTooShort(uint256 length);

    /// @notice Extract the fields fast-fill cares about from a CCTP v2 message.
    /// @return sourceDomain  CCTP domain of the source chain.
    /// @return messageSender The address that initiated the burn (our source adapter, for a legit order).
    /// @return mintRecipient The address USDC was minted to (must be our adapter).
    /// @return amount        The burned amount (pre destination fee).
    /// @return feeExecuted   The fast-transfer fee charged at mint (<= maxFee).
    /// @return hookData      The application payload (our abi.encode(Order)).
    function parse(bytes calldata message)
        internal
        pure
        returns (
            uint32 sourceDomain,
            bytes32 messageSender,
            bytes32 mintRecipient,
            uint256 amount,
            uint256 feeExecuted,
            bytes calldata hookData
        )
    {
        if (message.length < MIN_MESSAGE_LENGTH) revert MessageTooShort(message.length);

        sourceDomain = uint32(bytes4(message[SOURCE_DOMAIN_OFFSET:SOURCE_DOMAIN_OFFSET + 4]));

        uint256 b = MESSAGE_BODY_OFFSET;
        mintRecipient = bytes32(message[b + BODY_MINT_RECIPIENT_OFFSET:b + BODY_MINT_RECIPIENT_OFFSET + 32]);
        amount = uint256(bytes32(message[b + BODY_AMOUNT_OFFSET:b + BODY_AMOUNT_OFFSET + 32]));
        messageSender = bytes32(message[b + BODY_MESSAGE_SENDER_OFFSET:b + BODY_MESSAGE_SENDER_OFFSET + 32]);
        feeExecuted = uint256(bytes32(message[b + BODY_FEE_EXECUTED_OFFSET:b + BODY_FEE_EXECUTED_OFFSET + 32]));
        hookData = message[b + BODY_HOOK_DATA_OFFSET:];
    }
}
