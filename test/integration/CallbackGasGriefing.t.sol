// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../utils/Fixtures.sol";
import {Order} from "../../src/libraries/OrderLib.sol";
import {CallbackExecutor} from "../../src/CallbackExecutor.sol";
import {CctpAdapter} from "../../src/adapters/CctpAdapter.sol";
import {CctpExecutor} from "../../src/CctpExecutor.sol";
import {ChainConfig} from "../../src/interfaces/IFastFillConfig.sol";
import {MockFastFillConfig} from "../mocks/MockFastFillConfig.sol";
import {MockFastFillReceiver} from "../mocks/MockFastFillReceiver.sol";
import {GasGuzzlingUSDC} from "../mocks/GasGuzzlingUSDC.sol";

/// @notice The destination-callback gas guard. A relayer controls the TRANSACTION's total gas, but NOT
///         the signed `callbackGasLimit` (it is bound into the orderId, so altering it makes the order
///         un-settleable). The guard's invariant: a fill that COMMITS must always have delivered the
///         callback its full signed budget; a relayer that under-funds the tx must hit a revert (forcing
///         a retry), never silently route a starved/skipped callback to the fallback claim ledger.
///
///         These tests deliver an output token whose push transfer is deliberately expensive (modelling a
///         fresh-recipient real-USDC transfer). That is the exact condition under which a flat-buffer gas
///         check is defeated: the plain `MockUSDC` transfer is cheap enough that any reasonable buffer
///         over-covers it, so the regression is invisible there. Two regimes are covered: a transfer that
///         fits inside `TRANSFER_HEADROOM` (the widened outer fast-fail does the work) and one that
///         exceeds it (the exact in-frame guard is the fail-closed backstop).
contract CallbackGasGriefingTest is Fixtures {
    uint256 constant INPUT = 1_000e6;
    uint256 constant MAX_FEE = 1e6;
    uint64 constant WINDOW = 100;
    uint64 constant GAS_LIMIT = 2_000_000; // high budget => the double-63/64 slack is large

    bytes constant HOOK = hex"feed";

    GasGuzzlingUSDC internal gtoken;
    CctpAdapter internal gz;
    MockFastFillReceiver internal receiver;

    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp(); // gives us messengers, relayer, owner

        gtoken = new GasGuzzlingUSDC();
        MockFastFillConfig cfg = new MockFastFillConfig();
        cfg.set(SRC_CHAIN, ChainConfig(true, SRC_DOMAIN, 0, address(gtoken), address(messengerSrc)));
        cfg.set(DST_CHAIN, ChainConfig(true, DST_DOMAIN, 0, address(gtoken), address(messengerDst)));
        CctpExecutor exec = new CctpExecutor(address(cfg), owner);
        gz = new CctpAdapter(address(cfg), owner, MAX_FEE_RATE, address(exec));

        receiver = new MockFastFillReceiver();
        receiver.setMode(MockFastFillReceiver.Mode.Succeed);
    }

    function _order() internal view returns (Order memory o) {
        o = _cctpOrder(INPUT, MAX_FEE, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, 0, 0, 1);
        o.inputToken = _b32(address(gtoken));
        o.outputToken = _b32(address(gtoken));
        o.recipient = _b32(address(receiver));
        o.callbackGasLimit = GAS_LIMIT;
        o.hookData = HOOK;
    }

    function _prepFill(Order memory o) internal returns (uint256 payout) {
        vm.chainId(DST_CHAIN);
        (payout,) = gz.quoteFill(o, block.timestamp);
        gtoken.mint(relayer, payout);
        vm.prank(relayer);
        gtoken.approve(address(gz), payout);
    }

    function _selector(bytes memory b) internal pure returns (bytes4 s) {
        if (b.length >= 4) {
            /// @solidity memory-safe-assembly
            assembly {
                s := mload(add(b, 0x20))
            }
        }
    }

    /// @dev Core invariant: for ANY relayer-chosen `txGas`, the fill either reverts via the gas guard, or
    ///      commits having run the hook with (essentially) its full signed budget. Never a silent route to
    ///      the claim ledger. Fails before the fix (a band of `txGas` commits with a starved callback);
    ///      passes after.
    function _assertNoSilentGrief(uint256 burnGas, uint256 txGas) internal {
        gtoken.setBurnGas(burnGas);
        Order memory o = _order();
        _prepFill(o);

        vm.prank(relayer);
        try gz.fill{gas: txGas}(o) returns (bytes32) {
            // Committed => the callback must have actually run, with ~its full forwarded budget
            // (tolerate the callee's tiny ABI/dispatch prologue), and the funds must be delivered.
            assertGt(receiver.callCount(), 0, "fill committed but the hook was skipped (silent grief)");
            assertGe(receiver.gasAtEntry(), GAS_LIMIT - 2_000, "fill committed but the callback was starved");
            assertEq(gz.claimable(address(receiver), address(gtoken)), 0, "committed but funds went to claim ledger");
        } catch (bytes memory err) {
            // Reverting is acceptable ONLY when it is the gas guard (forcing a retry with more gas).
            assertEq(_selector(err), CallbackExecutor.InsufficientCallbackGas.selector, "reverted for a non-gas reason");
            assertEq(receiver.callCount(), 0, "reverted yet the hook ran");
            assertEq(gz.claimable(address(receiver), address(gtoken)), 0, "reverted yet funds were credited");
        }
    }

    /// @notice Transfer cost fits within `TRANSFER_HEADROOM`: the widened outer fast-fail guarantees the
    ///         budget, so committed fills always deliver the full budget. txGas range spans under/over.
    function testFuzz_noSilentGrief_transferWithinHeadroom(uint256 txGas) public {
        txGas = bound(txGas, 2_100_000, 2_900_000);
        _assertNoSilentGrief(70_000, txGas); // ~70k spin + ~25k real transfer < 120k headroom
    }

    /// @notice Transfer cost EXCEEDS `TRANSFER_HEADROOM`: the outer fast-fail under-estimates, so the exact
    ///         in-frame guard is what fails closed (re-reverted via the unforgeable 68-byte sentinel).
    function testFuzz_noSilentGrief_transferExceedsHeadroom(uint256 txGas) public {
        txGas = bound(txGas, 2_100_000, 2_900_000);
        _assertNoSilentGrief(150_000, txGas); // ~150k spin + ~25k real transfer > 120k headroom
    }

    /// @notice Ample gas: the hook runs once with its full budget and the funds are delivered (not deferred).
    function test_ampleGas_runsHookWithFullBudget() public {
        gtoken.setBurnGas(150_000);
        Order memory o = _order();
        _prepFill(o);

        vm.prank(relayer);
        gz.fill{gas: 3_600_000}(o);

        assertEq(receiver.callCount(), 1, "hook ran exactly once");
        assertGe(receiver.gasAtEntry(), GAS_LIMIT - 2_000, "callback received its full signed budget");
        assertEq(gz.claimable(address(receiver), address(gtoken)), 0, "delivered, nothing deferred");
    }

    /// @notice An under-funded fill reverts via the gas guard and commits NOTHING — no starved hook, no
    ///         claim-ledger credit, the order stays fillable.
    function test_underfundedTx_revertsAndCommitsNothing() public {
        gtoken.setBurnGas(150_000);
        Order memory o = _order();
        _prepFill(o);

        vm.prank(relayer);
        try gz.fill{gas: 2_120_000}(o) returns (bytes32) {
            assertTrue(false, "under-funded fill must revert, not commit");
        } catch (bytes memory err) {
            assertEq(_selector(err), CallbackExecutor.InsufficientCallbackGas.selector, "must revert via the gas guard");
        }
        assertEq(receiver.callCount(), 0, "hook never ran");
        assertEq(gz.claimable(address(receiver), address(gtoken)), 0, "no silent claim credited on revert");
    }
}
