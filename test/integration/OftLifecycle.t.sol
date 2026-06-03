// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../utils/Fixtures.sol";
import {Order, OrderLib, Execution} from "../../src/libraries/OrderLib.sol";
import {PricingLib} from "../../src/libraries/PricingLib.sol";
import {AddressCast} from "../../src/libraries/AddressCast.sol";
import {FastFillBase} from "../../src/FastFillBase.sol";
import {OftAdapter} from "../../src/adapters/OftAdapter.sol";
import {FillStatus, OrderRecord} from "../../src/interfaces/IFastFill.sol";

contract OftLifecycleTest is Fixtures {
    uint256 constant INPUT = 1_000e18;
    uint256 constant MIN_OUT = 999e18;
    uint256 constant RATE = 1e13; // 0.001% / sec
    uint64 constant WINDOW = 100;
    bytes internal constant HOOK = hex"feed";

    function setUp() public virtual {
        vm.warp(1_000_000);
        _setUpOft();
    }

    // ---------------------------------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------------------------------

    function _createOrder() internal returns (Order memory order, bytes32 orderId) {
        return _createOrderWithBase(0);
    }

    /// @dev `deliveryWindow` is relative; the adapter derives `expectedDeliveryTime = block.timestamp
    ///      + WINDOW`, equal to `start + WINDOW` here.
    function _createOrderWithBase(uint256 baseFee) internal returns (Order memory order, bytes32 orderId) {
        uint64 start = uint64(block.timestamp);
        oftToken.mint(user, INPUT);
        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        oftToken.approve(address(srcOft), INPUT);
        uint64 nonce;
        (orderId, nonce) =
            srcOft.initiateOFT(DST_CHAIN, _b32(recipient), INPUT, MIN_OUT, "", WINDOW, RATE, baseFee, Execution(0, ""));
        vm.stopPrank();
        order = _oftOrder(INPUT, MIN_OUT, start, start + WINDOW, RATE, baseFee, nonce);
    }

    function _fillAt(Order memory order, address filler, uint256 fillTime) internal returns (uint256 payout) {
        vm.chainId(DST_CHAIN);
        vm.warp(fillTime);
        (payout,) = dstOft.quoteFill(order, fillTime);
        oftToken.mint(filler, payout);
        vm.startPrank(filler);
        oftToken.approve(address(dstOft), payout);
        dstOft.fill(order);
        vm.stopPrank();
    }

    /// @dev Model the OFT crediting `arrived` to the dst adapter during _lzReceive, then the
    ///      endpoint driving the compose callback.
    function _deliver(Order memory order, uint256 arrived) internal {
        vm.chainId(DST_CHAIN);
        oftToken.mint(address(dstOft), arrived);
        lzEndpoint.deliver(
            address(dstOft), address(oftToken), SRC_EID, _b32(address(srcOft)), arrived, OrderLib.encode(order)
        );
    }

    // ---------------------------------------------------------------------------------------------
    // Source
    // ---------------------------------------------------------------------------------------------

    function test_initiate_encodesOrderAndSends() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        assertEq(orderId, OrderLib.hash(order));
        assertEq(keccak256(oftToken.lastComposeMsg()), keccak256(OrderLib.encode(order)), "composeMsg");
        assertEq(oftToken.lastTo(), _b32(address(dstOft)), "to");
        assertEq(oftToken.lastAmountLD(), INPUT, "amountLD");
        assertEq(oftToken.lastMinAmountLD(), MIN_OUT, "minAmountLD");
        assertEq(oftToken.balanceOf(user), 0, "user funds pulled");
        assertEq(oftToken.balanceOf(address(srcOft)), 0, "adapter debited by OFT");
    }

    function test_initiate_malformedRecipient_revertsBeforeSend() public {
        bytes32 malformed = bytes32(uint256(uint160(recipient)) | (uint256(1) << 160));
        oftToken.mint(user, INPUT);

        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        oftToken.approve(address(srcOft), INPUT);
        vm.expectRevert(abi.encodeWithSelector(AddressCast.InvalidAddress.selector, malformed));
        srcOft.initiateOFT(DST_CHAIN, malformed, INPUT, MIN_OUT, "", WINDOW, RATE, 0, Execution(0, ""));
        vm.stopPrank();

        assertEq(oftToken.sendCount(), 0, "no OFT send");
        assertEq(oftToken.balanceOf(user), INPUT, "funds not pulled");
    }

    function test_initiate_zeroRecipient_revertsBeforeSend() public {
        oftToken.mint(user, INPUT);

        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        oftToken.approve(address(srcOft), INPUT);
        vm.expectRevert(FastFillBase.ZeroRecipient.selector);
        srcOft.initiateOFT(DST_CHAIN, bytes32(0), INPUT, MIN_OUT, "", WINDOW, RATE, 0, Execution(0, ""));
        vm.stopPrank();

        assertEq(oftToken.sendCount(), 0, "no OFT send");
        assertEq(oftToken.balanceOf(user), INPUT, "funds not pulled");
    }

    function test_initiate_callbackGasLimitTooHigh_revertsBeforeSend() public {
        uint64 maxGas = srcOft.MAX_CALLBACK_GAS_LIMIT();
        uint64 tooHigh = maxGas + 1;
        oftToken.mint(user, INPUT);

        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        oftToken.approve(address(srcOft), INPUT);
        vm.expectRevert(abi.encodeWithSelector(FastFillBase.InvalidCallbackGasLimit.selector, tooHigh, maxGas));
        srcOft.initiateOFT(DST_CHAIN, _b32(recipient), INPUT, MIN_OUT, "", WINDOW, RATE, 0, Execution(tooHigh, HOOK));
        vm.stopPrank();

        assertEq(oftToken.sendCount(), 0, "no OFT send");
        assertEq(oftToken.balanceOf(user), INPUT, "funds not pulled");
    }

    function test_initiate_maxCallbackGasLimitAccepted() public {
        uint64 maxGas = srcOft.MAX_CALLBACK_GAS_LIMIT();
        uint64 start = uint64(block.timestamp);
        oftToken.mint(user, INPUT);

        vm.chainId(SRC_CHAIN);
        vm.startPrank(user);
        oftToken.approve(address(srcOft), INPUT);
        (bytes32 orderId, uint64 nonce) =
            srcOft.initiateOFT(DST_CHAIN, _b32(recipient), INPUT, MIN_OUT, "", WINDOW, RATE, 0, Execution(maxGas, HOOK));
        vm.stopPrank();

        Order memory order = _oftOrder(INPUT, MIN_OUT, start, start + WINDOW, RATE, 0, nonce);
        order.callbackGasLimit = maxGas;
        order.hookData = HOOK;
        assertEq(orderId, OrderLib.hash(order));
        assertEq(keccak256(oftToken.lastComposeMsg()), keccak256(OrderLib.encode(order)), "composeMsg");
        assertEq(oftToken.sendCount(), 1, "OFT send dispatched");
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
        assertEq(oftToken.balanceOf(recipient), payout, "recipient paid");
        assertEq(dstOft.getOrder(orderId).filler, relayer, "filler recorded");
    }

    function test_fillThenSettle_reimbursesFiller_surplusToRecipient() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        uint256 payout = _fillAt(order, relayer, order.startTime + 20);

        _deliver(order, INPUT); // arrived = INPUT > outputAmount, so there is a surplus
        uint256 owed = order.outputAmount;
        uint256 surplus = INPUT - owed;

        assertEq(oftToken.balanceOf(relayer), owed, "filler reimbursed outputAmount");
        assertEq(oftToken.balanceOf(recipient), payout + surplus, "recipient: payout + surplus");
        assertEq(uint8(dstOft.getOrder(orderId).status), uint8(FillStatus.Settled), "settled");
    }

    function test_settleUnfilled_paysRecipientEverything() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        _deliver(order, INPUT);
        assertEq(oftToken.balanceOf(recipient), INPUT, "recipient gets all");
        assertEq(uint8(dstOft.getOrder(orderId).status), uint8(FillStatus.Settled), "settled");
    }

    function test_baseFee_flatPremiumAppliedAndReimbursed() public {
        uint256 baseFee = 1e18; // a flat 1-token fee, additive to the time premium
        (Order memory order,) = _createOrderWithBase(baseFee);
        uint256 fillTime = order.startTime + 20;
        uint256 payout = _fillAt(order, relayer, fillTime);

        uint256 expFee = PricingLib.fee(
            order.outputAmount, order.startTime, order.expectedDeliveryTime, fillTime, RATE, MAX_FEE_RATE, baseFee
        );
        assertGe(expFee, baseFee, "fee includes at least the flat base");
        assertEq(payout, order.outputAmount - expFee, "payout reduced by base + time fee");

        _deliver(order, INPUT);
        uint256 surplus = INPUT - order.outputAmount;
        assertEq(oftToken.balanceOf(relayer), order.outputAmount, "filler reimbursed outputAmount");
        assertEq(oftToken.balanceOf(recipient), payout + surplus, "recipient: payout + surplus");
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
        dstOft.fill(order);
    }

    function test_fillAfterSettle_reverts() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        _deliver(order, INPUT);

        vm.chainId(DST_CHAIN);
        vm.expectRevert(abi.encodeWithSelector(FastFillBase.OrderAlreadyActive.selector, orderId));
        vm.prank(relayer);
        dstOft.fill(order);
    }

    function test_settleSameOrderTwice_revertsAlreadySettled() public {
        (Order memory order, bytes32 orderId) = _createOrder();
        _deliver(order, INPUT);

        vm.chainId(DST_CHAIN);
        oftToken.mint(address(dstOft), INPUT);
        vm.expectRevert(abi.encodeWithSelector(FastFillBase.AlreadySettled.selector, orderId));
        lzEndpoint.deliver(
            address(dstOft), address(oftToken), SRC_EID, _b32(address(srcOft)), INPUT, OrderLib.encode(order)
        );
    }

    // ---------------------------------------------------------------------------------------------
    // Adversarial — the three compose auth gates
    // ---------------------------------------------------------------------------------------------

    function test_lzCompose_notEndpoint_reverts() public {
        vm.chainId(DST_CHAIN);
        vm.expectRevert(abi.encodeWithSelector(OftAdapter.NotEndpoint.selector, address(this)));
        dstOft.lzCompose(address(oftToken), bytes32(0), "", address(0), "");
    }

    function test_lzCompose_untrustedLocalOFT_reverts() public {
        vm.chainId(DST_CHAIN);
        address fakeOFT = makeAddr("fakeOFT");
        vm.expectRevert(abi.encodeWithSelector(OftAdapter.UntrustedLocalOFT.selector, fakeOFT));
        lzEndpoint.composeRaw(address(dstOft), fakeOFT, "");
    }

    function test_lzCompose_untrustedPeer_reverts() public {
        (Order memory order,) = _createOrder();
        address attacker = makeAddr("attacker");

        vm.chainId(DST_CHAIN);
        oftToken.mint(address(dstOft), order.outputAmount);
        vm.expectRevert(abi.encodeWithSelector(OftAdapter.UntrustedPeer.selector, _b32(attacker)));
        lzEndpoint.deliver(
            address(dstOft), address(oftToken), SRC_EID, _b32(attacker), order.outputAmount, OrderLib.encode(order)
        );
    }

    function test_lzCompose_untrustedSourceEid_reverts() public {
        (Order memory order,) = _createOrder();

        vm.chainId(DST_CHAIN);
        oftToken.mint(address(dstOft), order.outputAmount);
        vm.expectRevert(abi.encodeWithSelector(OftAdapter.UntrustedSourceEid.selector, uint32(99_999)));
        lzEndpoint.deliver(
            address(dstOft),
            address(oftToken),
            99_999,
            _b32(address(srcOft)),
            order.outputAmount,
            OrderLib.encode(order)
        );
    }

    function test_fakeOrderFill_selfPunishes() public {
        Order memory fake =
            _oftOrder(500e18, 499e18, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, 999);
        bytes32 fakeId = OrderLib.hash(fake);

        uint256 payout = _fillAt(fake, relayer, fake.startTime + 10);
        assertEq(oftToken.balanceOf(recipient), payout, "recipient paid by relayer");
        assertEq(oftToken.balanceOf(relayer), 0, "relayer out of pocket");
        assertEq(dstOft.getOrder(fakeId).filler, relayer, "filler recorded but never reimbursed");
    }

    // ---------------------------------------------------------------------------------------------
    // Pull-payment fallback
    // ---------------------------------------------------------------------------------------------

    function test_settle_recipientBlocked_fallbackThenClaim() public {
        (Order memory order,) = _createOrder();
        oftToken.setBlocked(recipient, true);

        _deliver(order, INPUT);
        assertEq(oftToken.balanceOf(recipient), 0, "push failed");
        assertEq(dstOft.claimable(recipient, address(oftToken)), INPUT, "credited to ledger");

        oftToken.setBlocked(recipient, false);
        vm.prank(recipient);
        assertEq(dstOft.claim(address(oftToken)), INPUT, "claimed");
        assertEq(oftToken.balanceOf(recipient), INPUT, "recovered");
    }
}
