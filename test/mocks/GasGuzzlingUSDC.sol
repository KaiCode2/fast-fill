// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {MockERC20} from "./MockERC20.sol";

/// @notice A 6-decimal USDC stand-in whose `transfer` — the push to the recipient inside the adapter's
///         delivery frame — burns a configurable amount of gas, modelling an expensive real-world token
///         transfer (e.g. a first-ever-recipient SSTORE-from-zero on a proxy token like USDC). The plain
///         test `MockUSDC` transfer is cheap enough that a naive flat gas buffer over-covers it; this
///         token lets the destination-callback gas guard be exercised under the exact condition that
///         defeats a flat buffer. `transferFrom` (the pull from the relayer, which runs BEFORE the guard)
///         is left cheap, so only the in-frame push the guard protects is expensive.
contract GasGuzzlingUSDC is MockERC20 {
    uint256 public burnGas;

    constructor() MockERC20("Guzzler USDC", "gUSDC", 6) {}

    function setBurnGas(uint256 g) external {
        burnGas = g;
    }

    function transfer(address to, uint256 amount) public override returns (bool) {
        uint256 g = burnGas;
        if (g != 0) {
            uint256 floor = gasleft() > g ? gasleft() - g : 0;
            while (gasleft() > floor) {} // spin until ~`g` gas has been consumed
        }
        return super.transfer(to, amount);
    }
}
