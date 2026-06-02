// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";

import {CctpAdapter} from "../../src/adapters/CctpAdapter.sol";
import {OftAdapter} from "../../src/adapters/OftAdapter.sol";
import {Order, OrderLib} from "../../src/libraries/OrderLib.sol";
import {AddressCast} from "../../src/libraries/AddressCast.sol";
import {ChainConfig} from "../../src/interfaces/IFastFillConfig.sol";

import {MockUSDC} from "../mocks/MockUSDC.sol";
import {MockTokenMessengerV2} from "../mocks/MockTokenMessengerV2.sol";
import {MockMessageTransmitterV2} from "../mocks/MockMessageTransmitterV2.sol";
import {MockOFT} from "../mocks/MockOFT.sol";
import {MockLzEndpoint} from "../mocks/MockLzEndpoint.sol";
import {MockFastFillConfig} from "../mocks/MockFastFillConfig.sol";
import {CctpMessageBuilder} from "./CctpMessageBuilder.sol";

/// @notice Shared deployment + helpers. Two chains are modelled in one EVM by switching
///         `block.chainid` with `vm.chainId`. Because the production adapter resolves all
///         chain-specific data from the config registry at call time and the counterpart is always
///         `address(this)` (one deterministic CREATE2 address everywhere), a SINGLE adapter
///         instance faithfully plays both the source and the destination chain. `srcCctp`/`dstCctp`
///         (and `srcOft`/`dstOft`) are therefore aliases of one contract.
abstract contract Fixtures is Test {
    using AddressCast for address;

    uint32 internal constant SRC_CHAIN = 1; // e.g. Ethereum
    uint32 internal constant DST_CHAIN = 2; // e.g. an L2

    uint32 internal constant SRC_DOMAIN = 0; // CCTP domains
    uint32 internal constant DST_DOMAIN = 6;

    uint32 internal constant SRC_EID = 30_101; // LayerZero eids
    uint32 internal constant DST_EID = 30_184;

    uint256 internal constant MAX_FEE_RATE = 5e15; // 0.5% cap (WAD)

    address internal owner = makeAddr("owner");
    address internal user = makeAddr("user");
    address internal recipient = makeAddr("recipient");
    address internal relayer = makeAddr("relayer");
    address internal relayer2 = makeAddr("relayer2");

    // CCTP wiring (one adapter; per-chain transmitter/messenger pairs behind the config)
    MockUSDC internal usdc;
    MockMessageTransmitterV2 internal transmitterSrc;
    MockMessageTransmitterV2 internal transmitterDst;
    MockTokenMessengerV2 internal messengerSrc;
    MockTokenMessengerV2 internal messengerDst;
    MockTokenMessengerV2 internal tokenMessenger; // alias = messengerSrc (where initiate records the burn)
    MockMessageTransmitterV2 internal transmitter; // alias = transmitterDst (where settle mints)
    MockFastFillConfig internal cctpConfig;
    CctpAdapter internal cctp;
    CctpAdapter internal srcCctp; // alias = cctp
    CctpAdapter internal dstCctp; // alias = cctp

    // OFT wiring (one adapter; one OFT/token/endpoint serving both chains)
    MockOFT internal oftToken;
    MockLzEndpoint internal lzEndpoint;
    MockFastFillConfig internal oftConfig;
    OftAdapter internal oft;
    OftAdapter internal srcOft; // alias = oft
    OftAdapter internal dstOft; // alias = oft

    // ---------------------------------------------------------------------------------------------
    // Setup
    // ---------------------------------------------------------------------------------------------

    function _setUpCctp() internal {
        usdc = new MockUSDC();
        transmitterSrc = new MockMessageTransmitterV2(address(usdc), SRC_DOMAIN);
        transmitterDst = new MockMessageTransmitterV2(address(usdc), DST_DOMAIN);
        messengerSrc = new MockTokenMessengerV2(address(transmitterSrc));
        messengerDst = new MockTokenMessengerV2(address(transmitterDst));
        tokenMessenger = messengerSrc;
        transmitter = transmitterDst;

        cctpConfig = new MockFastFillConfig();
        cctpConfig.set(
            SRC_CHAIN, ChainConfig(true, SRC_DOMAIN, 0, address(usdc), address(messengerSrc), address(0), address(0))
        );
        cctpConfig.set(
            DST_CHAIN, ChainConfig(true, DST_DOMAIN, 0, address(usdc), address(messengerDst), address(0), address(0))
        );

        cctp = new CctpAdapter(address(cctpConfig), owner, MAX_FEE_RATE);
        srcCctp = cctp;
        dstCctp = cctp;
    }

    function _setUpOft() internal {
        lzEndpoint = new MockLzEndpoint(SRC_EID);
        oftToken = new MockOFT(address(lzEndpoint));

        oftConfig = new MockFastFillConfig();
        // One OFT/token instance plays the token on both chains (per-chain token distinctness is
        // covered by the OFT fork test against real USD₮0).
        oftConfig.set(
            SRC_CHAIN, ChainConfig(true, 0, SRC_EID, address(0), address(0), address(oftToken), address(oftToken))
        );
        oftConfig.set(
            DST_CHAIN, ChainConfig(true, 0, DST_EID, address(0), address(0), address(oftToken), address(oftToken))
        );

        oft = new OftAdapter(address(oftConfig), owner, MAX_FEE_RATE);
        srcOft = oft;
        dstOft = oft;
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
        uint256 baseFee,
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
            discountRate: discountRate,
            baseFee: baseFee
        });
    }

    function _oftOrder(
        uint256 inputAmount,
        uint256 minAmountLD,
        uint64 startTime,
        uint64 expectedDeliveryTime,
        uint256 discountRate,
        uint256 baseFee,
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
            discountRate: discountRate,
            baseFee: baseFee
        });
    }

    /// @notice Build a CCTP message that, when relayed through `cctp.settle`, settles `order`. With a
    ///         single deterministic adapter address, the source `messageSender`, the `mintRecipient`,
    ///         and the `destinationCaller` are all that one adapter.
    function _cctpMessage(Order memory order, uint256 feeExecuted, bytes32 nonce) internal view returns (bytes memory) {
        return CctpMessageBuilder.build(
            CctpMessageBuilder.Msg({
                sourceDomain: SRC_DOMAIN,
                destinationDomain: DST_DOMAIN,
                nonce: nonce,
                destinationCaller: address(cctp).toBytes32(),
                burnToken: address(usdc).toBytes32(),
                mintRecipient: address(cctp).toBytes32(),
                amount: order.inputAmount,
                messageSender: address(cctp).toBytes32(),
                maxFee: order.inputAmount - order.outputAmount,
                feeExecuted: feeExecuted,
                hookData: OrderLib.encode(order)
            })
        );
    }
}
