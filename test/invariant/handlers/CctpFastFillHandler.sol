// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";

import {CctpAdapter} from "../../../src/adapters/CctpAdapter.sol";
import {Order, OrderLib} from "../../../src/libraries/OrderLib.sol";
import {FillStatus} from "../../../src/interfaces/IFastFill.sol";
import {MockUSDC} from "../../mocks/MockUSDC.sol";
import {CctpMessageBuilder} from "../../utils/CctpMessageBuilder.sol";

/// @notice Drives random create/fill/settle/warp sequences against the CCTP adapter pair, tracking
///         ghost state for the invariant assertions. Constants mirror the Fixtures wiring.
contract CctpFastFillHandler is Test {
    uint32 constant SRC_CHAIN = 1;
    uint32 constant DST_CHAIN = 2;
    uint32 constant SRC_DOMAIN = 0;
    uint32 constant DST_DOMAIN = 6;

    CctpAdapter internal src;
    CctpAdapter internal dst;
    MockUSDC internal usdc;

    address internal userA = makeAddr("h_user");
    address[] internal relayers;
    address[] internal recipients;
    address[] internal accountList; // every address that could hold a claimable balance

    Order[] internal orders;
    bytes32[] internal orderIdList;
    mapping(bytes32 orderId => bool) internal seen;
    mapping(bytes32 orderId => bool) public settledOnce;
    mapping(bytes32 orderId => uint256) public settleCount;

    uint256 internal bridgeNonce;

    constructor(CctpAdapter src_, CctpAdapter dst_, MockUSDC usdc_) {
        src = src_;
        dst = dst_;
        usdc = usdc_;
        for (uint256 i; i < 3; i++) {
            address r = makeAddr(string.concat("h_relayer", vm.toString(i)));
            address rc = makeAddr(string.concat("h_recipient", vm.toString(i)));
            relayers.push(r);
            recipients.push(rc);
            accountList.push(r);
            accountList.push(rc);
        }
    }

    function accounts() external view returns (address[] memory) {
        return accountList;
    }

    function orderIds() external view returns (bytes32[] memory) {
        return orderIdList;
    }

    function _b32(address a) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(a)));
    }

    function createOrder(
        uint256 amountSeed,
        uint256 feeSeed,
        uint64 windowSeed,
        uint256 rateSeed,
        uint256 baseSeed,
        uint256 recipSeed
    ) external {
        uint256 amount = bound(amountSeed, 2, 1e12);
        uint256 maxFee = bound(feeSeed, 1, amount - 1);
        uint64 window = uint64(bound(windowSeed, 1, 1000));
        uint256 rate = bound(rateSeed, 0, 1e15);
        uint256 baseFee = bound(baseSeed, 0, amount - maxFee - 1); // must stay < outputAmount
        address recip = recipients[recipSeed % recipients.length];

        usdc.mint(userA, amount);
        vm.chainId(SRC_CHAIN);
        uint64 start = uint64(block.timestamp);
        vm.startPrank(userA);
        usdc.approve(address(src), amount);
        (bytes32 orderId, uint64 nonce) =
            src.initiateCCTP(DST_CHAIN, _b32(recip), amount, maxFee, 1000, window, rate, baseFee);
        vm.stopPrank();

        Order memory o = Order({
            bridgeType: OrderLib.BRIDGE_CCTP,
            srcChainId: SRC_CHAIN,
            dstChainId: DST_CHAIN,
            sender: _b32(userA),
            recipient: _b32(recip),
            inputToken: _b32(address(usdc)),
            outputToken: _b32(address(usdc)),
            inputAmount: amount,
            outputAmount: amount - maxFee,
            nonce: nonce,
            startTime: start,
            expectedDeliveryTime: start + window,
            discountRate: rate,
            baseFee: baseFee
        });
        orders.push(o);
        if (!seen[orderId]) {
            seen[orderId] = true;
            orderIdList.push(orderId);
        }
    }

    function fillOrder(uint256 orderSeed, uint256 relayerSeed) external {
        if (orders.length == 0) return;
        Order memory o = orders[orderSeed % orders.length];
        if (dst.getOrder(OrderLib.hash(o)).status != FillStatus.None) return;

        address r = relayers[relayerSeed % relayers.length];
        vm.chainId(DST_CHAIN);
        (uint256 payout,) = dst.quoteFill(o, block.timestamp);
        usdc.mint(r, payout);
        vm.startPrank(r);
        usdc.approve(address(dst), payout);
        try dst.fill(o) {} catch {}
        vm.stopPrank();
    }

    function settleOrder(uint256 orderSeed, uint256 feeSeed) external {
        if (orders.length == 0) return;
        Order memory o = orders[orderSeed % orders.length];
        bytes32 id = OrderLib.hash(o);
        if (settledOnce[id]) return;

        uint256 maxFee = o.inputAmount - o.outputAmount;
        uint256 feeExecuted = bound(feeSeed, 0, maxFee);
        bridgeNonce++;
        bytes memory message = CctpMessageBuilder.build(
            CctpMessageBuilder.Msg({
                sourceDomain: SRC_DOMAIN,
                destinationDomain: DST_DOMAIN,
                nonce: bytes32(bridgeNonce),
                destinationCaller: _b32(address(dst)),
                burnToken: _b32(address(usdc)),
                mintRecipient: _b32(address(dst)),
                amount: o.inputAmount,
                messageSender: _b32(address(src)),
                maxFee: maxFee,
                feeExecuted: feeExecuted,
                hookData: OrderLib.encode(o)
            })
        );
        vm.chainId(DST_CHAIN);
        try dst.settle(message, "") {
            settledOnce[id] = true;
            settleCount[id] += 1;
        } catch {}
    }

    function warp(uint256 dtSeed) external {
        vm.warp(block.timestamp + bound(dtSeed, 0, 500));
    }
}
