// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {ISwapRouter} from "../../src/interfaces/uniswap/ISwapRouter.sol";

/// @notice Minimal Uniswap-V3-`SwapRouter02` stand-in. Pulls `amountIn` of `tokenIn` from the caller and
///         pays `tokenOut` to `recipient` at a configurable rate. `setFail(true)` makes every swap revert
///         (no-liquidity), and `amountOutMinimum` is honoured (slippage) — the two failure modes the
///         UniswapSwapHook must convert into a `RedirectFunds(user)` refund. Fund it with `tokenOut`.
contract MockSwapRouter is ISwapRouter {
    using SafeTransferLib for address;

    /// @notice `amountOut = amountIn * rateBps / 10_000` (raw token units; decimals ignored for the demo).
    uint256 public rateBps = 10_000;
    bool public fail;

    function setRate(uint256 bps) external {
        rateBps = bps;
    }

    function setFail(bool f) external {
        fail = f;
    }

    function exactInputSingle(ExactInputSingleParams calldata p) external payable returns (uint256 amountOut) {
        require(!fail, "ROUTER_FAIL");
        p.tokenIn.safeTransferFrom(msg.sender, address(this), p.amountIn);
        amountOut = (p.amountIn * rateBps) / 10_000;
        require(amountOut >= p.amountOutMinimum, "Too little received");
        p.tokenOut.safeTransfer(p.recipient, amountOut);
    }
}
