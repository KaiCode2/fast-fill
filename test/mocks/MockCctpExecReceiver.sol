// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ICctpExecReceiver} from "../../src/interfaces/ICctpExecReceiver.sol";

/// @notice Configurable receiver for generic CctpExecutor hook-mode tests.
contract MockCctpExecReceiver is ICctpExecReceiver {
    enum Mode {
        Succeed,
        RevertEmpty,
        RevertRedirect,
        StealThenRevert
    }

    Mode public mode;
    address public redirectDest;
    address public stash;

    uint32 public lastSourceDomain;
    bytes32 public lastSender;
    address public lastUsdc;
    uint256 public lastAmount;
    bytes public lastPayload;
    uint256 public callCount;

    function setMode(Mode m) external {
        mode = m;
    }

    function setRedirect(address dest) external {
        redirectDest = dest;
    }

    function setStash(address s) external {
        stash = s;
    }

    function onCctpExecute(uint32 sourceDomain, bytes32 sender, address usdc, uint256 amount, bytes calldata payload)
        external
    {
        lastSourceDomain = sourceDomain;
        lastSender = sender;
        lastUsdc = usdc;
        lastAmount = amount;
        lastPayload = payload;
        callCount++;

        Mode m = mode;
        if (m == Mode.Succeed) return;
        if (m == Mode.RevertEmpty) revert();
        if (m == Mode.RevertRedirect) revert ICctpExecReceiver.RedirectFunds(redirectDest);
        if (m == Mode.StealThenRevert) {
            (bool moved,) = usdc.call(abi.encodeWithSelector(0xa9059cbb, stash, amount)); // transfer(stash, amount)
            moved;
            revert();
        }
    }
}
