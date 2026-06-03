// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {Order, OrderLib} from "../../src/libraries/OrderLib.sol";

contract OrderLibTest is Test {
    function _sample() internal pure returns (Order memory) {
        return Order({
            bridgeType: OrderLib.BRIDGE_CCTP,
            srcChainId: 1,
            dstChainId: 2,
            sender: bytes32(uint256(uint160(address(0xA11CE)))),
            recipient: bytes32(uint256(uint160(address(0xB0B)))),
            inputToken: bytes32(uint256(uint160(address(0x05DC)))),
            outputToken: bytes32(uint256(uint160(address(0x05DC)))),
            inputAmount: 1_000e6,
            outputAmount: 999e6,
            nonce: 7,
            startTime: 1_000_000,
            expectedDeliveryTime: 1_000_100,
            discountRate: 1e13,
            baseFee: 5e5,
            callbackGasLimit: 50_000,
            hookData: hex"abcd"
        });
    }

    function test_hash_isKeccakAbiEncode() public pure {
        Order memory o = _sample();
        assertEq(OrderLib.hash(o), keccak256(abi.encode(o)));
    }

    function test_encodeDecode_roundtrip() public pure {
        Order memory o = _sample();
        Order memory d = OrderLib.decode(OrderLib.encode(o));
        assertEq(OrderLib.hash(d), OrderLib.hash(o));
    }

    function testFuzz_encodeDecode_roundtrip(Order memory o) public pure {
        Order memory d = OrderLib.decode(OrderLib.encode(o));
        assertEq(OrderLib.hash(d), OrderLib.hash(o));
    }

    function test_anyFieldChange_flipsId() public pure {
        bytes32 id = OrderLib.hash(_sample());

        Order memory a = _sample();
        a.nonce += 1;
        assertTrue(OrderLib.hash(a) != id, "nonce");

        Order memory b = _sample();
        b.recipient = bytes32(uint256(b.recipient) ^ 1);
        assertTrue(OrderLib.hash(b) != id, "recipient");

        Order memory c = _sample();
        c.outputAmount += 1;
        assertTrue(OrderLib.hash(c) != id, "outputAmount");

        Order memory e = _sample();
        e.bridgeType = OrderLib.BRIDGE_OFT;
        assertTrue(OrderLib.hash(e) != id, "bridgeType");

        Order memory f = _sample();
        f.dstChainId = 3;
        assertTrue(OrderLib.hash(f) != id, "dstChainId");

        Order memory g = _sample();
        g.baseFee += 1;
        assertTrue(OrderLib.hash(g) != id, "baseFee");
    }
}
