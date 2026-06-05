// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {BaseFastFillHook} from "./BaseFastFillHook.sol";
import {IPool} from "../interfaces/aave/IPool.sol";

/// @title  AaveDepositHook
/// @notice Demo fast-fill hook: on delivery of funds, supply them to Aave V3 with the aTokens minted
///         directly to the user. If the supply fails (frozen/paused/unlisted reserve, supply cap), the
///         base contract redirects the original tokens to the user — a failed deposit degrades to a
///         plain transfer rather than a stuck balance.
///
///         `hookData = abi.encode(address user, uint16 referralCode)`. The pool is an immutable per-chain
///         singleton (Aave V3 `Pool`, address from `script/config/Addresses.sol`). Because `supply` takes
///         `onBehalfOf`, the interest-bearing position lands with the user with no extra forwarding hop.
contract AaveDepositHook is BaseFastFillHook {
    using SafeTransferLib for address;

    /// @notice The Aave V3 `Pool` this hook supplies into.
    IPool public immutable pool;

    constructor(address pool_) {
        pool = IPool(pool_);
    }

    /// @dev The beneficiary is the first field of `hookData`.
    function _userOf(bytes calldata hookData) internal pure override returns (address user) {
        user = abi.decode(hookData, (address));
    }

    /// @dev Approve the pool and supply the delivered token on behalf of `user` (aTokens mint to `user`).
    function _doAction(address token, uint256 amount, address user, bytes calldata hookData) internal override {
        (, uint16 referralCode) = abi.decode(hookData, (address, uint16));

        token.safeApproveWithRetry(address(pool), amount);
        pool.supply(token, amount, user, referralCode);
    }
}
