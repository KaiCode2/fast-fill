// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../../utils/Fixtures.sol";
import {Order} from "../../../src/libraries/OrderLib.sol";
import {UniswapSwapHook} from "../../../src/hooks/UniswapSwapHook.sol";
import {MockSwapRouter} from "../../mocks/MockSwapRouter.sol";
import {MockERC20} from "../../mocks/MockERC20.sol";

/// @notice UniswapSwapHook over the real CctpAdapter fill path with a mock router. Proves the success
///         path (delivered USDC swapped into the target token, sent to the user) and that ANY swap
///         failure (no liquidity, slippage) redirects the original USDC to the user — never stuck.
contract UniswapSwapHookTest is Fixtures {
    uint256 constant INPUT = 1_000e6;
    uint256 constant MAX_FEE = 1e6;
    uint256 constant RATE = 1e13;
    uint64 constant WINDOW = 100;
    uint64 constant GAS = 500_000;

    UniswapSwapHook internal hook;
    MockSwapRouter internal router;
    MockERC20 internal tokenOut;
    address internal finalUser = makeAddr("finalUser");

    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp();
        router = new MockSwapRouter();
        hook = new UniswapSwapHook(address(router));
        tokenOut = new MockERC20("Wrapped Ether", "WETH", 18);
        tokenOut.mint(address(router), 1_000_000e18); // router liquidity
    }

    function _hookData(uint256 minOut) internal view returns (bytes memory) {
        return abi.encode(finalUser, address(tokenOut), uint24(500), minOut, uint160(0));
    }

    function _order(bytes memory hookData) internal view returns (Order memory o) {
        o = _cctpOrder(INPUT, MAX_FEE, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, 1);
        o.recipient = _b32(address(hook));
        o.callbackGasLimit = GAS;
        o.hookData = hookData;
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

    function test_swapSuccess_userReceivesTokenOut() public {
        Order memory o = _order(_hookData(0));
        uint256 payout = _fill(o);

        assertEq(tokenOut.balanceOf(finalUser), payout, "user received tokenOut (1:1)");
        assertEq(usdc.balanceOf(address(hook)), 0, "hook holds no USDC");
        assertEq(usdc.balanceOf(finalUser), 0, "user holds no leftover USDC");
        assertEq(dstCctp.claimable(address(hook), address(usdc)), 0, "nothing stuck");
    }

    function test_swapReverts_redirectsUsdcToUser() public {
        router.setFail(true); // simulate no-liquidity / router failure
        Order memory o = _order(_hookData(0));
        uint256 payout = _fill(o);

        assertEq(usdc.balanceOf(finalUser), payout, "original USDC redirected to user");
        assertEq(tokenOut.balanceOf(finalUser), 0, "no swap output");
        assertEq(usdc.balanceOf(address(hook)), 0, "hook holds nothing");
        assertEq(dstCctp.claimable(address(hook), address(usdc)), 0, "clean redirect, not claimable");
    }

    function test_swapSlippage_redirectsUsdcToUser() public {
        // Demand more output than the pool can give -> exactInputSingle reverts -> redirect.
        Order memory o = _order(_hookData(type(uint256).max));
        uint256 payout = _fill(o);

        assertEq(usdc.balanceOf(finalUser), payout, "slippage failure redirects USDC to user");
        assertEq(tokenOut.balanceOf(finalUser), 0, "no swap output");
    }
}
