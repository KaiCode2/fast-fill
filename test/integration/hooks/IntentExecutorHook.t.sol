// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../../utils/Fixtures.sol";
import {Order} from "../../../src/libraries/OrderLib.sol";
import {IntentExecutorHook} from "../../../src/hooks/IntentExecutorHook.sol";
import {IIntentExecutor} from "../../../src/interfaces/intent/IIntentExecutor.sol";
import {MockIntentExecutor} from "../../mocks/MockIntentExecutor.sol";
import {MockERC7579Account} from "../../mocks/MockERC7579Account.sol";

/// @notice IntentExecutorHook over the real CctpAdapter fill path with a mock executor + account. Proves
///         that delivered funds are forwarded to the user's account and the user-signed ops run on it, and
///         that any failure (rejected signature, reverting op) redirects the original funds to the account.
contract IntentExecutorHookTest is Fixtures {
    uint256 constant INPUT = 1_000e6;
    uint256 constant MAX_FEE = 1e6;
    uint256 constant RATE = 1e13;
    uint64 constant WINDOW = 100;
    uint64 constant GAS = 800_000;
    uint256 constant OP_AMOUNT = 10e6;

    IntentExecutorHook internal hook;
    MockIntentExecutor internal exec;
    MockERC7579Account internal acct;
    address internal opDest = makeAddr("opDest");

    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp();
        exec = new MockIntentExecutor();
        hook = new IntentExecutorHook(address(exec));
        acct = new MockERC7579Account();
    }

    /// @dev A signed op that transfers `amount` USDC from the account to `opDest`.
    function _opTransfer(uint256 amount) internal view returns (bytes memory) {
        return
            abi.encode(address(usdc), uint256(0), abi.encodeWithSignature("transfer(address,uint256)", opDest, amount));
    }

    function _hookData(uint256 nonce, bytes memory opData, bytes memory sig) internal view returns (bytes memory) {
        IIntentExecutor.SingleChainOps memory ops = IIntentExecutor.SingleChainOps({
            account: address(acct), nonce: nonce, ops: IIntentExecutor.Operation({data: opData}), signature: sig
        });
        return abi.encode(uint8(0), abi.encode(ops)); // variant 0 = single-chain
    }

    function _order(bytes memory hookData) internal view returns (Order memory o) {
        o = _cctpOrder(INPUT, MAX_FEE, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, 1);
        o.recipient = _b32(address(hook));
        o.callbackGasLimit = GAS;
        o.hookData = hookData;
    }

    function _fill(Order memory o) internal returns (uint256 payout) {
        vm.chainId(DST_CHAIN);
        (payout,) = dstCctp.quoteFill(o, block.timestamp);
        usdc.mint(relayer, payout);
        vm.startPrank(relayer);
        usdc.approve(address(dstCctp), payout);
        dstCctp.fill(o);
        vm.stopPrank();
    }

    function test_executesSignedOps_onAccount() public {
        Order memory o = _order(_hookData(1, _opTransfer(OP_AMOUNT), hex"abcd"));
        uint256 payout = _fill(o);

        assertEq(exec.callCount(), 1, "executor invoked once");
        assertEq(exec.lastAccount(), address(acct), "ran on the signed account");
        assertEq(usdc.balanceOf(opDest), OP_AMOUNT, "signed op moved funds out of the account");
        assertEq(usdc.balanceOf(address(acct)), payout - OP_AMOUNT, "account keeps the remainder");
        assertEq(usdc.balanceOf(address(hook)), 0, "hook holds nothing");
    }

    function test_executorReverts_redirectsToAccount() public {
        exec.setRevert(true); // simulate a rejected signature / authorization failure
        Order memory o = _order(_hookData(1, _opTransfer(OP_AMOUNT), hex"abcd"));
        uint256 payout = _fill(o);

        assertEq(exec.callCount(), 0, "executor never succeeded");
        assertEq(usdc.balanceOf(address(acct)), payout, "funds redirected to the account");
        assertEq(usdc.balanceOf(opDest), 0, "no op ran");
        assertEq(dstCctp.claimable(address(hook), address(usdc)), 0, "clean redirect, not claimable");
    }

    function test_revertingOp_redirectsToAccount() public {
        // The signed op tries to move more than the account holds -> account exec reverts -> redirect.
        Order memory o = _order(_hookData(1, _opTransfer(type(uint256).max), hex"abcd"));
        uint256 payout = _fill(o);

        assertEq(usdc.balanceOf(address(acct)), payout, "funds redirected to the account on op failure");
        assertEq(usdc.balanceOf(opDest), 0, "no op effect");
    }

    function test_emptySignature_redirectsToAccount() public {
        // The mock rejects an empty signature, as the real module's EIP-712 verification would.
        Order memory o = _order(_hookData(1, _opTransfer(OP_AMOUNT), ""));
        uint256 payout = _fill(o);

        assertEq(usdc.balanceOf(address(acct)), payout, "invalid (empty) signature redirects to account");
        assertEq(exec.callCount(), 0, "executor rejected the ops");
    }
}
