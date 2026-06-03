// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {OftAdapter} from "./OftAdapter.sol";

/// @title  OftAdapterFactory
/// @notice Deploys `OftAdapter` instances — one per OFT, keyed by `oftId` (see `OftId`). Onboarding a
///         new OFT is a single call: `deploy(oftId)`. Everything else (the registry `config`, the
///         `owner`, the `maxFeeRate`) is baked into this factory as immutables and shared by every
///         adapter it creates, so the only per-OFT input is the id — and that id is the only thing
///         that varies the deployed address.
///
///         Why a factory rather than a plain script: the fast-fill OFT security model needs each
///         adapter to live at the SAME address on every chain (the bridge counterpart is always
///         `address(this)`, and `lzCompose` enforces `composeFrom == address(this)`). This factory
///         guarantees that property mechanically. `deploy(oftId)` CREATE2s the adapter with
///         `salt = oftId` and constructor args `(config, owner, maxFeeRate, oftId)`. Given the factory
///         itself is at one address across chains (deploy it via the canonical CREATE2 deployer with a
///         fixed salt) and those args are identical across chains (config is itself CREATE2-identical,
///         owner/maxFeeRate are constants), the resulting adapter address is identical on every chain.
///         Distinct `oftId`s yield distinct addresses, so each OFT keeps its own isolated pool.
///
///         `deploy` is permissionless: the address and the constructor args are fully determined by
///         this factory, so whoever calls it produces the one canonical contract (and a second call
///         for the same id simply reverts on the already-deployed address).
contract OftAdapterFactory {
    /// @notice The chain registry every deployed adapter reads (CREATE2-identical across chains).
    address public immutable config;

    /// @notice The owner set on every deployed adapter (governance: pause + maxFeeRate tuning).
    address public immutable adapterOwner;

    /// @notice The governance fee-rate cap (WAD) set on every deployed adapter.
    uint256 public immutable maxFeeRate;

    event OftAdapterDeployed(uint8 indexed oftId, address adapter);

    constructor(address config_, address adapterOwner_, uint256 maxFeeRate_) {
        config = config_;
        adapterOwner = adapterOwner_;
        maxFeeRate = maxFeeRate_;
    }

    /// @notice CREATE2-deploy the `OftAdapter` for `oftId`. Reverts if it already exists on this chain.
    /// @return adapter The deployed adapter (at the deterministic `adapterFor(oftId)` address).
    function deploy(uint8 oftId) external returns (OftAdapter adapter) {
        adapter = new OftAdapter{salt: _salt(oftId)}(config, adapterOwner, maxFeeRate, oftId);
        emit OftAdapterDeployed(oftId, address(adapter));
    }

    /// @notice The deterministic address `deploy(oftId)` produces (whether or not it is deployed yet).
    function adapterFor(uint8 oftId) public view returns (address) {
        bytes32 hash =
            keccak256(abi.encodePacked(bytes1(0xff), address(this), _salt(oftId), keccak256(_initCode(oftId))));
        return address(uint160(uint256(hash)));
    }

    /// @notice Whether the adapter for `oftId` has been deployed on this chain.
    function isDeployed(uint8 oftId) external view returns (bool) {
        return adapterFor(oftId).code.length != 0;
    }

    /// @dev The full CREATE2 init code (creation bytecode + abi-encoded constructor args).
    function _initCode(uint8 oftId) internal view returns (bytes memory) {
        return abi.encodePacked(type(OftAdapter).creationCode, abi.encode(config, adapterOwner, maxFeeRate, oftId));
    }

    /// @dev One salt per OFT id, so each OFT's adapter is a distinct (but cross-chain-stable) address.
    function _salt(uint8 oftId) internal pure returns (bytes32) {
        return bytes32(uint256(oftId));
    }
}
