// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Fabricates CCTP v2 message blobs in-memory (no FFI, no fixtures) matching the
///         MessageV2 + BurnMessageV2 byte layout that BurnMessageV2Lib parses. The fields are
///         abi.encodePacked at the exact sizes Circle uses, so a roundtrip proves our parser.
library CctpMessageBuilder {
    struct Msg {
        uint32 sourceDomain;
        uint32 destinationDomain;
        bytes32 nonce;
        bytes32 destinationCaller;
        bytes32 burnToken;
        bytes32 mintRecipient;
        uint256 amount;
        bytes32 messageSender; // body field: the source adapter that initiated the burn
        uint256 maxFee;
        uint256 feeExecuted;
        bytes hookData;
    }

    function build(Msg memory m) internal pure returns (bytes memory) {
        // BurnMessageV2 body: version(4) burnToken(32) mintRecipient(32) amount(32) messageSender(32)
        //                     maxFee(32) feeExecuted(32) expirationBlock(32) hookData
        bytes memory body = abi.encodePacked(
            uint32(1),
            m.burnToken,
            m.mintRecipient,
            m.amount,
            m.messageSender,
            m.maxFee,
            m.feeExecuted,
            uint256(0), // expirationBlock (unused by our parser)
            m.hookData
        );
        // MessageV2 header: version(4) sourceDomain(4) destinationDomain(4) nonce(32) sender(32)
        //                   recipient(32) destinationCaller(32) minFinality(4) finalityExecuted(4) body
        return abi.encodePacked(
            uint32(1),
            m.sourceDomain,
            m.destinationDomain,
            m.nonce,
            m.messageSender, // header sender (unused by our parser)
            m.mintRecipient, // header recipient (unused by our parser)
            m.destinationCaller,
            uint32(0), // minFinalityThreshold
            uint32(2000), // finalityThresholdExecuted
            body
        );
    }
}
