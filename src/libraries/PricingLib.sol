// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

/// @notice The optimistic-fill premium curve.
/// @dev Pure and self-contained so the exact pricing model can be swapped without
///      touching the order lifecycle. The v1 model: the fee a relayer earns is largest
///      right after `startTime` (it fronts capital longest) and decays linearly to zero at
///      `expectedDeliveryTime` — so a late or never-filled order costs the user nothing.
///
///        timeSaved = max(0, expectedDeliveryTime - max(fillTime, startTime))
///        rate      = min(discountRate * timeSaved, maxFeeRate)             [WAD]
///        fee       = outputAmount * rate / WAD          (<= outputAmount)
///        payout    = outputAmount - fee                 (paid to the recipient at fill)
library PricingLib {
    uint256 internal constant WAD = 1e18;

    /// @notice Fee owed to the filler, denominated in `outputToken`.
    /// @param outputAmount         The worst-case arriving amount the filler is owed at settle.
    /// @param startTime            Order creation time (absolute, source clock).
    /// @param expectedDeliveryTime Time by which the bridge is expected to settle (absolute).
    /// @param fillTime             The time the fill is being priced at (destination clock).
    /// @param discountRate         User-chosen premium accrual per second, WAD.
    /// @param maxFeeRate           Per-adapter governance cap on the rate, WAD (<= WAD).
    function fee(
        uint256 outputAmount,
        uint64 startTime,
        uint64 expectedDeliveryTime,
        uint256 fillTime,
        uint256 discountRate,
        uint256 maxFeeRate
    ) internal pure returns (uint256) {
        uint256 expected = expectedDeliveryTime;
        // Clamp the fill time up to startTime so timeSaved never exceeds the full window.
        uint256 effectiveFill = fillTime < startTime ? uint256(startTime) : fillTime;
        // No time saved (filled at/after expected delivery, or a malformed window): no fee.
        if (effectiveFill >= expected) return 0;

        uint256 timeSaved = expected - effectiveFill; // > 0

        // rate = min(discountRate * timeSaved, maxFeeRate), saturating the product so an
        // absurd user-supplied discountRate caps at maxFeeRate instead of reverting.
        uint256 rate;
        unchecked {
            if (discountRate != 0 && timeSaved > type(uint256).max / discountRate) {
                rate = maxFeeRate;
            } else {
                rate = discountRate * timeSaved;
            }
        }
        if (rate > maxFeeRate) rate = maxFeeRate;

        // rate <= maxFeeRate <= WAD, so fee <= outputAmount. fullMulDiv avoids 256-bit overflow.
        return FixedPointMathLib.fullMulDiv(outputAmount, rate, WAD);
    }

    /// @notice The amount paid to the recipient at fill time = outputAmount - fee.
    function payout(
        uint256 outputAmount,
        uint64 startTime,
        uint64 expectedDeliveryTime,
        uint256 fillTime,
        uint256 discountRate,
        uint256 maxFeeRate
    ) internal pure returns (uint256) {
        return outputAmount - fee(outputAmount, startTime, expectedDeliveryTime, fillTime, discountRate, maxFeeRate);
    }
}
