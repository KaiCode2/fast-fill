// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice EIP-712 witnesses for Permit2 `permitWitnessTransferFrom`, binding a signed token pull to
///         the protocol intent it is authorizing.
/// @dev Permit2 builds its digest as
///      keccak256("PermitWitnessTransferFrom(TokenPermissions permitted,address spender,uint256 nonce,uint256 deadline," + witnessTypeString).
///      So each `*_WITNESS_TYPE_STRING` is the remainder after that stub: the witness field, then the
///      nested struct definitions in EIP-712 alphabetical order (OrderIntent/FillAuthorization sort
///      before TokenPermissions).
library PermitLib {
    /// @notice A Permit2 signature bundle (the token + amount + spender come from the call context).
    struct Permit2Data {
        uint256 nonce; // Permit2's own (unordered) nonce
        uint256 deadline;
        bytes signature;
    }

    // --- Order intent (binds a sponsored `initiate*For`): a relayer cannot change the recipient,
    //     amounts, timing, pricing, or bridge mode the signer agreed to. `srcChainId`/`nonce`/
    //     `startTime` are contract-assigned and `sender` is the signer, so they are not part of the
    //     witness. Timing is bound as a relative `deliveryWindow` (seconds) — not an absolute
    //     timestamp — so the window the signer agreed to holds no matter when the relayer submits.
    //     `bridgeParams` is a per-bridge hash of the transport mode the user opted into (CCTP:
    //     maxFee + minFinalityThreshold; OFT: extraOptions), so a relayer cannot downgrade it.
    //     `hookDataHash` + `callbackGasLimit` bind the destination execution so a relayer cannot alter
    //     the callback payload or its gas budget. ---
    bytes32 internal constant ORDER_INTENT_TYPEHASH = keccak256(
        "OrderIntent(uint8 bridgeType,uint32 dstChainId,bytes32 recipient,uint256 inputAmount,uint256 outputAmount,uint64 deliveryWindow,uint256 discountRate,uint256 baseFee,bytes32 bridgeParams,bytes32 hookDataHash,uint64 callbackGasLimit)"
    );
    string internal constant ORDER_WITNESS_TYPE_STRING =
        "OrderIntent witness)OrderIntent(uint8 bridgeType,uint32 dstChainId,bytes32 recipient,uint256 inputAmount,uint256 outputAmount,uint64 deliveryWindow,uint256 discountRate,uint256 baseFee,bytes32 bridgeParams,bytes32 hookDataHash,uint64 callbackGasLimit)TokenPermissions(address token,uint256 amount)";

    // --- Fill authorization (binds a sponsored `fillFor`): a filler's funds can only be pulled to
    //     fill the exact order whose id they signed. ---
    bytes32 internal constant FILL_AUTH_TYPEHASH = keccak256("FillAuthorization(bytes32 orderId)");
    string internal constant FILL_WITNESS_TYPE_STRING =
        "FillAuthorization witness)FillAuthorization(bytes32 orderId)TokenPermissions(address token,uint256 amount)";

    function orderWitness(
        uint8 bridgeType,
        uint32 dstChainId,
        bytes32 recipient,
        uint256 inputAmount,
        uint256 outputAmount,
        uint64 deliveryWindow,
        uint256 discountRate,
        uint256 baseFee,
        bytes32 bridgeParams,
        bytes32 hookDataHash,
        uint64 callbackGasLimit
    ) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                ORDER_INTENT_TYPEHASH,
                bridgeType,
                dstChainId,
                recipient,
                inputAmount,
                outputAmount,
                deliveryWindow,
                discountRate,
                baseFee,
                bridgeParams,
                hookDataHash,
                callbackGasLimit
            )
        );
    }

    function fillWitness(bytes32 orderId) internal pure returns (bytes32) {
        return keccak256(abi.encode(FILL_AUTH_TYPEHASH, orderId));
    }
}
