// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {BaseFastFillHook} from "./BaseFastFillHook.sol";
import {ISwapRouter} from "../interfaces/uniswap/ISwapRouter.sol";

/// @title  UniswapSwapHook
/// @notice Demo fast-fill hook: on delivery of a stable (e.g. USDC/USDT), swap it into a user-chosen
///         target token via Uniswap V3 and send the output straight to the user. If the swap fails
///         (slippage, no liquidity, bad pool), the base contract redirects the original stable to the
///         user — so a failed swap degrades to a plain transfer rather than a stuck balance.
///
///         `hookData = abi.encode(address user, address tokenOut, uint24 poolFee, uint256 amountOutMinimum,
///         uint160 sqrtPriceLimitX96)`. The router is an immutable per-chain singleton (Uniswap
///         `SwapRouter02`, address from `script/config/Addresses.sol`).
contract UniswapSwapHook is BaseFastFillHook {
    using SafeTransferLib for address;

    /// @notice The Uniswap V3 `SwapRouter02` this hook routes swaps through.
    ISwapRouter public immutable router;

    constructor(address router_) {
        router = ISwapRouter(router_);
    }

    /// @dev The beneficiary is the first field of `hookData`.
    function _userOf(bytes calldata hookData) internal pure override returns (address user) {
        user = abi.decode(hookData, (address));
    }

    /// @dev Approve the router and swap the delivered token into `tokenOut`, sending output to `user`.
    function _doAction(address token, uint256 amount, address user, bytes calldata hookData) internal override {
        (, address tokenOut, uint24 poolFee, uint256 amountOutMinimum, uint160 sqrtPriceLimitX96) =
            abi.decode(hookData, (address, address, uint24, uint256, uint160));

        token.safeApproveWithRetry(address(router), amount);
        router.exactInputSingle(
            ISwapRouter.ExactInputSingleParams({
                tokenIn: token,
                tokenOut: tokenOut,
                fee: poolFee,
                recipient: user,
                amountIn: amount,
                amountOutMinimum: amountOutMinimum,
                sqrtPriceLimitX96: sqrtPriceLimitX96
            })
        );
    }
}
