// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../utils/Fixtures.sol";
import {CctpMessageBuilder} from "../utils/CctpMessageBuilder.sol";
import {MockCctpExecReceiver} from "../mocks/MockCctpExecReceiver.sol";
import {MockMessageTransmitterV2} from "../mocks/MockMessageTransmitterV2.sol";
import {Order, OrderLib, Execution} from "../../src/libraries/OrderLib.sol";
import {ExecHook, ExecHookLib} from "../../src/libraries/ExecHookLib.sol";
import {AddressCast} from "../../src/libraries/AddressCast.sol";
import {FastFillBase} from "../../src/FastFillBase.sol";
import {CctpAdapter} from "../../src/adapters/CctpAdapter.sol";
import {CctpExecutor} from "../../src/CctpExecutor.sol";
import {FillStatus, OrderRecord} from "../../src/interfaces/IFastFill.sol";

contract CctpExecutorTest is Fixtures {
    using AddressCast for address;

    uint256 constant INPUT = 1_000e6;
    uint256 constant MAX_FEE = 1e6;
    uint256 constant MINT_FEE = 2e6;
    uint256 constant RATE = 1e13;
    uint64 constant WINDOW = 100;
    bytes internal constant HOOK = hex"feed";
    address internal constant GENERIC_BURNER = address(uint160(0xBEEF));

    address mintRelayer = makeAddr("mintRelayer");

    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp();
    }

    function _createRoutedOrder(uint64 callbackGasLimit, bytes memory hook)
        internal
        returns (Order memory order, bytes32 orderId)
    {
        uint64 start = uint64(block.timestamp);
        usdc.mint(user, INPUT);
        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        usdc.approve(address(srcCctp), INPUT);
        uint64 nonce;
        (orderId, nonce) = srcCctp.initiateCCTP(
            DST_CHAIN,
            _b32(recipient),
            INPUT,
            MAX_FEE,
            MINT_FEE,
            1000,
            WINDOW,
            RATE,
            0,
            Execution(callbackGasLimit, hook)
        );
        vm.stopPrank();

        order = _cctpOrder(INPUT, MAX_FEE + MINT_FEE, start, start + WINDOW, RATE, 0, nonce);
        order.callbackGasLimit = callbackGasLimit;
        order.hookData = hook;
    }

    function _fillAt(Order memory order, address filler, uint256 fillTime) internal returns (uint256 payout) {
        vm.chainId(DST_CHAIN);
        vm.warp(fillTime);
        (payout,) = dstCctp.quoteFill(order, fillTime);
        usdc.mint(filler, payout);
        vm.startPrank(filler);
        usdc.approve(address(dstCctp), payout);
        dstCctp.fill(order);
        vm.stopPrank();
    }

    function _executeRouted(Order memory order, uint256 feeExecuted, bytes32 nonce) internal {
        vm.chainId(DST_CHAIN);
        vm.prank(mintRelayer);
        cctpExec.execute(_cctpRoutedMessage(order, MINT_FEE, feeExecuted, nonce), "");
    }

    function _execMessage(ExecHook memory h, uint256 amount, uint256 feeExecuted, bytes32 nonce)
        internal
        view
        returns (bytes memory)
    {
        return CctpMessageBuilder.build(
            CctpMessageBuilder.Msg({
                sourceDomain: SRC_DOMAIN,
                destinationDomain: DST_DOMAIN,
                nonce: nonce,
                destinationCaller: address(cctpExec).toBytes32(),
                burnToken: address(usdc).toBytes32(),
                mintRecipient: address(cctpExec).toBytes32(),
                amount: amount,
                messageSender: GENERIC_BURNER.toBytes32(),
                maxFee: MAX_FEE,
                feeExecuted: feeExecuted,
                hookData: ExecHookLib.encode(h)
            })
        );
    }

    function test_initiateRouted_encodesExecHookAndBurnsToExecutor() public {
        (Order memory order, bytes32 orderId) = _createRoutedOrder(200_000, HOOK);

        assertEq(orderId, OrderLib.hash(order), "orderId");
        assertEq(order.outputAmount, INPUT - MAX_FEE - MINT_FEE, "outputAmount");
        assertEq(tokenMessenger.lastMintRecipient(), address(cctpExec).toBytes32(), "mintRecipient");
        assertEq(tokenMessenger.lastDestinationCaller(), address(cctpExec).toBytes32(), "destinationCaller");
        assertEq(tokenMessenger.lastMaxFee(), MAX_FEE, "CCTP maxFee excludes mintFee");

        ExecHook memory h = ExecHookLib.decode(tokenMessenger.lastHookData());
        assertEq(h.mintFee, MINT_FEE, "mintFee");
        assertEq(h.target, address(srcCctp).toBytes32(), "target adapter");
        assertEq(h.gasLimit, uint64(550_000), "callback gas + settle overhead");
        assertEq(h.refundTo, _b32(recipient), "refundTo recipient");
        assertEq(OrderLib.hash(OrderLib.decode(h.payload)), orderId, "payload order");
    }

    function test_executeRouted_filled_paysMintRelayerAndSettlesFiller() public {
        (Order memory order, bytes32 orderId) = _createRoutedOrder(0, "");
        uint256 fillTime = order.startTime + 20;
        uint256 payout = _fillAt(order, relayer, fillTime);

        uint256 feeExecuted = 4e5;
        _executeRouted(order, feeExecuted, bytes32(uint256(1)));

        uint256 forwarded = INPUT - feeExecuted - MINT_FEE;
        uint256 surplus = forwarded - order.outputAmount;
        assertEq(usdc.balanceOf(mintRelayer), MINT_FEE, "mint relayer paid");
        assertEq(usdc.balanceOf(relayer), order.outputAmount, "filler reimbursed");
        assertEq(usdc.balanceOf(recipient), payout + surplus, "recipient got fill payout plus surplus");
        assertEq(usdc.balanceOf(address(cctpExec)), 0, "executor empty");

        OrderRecord memory rec = dstCctp.getOrder(orderId);
        assertEq(uint8(rec.status), uint8(FillStatus.Settled), "settled");
        assertEq(rec.filler, relayer, "filler preserved");
    }

    function test_executeRouted_unfilled_paysMintRelayerAndRecipient() public {
        (Order memory order, bytes32 orderId) = _createRoutedOrder(0, "");
        uint256 feeExecuted = 4e5;

        _executeRouted(order, feeExecuted, bytes32(uint256(2)));

        uint256 forwarded = INPUT - feeExecuted - MINT_FEE;
        assertEq(usdc.balanceOf(mintRelayer), MINT_FEE, "mint relayer paid");
        assertEq(usdc.balanceOf(recipient), forwarded, "recipient receives net minted funds");
        assertEq(usdc.balanceOf(address(cctpExec)), 0, "executor empty");
        assertEq(uint8(dstCctp.getOrder(orderId).status), uint8(FillStatus.Settled), "settled");
    }

    function test_executeRouted_feeExecutedEqualsMaxFee_noSurplus() public {
        (Order memory order,) = _createRoutedOrder(0, "");
        uint256 payout = _fillAt(order, relayer, order.startTime + 20);

        _executeRouted(order, MAX_FEE, bytes32(uint256(3)));

        assertEq(usdc.balanceOf(mintRelayer), MINT_FEE, "mint relayer paid");
        assertEq(usdc.balanceOf(relayer), order.outputAmount, "filler reimbursed");
        assertEq(usdc.balanceOf(recipient), payout, "no surplus");
    }

    function test_executeRouted_finalizedNoCctpFee() public {
        uint64 start = uint64(block.timestamp);
        usdc.mint(user, INPUT);
        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        usdc.approve(address(srcCctp), INPUT);
        (bytes32 orderId, uint64 nonce) = srcCctp.initiateCCTP(
            DST_CHAIN, _b32(recipient), INPUT, 0, MINT_FEE, 2000, WINDOW, RATE, 0, Execution(0, "")
        );
        vm.stopPrank();

        Order memory order = _cctpOrder(INPUT, MINT_FEE, start, start + WINDOW, RATE, 0, nonce);
        assertEq(orderId, OrderLib.hash(order), "orderId");

        vm.chainId(DST_CHAIN);
        vm.prank(mintRelayer);
        cctpExec.execute(_cctpRoutedMessage(order, MINT_FEE, 0, bytes32(uint256(4))), "");

        assertEq(usdc.balanceOf(mintRelayer), MINT_FEE, "mint relayer paid");
        assertEq(usdc.balanceOf(recipient), INPUT - MINT_FEE, "recipient gets finalized net");
        assertEq(uint8(dstCctp.getOrder(orderId).status), uint8(FillStatus.Settled), "settled");
    }

    function test_mutualExclusivity_directSettleCannotConsumeRoutedMessage() public {
        (Order memory order,) = _createRoutedOrder(0, "");
        bytes32 nonce = bytes32(uint256(5));
        bytes memory message = _cctpRoutedMessage(order, MINT_FEE, 4e5, nonce);

        vm.chainId(DST_CHAIN);
        vm.expectRevert(
            abi.encodeWithSelector(
                MockMessageTransmitterV2.UnauthorizedCaller.selector, address(dstCctp), address(cctpExec).toBytes32()
            )
        );
        dstCctp.settle(message, "");
        assertFalse(transmitter.usedNonces(nonce), "nonce remains redeemable");
    }

    function test_mutualExclusivity_executorCannotConsumeDirectMessage() public {
        uint64 start = uint64(block.timestamp);
        usdc.mint(user, INPUT);
        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        usdc.approve(address(srcCctp), INPUT);
        (, uint64 nonce) =
            srcCctp.initiateCCTP(DST_CHAIN, _b32(recipient), INPUT, MAX_FEE, 0, 1000, WINDOW, RATE, 0, Execution(0, ""));
        vm.stopPrank();

        Order memory order = _cctpOrder(INPUT, MAX_FEE, start, start + WINDOW, RATE, 0, nonce);
        bytes32 bridgeNonce = bytes32(uint256(6));
        bytes memory message = _cctpMessage(order, 4e5, bridgeNonce);

        vm.chainId(DST_CHAIN);
        vm.expectRevert(
            abi.encodeWithSelector(
                MockMessageTransmitterV2.UnauthorizedCaller.selector, address(cctpExec), address(dstCctp).toBytes32()
            )
        );
        cctpExec.execute(message, "");
        assertFalse(transmitter.usedNonces(bridgeNonce), "nonce remains redeemable");
    }

    function test_onCctpExecute_rejectsForgedSender() public {
        (Order memory order,) = _createRoutedOrder(0, "");
        address attacker = makeAddr("attacker");

        vm.chainId(DST_CHAIN);
        vm.prank(address(cctpExec));
        vm.expectRevert(abi.encodeWithSelector(CctpAdapter.UntrustedSender.selector, attacker.toBytes32()));
        dstCctp.onCctpExecute(SRC_DOMAIN, attacker.toBytes32(), address(usdc), INPUT, OrderLib.encode(order));
    }

    function test_forwardOnly_deliversWithoutHook() public {
        address plainRecipient = makeAddr("plainExecRecipient");
        ExecHook memory h = ExecHook({
            mintFee: MINT_FEE,
            target: plainRecipient.toBytes32(),
            gasLimit: 0,
            refundTo: plainRecipient.toBytes32(),
            payload: hex""
        });
        uint256 feeExecuted = 4e5;
        bytes memory message = _execMessage(h, INPUT, feeExecuted, bytes32(uint256(7)));

        vm.chainId(DST_CHAIN);
        vm.prank(mintRelayer);
        cctpExec.execute(message, "");

        assertEq(usdc.balanceOf(mintRelayer), MINT_FEE, "mint relayer paid");
        assertEq(usdc.balanceOf(plainRecipient), INPUT - feeExecuted - MINT_FEE, "forwarded");
    }

    function test_hookMode_success_callsReceiverWithAuthenticatedFields() public {
        MockCctpExecReceiver receiver = new MockCctpExecReceiver();
        bytes32 sender = GENERIC_BURNER.toBytes32();
        bytes memory payload = hex"c0ffee";
        ExecHook memory h = ExecHook({
            mintFee: MINT_FEE,
            target: address(receiver).toBytes32(),
            gasLimit: 200_000,
            refundTo: recipient.toBytes32(),
            payload: payload
        });
        uint256 feeExecuted = 4e5;

        vm.chainId(DST_CHAIN);
        vm.prank(mintRelayer);
        cctpExec.execute(_execMessage(h, INPUT, feeExecuted, bytes32(uint256(8))), "");

        uint256 forwarded = INPUT - feeExecuted - MINT_FEE;
        assertEq(receiver.callCount(), 1, "hook called");
        assertEq(receiver.lastSourceDomain(), SRC_DOMAIN, "sourceDomain");
        assertEq(receiver.lastSender(), sender, "sender");
        assertEq(receiver.lastUsdc(), address(usdc), "usdc");
        assertEq(receiver.lastAmount(), forwarded, "amount");
        assertEq(receiver.lastPayload(), payload, "payload");
        assertEq(usdc.balanceOf(address(receiver)), forwarded, "receiver kept funds");
    }

    function test_hookStealsThenReverts_clawsBackToRefundClaimant() public {
        MockCctpExecReceiver receiver = new MockCctpExecReceiver();
        address thief = makeAddr("execThief");
        address refundTo = makeAddr("execRefund");
        receiver.setMode(MockCctpExecReceiver.Mode.StealThenRevert);
        receiver.setStash(thief);

        ExecHook memory h = ExecHook({
            mintFee: MINT_FEE,
            target: address(receiver).toBytes32(),
            gasLimit: 250_000,
            refundTo: refundTo.toBytes32(),
            payload: hex""
        });
        uint256 feeExecuted = 4e5;
        vm.chainId(DST_CHAIN);
        vm.prank(mintRelayer);
        cctpExec.execute(_execMessage(h, INPUT, feeExecuted, bytes32(uint256(9))), "");

        uint256 forwarded = INPUT - feeExecuted - MINT_FEE;
        assertEq(usdc.balanceOf(thief), 0, "stolen transfer rolled back");
        assertEq(usdc.balanceOf(address(receiver)), 0, "receiver kept nothing");
        assertEq(cctpExec.claimable(refundTo, address(usdc)), forwarded, "refund claimant credited");
        assertEq(usdc.balanceOf(mintRelayer), MINT_FEE, "mint relayer still paid");
    }

    function test_mintFeeToBlacklistedRelayer_revertsAndIsRetryable() public {
        (Order memory order,) = _createRoutedOrder(0, "");
        bytes32 nonce = bytes32(uint256(10));
        bytes memory message = _cctpRoutedMessage(order, MINT_FEE, 4e5, nonce);

        usdc.setBlocked(mintRelayer, true);
        vm.chainId(DST_CHAIN);
        vm.prank(mintRelayer);
        vm.expectRevert();
        cctpExec.execute(message, "");
        assertFalse(transmitter.usedNonces(nonce), "nonce unconsumed");
        assertEq(usdc.balanceOf(address(cctpExec)), 0, "mint rolled back");

        usdc.setBlocked(mintRelayer, false);
        vm.prank(mintRelayer);
        cctpExec.execute(message, "");
        assertTrue(transmitter.usedNonces(nonce), "retry consumed nonce");
        assertEq(usdc.balanceOf(mintRelayer), MINT_FEE, "relayer paid on retry");
    }

    function test_invalidRefundToExecutor_revertsAndIsRetryable() public {
        MockCctpExecReceiver receiver = new MockCctpExecReceiver();
        ExecHook memory h = ExecHook({
            mintFee: MINT_FEE,
            target: address(receiver).toBytes32(),
            gasLimit: 200_000,
            refundTo: address(cctpExec).toBytes32(),
            payload: hex""
        });
        bytes32 nonce = bytes32(uint256(11));
        bytes memory message = _execMessage(h, INPUT, 4e5, nonce);

        vm.chainId(DST_CHAIN);
        vm.prank(mintRelayer);
        vm.expectRevert(abi.encodeWithSelector(CctpExecutor.InvalidRefundTo.selector, address(cctpExec)));
        cctpExec.execute(message, "");
        assertFalse(transmitter.usedNonces(nonce), "nonce unconsumed");
    }

    function test_routedGasLimitAboveExecutorCap_revertsBeforeBurn() public {
        uint64 max = srcCctp.MAX_CALLBACK_GAS_LIMIT();
        uint64 tooHighForRouted = max - 350_000 + 1;
        usdc.mint(user, INPUT);
        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        usdc.approve(address(srcCctp), INPUT);
        vm.expectRevert(abi.encodeWithSelector(FastFillBase.InvalidCallbackGasLimit.selector, max + 1, max));
        srcCctp.initiateCCTP(
            DST_CHAIN, _b32(recipient), INPUT, MAX_FEE, MINT_FEE, 1000, WINDOW, RATE, 0, Execution(tooHighForRouted, "")
        );
        vm.stopPrank();

        assertEq(tokenMessenger.burnCount(), 0, "no CCTP burn");
        assertEq(usdc.balanceOf(user), INPUT, "funds not pulled");
    }

    // ---------------------------------------------------------------------------------------------
    // Batched relay (executeBatch) + caller-directed fee (executeTo)
    // ---------------------------------------------------------------------------------------------

    function _fwdMessage(address target, uint256 mintFee, uint256 amount, uint256 feeExecuted, bytes32 nonce)
        internal
        view
        returns (bytes memory)
    {
        ExecHook memory h = ExecHook({
            mintFee: mintFee,
            target: target.toBytes32(),
            gasLimit: 0, // forward-only
            refundTo: target.toBytes32(),
            payload: hex""
        });
        return _execMessage(h, amount, feeExecuted, nonce);
    }

    function test_executeBatch_partialSuccess_skipsAlreadyRelayedAndInvalid() public {
        address treasury = makeAddr("treasury");
        address t0 = makeAddr("t0");
        address t3 = makeAddr("t3");
        uint256 fe = 4e5;
        uint256 forwarded = INPUT - fe - MINT_FEE;

        bytes memory m0 = _fwdMessage(t0, MINT_FEE, INPUT, fe, bytes32(uint256(20)));
        bytes memory m1 = _fwdMessage(makeAddr("t1"), MINT_FEE, INPUT, fe, bytes32(uint256(21)));
        bytes memory m2 = _fwdMessage(makeAddr("t2"), INPUT, INPUT, fe, bytes32(uint256(22))); // mintFee > minted
        bytes memory m3 = _fwdMessage(t3, MINT_FEE, INPUT, fe, bytes32(uint256(23)));

        // Pre-relay m1 (via a different relayer) so its nonce is already consumed.
        vm.chainId(DST_CHAIN);
        vm.prank(makeAddr("otherRelayer"));
        cctpExec.execute(m1, "");

        bytes[] memory msgs = new bytes[](4);
        msgs[0] = m0;
        msgs[1] = m1;
        msgs[2] = m2;
        msgs[3] = m3;
        bytes[] memory atts = new bytes[](4);

        vm.prank(mintRelayer);
        bool[] memory filled = cctpExec.executeBatch(msgs, atts, treasury);

        assertEq(filled.length, 4, "result length");
        assertTrue(filled[0], "m0 filled");
        assertFalse(filled[1], "m1 already relayed => skipped");
        assertFalse(filled[2], "m2 invalid (mintFee>minted) => skipped");
        assertTrue(filled[3], "m3 filled");

        // Only m0 + m3 delivered; treasury (not the submitter) earned both fees.
        assertEq(usdc.balanceOf(t0), forwarded, "t0 forwarded");
        assertEq(usdc.balanceOf(t3), forwarded, "t3 forwarded");
        assertEq(usdc.balanceOf(treasury), 2 * MINT_FEE, "treasury earned both successful fees");
        assertEq(usdc.balanceOf(mintRelayer), 0, "submitter earned no fee (redirected)");

        // The skipped invalid item's nonce stays redeemable; the successes consumed theirs.
        assertFalse(transmitter.usedNonces(bytes32(uint256(22))), "m2 nonce unconsumed");
        assertTrue(transmitter.usedNonces(bytes32(uint256(20))), "m0 consumed");
        assertTrue(transmitter.usedNonces(bytes32(uint256(23))), "m3 consumed");
    }

    function test_executeBatch_lengthMismatch_reverts() public {
        bytes[] memory msgs = new bytes[](2);
        bytes[] memory atts = new bytes[](1);
        vm.chainId(DST_CHAIN);
        vm.expectRevert(CctpExecutor.LengthMismatch.selector);
        cctpExec.executeBatch(msgs, atts, address(0));
    }

    function test_executeTo_redirectsFeeButNotDelivery() public {
        address treasury = makeAddr("treasury2");
        address target = makeAddr("execTarget");
        uint256 fe = 4e5;
        bytes memory m = _fwdMessage(target, MINT_FEE, INPUT, fe, bytes32(uint256(30)));

        vm.chainId(DST_CHAIN);
        vm.prank(mintRelayer);
        cctpExec.executeTo(m, "", treasury);

        // Fee follows the caller's choice; the attested delivery target is untouched.
        assertEq(usdc.balanceOf(treasury), MINT_FEE, "fee to treasury");
        assertEq(usdc.balanceOf(mintRelayer), 0, "submitter earned nothing");
        assertEq(usdc.balanceOf(target), INPUT - fe - MINT_FEE, "delivery unchanged");
    }

    function test_executeTo_zeroFeeRecipient_defaultsToSender() public {
        address target = makeAddr("execTarget2");
        bytes memory m = _fwdMessage(target, MINT_FEE, INPUT, 4e5, bytes32(uint256(31)));
        vm.chainId(DST_CHAIN);
        vm.prank(mintRelayer);
        cctpExec.executeTo(m, "", address(0));
        assertEq(usdc.balanceOf(mintRelayer), MINT_FEE, "fee defaults to msg.sender");
    }
}
