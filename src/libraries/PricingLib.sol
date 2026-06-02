// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

/// @notice The optimistic-fill premium curve.
/// @dev Pure and self-contained so the exact pricing model can be swapped without
///      touching the order lifecycle. The fee has two additive parts:
///        • a flat `baseFee` (output-token units) owed on ANY fill — it does not decay, so a user
///          can opt into a fixed price (e.g. $0.01) for the relaying service regardless of timing;
///        • a time premium that is largest right after `startTime` (the filler fronts capital
///          longest) and decays linearly to zero at `expectedDeliveryTime`.
///      The total is capped at `outputAmount` so the recipient payout never underflows. With
///      `baseFee == 0` this is the original pure time-decay curve; with `discountRate == 0` it is a
///      pure flat fee.
///
///        timeSaved = max(0, expectedDeliveryTime - max(fillTime, startTime))
///        rate      = min(discountRate * timeSaved, maxFeeRate)             [WAD]
///        timeFee   = outputAmount * rate / WAD
///        fee       = min(baseFee + timeFee, outputAmount)
///        payout    = outputAmount - fee                 (paid to the recipient at fill)
library PricingLib {
    uint256 internal constant WAD = 1e18;

    /// @notice Fee owed to the filler, denominated in `outputToken`.
    /// @param outputAmount         The worst-case arriving amount the filler is owed at settle.
    /// @param startTime            Order creation time (absolute, source clock).
    /// @param expectedDeliveryTime Time by which the bridge is expected to settle (absolute).
    /// @param fillTime             The time the fill is being priced at (destination clock).
    /// @param discountRate         User-chosen time-premium accrual per second, WAD.
    /// @param maxFeeRate           Per-adapter governance cap on the time-premium rate, WAD (<= WAD).
    /// @param baseFee              User-chosen flat fee in output-token units, added before the cap.
    function fee(
        uint256 outputAmount,
        uint64 startTime,
        uint64 expectedDeliveryTime,
        uint256 fillTime,
        uint256 discountRate,
        uint256 maxFeeRate,
        uint256 baseFee
    ) internal pure returns (uint256) {
        uint256 expected = expectedDeliveryTime;
        // Clamp the fill time up to startTime so timeSaved never exceeds the full window.
        uint256 effectiveFill = fillTime < startTime ? uint256(startTime) : fillTime;

        uint256 timeFee;
        // Time premium only accrues while there is time left to save; the flat baseFee still applies.
        if (effectiveFill < expected) {
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

            // rate <= maxFeeRate <= WAD, so timeFee <= outputAmount. fullMulDiv avoids 256-bit overflow.
            timeFee = FixedPointMathLib.fullMulDiv(outputAmount, rate, WAD);
        }

        // baseFee is validated < outputAmount at create time and timeFee <= outputAmount, so the sum
        // cannot overflow at any realistic magnitude; cap it so the recipient payout never underflows.
        uint256 total = baseFee + timeFee;
        return total > outputAmount ? outputAmount : total;
    }

    /// @notice The amount paid to the recipient at fill time = outputAmount - fee.
    function payout(
        uint256 outputAmount,
        uint64 startTime,
        uint64 expectedDeliveryTime,
        uint256 fillTime,
        uint256 discountRate,
        uint256 maxFeeRate,
        uint256 baseFee
    ) internal pure returns (uint256) {
        return outputAmount
            - fee(outputAmount, startTime, expectedDeliveryTime, fillTime, discountRate, maxFeeRate, baseFee);
    }
}
