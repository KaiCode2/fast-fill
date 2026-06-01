// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../utils/Fixtures.sol";
import {CctpFastFillHandler} from "./handlers/CctpFastFillHandler.sol";
import {OrderLib} from "../../src/libraries/OrderLib.sol";
import {FillStatus} from "../../src/interfaces/IFastFill.sol";

/// @notice Invariants over the shared fast-fill lifecycle (driven through the CCTP adapter; the
///         settle/payout/state-machine logic is the same base for OFT).
contract FastFillInvariantsTest is Fixtures {
    CctpFastFillHandler internal handler;

    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp();
        handler = new CctpFastFillHandler(srcCctp, dstCctp, usdc);
        targetContract(address(handler));
    }

    /// @notice The destination adapter never accumulates or leaks the bridged token: everything it
    ///         holds is exactly the sum of unclaimed (deferred) payouts.
    function invariant_adapterBalanceEqualsClaimable() public view {
        address[] memory accts = handler.accounts();
        uint256 totalClaimable;
        for (uint256 i; i < accts.length; i++) {
            totalClaimable += dstCctp.claimable(accts[i], address(usdc));
        }
        assertEq(usdc.balanceOf(address(dstCctp)), totalClaimable, "adapter holds exactly the claim ledger");
    }

    /// @notice Each order is settled at most once, and settled orders are terminal.
    function invariant_eachOrderSettledAtMostOnceAndTerminal() public view {
        bytes32[] memory ids = handler.orderIds();
        for (uint256 i; i < ids.length; i++) {
            assertLe(handler.settleCount(ids[i]), 1, "settled at most once");
            if (handler.settledOnce(ids[i])) {
                assertEq(uint8(dstCctp.getOrder(ids[i]).status), uint8(FillStatus.Settled), "terminal");
            }
        }
    }
}
