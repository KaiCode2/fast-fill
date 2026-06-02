// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice The sliver of the LayerZero v2 EndpointV2 the fast-fill OFT adapter reads: the local
///         endpoint id, cross-checked against the config registry at call time.
interface ILayerZeroEndpointV2 {
    function eid() external view returns (uint32);
}
