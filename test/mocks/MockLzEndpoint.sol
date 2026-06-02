// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ILayerZeroComposer} from "../../src/interfaces/layerzero/ILayerZeroComposer.sol";
import {OFTComposeMsgCodec} from "../../src/libraries/OFTComposeMsgCodec.sol";

/// @notice Mock LayerZero v2 Endpoint, just enough to drive the compose callback. The endpoint is
///         the `msg.sender` of `lzCompose`, which is exactly the authentication our adapter checks.
contract MockLzEndpoint {
    /// @dev Local endpoint id (read + cross-checked by the adapter). Auto-getter serves `eid()`.
    uint32 public immutable eid;

    constructor(uint32 eid_) {
        eid = eid_;
    }

    /// @notice Build a well-formed composed message and deliver it (the happy path).
    /// @dev Credit the destination adapter with the bridged tokens BEFORE calling this (the real
    ///      OFT does that during _lzReceive); this only drives the subsequent compose callback.
    function deliver(
        address to,
        address fromOFT,
        uint32 srcEid,
        bytes32 composeFrom,
        uint256 amountLD,
        bytes calldata orderBytes
    ) external {
        bytes memory composed =
            OFTComposeMsgCodec.encode(0, srcEid, amountLD, abi.encodePacked(composeFrom, orderBytes));
        ILayerZeroComposer(to).lzCompose(fromOFT, bytes32(0), composed, address(this), "");
    }

    /// @notice Deliver a raw (possibly malformed) composed message — for adversarial tests.
    function composeRaw(address to, address fromOFT, bytes calldata composed) external {
        ILayerZeroComposer(to).lzCompose(fromOFT, bytes32(0), composed, address(this), "");
    }
}
