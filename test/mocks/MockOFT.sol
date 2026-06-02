// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {MockERC20} from "./MockERC20.sol";
import {IOFT, SendParam, MessagingFee, MessagingReceipt, OFTReceipt} from "../../src/interfaces/layerzero/IOFT.sol";

/// @notice Mock native LayerZero v2 OFT (the OFT is itself the ERC20, so `token() == address(this)`).
///         `send` debits the caller and records the send; cross-chain delivery is driven separately
///         by the test (credit the destination via `mint`, then MockLzEndpoint invokes lzCompose).
///         Inherits MockERC20 for `mint` and the optional blacklist (`setBlocked`).
contract MockOFT is MockERC20, IOFT {
    /// @dev The LZ endpoint this OFT reports (read by the adapter). Auto-getter satisfies IOFT.endpoint().
    address public immutable endpoint;

    uint32 public lastDstEid;
    bytes32 public lastTo;
    uint256 public lastAmountLD;
    uint256 public lastMinAmountLD;
    bytes public lastComposeMsg;
    uint256 public sendCount;

    event Sent(uint32 dstEid, bytes32 to, uint256 amountLD, uint256 minAmountLD, bytes composeMsg);

    constructor(address endpoint_) MockERC20("Mock OFT", "mOFT", 18) {
        endpoint = endpoint_;
    }

    function token() external view returns (address) {
        return address(this);
    }

    function send(SendParam calldata sendParam, MessagingFee calldata, address)
        external
        payable
        returns (MessagingReceipt memory msgReceipt, OFTReceipt memory oftReceipt)
    {
        _burn(msg.sender, sendParam.amountLD); // debit the source adapter
        lastDstEid = sendParam.dstEid;
        lastTo = sendParam.to;
        lastAmountLD = sendParam.amountLD;
        lastMinAmountLD = sendParam.minAmountLD;
        lastComposeMsg = sendParam.composeMsg;
        sendCount++;
        emit Sent(sendParam.dstEid, sendParam.to, sendParam.amountLD, sendParam.minAmountLD, sendParam.composeMsg);

        msgReceipt = MessagingReceipt({guid: bytes32(0), nonce: 0, fee: MessagingFee({nativeFee: 0, lzTokenFee: 0})});
        oftReceipt = OFTReceipt({amountSentLD: sendParam.amountLD, amountReceivedLD: sendParam.amountLD});
    }

    function quoteSend(SendParam calldata, bool) external pure returns (MessagingFee memory) {
        return MessagingFee({nativeFee: 0, lzTokenFee: 0});
    }
}
