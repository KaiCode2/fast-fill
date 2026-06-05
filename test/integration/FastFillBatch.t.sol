// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../utils/Fixtures.sol";
import {Order, OrderLib} from "../../src/libraries/OrderLib.sol";

/// @notice Batched optimistic fill (`fillBatch`, partial success) and the caller-directed
///         reimbursement-beneficiary flavour (`fillTo`). `msg.sender` always funds the payout; the
///         recorded filler (who is reimbursed at settle) is caller-chosen; the user-signed `recipient`
///         who receives the payout now is NEVER redirected by either flavour.
contract FastFillBatchTest is Fixtures {
    uint256 constant INPUT = 1_000e6;
    uint256 constant MAX_FEE = 1e6;
    uint64 constant WINDOW = 100;

    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp();
    }

    function _order(uint64 nonce) internal view returns (Order memory o) {
        o = _cctpOrder(INPUT, MAX_FEE, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, 0, 0, nonce);
        // recipient stays the fixture `recipient` (an EOA); empty hookData => plain delivery.
    }

    function _fundAndApprove(address who, uint256 amount) internal {
        usdc.mint(who, amount);
        vm.prank(who);
        usdc.approve(address(dstCctp), amount);
    }

    // ---------------------------------------------------------------------------------------------
    // fillTo — payer funds, beneficiary is the recorded/reimbursed filler
    // ---------------------------------------------------------------------------------------------

    function test_fillTo_fundsFromSender_recordsBeneficiary_recipientUntouched() public {
        address beneficiary = makeAddr("beneficiary");
        Order memory o = _order(1);
        vm.chainId(DST_CHAIN);
        (uint256 payout,) = dstCctp.quoteFill(o, block.timestamp);
        _fundAndApprove(relayer, payout);

        vm.prank(relayer);
        bytes32 orderId = dstCctp.fillTo(o, beneficiary);

        assertEq(usdc.balanceOf(recipient), payout, "user-signed recipient receives the payout");
        assertEq(dstCctp.getOrder(orderId).filler, beneficiary, "beneficiary recorded as filler");
        assertEq(usdc.balanceOf(relayer), 0, "payer funded the payout");
        assertEq(usdc.balanceOf(beneficiary), 0, "beneficiary is paid only at settle, not at fill");
    }

    function test_fillTo_beneficiaryReimbursedAtSettle_notThePayer() public {
        address beneficiary = makeAddr("beneficiary2");
        Order memory o = _order(2);
        vm.chainId(DST_CHAIN);
        (uint256 payout,) = dstCctp.quoteFill(o, block.timestamp);
        _fundAndApprove(relayer, payout);
        vm.prank(relayer);
        dstCctp.fillTo(o, beneficiary);

        // Bridge message arrives (arrived > outputAmount, so owed == outputAmount).
        dstCctp.settle(_cctpMessage(o, 4e5, bytes32(uint256(101))), "");

        assertEq(usdc.balanceOf(beneficiary), o.outputAmount, "beneficiary reimbursed at settle");
        assertEq(usdc.balanceOf(relayer), 0, "payer is not the reimbursement target");
    }

    function test_fillTo_zeroBeneficiary_defaultsToSender() public {
        Order memory o = _order(3);
        vm.chainId(DST_CHAIN);
        (uint256 payout,) = dstCctp.quoteFill(o, block.timestamp);
        _fundAndApprove(relayer, payout);
        vm.prank(relayer);
        bytes32 orderId = dstCctp.fillTo(o, address(0));
        assertEq(dstCctp.getOrder(orderId).filler, relayer, "zero beneficiary => msg.sender recorded");
    }

    // ---------------------------------------------------------------------------------------------
    // fillBatch — partial success, single funder + beneficiary for the batch
    // ---------------------------------------------------------------------------------------------

    function test_fillBatch_partialSuccess_skipsAlreadyFilled() public {
        Order memory o0 = _order(1);
        Order memory o1 = _order(2);
        Order memory o2 = _order(3);
        vm.chainId(DST_CHAIN);
        (uint256 p0,) = dstCctp.quoteFill(o0, block.timestamp);
        (uint256 p1,) = dstCctp.quoteFill(o1, block.timestamp);
        (uint256 p2,) = dstCctp.quoteFill(o2, block.timestamp);

        // Pre-fill o1 via a different filler so it is already active when the batch runs.
        address other = makeAddr("otherFiller");
        _fundAndApprove(other, p1);
        vm.prank(other);
        dstCctp.fill(o1);

        // Batch [o0, o1(already filled), o2], funded by relayer.
        _fundAndApprove(relayer, p0 + p2);
        Order[] memory orders = new Order[](3);
        orders[0] = o0;
        orders[1] = o1;
        orders[2] = o2;
        vm.prank(relayer);
        bool[] memory filled = dstCctp.fillBatch(orders, address(0));

        assertTrue(filled[0], "o0 filled");
        assertFalse(filled[1], "o1 already filled => skipped");
        assertTrue(filled[2], "o2 filled");

        assertEq(usdc.balanceOf(recipient), p0 + p1 + p2, "recipient got all three payouts");
        assertEq(usdc.balanceOf(relayer), 0, "payer funded exactly o0+o2 (no double-pull for the skip)");
        assertEq(dstCctp.getOrder(OrderLib.hash(o0)).filler, relayer, "o0 filler = batch payer");
        assertEq(dstCctp.getOrder(OrderLib.hash(o1)).filler, other, "o1 filler unchanged (the pre-filler)");
        assertEq(dstCctp.getOrder(OrderLib.hash(o2)).filler, relayer, "o2 filler = batch payer");
    }

    function test_fillBatch_recordsBeneficiaryForAll_recipientUntouched() public {
        address beneficiary = makeAddr("batchBeneficiary");
        Order memory o0 = _order(1);
        Order memory o1 = _order(2);
        vm.chainId(DST_CHAIN);
        (uint256 p0,) = dstCctp.quoteFill(o0, block.timestamp);
        (uint256 p1,) = dstCctp.quoteFill(o1, block.timestamp);
        _fundAndApprove(relayer, p0 + p1);

        Order[] memory orders = new Order[](2);
        orders[0] = o0;
        orders[1] = o1;
        vm.prank(relayer);
        bool[] memory filled = dstCctp.fillBatch(orders, beneficiary);

        assertTrue(filled[0] && filled[1], "both filled");
        assertEq(dstCctp.getOrder(OrderLib.hash(o0)).filler, beneficiary, "o0 beneficiary recorded");
        assertEq(dstCctp.getOrder(OrderLib.hash(o1)).filler, beneficiary, "o1 beneficiary recorded");
        assertEq(usdc.balanceOf(recipient), p0 + p1, "recipient (not beneficiary) receives both payouts");
        assertEq(usdc.balanceOf(beneficiary), 0, "beneficiary paid only at settle");
    }
}
