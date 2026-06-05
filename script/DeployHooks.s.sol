// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script, console2} from "forge-std/Script.sol";

import {UniswapSwapHook} from "../src/hooks/UniswapSwapHook.sol";
import {AaveDepositHook} from "../src/hooks/AaveDepositHook.sol";
import {IntentExecutorHook} from "../src/hooks/IntentExecutorHook.sol";
import {Addresses} from "./config/Addresses.sol";

/// @notice CREATE2-deploy the demo fast-fill hooks for the current chain. Each hook's dependency (Uniswap
///         `SwapRouter02`, Aave V3 `Pool`, Rhinestone `IntentExecutor`) is resolved from
///         `script/config/Addresses.sol` by `block.chainid`, overridable via env. A hook whose dependency
///         resolves to `address(0)` (e.g. the IntentExecutor not yet deployed on this chain) is skipped.
/// @dev Env (all optional): UNISWAP_ROUTER, AAVE_POOL, INTENT_EXECUTOR to override the registry. Because
///      each hook embeds its dependency as an immutable, the CREATE2 address is chain-specific (it is NOT
///      the same across chains, unlike the adapters whose args are identical everywhere). Record the
///      printed addresses back into `Addresses.sol` after deploying.
contract DeployHooks is Script {
    bytes32 internal constant SALT_UNISWAP = keccak256("fast-fill.hook.uniswap-swap.v1");
    bytes32 internal constant SALT_AAVE = keccak256("fast-fill.hook.aave-deposit.v1");
    bytes32 internal constant SALT_INTENT = keccak256("fast-fill.hook.intent-executor.v1");

    function run() external {
        uint256 chainId = block.chainid;
        address router = vm.envOr("UNISWAP_ROUTER", Addresses.uniswapSwapRouter(chainId));
        address pool = vm.envOr("AAVE_POOL", Addresses.aaveV3Pool(chainId));
        address executor = vm.envOr("INTENT_EXECUTOR", Addresses.intentExecutor(chainId));

        vm.startBroadcast();

        if (router != address(0)) {
            UniswapSwapHook hook = new UniswapSwapHook{salt: SALT_UNISWAP}(router);
            console2.log("UniswapSwapHook:", address(hook), "router:", router);
        } else {
            console2.log("UniswapSwapHook: skipped, no Uniswap router for chain", chainId);
        }

        if (pool != address(0)) {
            AaveDepositHook hook = new AaveDepositHook{salt: SALT_AAVE}(pool);
            console2.log("AaveDepositHook:", address(hook), "pool:", pool);
        } else {
            console2.log("AaveDepositHook: skipped, no Aave V3 pool for chain", chainId);
        }

        if (executor != address(0)) {
            IntentExecutorHook hook = new IntentExecutorHook{salt: SALT_INTENT}(executor);
            console2.log("IntentExecutorHook:", address(hook), "executor:", executor);
        } else {
            console2.log("IntentExecutorHook: skipped, no IntentExecutor for chain", chainId);
        }

        vm.stopBroadcast();
    }
}
