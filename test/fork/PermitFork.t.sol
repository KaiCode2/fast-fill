// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ForkBase} from "./ForkBase.sol";
import {CctpAdapter} from "../../src/adapters/CctpAdapter.sol";
import {FastFillConfig} from "../../src/config/FastFillConfig.sol";
import {Order, OrderLib, Execution} from "../../src/libraries/OrderLib.sol";
import {AddressCast} from "../../src/libraries/AddressCast.sol";
import {PermitLib} from "../../src/libraries/PermitLib.sol";
import {Addresses} from "../../script/config/Addresses.sol";

interface IERC20P {
    function approve(address, uint256) external returns (bool);
    function balanceOf(address) external view returns (uint256);
}

interface IPermit2DomainSeparator {
    function DOMAIN_SEPARATOR() external view returns (bytes32);
}

/// @notice Validates the sponsored (Permit2) initiate path against the REAL Permit2 on an Ethereum
///         fork: a user signs an off-chain bridge intent (an EIP-712 PermitWitnessTransferFrom with
///         our OrderIntent witness), and a separate relayer submits it. This is the only way to
///         confirm the witness type strings are exactly what Permit2 expects — a wrong string would
///         make every real signature fail. Includes the negative case: a tampered order is rejected.
contract PermitForkTest is ForkBase {
    using AddressCast for address;

    bytes32 constant TOKEN_PERMISSIONS_TYPEHASH = keccak256("TokenPermissions(address token,uint256 amount)");
    string constant WITNESS_STUB =
        "PermitWitnessTransferFrom(TokenPermissions permitted,address spender,uint256 nonce,uint256 deadline,";

    uint32 constant ETH_CHAIN = 1;
    uint32 constant BASE_CHAIN = 8453;
    uint256 constant AMOUNT = 5e6;
    uint64 constant WINDOW = 600;
    uint256 constant RATE = 1e13;

    CctpAdapter adapter;
    address usdc = Addresses.USDC_ETHEREUM;
    address relayer = makeAddr("sponsoringRelayer");

    function test_fork_sponsoredInitiate_realPermit2_pullsFromSigner() external {
        if (!_forkMainnetOrSkip()) return;
        adapter = new CctpAdapter(address(new FastFillConfig()), address(this), 5e15);

        (address user, uint256 userKey) = makeAddrAndKey("intentSigner");
        _fund(user);
        uint64 exp = uint64(block.timestamp) + WINDOW;
        bytes32 recipient = makeAddr("recipient").toBytes32();
        bytes memory sig = _signOrder(userKey, recipient, exp);

        // A relayer (not the user) submits the signed intent.
        bytes32 orderId = _submitFor(recipient, user, exp, sig);

        assertEq(IERC20P(usdc).balanceOf(user), 0, "USDC pulled from the signer and burned");
        assertEq(orderId, _expectedOrderId(user, recipient, exp), "order built from the signer's exact intent");
    }

    /// @dev The sponsored submit, factored into its own frame to keep the test body readable.
    function _submitFor(bytes32 recipient, address user, uint64 exp, bytes memory sig)
        internal
        returns (bytes32 orderId)
    {
        vm.prank(relayer);
        (orderId,) = adapter.initiateCCTPFor(
            BASE_CHAIN,
            recipient,
            AMOUNT,
            0,
            2000,
            WINDOW,
            RATE,
            0,
            Execution(0, ""),
            user,
            PermitLib.Permit2Data(0, exp, sig)
        );
    }

    function test_fork_sponsoredInitiate_tamperedOrder_rejected() external {
        if (!_forkMainnetOrSkip()) return;
        adapter = new CctpAdapter(address(new FastFillConfig()), address(this), 5e15);

        (address user, uint256 userKey) = makeAddrAndKey("intentSigner");
        _fund(user);
        uint64 exp = uint64(block.timestamp) + WINDOW;
        bytes memory sig = _signOrder(userKey, makeAddr("recipient").toBytes32(), exp);

        // The relayer tries to redirect the funds to a recipient the user never signed.
        bytes32 attackerRecipient = makeAddr("attacker").toBytes32();
        vm.prank(relayer);
        vm.expectRevert(); // Permit2 recovers a different signer for the tampered witness -> InvalidSigner
        adapter.initiateCCTPFor(
            BASE_CHAIN,
            attackerRecipient,
            AMOUNT,
            0,
            2000,
            WINDOW,
            RATE,
            0,
            Execution(0, ""),
            user,
            PermitLib.Permit2Data(0, exp, sig)
        );
    }

    function test_fork_sponsoredInitiate_tamperedBridgeMode_rejected() external {
        if (!_forkMainnetOrSkip()) return;
        adapter = new CctpAdapter(address(new FastFillConfig()), address(this), 5e15);

        (address user, uint256 userKey) = makeAddrAndKey("intentSigner");
        _fund(user);
        uint64 exp = uint64(block.timestamp) + WINDOW;
        bytes32 recipient = makeAddr("recipient").toBytes32();
        // The user signs for the FINALIZED mode (minFinalityThreshold = 2000), bound via bridgeParams.
        bytes memory sig = _signOrder(userKey, recipient, exp);

        // The relayer submits the SAME order but flips the bridge speed to fast (1000): bridgeParams
        // differ, so Permit2 recovers a different signer for the tampered witness and reverts.
        vm.prank(relayer);
        vm.expectRevert();
        adapter.initiateCCTPFor(
            BASE_CHAIN,
            recipient,
            AMOUNT,
            0,
            1000,
            WINDOW,
            RATE,
            0,
            Execution(0, ""),
            user,
            PermitLib.Permit2Data(0, exp, sig)
        );
    }

    function _fund(address user) internal {
        deal(usdc, user, AMOUNT);
        address permit2 = adapter.PERMIT2(); // read before prank: a call here would consume it
        vm.prank(user);
        IERC20P(usdc).approve(permit2, type(uint256).max);
    }

    function _signOrder(uint256 userKey, bytes32 recipient, uint64 exp) internal view returns (bytes memory) {
        // Timing is signed as the relative WINDOW; the bridge mode (maxFee = 0, minFinalityThreshold =
        // 2000) is bound via bridgeParams; baseFee = 0. `exp` is reused only as the permit deadline.
        bytes32 witness = PermitLib.orderWitness(
            OrderLib.BRIDGE_CCTP,
            BASE_CHAIN,
            recipient,
            AMOUNT,
            AMOUNT,
            WINDOW,
            RATE,
            0,
            keccak256(abi.encode(uint256(0), uint32(2000))),
            keccak256(""),
            uint64(0)
        );
        return _signPermit2(userKey, exp, witness);
    }

    function _expectedOrderId(address user, bytes32 recipient, uint64 exp) internal view returns (bytes32) {
        return OrderLib.hash(
            Order({
                bridgeType: OrderLib.BRIDGE_CCTP,
                srcChainId: ETH_CHAIN,
                dstChainId: BASE_CHAIN,
                sender: user.toBytes32(),
                recipient: recipient,
                inputToken: usdc.toBytes32(),
                outputToken: Addresses.USDC_BASE.toBytes32(),
                inputAmount: AMOUNT,
                outputAmount: AMOUNT,
                nonce: 0,
                startTime: uint64(block.timestamp),
                expectedDeliveryTime: exp,
                discountRate: RATE,
                baseFee: 0,
                callbackGasLimit: 0,
                hookData: ""
            })
        );
    }

    /// @dev Build + sign a Permit2 PermitWitnessTransferFrom digest exactly as Permit2 hashes it.
    ///      `spender` = the adapter, `permitted.amount` = AMOUNT, `nonce` = 0 (held constant here).
    function _signPermit2(uint256 ownerKey, uint256 deadline, bytes32 witness) internal view returns (bytes memory) {
        bytes32 structHash = keccak256(
            abi.encode(
                keccak256(abi.encodePacked(WITNESS_STUB, PermitLib.ORDER_WITNESS_TYPE_STRING)),
                keccak256(abi.encode(TOKEN_PERMISSIONS_TYPEHASH, usdc, AMOUNT)),
                address(adapter),
                uint256(0),
                deadline,
                witness
            )
        );
        bytes32 digest = keccak256(
            abi.encodePacked("\x19\x01", IPermit2DomainSeparator(adapter.PERMIT2()).DOMAIN_SEPARATOR(), structHash)
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerKey, digest);
        return abi.encodePacked(r, s, v);
    }
}
