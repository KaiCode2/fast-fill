// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../../utils/Fixtures.sol";
import {Order} from "../../../src/libraries/OrderLib.sol";
import {AaveDepositHook} from "../../../src/hooks/AaveDepositHook.sol";
import {MockAavePool} from "../../mocks/MockAavePool.sol";
import {MockERC20} from "../../mocks/MockERC20.sol";

/// @notice AaveDepositHook over the real CctpAdapter fill path with a mock pool. Proves the success path
///         (delivered USDC supplied to Aave, aTokens minted to the user) and that a failed supply
///         (frozen reserve) redirects the original USDC to the user — never stuck.
contract AaveDepositHookTest is Fixtures {
    uint256 constant INPUT = 1_000e6;
    uint256 constant MAX_FEE = 1e6;
    uint256 constant RATE = 1e13;
    uint64 constant WINDOW = 100;
    uint64 constant GAS = 500_000;

    AaveDepositHook internal hook;
    MockAavePool internal pool;
    MockERC20 internal aToken;
    address internal finalUser = makeAddr("finalUser");

    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp();
        pool = new MockAavePool();
        hook = new AaveDepositHook(address(pool));
        aToken = new MockERC20("Aave USDC", "aUSDC", 6);
        pool.setAToken(address(usdc), address(aToken));
    }

    function _hookData() internal view returns (bytes memory) {
        return abi.encode(finalUser, uint16(0));
    }

    function _order() internal view returns (Order memory o) {
        o = _cctpOrder(INPUT, MAX_FEE, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, 1);
        o.recipient = _b32(address(hook));
        o.callbackGasLimit = GAS;
        o.hookData = _hookData();
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

    function test_supplySuccess_userReceivesATokens() public {
        Order memory o = _order();
        uint256 payout = _fill(o);

        assertEq(aToken.balanceOf(finalUser), payout, "user received aTokens (1:1)");
        assertEq(usdc.balanceOf(address(pool)), payout, "pool holds the supplied USDC");
        assertEq(usdc.balanceOf(address(hook)), 0, "hook holds no USDC");
        assertEq(dstCctp.claimable(address(hook), address(usdc)), 0, "nothing stuck");
    }

    function test_supplyReverts_redirectsUsdcToUser() public {
        pool.setFail(true); // simulate frozen/paused reserve
        Order memory o = _order();
        uint256 payout = _fill(o);

        assertEq(usdc.balanceOf(finalUser), payout, "original USDC redirected to user");
        assertEq(aToken.balanceOf(finalUser), 0, "no aTokens minted");
        assertEq(usdc.balanceOf(address(hook)), 0, "hook holds nothing");
        assertEq(dstCctp.claimable(address(hook), address(usdc)), 0, "clean redirect, not claimable");
    }
}
