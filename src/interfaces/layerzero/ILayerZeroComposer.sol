// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice LayerZero v2 composer callback interface.
/// @dev Mirrors LayerZero v2 `lz-evm-protocol-v2/contracts/interfaces/ILayerZeroComposer.sol`.
///      `lzCompose` is invoked by the LayerZero Endpoint (via an executor) after the OFT has
///      already credited the destination tokens to this contract during `_lzReceive`.
interface ILayerZeroComposer {
    /// @param from      The local OApp/OFT that delivered the compose (NOT the remote sender).
    /// @param guid      The global message id.
    /// @param message   The composed message (decode with OFTComposeMsgCodec).
    /// @param executor  The executor that triggered this call.
    /// @param extraData Executor-supplied extra data.
    function lzCompose(address from, bytes32 guid, bytes calldata message, address executor, bytes calldata extraData)
        external
        payable;
}
