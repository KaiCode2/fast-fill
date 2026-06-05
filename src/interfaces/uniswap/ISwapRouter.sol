// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title  ISwapRouter
/// @notice Minimal, hand-written interface for the Uniswap V3 `SwapRouter02` `exactInputSingle`.
/// @dev    Hand-written (rather than importing the v3-periphery package) to match this repo's
///         convention of vendoring only `forge-std` + `solady` and writing faithful ^0.8 interfaces
///         for external protocols. This is the `SwapRouter02` shape — it has NO `deadline` field
///         (the original `SwapRouter` did); deadline protection for fast-fill is provided by the
///         order's delivery window, so its absence is intentional. Addresses (per chain) live in
///         `script/config/Addresses.sol`.
interface ISwapRouter {
    /// @notice Parameters for a single-pool exact-input swap. Mirrors `ISwapRouter02.ExactInputSingleParams`.
    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24 fee;
        address recipient;
        uint256 amountIn;
        uint256 amountOutMinimum;
        uint160 sqrtPriceLimitX96;
    }

    /// @notice Swaps `amountIn` of `tokenIn` for as much `tokenOut` as possible, in a single pool.
    /// @dev Reverts if the output is below `amountOutMinimum` (slippage) or the pool has no liquidity —
    ///      which is exactly the failure a fast-fill hook converts into a `RedirectFunds(user)` refund.
    /// @return amountOut The amount of `tokenOut` received.
    function exactInputSingle(ExactInputSingleParams calldata params) external payable returns (uint256 amountOut);
}
