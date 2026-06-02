// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IMessageTransmitterV2} from "../../src/interfaces/cctp/IMessageTransmitterV2.sol";
import {BurnMessageV2Lib} from "../../src/libraries/BurnMessageV2Lib.sol";
import {AddressCast} from "../../src/libraries/AddressCast.sol";
import {MockERC20} from "./MockERC20.sol";

/// @notice Mock CCTP v2 MessageTransmitter. Faithfully reproduces the properties fast-fill relies
///         on: enforces a non-zero `destinationCaller`, mints `amount - feeExecuted` of the
///         destination token to `mintRecipient`, and consumes the message nonce (replay guard).
///         The attestation is opaque; `setAcceptAttestation(false)` simulates Circle rejecting a
///         forged/unattested message.
contract MockMessageTransmitterV2 is IMessageTransmitterV2 {
    using AddressCast for bytes32;

    address public immutable mintToken;
    /// @dev This chain's CCTP domain (read + cross-checked by the adapter). Auto-getter satisfies
    ///      IMessageTransmitterV2.localDomain().
    uint32 public immutable localDomain;
    bool public acceptAttestation = true;
    mapping(bytes32 nonce => bool used) public usedNonces;

    error AttestationRejected();
    error UnauthorizedCaller(address caller, bytes32 destinationCaller);
    error NonceAlreadyUsed(bytes32 nonce);

    constructor(address mintToken_, uint32 localDomain_) {
        mintToken = mintToken_;
        localDomain = localDomain_;
    }

    function setAcceptAttestation(bool value) external {
        acceptAttestation = value;
    }

    function receiveMessage(
        bytes calldata message,
        bytes calldata /*attestation*/
    )
        external
        returns (bool)
    {
        if (!acceptAttestation) revert AttestationRejected();

        // destinationCaller lives at header offset 108; nonce at header offset 12.
        bytes32 destinationCaller = bytes32(message[108:140]);
        if (destinationCaller != bytes32(0) && destinationCaller.toAddress() != msg.sender) {
            revert UnauthorizedCaller(msg.sender, destinationCaller);
        }
        bytes32 nonce = bytes32(message[12:44]);
        if (usedNonces[nonce]) revert NonceAlreadyUsed(nonce);
        usedNonces[nonce] = true;

        (,, bytes32 mintRecipient, uint256 amount, uint256 feeExecuted,) = BurnMessageV2Lib.parse(message);
        MockERC20(mintToken).mint(mintRecipient.toAddress(), amount - feeExecuted);
        return true;
    }
}
