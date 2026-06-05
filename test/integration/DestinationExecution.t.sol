// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../utils/Fixtures.sol";
import {Order, OrderLib} from "../../src/libraries/OrderLib.sol";
import {FastFillBase} from "../../src/FastFillBase.sol";
import {CallbackExecutor} from "../../src/CallbackExecutor.sol";
import {IFastFill} from "../../src/interfaces/IFastFill.sol";
import {ICallbackExecutor} from "../../src/interfaces/ICallbackExecutor.sol";
import {MockFastFillReceiver} from "../mocks/MockFastFillReceiver.sol";

/// @notice Destination executions: when an order carries `hookData` and the recipient is a contract,
///         the adapter calls `onFastFill` in the same atomic frame as the transfer — at the optimistic
///         fill (filled path) and at settlement (unfilled path). The receiver's revert data governs the
///         fallback (deliver / RedirectFunds(dest) / claimable). A failing or hostile receiver can never
///         brick the fill/settle or keep funds it wasn't owed. Tested for both the CCTP and OFT adapters.
contract DestinationExecutionTest is Fixtures {
    uint256 constant INPUT = 1_000e6;
    uint256 constant MAX_FEE = 1e6;
    uint256 constant RATE = 1e13;
    uint64 constant WINDOW = 100;

    uint256 constant OFT_INPUT = 1_000e18;
    uint256 constant OFT_MIN = 999e18;

    MockFastFillReceiver internal receiver;
    bytes internal constant HOOK = hex"feed";

    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp();
        _setUpOft();
        receiver = new MockFastFillReceiver();
    }

    // ---------------------------------------------------------------------------------------------
    // CCTP helpers
    // ---------------------------------------------------------------------------------------------

    function _order(address recip, uint64 gasLimit, bytes memory hook) internal view returns (Order memory o) {
        o = _cctpOrder(INPUT, MAX_FEE, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, 1);
        o.recipient = _b32(recip);
        o.callbackGasLimit = gasLimit;
        o.hookData = hook;
    }

    function _fill(Order memory o) internal returns (uint256 payout) {
        vm.chainId(DST_CHAIN);
        (payout,) = dstCctp.quoteFill(o, block.timestamp);
        usdc.mint(relayer, payout);
        vm.startPrank(relayer);
        usdc.approve(address(dstCctp), payout);
        dstCctp.fill(o);
        vm.stopPrank();
    }

    function _settleUnfilled(Order memory o, uint256 nonce) internal returns (uint256 arrived) {
        arrived = o.inputAmount - 4e5;
        vm.chainId(DST_CHAIN);
        dstCctp.settle(_cctpMessage(o, 4e5, bytes32(nonce)), "");
    }

    // ---------------------------------------------------------------------------------------------
    // No callback paths
    // ---------------------------------------------------------------------------------------------

    function test_emptyHookData_noCallback_plainDelivery() public {
        // Recipient is a contract, but empty hookData => no callback, just a transfer.
        Order memory o = _order(address(receiver), 200_000, "");
        uint256 payout = _fill(o);
        assertEq(usdc.balanceOf(address(receiver)), payout, "funds delivered");
        assertEq(receiver.callCount(), 0, "no callback when hookData empty");
    }

    function test_noCodeRecipient_hookSkipped_fundsDelivered() public {
        // hookData present but the recipient is an EOA (no code): skip the call, deliver the funds.
        address eoa = makeAddr("plainRecipient");
        Order memory o = _order(eoa, 200_000, HOOK);
        uint256 payout = _fill(o);
        assertEq(usdc.balanceOf(eoa), payout, "EOA still receives the funds");
    }

    // ---------------------------------------------------------------------------------------------
    // Success
    // ---------------------------------------------------------------------------------------------

    function test_hookSuccess_atFill_callsReceiverWithArgs() public {
        receiver.setMode(MockFastFillReceiver.Mode.Succeed);
        Order memory o = _order(address(receiver), 200_000, HOOK);
        bytes32 orderId = OrderLib.hash(o);

        uint256 payout = _fill(o);

        assertEq(usdc.balanceOf(address(receiver)), payout, "funds delivered");
        assertEq(receiver.callCount(), 1, "hook ran once");
        assertEq(receiver.lastOrderId(), orderId, "orderId passed");
        assertEq(receiver.lastToken(), address(usdc), "token passed");
        assertEq(receiver.lastAmount(), payout, "amount = payout");
        assertEq(receiver.lastHookData(), HOOK, "hookData passed through");
    }

    function test_hookSuccess_atSettleUnfilled_callsReceiver() public {
        receiver.setMode(MockFastFillReceiver.Mode.Succeed);
        Order memory o = _order(address(receiver), 200_000, HOOK);
        uint256 arrived = _settleUnfilled(o, 1);

        assertEq(usdc.balanceOf(address(receiver)), arrived, "arrived delivered at settle");
        assertEq(receiver.callCount(), 1, "hook ran at settle");
        assertEq(receiver.lastAmount(), arrived, "amount = arrived");
    }

    // ---------------------------------------------------------------------------------------------
    // Failure routing (governed by the receiver's revert data)
    // ---------------------------------------------------------------------------------------------

    function test_hookRevertsEmpty_routesToClaimable() public {
        receiver.setMode(MockFastFillReceiver.Mode.RevertEmpty);
        Order memory o = _order(address(receiver), 200_000, HOOK);
        uint256 payout = _fill(o); // fill still succeeds

        assertEq(usdc.balanceOf(address(receiver)), 0, "transfer rolled back");
        assertEq(dstCctp.claimable(address(receiver), address(usdc)), payout, "credited to claim ledger");
        assertEq(receiver.callCount(), 0, "callback state rolled back");

        // The recipient can recover the funds via claim().
        vm.prank(address(receiver));
        assertEq(dstCctp.claim(address(usdc)), payout, "claimed");
    }

    function test_hookRevertsString_routesToClaimable() public {
        receiver.setMode(MockFastFillReceiver.Mode.RevertString);
        Order memory o = _order(address(receiver), 200_000, HOOK);
        uint256 payout = _fill(o);
        assertEq(dstCctp.claimable(address(receiver), address(usdc)), payout, "non-redirect revert => claimable");
    }

    function test_hookRevertsRedirect_routesToDest() public {
        address dest = makeAddr("fallbackWallet");
        receiver.setMode(MockFastFillReceiver.Mode.RevertRedirect);
        receiver.setRedirect(dest);
        Order memory o = _order(address(receiver), 200_000, HOOK);

        uint256 payout = _fill(o);

        assertEq(usdc.balanceOf(dest), payout, "funds redirected to dest");
        assertEq(usdc.balanceOf(address(receiver)), 0, "receiver kept nothing");
        assertEq(dstCctp.claimable(address(receiver), address(usdc)), 0, "not claimable");
    }

    function test_hookReturnBomb_handled_routesToClaimable() public {
        receiver.setMode(MockFastFillReceiver.Mode.ReturnBomb);
        Order memory o = _order(address(receiver), 200_000, HOOK);
        uint256 payout = _fill(o); // bounded copy => the outer tx is fine
        assertEq(dstCctp.claimable(address(receiver), address(usdc)), payout, "return-bomb => claimable");
    }

    function test_hookStealsThenReverts_clawedBack() public {
        address thief = makeAddr("thiefStash");
        receiver.setMode(MockFastFillReceiver.Mode.StealThenRevert);
        receiver.setStash(thief);
        Order memory o = _order(address(receiver), 200_000, HOOK);
        uint256 payout = _fill(o);

        // The receiver moved the funds out then reverted; the atomic frame undoes BOTH transfers.
        assertEq(usdc.balanceOf(thief), 0, "stolen transfer rolled back");
        assertEq(usdc.balanceOf(address(receiver)), 0, "receiver kept nothing");
        assertEq(dstCctp.claimable(address(receiver), address(usdc)), payout, "funds safe in the claim ledger");
    }

    // ---------------------------------------------------------------------------------------------
    // Safety: gas budget + reentrancy
    // ---------------------------------------------------------------------------------------------

    function test_hookGasBudget_relayerUnderfunds_reverts() public {
        // An absurd signed gas budget the relayer cannot forward => the fill reverts (forces a retry
        // with more gas) rather than silently starving the user's execution.
        Order memory o = _order(address(receiver), uint64(1e18), HOOK);
        vm.chainId(DST_CHAIN);
        (uint256 payout,) = dstCctp.quoteFill(o, block.timestamp);
        usdc.mint(relayer, payout);
        vm.startPrank(relayer);
        usdc.approve(address(dstCctp), payout);
        vm.expectPartialRevert(CallbackExecutor.InsufficientCallbackGas.selector);
        dstCctp.fill(o);
        vm.stopPrank();
    }

    function test_hookBurnsGas_routesToClaimable() public {
        // The hook consumes all of its forwarded gas (INVALID); only `callbackGasLimit` is forwarded,
        // so the outer tx survives and the funds route to the claim ledger.
        receiver.setMode(MockFastFillReceiver.Mode.BurnGas);
        Order memory o = _order(address(receiver), 100_000, HOOK);
        uint256 payout = _fill(o);
        assertEq(dstCctp.claimable(address(receiver), address(usdc)), payout, "gas-exhausted hook => claimable");
    }

    function test_callbackCannotForgeGasSentinel_36ByteSelector() public {
        // A hostile callback reverts with exactly 36 bytes whose first 4 ARE the InsufficientCallbackGas
        // selector. The catch matches the in-frame gas guard by LENGTH (== 68), so this is NOT mistaken for
        // it: the fill is not aborted, the funds route to the claim ledger. (If a callback could forge the
        // guard it could abort fills at will and grief relayers.)
        bytes memory forged = abi.encodePacked(CallbackExecutor.InsufficientCallbackGas.selector, bytes32(uint256(1)));
        assertEq(forged.length, 36, "precondition: 36-byte payload");
        receiver.setMode(MockFastFillReceiver.Mode.RevertCustom);
        receiver.setRevertData(forged);
        Order memory o = _order(address(receiver), 200_000, HOOK);
        uint256 payout = _fill(o);
        assertEq(dstCctp.claimable(address(receiver), address(usdc)), payout, "forged 36-byte sentinel => claimable");
    }

    function test_callbackCannotForgeGasSentinel_full68ByteEncoding() public {
        // A hostile callback reverts with the FULL 68-byte InsufficientCallbackGas encoding. `tryCall` caps
        // the bubbled returndata at CALLBACK_RETURNDATA_LIMIT (36 bytes), so it can never reach the catch as
        // 68 bytes — it is truncated and routes to the claim ledger rather than aborting the fill.
        bytes memory forged =
            abi.encodeWithSelector(CallbackExecutor.InsufficientCallbackGas.selector, uint256(7), uint256(9));
        assertEq(forged.length, 68, "precondition: 68-byte payload");
        receiver.setMode(MockFastFillReceiver.Mode.RevertCustom);
        receiver.setRevertData(forged);
        Order memory o = _order(address(receiver), 200_000, HOOK);
        uint256 payout = _fill(o);
        assertEq(
            dstCctp.claimable(address(receiver), address(usdc)), payout, "full 68-byte encoding truncated => claimable"
        );
    }

    function test_hookReentrancy_blocked_routesToClaimable() public {
        // The receiver tries to re-enter the adapter; the nonReentrant guard reverts the inner call,
        // so onFastFill reverts and the funds route to the claim ledger.
        receiver.setMode(MockFastFillReceiver.Mode.Reenter);
        receiver.setReenter(address(dstCctp), abi.encodeCall(ICallbackExecutor.claim, (address(usdc))));
        Order memory o = _order(address(receiver), 200_000, HOOK);
        uint256 payout = _fill(o);
        assertEq(dstCctp.claimable(address(receiver), address(usdc)), payout, "re-entry blocked => claimable");
        assertEq(receiver.callCount(), 0, "callback rolled back");
    }

    // ---------------------------------------------------------------------------------------------
    // Filled order: hook runs once (at fill), surplus at settle has no second callback
    // ---------------------------------------------------------------------------------------------

    function test_filledThenSettle_surplusHasNoSecondHook() public {
        receiver.setMode(MockFastFillReceiver.Mode.Succeed);
        Order memory o = _order(address(receiver), 200_000, HOOK);
        uint256 payout = _fill(o); // hook runs here (callCount = 1)
        assertEq(receiver.callCount(), 1, "hook ran at fill");

        uint256 arrived = _settleUnfilled(o, 2); // same order id; now filled -> reimburse relayer + surplus
        uint256 surplus = arrived - o.outputAmount;
        assertEq(receiver.callCount(), 1, "no second hook for the settle surplus");
        assertEq(usdc.balanceOf(relayer), o.outputAmount, "relayer reimbursed");
        assertEq(usdc.balanceOf(address(receiver)), payout + surplus, "recipient: payout + surplus");
    }

    // ---------------------------------------------------------------------------------------------
    // OFT parity (the callback logic lives in the shared base)
    // ---------------------------------------------------------------------------------------------

    function _oftHookOrder(address recip, bytes memory hook) internal view returns (Order memory o) {
        o = _oftOrder(OFT_INPUT, OFT_MIN, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, 1);
        o.recipient = _b32(recip);
        o.callbackGasLimit = 200_000;
        o.hookData = hook;
    }

    function test_oft_hookSuccess_atSettle() public {
        receiver.setMode(MockFastFillReceiver.Mode.Succeed);
        Order memory o = _oftHookOrder(address(receiver), HOOK);

        vm.chainId(DST_CHAIN);
        oftToken.mint(address(dstOft), OFT_INPUT);
        lzEndpoint.deliver(
            address(dstOft), address(oftToken), SRC_EID, _b32(address(srcOft)), OFT_INPUT, OrderLib.encode(o)
        );

        assertEq(oftToken.balanceOf(address(receiver)), OFT_INPUT, "OFT funds delivered");
        assertEq(receiver.callCount(), 1, "hook ran on OFT settle");
        assertEq(receiver.lastToken(), address(oftToken), "OFT token passed");
    }

    function test_oft_hookRevertsEmpty_routesToClaimable() public {
        receiver.setMode(MockFastFillReceiver.Mode.RevertEmpty);
        Order memory o = _oftHookOrder(address(receiver), HOOK);

        vm.chainId(DST_CHAIN);
        oftToken.mint(address(dstOft), OFT_INPUT);
        lzEndpoint.deliver(
            address(dstOft), address(oftToken), SRC_EID, _b32(address(srcOft)), OFT_INPUT, OrderLib.encode(o)
        );

        assertEq(oftToken.balanceOf(address(receiver)), 0, "transfer rolled back");
        assertEq(dstOft.claimable(address(receiver), address(oftToken)), OFT_INPUT, "OFT funds claimable");
    }
}
