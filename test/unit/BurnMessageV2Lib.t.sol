// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {BurnMessageV2Lib} from "../../src/libraries/BurnMessageV2Lib.sol";
import {CctpMessageBuilder} from "../utils/CctpMessageBuilder.sol";
import {ParseHarness} from "../utils/ParseHarness.sol";

contract BurnMessageV2LibTest is Test {
    ParseHarness harness;

    function setUp() public {
        harness = new ParseHarness();
    }

    function test_parse_extractsFields() public view {
        bytes memory hookData = hex"deadbeefcafe";
        bytes memory message = CctpMessageBuilder.build(
            CctpMessageBuilder.Msg({
                sourceDomain: 7,
                destinationDomain: 6,
                nonce: bytes32(uint256(123)),
                destinationCaller: bytes32(uint256(uint160(0xABCD))),
                burnToken: bytes32(uint256(uint160(0x1111))),
                mintRecipient: bytes32(uint256(uint160(0x2222))),
                amount: 1_000e6,
                messageSender: bytes32(uint256(uint160(0x3333))),
                maxFee: 5e6,
                feeExecuted: 3e6,
                hookData: hookData
            })
        );

        (uint32 sd, bytes32 ms, bytes32 mr, uint256 amt, uint256 fe, bytes memory hd) = harness.parse(message);
        assertEq(sd, 7, "sourceDomain");
        assertEq(ms, bytes32(uint256(uint160(0x3333))), "messageSender");
        assertEq(mr, bytes32(uint256(uint160(0x2222))), "mintRecipient");
        assertEq(amt, 1_000e6, "amount");
        assertEq(fe, 3e6, "feeExecuted");
        assertEq(keccak256(hd), keccak256(hookData), "hookData");
    }

    function testFuzz_parse_roundtrip(
        uint32 sourceDomain,
        uint256 amount,
        uint256 feeExecuted,
        bytes calldata hookData,
        address messageSender,
        address mintRecipient
    ) public view {
        bytes memory message = CctpMessageBuilder.build(
            CctpMessageBuilder.Msg({
                sourceDomain: sourceDomain,
                destinationDomain: 6,
                nonce: bytes32(uint256(1)),
                destinationCaller: bytes32(0),
                burnToken: bytes32(0),
                mintRecipient: bytes32(uint256(uint160(mintRecipient))),
                amount: amount,
                messageSender: bytes32(uint256(uint160(messageSender))),
                maxFee: 0,
                feeExecuted: feeExecuted,
                hookData: hookData
            })
        );
        (uint32 sd, bytes32 ms, bytes32 mr, uint256 amt, uint256 fe, bytes memory hd) = harness.parse(message);
        assertEq(sd, sourceDomain);
        assertEq(ms, bytes32(uint256(uint160(messageSender))));
        assertEq(mr, bytes32(uint256(uint160(mintRecipient))));
        assertEq(amt, amount);
        assertEq(fe, feeExecuted);
        assertEq(keccak256(hd), keccak256(hookData));
    }

    function test_parse_revertsOnTooShort() public {
        bytes memory tooShort = new bytes(375); // MIN_MESSAGE_LENGTH is 376
        vm.expectRevert(abi.encodeWithSelector(BurnMessageV2Lib.MessageTooShort.selector, uint256(375)));
        harness.parse(tooShort);
    }
}
