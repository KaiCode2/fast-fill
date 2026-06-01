// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";

/// @notice Shared RPC resolution for fork tests. Prefers ETH_RPC_URL; otherwise builds an Alchemy
///         mainnet URL from ALCHEMY_API_KEY. If neither is set, fork tests self-skip.
abstract contract ForkBase is Test {
    function _ethRpcUrl() internal view returns (string memory) {
        string memory url = vm.envOr("ETH_RPC_URL", string(""));
        if (bytes(url).length != 0) return url;
        string memory key = vm.envOr("ALCHEMY_API_KEY", string(""));
        if (bytes(key).length != 0) return string.concat("https://eth-mainnet.g.alchemy.com/v2/", key);
        return "";
    }

    /// @dev Selects a mainnet fork, or marks the test skipped and returns false.
    function _forkMainnetOrSkip() internal returns (bool) {
        string memory url = _ethRpcUrl();
        if (bytes(url).length == 0) {
            vm.skip(true);
            return false;
        }
        vm.createSelectFork(url);
        return true;
    }
}
