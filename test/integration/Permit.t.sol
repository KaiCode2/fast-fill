// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../utils/Fixtures.sol";
import {Order, OrderLib} from "../../src/libraries/OrderLib.sol";
import {PermitLib} from "../../src/libraries/PermitLib.sol";
import {FastFillBase} from "../../src/FastFillBase.sol";
import {FillStatus} from "../../src/interfaces/IFastFill.sol";
import {MockPermit2} from "../mocks/MockPermit2.sol";

interface IERC2612 {
    function nonces(address) external view returns (uint256);
    function DOMAIN_SEPARATOR() external view returns (bytes32);
}

/// @notice Gasless approval flows: EIP-2612 self-permit batched via `multicall` (single-tx,
///         self-submitted) and Permit2-witness sponsored flows where the funds come from a signer
///         who is not `msg.sender` (a relayer submits the user's signed bridge intent). Permit2 is
///         modelled by a mock etched at the canonical address; the real signature + witness binding
///         is validated against the live Permit2 in test/fork/PermitFork.t.sol.
contract PermitTest is Fixtures {
    uint256 constant INPUT = 1_000e6;
    uint256 constant MAX_FEE = 1e6;
    uint256 constant RATE = 1e13;
    uint64 constant WINDOW = 100;

    bytes32 constant PERMIT_TYPEHASH =
        keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");

    address permitUser;
    uint256 permitUserKey;
    MockPermit2 mockPermit2;

    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp();
        (permitUser, permitUserKey) = makeAddrAndKey("permitUser");
        // Etch a Permit2 stand-in at the canonical address the adapter calls.
        mockPermit2 = new MockPermit2();
        vm.etch(cctp.PERMIT2(), address(mockPermit2).code);
    }

    // --- EIP-2612: approve + initiate in one self-submitted transaction via multicall ---

    function test_selfPermit_multicall_initiate_singleTx() public {
        // Sign under the chain the call executes on, so the token's EIP-712 domain matches.
        vm.chainId(SRC_CHAIN);
        usdc.mint(permitUser, INPUT);
        uint256 deadline = block.timestamp + 1 hours;
        (uint8 v, bytes32 r, bytes32 s) = _sign2612(permitUser, permitUserKey, address(cctp), INPUT, deadline);

        bytes[] memory calls = new bytes[](2);
        calls[0] = abi.encodeCall(cctp.selfPermit, (address(usdc), INPUT, deadline, v, r, s));
        calls[1] = abi.encodeCall(
            cctp.initiateCCTP, (DST_CHAIN, _b32(recipient), INPUT, MAX_FEE, 1000, WINDOW, RATE, uint256(0))
        );

        // No prior approve() — the permit inside the same tx authorizes the pull.
        vm.prank(permitUser);
        cctp.multicall(calls);

        assertEq(usdc.balanceOf(permitUser), 0, "funds pulled via permit");
        Order memory burned = OrderLib.decode(tokenMessenger.lastHookData());
        assertEq(burned.sender, _b32(permitUser), "order sender = user");
    }

    // --- Permit2 sponsored initiate: relayer submits, funds + sender are the signer ---

    function test_initiateCCTPFor_pullsFromSigner_bindsOrderWitness() public {
        usdc.mint(permitUser, INPUT);
        vm.prank(permitUser);
        usdc.approve(cctp.PERMIT2(), type(uint256).max); // one-time Permit2 approval

        vm.chainId(SRC_CHAIN);
        address relayerSubmitter = makeAddr("relayerSubmitter");
        vm.prank(relayerSubmitter);
        cctp.initiateCCTPFor(
            DST_CHAIN,
            _b32(recipient),
            INPUT,
            MAX_FEE,
            1000,
            WINDOW,
            RATE,
            0,
            permitUser,
            PermitLib.Permit2Data({nonce: 0, deadline: block.timestamp + 1, signature: ""})
        );

        assertEq(usdc.balanceOf(permitUser), 0, "USDC pulled from the signer, not the submitter");
        Order memory burned = OrderLib.decode(tokenMessenger.lastHookData());
        assertEq(burned.sender, _b32(permitUser), "order sender = signer");
        // The witness binds the relative window (WINDOW), the base fee (0), and the bridge mode the
        // user opted into (maxFee + minFinalityThreshold), so a relayer cannot alter any of them.
        assertEq(
            MockPermit2(cctp.PERMIT2()).lastWitness(),
            PermitLib.orderWitness(
                OrderLib.BRIDGE_CCTP,
                DST_CHAIN,
                _b32(recipient),
                INPUT,
                INPUT - MAX_FEE,
                WINDOW,
                RATE,
                0,
                keccak256(abi.encode(MAX_FEE, uint32(1000)))
            ),
            "signature bound to the order intent + bridge mode"
        );
    }

    function test_initiateCCTPFor_permit2Rejects_reverts() public {
        usdc.mint(permitUser, INPUT);
        vm.prank(permitUser);
        usdc.approve(cctp.PERMIT2(), type(uint256).max);
        MockPermit2(cctp.PERMIT2()).setReject(true);

        vm.chainId(SRC_CHAIN);
        vm.expectRevert(MockPermit2.Rejected.selector);
        cctp.initiateCCTPFor(
            DST_CHAIN,
            _b32(recipient),
            INPUT,
            MAX_FEE,
            1000,
            WINDOW,
            RATE,
            0,
            permitUser,
            PermitLib.Permit2Data({nonce: 0, deadline: block.timestamp + 1, signature: ""})
        );
    }

    // --- Permit2 sponsored fill: a third party submits a filler's signed fill ---

    function test_fillFor_pullsFromFiller_bindsOrderId() public {
        Order memory order =
            _cctpOrder(INPUT, MAX_FEE, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, 0);
        bytes32 orderId = OrderLib.hash(order);

        address signedFiller = makeAddr("signedFiller");
        vm.chainId(DST_CHAIN);
        vm.warp(order.startTime + 10);
        (uint256 payout,) = cctp.quoteFill(order, block.timestamp);
        usdc.mint(signedFiller, payout);
        vm.prank(signedFiller);
        usdc.approve(cctp.PERMIT2(), type(uint256).max);

        address sponsor = makeAddr("sponsor");
        vm.prank(sponsor);
        cctp.fillFor(
            order, signedFiller, PermitLib.Permit2Data({nonce: 0, deadline: block.timestamp + 1, signature: ""})
        );

        assertEq(usdc.balanceOf(recipient), payout, "recipient paid from the filler's funds");
        assertEq(usdc.balanceOf(signedFiller), 0, "filler funded the payout");
        assertEq(cctp.getOrder(orderId).filler, signedFiller, "filler recorded = signer, not submitter");
        assertEq(MockPermit2(cctp.PERMIT2()).lastWitness(), PermitLib.fillWitness(orderId), "bound to orderId");
    }

    // ---------------------------------------------------------------------------------------------

    function _sign2612(address owner, uint256 ownerKey, address spender, uint256 value, uint256 deadline)
        internal
        view
        returns (uint8 v, bytes32 r, bytes32 s)
    {
        uint256 nonce = IERC2612(address(usdc)).nonces(owner);
        bytes32 structHash = keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonce, deadline));
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", IERC2612(address(usdc)).DOMAIN_SEPARATOR(), structHash));
        (v, r, s) = vm.sign(ownerKey, digest);
    }
}
