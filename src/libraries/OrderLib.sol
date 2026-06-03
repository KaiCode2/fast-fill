// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {EfficientHashLib} from "solady/utils/EfficientHashLib.sol";

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
    bytes32 recipient; // final recipient on the destination chain (canonical address.toBytes32())
    bytes32 inputToken; // token pulled from the user on the source chain
    bytes32 outputToken; // token delivered on the destination chain
    uint256 inputAmount; // amount pulled from the user (pre bridge fee)
    uint256 outputAmount; // deterministic worst-case arriving amount the filler is owed
    uint64 nonce; // per-source-adapter monotonic counter -> global uniqueness
    uint64 startTime; // absolute timestamp (source) — pricing baseline
    uint64 expectedDeliveryTime; // absolute timestamp — time premium decays to 0 at/after this point
    uint256 discountRate; // WAD per second — user-chosen time-premium accrual rate
    uint256 baseFee; // flat fee (output-token units) owed to the filler on any fill — additive, no decay
    uint64 callbackGasLimit; // gas forwarded to the recipient's onFastFill hook (signed; priced into baseFee)
    bytes hookData; // optional destination-execution payload; empty = deliver funds only, no callback
}

/// @notice A user-requested destination execution, passed to `initiate*`: the gas budget forwarded to
///         the recipient's `onFastFill` hook plus the payload. Empty `data` = deliver funds, no callback.
///         Bundled as one struct so the initiate entrypoints stay under the stack-depth limit.
struct Execution {
    uint64 gasLimit;
    bytes data;
}

library OrderLib {
    uint8 internal constant BRIDGE_CCTP = 0;
    uint8 internal constant BRIDGE_OFT = 1;

    /// @notice The canonical order id. Must be byte-identical wherever it is computed.
    function hash(Order memory order) internal pure returns (bytes32 result) {
        uint256 hookWords = (order.hookData.length + 31) >> 5;
        bytes32[] memory buffer = EfficientHashLib.malloc(18 + hookWords);

        // `abi.encode(order)` encodes one dynamic struct argument: a top-level offset word,
        // then the struct head, then the padded `hookData` tail.
        EfficientHashLib.set(buffer, 0, 0x20);
        EfficientHashLib.set(buffer, 1, uint256(order.bridgeType));
        EfficientHashLib.set(buffer, 2, uint256(order.srcChainId));
        EfficientHashLib.set(buffer, 3, uint256(order.dstChainId));
        EfficientHashLib.set(buffer, 4, order.sender);
        EfficientHashLib.set(buffer, 5, order.recipient);
        EfficientHashLib.set(buffer, 6, order.inputToken);
        EfficientHashLib.set(buffer, 7, order.outputToken);
        EfficientHashLib.set(buffer, 8, order.inputAmount);
        EfficientHashLib.set(buffer, 9, order.outputAmount);
        EfficientHashLib.set(buffer, 10, uint256(order.nonce));
        EfficientHashLib.set(buffer, 11, uint256(order.startTime));
        EfficientHashLib.set(buffer, 12, uint256(order.expectedDeliveryTime));
        EfficientHashLib.set(buffer, 13, order.discountRate);
        EfficientHashLib.set(buffer, 14, order.baseFee);
        EfficientHashLib.set(buffer, 15, uint256(order.callbackGasLimit));
        EfficientHashLib.set(buffer, 16, 0x200);
        EfficientHashLib.set(buffer, 17, order.hookData.length);

        bytes memory hookData = order.hookData;
        /// @solidity memory-safe-assembly
        assembly {
            let len := mload(hookData)
            if len {
                let src := add(hookData, 0x20)
                let dst := add(buffer, 0x260)
                let end := add(src, len)
                for {} lt(src, end) {
                    src := add(src, 0x20)
                    dst := add(dst, 0x20)
                } {
                    mstore(dst, mload(src))
                }

                let rem := and(len, 0x1f)
                if rem {
                    let last := sub(dst, 0x20)
                    let mask := not(sub(shl(shl(3, sub(0x20, rem)), 1), 1))
                    mstore(last, and(mload(last), mask))
                }
            }
        }

        result = EfficientHashLib.hash(buffer);
        EfficientHashLib.free(buffer);
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
