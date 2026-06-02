// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ERC20} from "solady/tokens/ERC20.sol";

/// @notice Minimal test ERC20 with open mint/burn and configurable metadata.
contract MockERC20 is ERC20 {
    string private _name;
    string private _symbol;
    uint8 private immutable _decimals;

    /// @notice Addresses whose inbound transfers revert — models a USDC-style blacklist so tests
    ///         can exercise the pull-payment fallback.
    mapping(address account => bool) public blocked;

    error Blocked(address account);

    constructor(string memory name_, string memory symbol_, uint8 decimals_) {
        _name = name_;
        _symbol = symbol_;
        _decimals = decimals_;
    }

    function setBlocked(address account, bool value) external {
        blocked[account] = value;
    }

    function _beforeTokenTransfer(address, address to, uint256) internal view override {
        if (blocked[to]) revert Blocked(to);
    }

    function name() public view override returns (string memory) {
        return _name;
    }

    function symbol() public view override returns (string memory) {
        return _symbol;
    }

    function decimals() public view override returns (uint8) {
        return _decimals;
    }

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) external {
        _burn(from, amount);
    }
}
