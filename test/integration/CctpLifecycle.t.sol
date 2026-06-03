// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../utils/Fixtures.sol";
import {CctpMessageBuilder} from "../utils/CctpMessageBuilder.sol";
import {Order, OrderLib, Execution} from "../../src/libraries/OrderLib.sol";
import {PricingLib} from "../../src/libraries/PricingLib.sol";
import {AddressCast} from "../../src/libraries/AddressCast.sol";
import {FastFillBase} from "../../src/FastFillBase.sol";
import {CctpAdapter} from "../../src/adapters/CctpAdapter.sol";
import {FillStatus, OrderRecord} from "../../src/interfaces/IFastFill.sol";
import {MockMessageTransmitterV2} from "../mocks/MockMessageTransmitterV2.sol";

contract CctpLifecycleTest is Fixtures {
    uint256 constant INPUT = 1_000e6;
    uint256 constant MAX_FEE = 1e6;
    uint256 constant RATE = 1e13; // 0.001% / sec
    uint64 constant WINDOW = 100;
    bytes internal constant HOOK = hex"feed";

    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp();
    }

    // ---------------------------------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------------------------------

    function _createOrder() internal returns (Order memory order, bytes32 orderId) {
        return _createOrderWithBase(0);
    }

    /// @dev `deliveryWindow` is now relative — the adapter derives `expectedDeliveryTime` on-chain as
    ///      `block.timestamp + WINDOW`, which equals `start + WINDOW` here (same block as `start`).
    function _createOrderWithBase(uint256 baseFee) internal returns (Order memory order, bytes32 orderId) {
        uint64 start = uint64(block.timestamp);
        usdc.mint(user, INPUT);
        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        usdc.approve(address(srcCctp), INPUT);
        uint64 nonce;
        (orderId, nonce) = srcCctp.initiateCCTP(
            DST_CHAIN, _b32(recipient), INPUT, MAX_FEE, 1000, WINDOW, RATE, baseFee, Execution(0, "")
        );
        vm.stopPrank();
        order = _cctpOrder(INPUT, MAX_FEE, start, start + WINDOW, RATE, baseFee, nonce);
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

    function _settle(Order memory order, uint256 feeExecuted, uint256 bridgeNonce) internal {
        vm.chainId(DST_CHAIN);
        dstCctp.settle(_cctpMessage(order, feeExecuted, bytes32(bridgeNonce)), "");
    }

    // ---------------------------------------------------------------------------------------------
    // Source
    // ---------------------------------------------------------------------------------------------

    function test_initiate_encodesOrderAndBurns() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        assertEq(orderId, OrderLib.hash(order));
        assertEq(keccak256(tokenMessenger.lastHookData()), keccak256(OrderLib.encode(order)), "hookData");
        assertEq(tokenMessenger.lastMintRecipient(), _b32(address(dstCctp)), "mintRecipient");
        assertEq(tokenMessenger.lastDestinationCaller(), _b32(address(dstCctp)), "destinationCaller");
        assertEq(tokenMessenger.lastAmount(), INPUT, "burn amount");
        assertEq(tokenMessenger.lastDestinationDomain(), DST_DOMAIN, "destination domain");
        assertEq(order.outputAmount, INPUT - MAX_FEE, "outputAmount");
        assertEq(usdc.balanceOf(user), 0, "user funds pulled");
    }

    function test_initiate_malformedRecipient_revertsBeforeBurn() public {
        bytes32 malformed = bytes32(uint256(uint160(recipient)) | (uint256(1) << 160));
        usdc.mint(user, INPUT);

        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        usdc.approve(address(srcCctp), INPUT);
        vm.expectRevert(abi.encodeWithSelector(AddressCast.InvalidAddress.selector, malformed));
        srcCctp.initiateCCTP(DST_CHAIN, malformed, INPUT, MAX_FEE, 1000, WINDOW, RATE, 0, Execution(0, ""));
        vm.stopPrank();

        assertEq(tokenMessenger.burnCount(), 0, "no CCTP burn");
        assertEq(usdc.balanceOf(user), INPUT, "funds not pulled");
    }

    function test_initiate_zeroRecipient_revertsBeforeBurn() public {
        usdc.mint(user, INPUT);

        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        usdc.approve(address(srcCctp), INPUT);
        vm.expectRevert(FastFillBase.ZeroRecipient.selector);
        srcCctp.initiateCCTP(DST_CHAIN, bytes32(0), INPUT, MAX_FEE, 1000, WINDOW, RATE, 0, Execution(0, ""));
        vm.stopPrank();

        assertEq(tokenMessenger.burnCount(), 0, "no CCTP burn");
        assertEq(usdc.balanceOf(user), INPUT, "funds not pulled");
    }

    function test_initiate_callbackGasLimitTooHigh_revertsBeforeBurn() public {
        uint64 maxGas = srcCctp.MAX_CALLBACK_GAS_LIMIT();
        uint64 tooHigh = maxGas + 1;
        usdc.mint(user, INPUT);

        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        usdc.approve(address(srcCctp), INPUT);
        vm.expectRevert(abi.encodeWithSelector(FastFillBase.InvalidCallbackGasLimit.selector, tooHigh, maxGas));
        srcCctp.initiateCCTP(
            DST_CHAIN, _b32(recipient), INPUT, MAX_FEE, 1000, WINDOW, RATE, 0, Execution(tooHigh, HOOK)
        );
        vm.stopPrank();

        assertEq(tokenMessenger.burnCount(), 0, "no CCTP burn");
        assertEq(usdc.balanceOf(user), INPUT, "funds not pulled");
    }

    function test_initiate_maxCallbackGasLimitAccepted() public {
        uint64 maxGas = srcCctp.MAX_CALLBACK_GAS_LIMIT();
        uint64 start = uint64(block.timestamp);
        usdc.mint(user, INPUT);

        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        usdc.approve(address(srcCctp), INPUT);
        (bytes32 orderId, uint64 nonce) = srcCctp.initiateCCTP(
            DST_CHAIN, _b32(recipient), INPUT, MAX_FEE, 1000, WINDOW, RATE, 0, Execution(maxGas, HOOK)
        );
        vm.stopPrank();

        Order memory order = _cctpOrder(INPUT, MAX_FEE, start, start + WINDOW, RATE, 0, nonce);
        order.callbackGasLimit = maxGas;
        order.hookData = HOOK;
        assertEq(orderId, OrderLib.hash(order));
        assertEq(keccak256(tokenMessenger.lastHookData()), keccak256(OrderLib.encode(order)), "hookData");
        assertEq(tokenMessenger.burnCount(), 1, "CCTP burn dispatched");
    }

    // ---------------------------------------------------------------------------------------------
    // Happy path
    // ---------------------------------------------------------------------------------------------

    function test_fill_paysDiscountedAndRecordsFiller() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        uint256 fillTime = order.startTime + 20;
        uint256 payout = _fillAt(order, relayer, fillTime);

        uint256 expFee = PricingLib.fee(
            order.outputAmount, order.startTime, order.expectedDeliveryTime, fillTime, RATE, MAX_FEE_RATE, order.baseFee
        );
        assertEq(payout, order.outputAmount - expFee, "payout");
        assertGt(expFee, 0, "fee charged");
        assertEq(usdc.balanceOf(recipient), payout, "recipient paid");

        OrderRecord memory rec = dstCctp.getOrder(orderId);
        assertEq(rec.filler, relayer, "filler recorded");
        assertEq(uint8(rec.status), uint8(FillStatus.Filled), "status filled");
    }

    function test_fillThenSettle_reimbursesFiller_surplusToRecipient() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        uint256 fillTime = order.startTime + 20;
        uint256 payout = _fillAt(order, relayer, fillTime);

        uint256 feeExecuted = 4e5; // < MAX_FEE, so a surplus arrives
        _settle(order, feeExecuted, 1);

        uint256 arrived = INPUT - feeExecuted;
        uint256 owed = order.outputAmount;
        uint256 surplus = arrived - owed;

        assertEq(usdc.balanceOf(relayer), owed, "filler reimbursed outputAmount");
        assertEq(usdc.balanceOf(recipient), payout + surplus, "recipient: payout + surplus");
        assertEq(uint8(dstCctp.getOrder(orderId).status), uint8(FillStatus.Settled), "settled");
        // The relayer's realized profit is exactly the fill fee it charged (owed - what it paid out).
        uint256 expFee = PricingLib.fee(
            order.outputAmount, order.startTime, order.expectedDeliveryTime, fillTime, RATE, MAX_FEE_RATE, order.baseFee
        );
        assertEq(owed - payout, expFee, "relayer profit == fill fee");
    }

    function test_settleUnfilled_paysRecipientEverything() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        uint256 feeExecuted = 4e5;
        _settle(order, feeExecuted, 2);

        uint256 arrived = INPUT - feeExecuted;
        assertEq(usdc.balanceOf(recipient), arrived, "recipient gets all");
        assertEq(usdc.balanceOf(relayer), 0, "no filler");
        assertEq(uint8(dstCctp.getOrder(orderId).status), uint8(FillStatus.Settled), "settled");
    }

    function test_baseFee_flatPremiumAppliedAndReimbursed() public {
        uint256 baseFee = 5e5; // $0.50 flat, additive to the time premium
        (Order memory order,) = _createOrderWithBase(baseFee);
        uint256 fillTime = order.startTime + 20;
        uint256 payout = _fillAt(order, relayer, fillTime);

        uint256 expFee = PricingLib.fee(
            order.outputAmount, order.startTime, order.expectedDeliveryTime, fillTime, RATE, MAX_FEE_RATE, baseFee
        );
        assertGe(expFee, baseFee, "fee includes at least the flat base");
        assertEq(payout, order.outputAmount - expFee, "payout reduced by base + time fee");
        assertEq(usdc.balanceOf(recipient), payout, "recipient paid the discounted payout");

        // Settle still reimburses the filler exactly outputAmount; surplus goes to the recipient.
        _settle(order, 4e5, 30);
        uint256 surplus = (INPUT - 4e5) - order.outputAmount;
        assertEq(usdc.balanceOf(relayer), order.outputAmount, "filler reimbursed outputAmount");
        assertEq(usdc.balanceOf(recipient), payout + surplus, "recipient: payout + surplus");
    }

    function test_initiate_baseFeeTooHigh_reverts() public {
        usdc.mint(user, INPUT);
        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        usdc.approve(address(srcCctp), INPUT);
        // baseFee >= outputAmount (INPUT - MAX_FEE) leaves the recipient nothing at fill -> rejected.
        vm.expectRevert(abi.encodeWithSelector(FastFillBase.InvalidBaseFee.selector, INPUT - MAX_FEE, INPUT - MAX_FEE));
        srcCctp.initiateCCTP(
            DST_CHAIN, _b32(recipient), INPUT, MAX_FEE, 1000, WINDOW, RATE, INPUT - MAX_FEE, Execution(0, "")
        );
        vm.stopPrank();
    }

    // ---------------------------------------------------------------------------------------------
    // Races
    // ---------------------------------------------------------------------------------------------

    function test_doubleFill_reverts() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        _fillAt(order, relayer, order.startTime + 10);

        vm.chainId(DST_CHAIN);
        vm.expectRevert(abi.encodeWithSelector(FastFillBase.OrderAlreadyActive.selector, orderId));
        vm.prank(relayer2);
        dstCctp.fill(order);
    }

    function test_twoRelayers_firstWinsAndIsReimbursed() public {
        (Order memory order,) = _createOrder();
        _fillAt(order, relayer, order.startTime + 10);

        vm.chainId(DST_CHAIN);
        vm.expectRevert();
        vm.prank(relayer2);
        dstCctp.fill(order);

        _settle(order, 4e5, 3);
        assertEq(usdc.balanceOf(relayer), order.outputAmount, "first relayer reimbursed");
        assertEq(usdc.balanceOf(relayer2), 0, "loser not paid");
    }

    function test_fillAfterSettle_reverts() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        _settle(order, 4e5, 4);

        vm.chainId(DST_CHAIN);
        vm.expectRevert(abi.encodeWithSelector(FastFillBase.OrderAlreadyActive.selector, orderId));
        vm.prank(relayer);
        dstCctp.fill(order);
    }

    function test_settleSameMessageTwice_revertsOnBridgeNonce() public {
        (Order memory order,) = _createOrder();
        _settle(order, 4e5, 5);

        vm.chainId(DST_CHAIN);
        bytes memory message = _cctpMessage(order, 4e5, bytes32(uint256(5)));
        vm.expectRevert(abi.encodeWithSelector(MockMessageTransmitterV2.NonceAlreadyUsed.selector, bytes32(uint256(5))));
        dstCctp.settle(message, "");
    }

    function test_settleSameOrderDifferentMessage_revertsAlreadySettled() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        _settle(order, 4e5, 6);

        // A second, distinct bridge message carrying the SAME order cannot double-disburse.
        vm.chainId(DST_CHAIN);
        bytes memory message2 = _cctpMessage(order, 4e5, bytes32(uint256(7)));
        vm.expectRevert(abi.encodeWithSelector(FastFillBase.AlreadySettled.selector, orderId));
        dstCctp.settle(message2, "");
    }

    // ---------------------------------------------------------------------------------------------
    // Adversarial
    // ---------------------------------------------------------------------------------------------

    function test_fakeOrderFill_selfPunishes() public {
        // An order never created on the source: the filler pays the recipient from its own funds
        // and is never reimbursed because no matching message will ever arrive.
        Order memory fake =
            _cctpOrder(500e6, MAX_FEE, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, 999);
        bytes32 fakeId = OrderLib.hash(fake);

        uint256 payout = _fillAt(fake, relayer, fake.startTime + 10);

        assertEq(usdc.balanceOf(recipient), payout, "recipient paid by relayer");
        assertEq(usdc.balanceOf(relayer), 0, "relayer out of pocket");
        assertEq(dstCctp.getOrder(fakeId).filler, relayer, "filler recorded");
        assertEq(dstCctp.claimable(relayer, address(usdc)), 0, "relayer has no claim");
        assertEq(uint8(dstCctp.getOrder(fakeId).status), uint8(FillStatus.Filled), "never settles");
    }

    function test_settle_forgedAttestation_rejected() public {
        (Order memory order,) = _createOrder();
        transmitter.setAcceptAttestation(false);

        vm.chainId(DST_CHAIN);
        bytes memory message = _cctpMessage(order, 4e5, bytes32(uint256(8)));
        vm.expectRevert(MockMessageTransmitterV2.AttestationRejected.selector);
        dstCctp.settle(message, "");
    }

    function test_settle_untrustedSender_rejected() public {
        (Order memory order,) = _createOrder();
        address attacker = makeAddr("attacker");

        // A real CCTP burn that anyone could make: mintRecipient = our adapter, but messageSender
        // is the attacker (not our registered source adapter).
        bytes memory message = CctpMessageBuilder.build(
            CctpMessageBuilder.Msg({
                sourceDomain: SRC_DOMAIN,
                destinationDomain: DST_DOMAIN,
                nonce: bytes32(uint256(9)),
                destinationCaller: _b32(address(dstCctp)),
                burnToken: _b32(address(usdc)),
                mintRecipient: _b32(address(dstCctp)),
                amount: order.inputAmount,
                messageSender: _b32(attacker),
                maxFee: order.inputAmount - order.outputAmount,
                feeExecuted: 4e5,
                hookData: OrderLib.encode(order)
            })
        );
        vm.chainId(DST_CHAIN);
        vm.expectRevert(abi.encodeWithSelector(CctpAdapter.UntrustedSender.selector, _b32(attacker)));
        dstCctp.settle(message, "");
    }

    function test_settle_wrongDestinationCaller_rejected() public {
        (Order memory order,) = _createOrder();
        address attacker = makeAddr("attacker");

        bytes memory message = CctpMessageBuilder.build(
            CctpMessageBuilder.Msg({
                sourceDomain: SRC_DOMAIN,
                destinationDomain: DST_DOMAIN,
                nonce: bytes32(uint256(10)),
                destinationCaller: _b32(attacker), // not the dst adapter
                burnToken: _b32(address(usdc)),
                mintRecipient: _b32(address(dstCctp)),
                amount: order.inputAmount,
                messageSender: _b32(address(srcCctp)),
                maxFee: order.inputAmount - order.outputAmount,
                feeExecuted: 4e5,
                hookData: OrderLib.encode(order)
            })
        );
        vm.chainId(DST_CHAIN);
        vm.expectRevert(
            abi.encodeWithSelector(
                MockMessageTransmitterV2.UnauthorizedCaller.selector, address(dstCctp), _b32(attacker)
            )
        );
        dstCctp.settle(message, "");
    }

    function test_settle_mintRecipientMismatch_rejected() public {
        (Order memory order,) = _createOrder();
        address attacker = makeAddr("attacker");

        bytes memory message = CctpMessageBuilder.build(
            CctpMessageBuilder.Msg({
                sourceDomain: SRC_DOMAIN,
                destinationDomain: DST_DOMAIN,
                nonce: bytes32(uint256(11)),
                destinationCaller: _b32(address(dstCctp)),
                burnToken: _b32(address(usdc)),
                mintRecipient: _b32(attacker), // minted elsewhere
                amount: order.inputAmount,
                messageSender: _b32(address(srcCctp)),
                maxFee: order.inputAmount - order.outputAmount,
                feeExecuted: 4e5,
                hookData: OrderLib.encode(order)
            })
        );
        vm.chainId(DST_CHAIN);
        vm.expectRevert(abi.encodeWithSelector(CctpAdapter.MintRecipientMismatch.selector, _b32(attacker)));
        dstCctp.settle(message, "");
    }

    // ---------------------------------------------------------------------------------------------
    // Pull-payment fallback
    // ---------------------------------------------------------------------------------------------

    function test_settle_recipientBlocked_fallbackThenClaim() public {
        (Order memory order,) = _createOrder();
        usdc.setBlocked(recipient, true);

        _settle(order, 4e5, 12);
        uint256 arrived = INPUT - 4e5;
        assertEq(usdc.balanceOf(recipient), 0, "push failed");
        assertEq(dstCctp.claimable(recipient, address(usdc)), arrived, "credited to ledger");

        usdc.setBlocked(recipient, false);
        vm.prank(recipient);
        uint256 claimed = dstCctp.claim(address(usdc));
        assertEq(claimed, arrived, "claimed");
        assertEq(usdc.balanceOf(recipient), arrived, "recipient recovered funds");
    }

    function test_fill_recipientBlocked_fallback() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        usdc.setBlocked(recipient, true);

        uint256 payout = _fillAt(order, relayer, order.startTime + 10);
        assertEq(usdc.balanceOf(recipient), 0, "push failed");
        assertEq(dstCctp.claimable(recipient, address(usdc)), payout, "credited to ledger");
        assertEq(dstCctp.getOrder(orderId).filler, relayer, "still recorded as filled");
    }

    // ---------------------------------------------------------------------------------------------
    // Admin
    // ---------------------------------------------------------------------------------------------

    function test_paused_blocksFill() public {
        (Order memory order,) = _createOrder();
        vm.prank(owner);
        dstCctp.setPaused(true);

        vm.chainId(DST_CHAIN);
        vm.expectRevert(FastFillBase.Paused.selector);
        vm.prank(relayer);
        dstCctp.fill(order);
    }
}
