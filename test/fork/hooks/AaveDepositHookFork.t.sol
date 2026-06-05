// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {ForkBase} from "../ForkBase.sol";
import {AaveDepositHook} from "../../../src/hooks/AaveDepositHook.sol";
import {IFastFillReceiver} from "../../../src/interfaces/IFastFillReceiver.sol";
import {Addresses} from "../../../script/config/Addresses.sol";
import {MockERC20} from "../../mocks/MockERC20.sol";

/// @dev Test-only view into Aave V3 `getReserveData` to resolve the aToken without hardcoding addresses.
///      Matches the Aave V3.x `ReserveDataLegacy` layout; `aTokenAddress` is the only field we read.
interface IAavePoolView {
    struct ReserveDataLegacy {
        uint256 configuration;
        uint128 liquidityIndex;
        uint128 currentLiquidityRate;
        uint128 variableBorrowIndex;
        uint128 currentVariableBorrowRate;
        uint128 currentStableBorrowRate;
        uint40 lastUpdateTimestamp;
        uint16 id;
        address aTokenAddress;
        address stableDebtTokenAddress;
        address variableDebtTokenAddress;
        address interestRateStrategyAddress;
        uint128 accruedToTreasury;
        uint128 unbacked;
        uint128 isolationModeTotalDebt;
    }

    function getReserveData(address asset) external view returns (ReserveDataLegacy memory);
}

/// @notice AaveDepositHook against the REAL Aave V3 `Pool` + real USDC on each supported chain. Proves a
///         live supply mints aTokens to the user, and that the real pool rejecting an unlisted asset
///         re-throws the `RedirectFunds(user)` sentinel. Adapter-side transfer/redirect plumbing is
///         covered by the mock-based integration tests.
///
///         Env-gated: each chain runs only if its `*_RPC_URL` (or `ALCHEMY_API_KEY`) is set; otherwise the
///         whole test self-skips. Run with `FOUNDRY_PROFILE=fork forge test --match-path 'test/fork/hooks/*'`.
contract AaveDepositHookForkTest is ForkBase {
    address internal finalUser = makeAddr("finalUser");
    uint256 constant AMOUNT = 1_000e6; // 1,000 USDC

    function _run(uint256 id) internal {
        address usdc = Addresses.usdc(id);
        address pool = Addresses.aaveV3Pool(id);
        AaveDepositHook hook = new AaveDepositHook(pool);
        address aToken = IAavePoolView(pool).getReserveData(usdc).aTokenAddress;

        // Success: a live supply mints ~AMOUNT aUSDC to the user (1:1 at deposit), consuming the USDC.
        deal(usdc, address(hook), AMOUNT);
        bytes memory good = abi.encode(finalUser, uint16(0));
        hook.onFastFill(bytes32(uint256(1)), usdc, AMOUNT, good);
        assertApproxEqAbs(SafeTransferLib.balanceOf(aToken, finalUser), AMOUNT, 2, "user received aUSDC");
        assertEq(SafeTransferLib.balanceOf(usdc, address(hook)), 0, "hook supplied all USDC");

        // Failure: the real pool reverts on an unlisted asset -> hook re-throws RedirectFunds(user).
        MockERC20 unlisted = new MockERC20("Unlisted", "UNL", 6);
        unlisted.mint(address(hook), AMOUNT);
        bytes memory bad = abi.encode(finalUser, uint16(0));
        vm.expectRevert(abi.encodeWithSelector(IFastFillReceiver.RedirectFunds.selector, finalUser));
        hook.onFastFill(bytes32(uint256(2)), address(unlisted), AMOUNT, bad);
    }

    function test_fork_supplyUsdcToAave() public {
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
