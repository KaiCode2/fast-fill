// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {ForkBase} from "../ForkBase.sol";
import {UniswapSwapHook} from "../../../src/hooks/UniswapSwapHook.sol";
import {IFastFillReceiver} from "../../../src/interfaces/IFastFillReceiver.sol";
import {Addresses} from "../../../script/config/Addresses.sol";

/// @notice UniswapSwapHook against the REAL Uniswap V3 `SwapRouter02` + real USDC on each supported chain.
///         The adapter delivers funds to the hook then calls `onFastFill`; here we reproduce that by
///         dealing USDC to the hook and invoking the callback directly. The adapter's transfer/redirect
///         plumbing is covered by the mock-based integration tests; this proves the hook's real-protocol
///         integration: a live swap routes the output to the user, and an impossible swap re-throws the
///         `RedirectFunds(user)` sentinel the adapter turns into a refund.
///
///         Env-gated: each chain runs only if its `*_RPC_URL` (or `ALCHEMY_API_KEY`) is set; otherwise the
///         whole test self-skips. Run with `FOUNDRY_PROFILE=fork forge test --match-path 'test/fork/hooks/*'`.
contract UniswapSwapHookForkTest is ForkBase {
    address internal finalUser = makeAddr("finalUser");
    uint256 constant AMOUNT = 1_000e6; // 1,000 USDC

    /// @dev Canonical WETH per chain — the deep-liquidity 0.05% USDC pair used as the swap target.
    function _weth(uint256 id) internal pure returns (address) {
        if (id == Addresses.CHAIN_ETHEREUM) return 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
        if (id == Addresses.CHAIN_OPTIMISM) return 0x4200000000000000000000000000000000000006;
        if (id == Addresses.CHAIN_ARBITRUM) return 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
        if (id == Addresses.CHAIN_BASE) return 0x4200000000000000000000000000000000000006;
        return address(0);
    }

    function _run(uint256 id) internal {
        address usdc = Addresses.usdc(id);
        address router = Addresses.uniswapSwapRouter(id);
        address weth = _weth(id);
        UniswapSwapHook hook = new UniswapSwapHook(router);

        // Success: a live swap sends WETH straight to the user, consuming all the delivered USDC.
        deal(usdc, address(hook), AMOUNT);
        bytes memory good = abi.encode(finalUser, weth, uint24(500), uint256(1), uint160(0));
        hook.onFastFill(bytes32(uint256(1)), usdc, AMOUNT, good);
        assertGt(SafeTransferLib.balanceOf(weth, finalUser), 0, "user received WETH from a real swap");
        assertEq(SafeTransferLib.balanceOf(usdc, address(hook)), 0, "hook spent all USDC");

        // Failure: demand impossible output -> the real router reverts -> hook re-throws RedirectFunds(user).
        deal(usdc, address(hook), AMOUNT);
        bytes memory bad = abi.encode(finalUser, weth, uint24(500), type(uint256).max, uint160(0));
        vm.expectRevert(abi.encodeWithSelector(IFastFillReceiver.RedirectFunds.selector, finalUser));
        hook.onFastFill(bytes32(uint256(2)), usdc, AMOUNT, bad);
    }

    function test_fork_swapUsdcToWeth() public {
        uint256[4] memory ids = [
            uint256(Addresses.CHAIN_ETHEREUM), Addresses.CHAIN_OPTIMISM, Addresses.CHAIN_ARBITRUM, Addresses.CHAIN_BASE
        ];
        string[4] memory rpcs = [_ethRpcUrl(), _opRpcUrl(), _arbRpcUrl(), _baseRpcUrl()];

        bool ranAny;
        for (uint256 i; i < 4; ++i) {
            if (bytes(rpcs[i]).length == 0) continue;
            vm.createSelectFork(rpcs[i]);
            _run(ids[i]);
            ranAny = true;
        }
        if (!ranAny) vm.skip(true);
    }
}
