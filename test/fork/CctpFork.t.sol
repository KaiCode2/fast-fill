// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ForkBase} from "./ForkBase.sol";
import {CctpAdapter} from "../../src/adapters/CctpAdapter.sol";
import {Addresses} from "../../script/config/Addresses.sol";

interface IDecimals {
    function decimals() external view returns (uint8);
}

/// @notice Fork checks against the real CCTP v2 contracts. Self-skips when ETH_RPC_URL is unset, so
///         the default/ci profiles run it as a no-op.
///
///         Note: a full settle requires a real Circle attestation, which cannot be produced
///         offline. To exercise the settle path on a fork, capture a known-good (message,
///         attestation) pair from a historical CCTP v2 burn whose destinationCaller/mintRecipient
///         is a CctpAdapter you control, then replay it through `settle`.
contract CctpForkTest is ForkBase {
    function test_fork_cctpContractsAndAdapterWiring() external {
        if (!_forkMainnetOrSkip()) return;

        assertGt(Addresses.CCTP_TOKEN_MESSENGER_V2.code.length, 0, "TokenMessengerV2 has code");
        assertGt(Addresses.CCTP_MESSAGE_TRANSMITTER_V2.code.length, 0, "MessageTransmitterV2 has code");
        assertEq(IDecimals(Addresses.USDC_ETHEREUM).decimals(), 6, "USDC decimals");

        CctpAdapter adapter = new CctpAdapter(
            address(this),
            5e15,
            Addresses.CCTP_TOKEN_MESSENGER_V2,
            Addresses.CCTP_MESSAGE_TRANSMITTER_V2,
            Addresses.USDC_ETHEREUM
        );
        assertEq(address(adapter.tokenMessenger()), Addresses.CCTP_TOKEN_MESSENGER_V2, "messenger wired");
        assertEq(address(adapter.messageTransmitter()), Addresses.CCTP_MESSAGE_TRANSMITTER_V2, "transmitter wired");
        assertEq(adapter.usdc(), Addresses.USDC_ETHEREUM, "usdc wired");
    }
}
