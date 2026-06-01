// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {PricingLib} from "../../src/libraries/PricingLib.sol";

contract PricingLibTest is Test {
    uint256 constant WAD = 1e18;
    uint256 constant MAX = 5e15; // 0.5%
    uint256 constant RATE = 1e13; // 0.001% per second

    function test_fee_zeroAtAndAfterExpected() public pure {
        assertEq(PricingLib.fee(1_000e6, 0, 100, 100, RATE, MAX), 0, "at expected");
        assertEq(PricingLib.fee(1_000e6, 0, 100, 200, RATE, MAX), 0, "after expected");
    }

    function test_fee_largestAtStartDecaysToZero() public pure {
        uint256 amount = 1_000e6;
        uint256 fStart = PricingLib.fee(amount, 0, 100, 0, RATE, WAD); // timeSaved = 100
        uint256 fMid = PricingLib.fee(amount, 0, 100, 50, RATE, WAD); // timeSaved = 50
        uint256 fLate = PricingLib.fee(amount, 0, 100, 99, RATE, WAD); // timeSaved = 1
        assertGt(fStart, fMid);
        assertGt(fMid, fLate);
        assertGt(fLate, 0);
    }

    function test_fee_value() public pure {
        // timeSaved = 80, rate = RATE * 80 = 8e14, fee = amount * 8e14 / 1e18
        uint256 amount = 1_000_000e6;
        uint256 fee = PricingLib.fee(amount, 0, 100, 20, RATE, WAD);
        assertEq(fee, amount * (RATE * 80) / WAD);
    }

    function test_fee_cappedByMaxFeeRate() public pure {
        // An absurd discountRate saturates and is capped at maxFeeRate (no revert).
        uint256 amount = 1_000e6;
        uint256 fee = PricingLib.fee(amount, 0, 1000, 0, type(uint256).max, MAX);
        assertEq(fee, amount * MAX / WAD);
    }

    function test_fee_clampsFillBeforeStart() public pure {
        uint256 amount = 1_000e6;
        uint256 atStart = PricingLib.fee(amount, 100, 200, 100, RATE, WAD); // effectiveFill = 100
        uint256 beforeStart = PricingLib.fee(amount, 100, 200, 50, RATE, WAD); // clamped to 100
        assertEq(atStart, beforeStart);
    }

    function test_fee_malformedWindowIsZero() public pure {
        // startTime >= expectedDeliveryTime: never any time saved.
        assertEq(PricingLib.fee(1_000e6, 200, 100, 150, RATE, WAD), 0);
    }

    function test_payout_isOutputMinusFee() public pure {
        uint256 amount = 1_000e6;
        uint256 fee = PricingLib.fee(amount, 0, 100, 30, RATE, MAX);
        assertEq(PricingLib.payout(amount, 0, 100, 30, RATE, MAX), amount - fee);
    }

    function testFuzz_fee_neverExceedsOutputAmount(
        uint256 amount,
        uint64 start,
        uint64 expected,
        uint256 fillTime,
        uint256 rate
    ) public pure {
        amount = bound(amount, 0, 1e40);
        // maxFeeRate = WAD allows up to 100%, so this is the tightest bound.
        uint256 fee = PricingLib.fee(amount, start, expected, fillTime, rate, WAD);
        assertLe(fee, amount);
    }

    function testFuzz_fee_monotonicNonIncreasingInFillTime(
        uint256 amount,
        uint64 expected,
        uint256 rate,
        uint256 t1,
        uint256 t2
    ) public pure {
        amount = bound(amount, 0, 1e30);
        expected = uint64(bound(expected, 1, type(uint64).max));
        rate = bound(rate, 0, 1e16);
        t1 = bound(t1, 0, expected);
        t2 = bound(t2, t1, uint256(expected) + 1000); // t2 >= t1
        uint256 f1 = PricingLib.fee(amount, 0, expected, t1, rate, WAD);
        uint256 f2 = PricingLib.fee(amount, 0, expected, t2, rate, WAD);
        assertGe(f1, f2);
    }
}
