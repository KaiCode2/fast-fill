// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Vm} from "forge-std/Vm.sol";
import {ForkBase} from "./ForkBase.sol";
import {CctpAdapter} from "../../src/adapters/CctpAdapter.sol";
import {Order, OrderLib} from "../../src/libraries/OrderLib.sol";
import {AddressCast} from "../../src/libraries/AddressCast.sol";
import {Addresses} from "../../script/config/Addresses.sol";
import {ParseHarness} from "../utils/ParseHarness.sol";

interface IERC20Min {
    function approve(address, uint256) external returns (bool);
    function balanceOf(address) external view returns (uint256);
}

/// @notice End-to-end source-side check against the REAL CCTP v2 contracts on a mainnet fork: a real
///         `depositForBurnWithHook` burn, then validate our BurnMessageV2Lib parser and order
///         encoding against the REAL emitted message. This is the highest-value de-risking step
///         before spending real funds — it catches any message-format mismatch for free.
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

    CctpAdapter src;
    CctpAdapter dst;

    function test_fork_realBurn_emitsParseableMessage() external {
        if (!_forkMainnetOrSkip()) return;
        _deploy();

        (bytes memory message, uint64 nonce, uint64 start) = _burn();
        Order memory order = _expectedOrder(nonce, start);

        assertEq(OrderLib.hash(order), _orderIdFromInit, "orderId matches reconstructed order");
        assertEq(IERC20Min(usdc).balanceOf(user), 0, "user USDC burned");

        _assertParsedMessage(message, order);
    }

    bytes32 internal _orderIdFromInit;

    function _deploy() internal {
        src = new CctpAdapter(
            address(this), 5e15, Addresses.CCTP_TOKEN_MESSENGER_V2, Addresses.CCTP_MESSAGE_TRANSMITTER_V2, usdc
        );
        dst = new CctpAdapter(
            address(this), 5e15, Addresses.CCTP_TOKEN_MESSENGER_V2, Addresses.CCTP_MESSAGE_TRANSMITTER_V2, usdc
        );
        src.setDomain(ETH_CHAIN, Addresses.DOMAIN_ETHEREUM);
        src.setDomain(BASE_CHAIN, Addresses.DOMAIN_BASE);
        src.setRemoteAdapter(BASE_CHAIN, address(dst).toBytes32());
        src.setRemoteUsdc(BASE_CHAIN, usdc); // demo loopback: treat Eth USDC as the "remote" output token
    }

    function _burn() internal returns (bytes memory message, uint64 nonce, uint64 start) {
        deal(usdc, user, AMOUNT);
        start = uint64(block.timestamp);

        vm.recordLogs();
        vm.startPrank(user);
        IERC20Min(usdc).approve(address(src), AMOUNT);
        // maxFee = 0, minFinalityThreshold = 2000 => a standard (finalized) transfer, no fast fee.
        (_orderIdFromInit, nonce) =
            src.initiateCCTP(BASE_CHAIN, recipient.toBytes32(), AMOUNT, 0, 2000, start + WINDOW, RATE);
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
            outputToken: usdc.toBytes32(),
            inputAmount: AMOUNT,
            outputAmount: AMOUNT,
            nonce: nonce,
            startTime: start,
            expectedDeliveryTime: start + WINDOW,
            discountRate: RATE
        });
    }

    function _assertParsedMessage(bytes memory message, Order memory order) internal {
        ParseHarness h = new ParseHarness();
        (uint32 sourceDomain, bytes32 messageSender, bytes32 mintRecipient, uint256 amt, uint256 feeExecuted,) =
            h.parse(message);

        assertEq(sourceDomain, Addresses.DOMAIN_ETHEREUM, "sourceDomain = Ethereum");
        assertEq(mintRecipient, address(dst).toBytes32(), "mintRecipient = destination adapter");
        assertEq(messageSender, address(src).toBytes32(), "messageSender = source adapter (anti-forgery anchor)");
        assertEq(amt, AMOUNT, "burn amount");
        assertEq(feeExecuted, 0, "no fee on a standard transfer");

        (,,,,, bytes memory hookData) = h.parse(message);
        assertEq(keccak256(hookData), keccak256(OrderLib.encode(order)), "hookData == our encoded order");
    }
}
