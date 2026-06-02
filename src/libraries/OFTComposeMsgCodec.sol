// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Decodes the LayerZero OFT composed message delivered to `lzCompose`.
/// @dev Mirrors LayerZero `oft-evm/contracts/libs/OFTComposeMsgCodec.sol`. Layout
///      (packed): nonce(8) | srcEid(4) | amountLD(32) | composeFrom(32) | composeMsg(variable).
///      `composeFrom` is the source-chain sender (i.e. the remote fast-fill adapter that called
///      `OFT.send`); `composeMsg` is the application payload (our abi.encode(Order)).
library OFTComposeMsgCodec {
    uint256 private constant SRC_EID_OFFSET = 8;
    uint256 private constant AMOUNT_LD_OFFSET = 12;
    uint256 private constant COMPOSE_FROM_OFFSET = 44;
    uint256 private constant COMPOSE_MSG_OFFSET = 76;

    function nonce(bytes calldata composed) internal pure returns (uint64) {
        return uint64(bytes8(composed[0:SRC_EID_OFFSET]));
    }

    function srcEid(bytes calldata composed) internal pure returns (uint32) {
        return uint32(bytes4(composed[SRC_EID_OFFSET:AMOUNT_LD_OFFSET]));
    }

    function amountLD(bytes calldata composed) internal pure returns (uint256) {
        return uint256(bytes32(composed[AMOUNT_LD_OFFSET:COMPOSE_FROM_OFFSET]));
    }

    function composeFrom(bytes calldata composed) internal pure returns (bytes32) {
        return bytes32(composed[COMPOSE_FROM_OFFSET:COMPOSE_MSG_OFFSET]);
    }

    function composeMsg(bytes calldata composed) internal pure returns (bytes calldata) {
        return composed[COMPOSE_MSG_OFFSET:];
    }

    /// @notice Build a composed message. `composeMsg_` must already be prefixed with the
    ///         32-byte `composeFrom` (matching LayerZero's `_lzReceive` re-encoding).
    function encode(uint64 nonce_, uint32 srcEid_, uint256 amountLD_, bytes memory composeMsg_)
        internal
        pure
        returns (bytes memory)
    {
        return abi.encodePacked(nonce_, srcEid_, amountLD_, composeMsg_);
    }
}
