// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @dev Mirrors the LayerZero v2 OFT structs/interface
///      (LayerZero `oft-evm/contracts/interfaces/IOFT.sol`). Hand-written to avoid wiring
///      the LayerZero devtools monorepo into a standalone Foundry build.

struct SendParam {
    uint32 dstEid; // destination endpoint id
    bytes32 to; // recipient on the destination (bytes32)
    uint256 amountLD; // amount to send, in local decimals
    uint256 minAmountLD; // min amount to receive, in local decimals (slippage floor)
    bytes extraOptions; // executor/DVN options (must include a compose-gas allowance)
    bytes composeMsg; // composed message payload delivered via lzCompose
    bytes oftCmd; // OFT command (unused for the default OFT)
}

struct MessagingFee {
    uint256 nativeFee;
    uint256 lzTokenFee;
}

struct MessagingReceipt {
    bytes32 guid;
    uint64 nonce;
    MessagingFee fee;
}

struct OFTReceipt {
    uint256 amountSentLD;
    uint256 amountReceivedLD;
}

interface IOFT {
    function send(SendParam calldata sendParam, MessagingFee calldata fee, address refundAddress)
        external
        payable
        returns (MessagingReceipt memory msgReceipt, OFTReceipt memory oftReceipt);

    function quoteSend(SendParam calldata sendParam, bool payInLzToken) external view returns (MessagingFee memory fee);

    /// @notice The underlying ERC20 delivered/locked by this OFT.
    function token() external view returns (address);

    /// @notice The LayerZero v2 endpoint this OFT is wired to. Read by the adapter so the endpoint
    ///         address never has to be configured separately.
    function endpoint() external view returns (address);
}
