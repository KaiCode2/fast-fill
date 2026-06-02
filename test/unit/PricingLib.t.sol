// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {PricingLib} from "../../src/libraries/PricingLib.sol";

contract PricingLibTest is Test {
    uint256 constant WAD = 1e18;
    uint256 constant MAX = 5e15; // 0.5%
    uint256 constant RATE = 1e13; // 0.001% per second

    // --- Time-decay curve (baseFee = 0 reproduces the original model) ---

    function test_fee_zeroAtAndAfterExpected() public pure {
        assertEq(PricingLib.fee(1_000e6, 0, 100, 100, RATE, MAX, 0), 0, "at expected");
        assertEq(PricingLib.fee(1_000e6, 0, 100, 200, RATE, MAX, 0), 0, "after expected");
    }

    function test_fee_largestAtStartDecaysToZero() public pure {
        uint256 amount = 1_000e6;
        uint256 fStart = PricingLib.fee(amount, 0, 100, 0, RATE, WAD, 0); // timeSaved = 100
        uint256 fMid = PricingLib.fee(amount, 0, 100, 50, RATE, WAD, 0); // timeSaved = 50
        uint256 fLate = PricingLib.fee(amount, 0, 100, 99, RATE, WAD, 0); // timeSaved = 1
        assertGt(fStart, fMid);
        assertGt(fMid, fLate);
        assertGt(fLate, 0);
    }

    function test_fee_value() public pure {
        // timeSaved = 80, rate = RATE * 80 = 8e14, fee = amount * 8e14 / 1e18
        uint256 amount = 1_000_000e6;
        uint256 fee = PricingLib.fee(amount, 0, 100, 20, RATE, WAD, 0);
        assertEq(fee, amount * (RATE * 80) / WAD);
    }

    function test_fee_cappedByMaxFeeRate() public pure {
        // An absurd discountRate saturates and is capped at maxFeeRate (no revert).
        uint256 amount = 1_000e6;
        uint256 fee = PricingLib.fee(amount, 0, 1000, 0, type(uint256).max, MAX, 0);
        assertEq(fee, amount * MAX / WAD);
    }

    function test_fee_clampsFillBeforeStart() public pure {
        uint256 amount = 1_000e6;
        uint256 atStart = PricingLib.fee(amount, 100, 200, 100, RATE, WAD, 0); // effectiveFill = 100
        uint256 beforeStart = PricingLib.fee(amount, 100, 200, 50, RATE, WAD, 0); // clamped to 100
        assertEq(atStart, beforeStart);
    }

    function test_fee_malformedWindowIsZero() public pure {
        // startTime >= expectedDeliveryTime: never any time saved (and no base fee here).
        assertEq(PricingLib.fee(1_000e6, 200, 100, 150, RATE, WAD, 0), 0);
    }

    function test_payout_isOutputMinusFee() public pure {
        uint256 amount = 1_000e6;
        uint256 fee = PricingLib.fee(amount, 0, 100, 30, RATE, MAX, 0);
        assertEq(PricingLib.payout(amount, 0, 100, 30, RATE, MAX, 0), amount - fee);
    }

    // --- Flat base fee (additive, no decay, capped at outputAmount) ---

    function test_baseFee_flatOnly_whenRateZero() public pure {
        uint256 amount = 1_000e6;
        uint256 base = 1e6;
        // rate = 0 -> no time premium; the flat base is owed at any fill time, even after expected.
        assertEq(PricingLib.fee(amount, 0, 100, 0, 0, MAX, base), base, "at start");
        assertEq(PricingLib.fee(amount, 0, 100, 50, 0, MAX, base), base, "mid");
        assertEq(PricingLib.fee(amount, 0, 100, 100, 0, MAX, base), base, "at expected");
        assertEq(PricingLib.fee(amount, 0, 100, 500, 0, MAX, base), base, "after expected");
    }

    function test_baseFee_additiveToTimePremium() public pure {
        uint256 amount = 1_000_000e6;
        uint256 base = 3e6;
        uint256 timeOnly = PricingLib.fee(amount, 0, 100, 20, RATE, WAD, 0);
        uint256 withBase = PricingLib.fee(amount, 0, 100, 20, RATE, WAD, base);
        assertEq(withBase, timeOnly + base, "fee = base + time premium");
        assertGt(timeOnly, 0, "time premium present");
    }

    function test_baseFee_appliesAfterExpected() public pure {
        uint256 amount = 1_000e6;
        uint256 base = 7e5;
        // No time saved (filled after expected) but the flat base still applies.
        assertEq(PricingLib.fee(amount, 0, 100, 300, RATE, MAX, base), base);
    }

    function test_baseFee_cappedAtOutputAmount() public pure {
        uint256 amount = 1_000e6;
        // base alone below amount, but base + a saturating time premium would exceed it -> capped.
        uint256 base = amount - 1;
        assertEq(PricingLib.fee(amount, 0, 1000, 0, type(uint256).max, WAD, base), amount, "capped at outputAmount");
    }

    // --- Fuzz ---

    function testFuzz_fee_neverExceedsOutputAmount(
        uint256 amount,
        uint64 start,
        uint64 expected,
        uint256 fillTime,
        uint256 rate,
        uint256 baseFee
    ) public pure {
        amount = bound(amount, 0, 1e40);
        baseFee = bound(baseFee, 0, 2e40); // intentionally allowed to exceed amount; the cap must hold
        // maxFeeRate = WAD allows up to 100%, so this is the tightest bound.
        uint256 fee = PricingLib.fee(amount, start, expected, fillTime, rate, WAD, baseFee);
        assertLe(fee, amount);
    }

    function testFuzz_fee_monotonicNonIncreasingInFillTime(
        uint256 amount,
        uint64 expected,
        uint256 rate,
        uint256 baseFee,
        uint256 t1,
        uint256 t2
    ) public pure {
        amount = bound(amount, 0, 1e30);
        baseFee = bound(baseFee, 0, 1e30); // a constant base preserves monotonicity in fillTime
        expected = uint64(bound(expected, 1, type(uint64).max));
        rate = bound(rate, 0, 1e16);
        t1 = bound(t1, 0, expected);
        t2 = bound(t2, t1, uint256(expected) + 1000); // t2 >= t1
        uint256 f1 = PricingLib.fee(amount, 0, expected, t1, rate, WAD, baseFee);
        uint256 f2 = PricingLib.fee(amount, 0, expected, t2, rate, WAD, baseFee);
        assertGe(f1, f2);
    }
}
