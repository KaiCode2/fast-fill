// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";

import {CctpAdapter} from "../../src/adapters/CctpAdapter.sol";
import {OftAdapter} from "../../src/adapters/OftAdapter.sol";
import {Order, OrderLib} from "../../src/libraries/OrderLib.sol";
import {AddressCast} from "../../src/libraries/AddressCast.sol";

import {MockUSDC} from "../mocks/MockUSDC.sol";
import {MockTokenMessengerV2} from "../mocks/MockTokenMessengerV2.sol";
import {MockMessageTransmitterV2} from "../mocks/MockMessageTransmitterV2.sol";
import {MockOFT} from "../mocks/MockOFT.sol";
import {MockLzEndpoint} from "../mocks/MockLzEndpoint.sol";
import {CctpMessageBuilder} from "./CctpMessageBuilder.sol";

/// @notice Shared deployment + helpers. Two chains are modelled in one EVM by switching
///         `block.chainid` with `vm.chainId` around source vs destination actions.
abstract contract Fixtures is Test {
    using AddressCast for address;

    uint32 internal constant SRC_CHAIN = 1; // e.g. Ethereum
    uint32 internal constant DST_CHAIN = 2; // e.g. an L2

    uint32 internal constant SRC_DOMAIN = 0; // CCTP domains
    uint32 internal constant DST_DOMAIN = 6;

    uint32 internal constant SRC_EID = 30101; // LayerZero eids
    uint32 internal constant DST_EID = 30184;

    uint256 internal constant MAX_FEE_RATE = 5e15; // 0.5% cap (WAD)

    address internal owner = makeAddr("owner");
    address internal user = makeAddr("user");
    address internal recipient = makeAddr("recipient");
    address internal relayer = makeAddr("relayer");
    address internal relayer2 = makeAddr("relayer2");

    // CCTP wiring
    MockUSDC internal usdc;
    MockTokenMessengerV2 internal tokenMessenger;
    MockMessageTransmitterV2 internal transmitter;
    CctpAdapter internal srcCctp;
    CctpAdapter internal dstCctp;

    // OFT wiring
    MockOFT internal oftToken;
    MockLzEndpoint internal lzEndpoint;
    OftAdapter internal srcOft;
    OftAdapter internal dstOft;

    // ---------------------------------------------------------------------------------------------
    // Setup
    // ---------------------------------------------------------------------------------------------

    function _setUpCctp() internal {
        usdc = new MockUSDC();
        tokenMessenger = new MockTokenMessengerV2();
        transmitter = new MockMessageTransmitterV2(address(usdc));

        srcCctp = new CctpAdapter(owner, MAX_FEE_RATE, address(tokenMessenger), address(transmitter), address(usdc));
        dstCctp = new CctpAdapter(owner, MAX_FEE_RATE, address(tokenMessenger), address(transmitter), address(usdc));

        vm.startPrank(owner);
        srcCctp.setDomain(SRC_CHAIN, SRC_DOMAIN);
        srcCctp.setDomain(DST_CHAIN, DST_DOMAIN);
        srcCctp.setRemoteAdapter(DST_CHAIN, address(dstCctp).toBytes32());

        dstCctp.setDomain(SRC_CHAIN, SRC_DOMAIN);
        dstCctp.setDomain(DST_CHAIN, DST_DOMAIN);
        dstCctp.setRemoteAdapter(SRC_CHAIN, address(srcCctp).toBytes32());
        vm.stopPrank();
    }

    function _setUpOft() internal {
        oftToken = new MockOFT();
        lzEndpoint = new MockLzEndpoint();

        srcOft = new OftAdapter(owner, MAX_FEE_RATE, address(lzEndpoint), address(oftToken));
        dstOft = new OftAdapter(owner, MAX_FEE_RATE, address(lzEndpoint), address(oftToken));

        vm.startPrank(owner);
        srcOft.setEid(SRC_CHAIN, SRC_EID);
        srcOft.setEid(DST_CHAIN, DST_EID);
        srcOft.setRemoteAdapter(DST_CHAIN, address(dstOft).toBytes32());

        dstOft.setEid(SRC_CHAIN, SRC_EID);
        dstOft.setEid(DST_CHAIN, DST_EID);
        dstOft.setRemoteAdapter(SRC_CHAIN, address(srcOft).toBytes32());
        vm.stopPrank();
    }

    function _b32(address a) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(a)));
    }

    // ---------------------------------------------------------------------------------------------
    // Order + message helpers
    // ---------------------------------------------------------------------------------------------

    function _cctpOrder(
        uint256 inputAmount,
        uint256 maxFee,
        uint64 startTime,
        uint64 expectedDeliveryTime,
        uint256 discountRate,
        uint64 nonce
    ) internal view returns (Order memory) {
        return Order({
            bridgeType: OrderLib.BRIDGE_CCTP,
            srcChainId: SRC_CHAIN,
            dstChainId: DST_CHAIN,
            sender: user.toBytes32(),
            recipient: recipient.toBytes32(),
            inputToken: address(usdc).toBytes32(),
            outputToken: address(usdc).toBytes32(),
            inputAmount: inputAmount,
            outputAmount: inputAmount - maxFee,
            nonce: nonce,
            startTime: startTime,
            expectedDeliveryTime: expectedDeliveryTime,
            discountRate: discountRate
        });
    }

    function _oftOrder(
        uint256 inputAmount,
        uint256 minAmountLD,
        uint64 startTime,
        uint64 expectedDeliveryTime,
        uint256 discountRate,
        uint64 nonce
    ) internal view returns (Order memory) {
        return Order({
            bridgeType: OrderLib.BRIDGE_OFT,
            srcChainId: SRC_CHAIN,
            dstChainId: DST_CHAIN,
            sender: user.toBytes32(),
            recipient: recipient.toBytes32(),
            inputToken: address(oftToken).toBytes32(),
            outputToken: address(oftToken).toBytes32(),
            inputAmount: inputAmount,
            outputAmount: minAmountLD,
            nonce: nonce,
            startTime: startTime,
            expectedDeliveryTime: expectedDeliveryTime,
            discountRate: discountRate
        });
    }

    /// @notice Build a CCTP message that, when relayed through `dstCctp.settle`, settles `order`.
    function _cctpMessage(Order memory order, uint256 feeExecuted, bytes32 nonce) internal view returns (bytes memory) {
        return CctpMessageBuilder.build(
            CctpMessageBuilder.Msg({
                sourceDomain: SRC_DOMAIN,
                destinationDomain: DST_DOMAIN,
                nonce: nonce,
                destinationCaller: address(dstCctp).toBytes32(),
                burnToken: address(usdc).toBytes32(),
                mintRecipient: address(dstCctp).toBytes32(),
                amount: order.inputAmount,
                messageSender: address(srcCctp).toBytes32(),
                maxFee: order.inputAmount - order.outputAmount,
                feeExecuted: feeExecuted,
                hookData: OrderLib.encode(order)
            })
        );
    }
}
