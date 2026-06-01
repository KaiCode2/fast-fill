// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice A cross-chain optimistic-fill order.
/// @dev Encoded verbatim (via `abi.encode`) into the bridge's hook payload (CCTP `hookData`)
///      or compose payload (LayerZero `composeMsg`), and re-hashed identically at fill and
///      settle time. The order id `keccak256(abi.encode(order))` is the single value that
///      binds the source, the optimistic fill, and the final settlement together.
///
///      Transport identifiers (CCTP `domain`, LayerZero `eid`) are deliberately NOT stored
///      here — they are resolved at the contract edges via admin maps — so the order id stays
///      stable across transport remappings and a relayer only needs the emitted order to
///      reconstruct the hash.
struct Order {
    uint8 bridgeType; // 0 = CCTP, 1 = OFT (binds the order to one adapter family)
    uint32 srcChainId; // EVM chainId of the source chain
    uint32 dstChainId; // EVM chainId of the destination chain
    bytes32 sender; // user on the source chain (bytes32-encoded address)
    bytes32 recipient; // final recipient on the destination chain
    bytes32 inputToken; // token pulled from the user on the source chain
    bytes32 outputToken; // token delivered on the destination chain
    uint256 inputAmount; // amount pulled from the user (pre bridge fee)
    uint256 outputAmount; // deterministic worst-case arriving amount the filler is owed
    uint64 nonce; // per-source-adapter monotonic counter -> global uniqueness
    uint64 startTime; // absolute timestamp (source) — pricing baseline
    uint64 expectedDeliveryTime; // absolute timestamp — fee decays to 0 at/after this point
    uint256 discountRate; // WAD per second — user-chosen premium rate
}

library OrderLib {
    uint8 internal constant BRIDGE_CCTP = 0;
    uint8 internal constant BRIDGE_OFT = 1;

    /// @notice The canonical order id. Must be byte-identical wherever it is computed.
    function hash(Order memory order) internal pure returns (bytes32) {
        return keccak256(abi.encode(order));
    }

    /// @notice Encode an order for transport in a bridge hook/compose payload.
    function encode(Order memory order) internal pure returns (bytes memory) {
        return abi.encode(order);
    }

    /// @notice Decode an order from a bridge hook/compose payload.
    function decode(bytes memory data) internal pure returns (Order memory) {
        return abi.decode(data, (Order));
    }
}
