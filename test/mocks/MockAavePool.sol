// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {IPool} from "../../src/interfaces/aave/IPool.sol";
import {MockERC20} from "./MockERC20.sol";

/// @notice Minimal Aave-V3-`Pool` stand-in. Pulls `amount` of `asset` from the caller and mints the
///         registered aToken 1:1 to `onBehalfOf`. `setFail(true)` makes every supply revert (frozen
///         reserve) — the failure the AaveDepositHook must convert into a `RedirectFunds(user)` refund.
contract MockAavePool is IPool {
    using SafeTransferLib for address;

    mapping(address asset => address aToken) public aTokenOf;
    bool public fail;

    function setAToken(address asset, address aToken) external {
        aTokenOf[asset] = aToken;
    }

    function setFail(bool f) external {
        fail = f;
    }

    function supply(address asset, uint256 amount, address onBehalfOf, uint16) external {
        require(!fail, "POOL_FAIL");
        asset.safeTransferFrom(msg.sender, address(this), amount);
        MockERC20(aTokenOf[asset]).mint(onBehalfOf, amount);
    }
}
