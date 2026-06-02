// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Vm} from "forge-std/Vm.sol";
import {console2} from "forge-std/console2.sol";
import {ForkBase} from "./ForkBase.sol";
import {CctpAdapter} from "../../src/adapters/CctpAdapter.sol";
import {FastFillConfig} from "../../src/config/FastFillConfig.sol";
import {Order, OrderLib} from "../../src/libraries/OrderLib.sol";
import {AddressCast} from "../../src/libraries/AddressCast.sol";
import {Addresses} from "../../script/config/Addresses.sol";
import {ParseHarness} from "../utils/ParseHarness.sol";

interface IERC20Min {
    function approve(address, uint256) external returns (bool);
    function balanceOf(address) external view returns (uint256);
}

/// @notice End-to-end source-side check against the REAL CCTP v2 contracts on an Ethereum fork: a
///         real `depositForBurnWithHook` burn driven through the config-based adapter, then validate
///         our BurnMessageV2Lib parser and order encoding against the REAL emitted message. Also
///         exercises the live-domain cross-check (the burn reverts if the registry's Ethereum domain
///         disagrees with the real MessageTransmitter). The highest-value de-risk before live funds.
contract CctpForkE2ETest is ForkBase {
    using AddressCast for address;

    uint32 constant ETH_CHAIN = 1;
    uint32 constant BASE_CHAIN = 8453;
    uint256 constant AMOUNT = 5e6; // ~$5 USDC
    uint64 constant WINDOW = 600;
    uint256 constant RATE = 1e13;

    address user = makeAddr("user");
    address recipient = makeAddr("recipient");
    address usdc = Addresses.USDC_ETHEREUM;

    CctpAdapter adapter;
    bytes32 internal _orderIdFromInit;

    function test_fork_realBurn_emitsParseableMessage() external {
        if (!_forkMainnetOrSkip()) return;
        adapter = new CctpAdapter(address(new FastFillConfig()), address(this), 5e15);

        (bytes memory message, uint64 nonce, uint64 start) = _burn();
        Order memory order = _expectedOrder(nonce, start);

        assertEq(OrderLib.hash(order), _orderIdFromInit, "orderId matches reconstructed order");
        assertEq(IERC20Min(usdc).balanceOf(user), 0, "user USDC burned");

        _assertParsedMessage(message, order);
    }

    function _burn() internal returns (bytes memory message, uint64 nonce, uint64 start) {
        deal(usdc, user, AMOUNT);
        start = uint64(block.timestamp);

        vm.recordLogs();
        vm.startPrank(user);
        IERC20Min(usdc).approve(address(adapter), AMOUNT);
        // maxFee = 0, minFinalityThreshold = 2000 => a standard (finalized) transfer, no fast fee.
        // deliveryWindow is relative; expectedDeliveryTime is derived on-chain as start + WINDOW.
        uint256 g0 = gasleft();
        (_orderIdFromInit, nonce) =
            adapter.initiateCCTP(BASE_CHAIN, recipient.toBytes32(), AMOUNT, 0, 2000, WINDOW, RATE, 0);
        // Real end-to-end source gas through fast-fill (vs the mock numbers in test/gas/GasBench).
        console2.log("FORK real initiateCCTP gas (incl. real CCTP burn):", g0 - gasleft());
        vm.stopPrank();

        Vm.Log[] memory logs = vm.getRecordedLogs();
        for (uint256 i; i < logs.length; i++) {
            if (logs[i].topics[0] == keccak256("MessageSent(bytes)")) {
                message = abi.decode(logs[i].data, (bytes));
            }
        }
        assertGt(message.length, 0, "captured a real MessageSent");
    }

    function _expectedOrder(uint64 nonce, uint64 start) internal view returns (Order memory) {
        return Order({
            bridgeType: OrderLib.BRIDGE_CCTP,
            srcChainId: ETH_CHAIN,
            dstChainId: BASE_CHAIN,
            sender: user.toBytes32(),
            recipient: recipient.toBytes32(),
            inputToken: usdc.toBytes32(),
            outputToken: Addresses.USDC_BASE.toBytes32(), // the destination chain's USDC (per the registry)
            inputAmount: AMOUNT,
            outputAmount: AMOUNT,
            nonce: nonce,
            startTime: start,
            expectedDeliveryTime: start + WINDOW,
            discountRate: RATE,
            baseFee: 0
        });
    }

    function _assertParsedMessage(bytes memory message, Order memory order) internal {
        ParseHarness h = new ParseHarness();
        (uint32 sourceDomain, bytes32 messageSender, bytes32 mintRecipient, uint256 amt, uint256 feeExecuted,) =
            h.parse(message);

        assertEq(sourceDomain, Addresses.DOMAIN_ETHEREUM, "sourceDomain = Ethereum");
        // The adapter is its own counterpart (one deterministic address), so it is both the
        // mintRecipient and the messageSender.
        assertEq(mintRecipient, address(adapter).toBytes32(), "mintRecipient = our adapter");
        assertEq(messageSender, address(adapter).toBytes32(), "messageSender = our adapter (anti-forgery anchor)");
        assertEq(amt, AMOUNT, "burn amount");
        assertEq(feeExecuted, 0, "no fee on a standard transfer");

        (,,,,, bytes memory hookData) = h.parse(message);
        assertEq(keccak256(hookData), keccak256(OrderLib.encode(order)), "hookData == our encoded order");
    }
}
