// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ForkBase} from "./ForkBase.sol";
import {Addresses} from "../../script/config/Addresses.sol";
import {OftAdapter} from "../../src/adapters/OftAdapter.sol";
import {Order, OrderLib} from "../../src/libraries/OrderLib.sol";
import {AddressCast} from "../../src/libraries/AddressCast.sol";
import {OFTComposeMsgCodec} from "../../src/libraries/OFTComposeMsgCodec.sol";
import {FillStatus} from "../../src/interfaces/IFastFill.sol";
import {SendParam, MessagingFee} from "../../src/interfaces/layerzero/IOFT.sol";

/// @dev LayerZero v2 inbound message origin.
struct Origin {
    uint32 srcEid;
    bytes32 sender;
    uint64 nonce;
}

/// @dev Just the bits of the real USDT0 OFT we touch on a fork.
interface IUsdt0Oft {
    function token() external view returns (address);
    function endpoint() external view returns (address);
    function approvalRequired() external view returns (bool);
    function peers(uint32 eid) external view returns (bytes32);
    function quoteSend(SendParam calldata sendParam, bool payInLzToken) external view returns (MessagingFee memory);
    function lzReceive(
        Origin calldata origin,
        bytes32 guid,
        bytes calldata message,
        address executor,
        bytes calldata extra
    ) external payable;
}

/// @dev The compose half of the real LayerZero EndpointV2.
interface ILzEndpointCompose {
    function composeQueue(address from, address to, bytes32 guid, uint16 index) external view returns (bytes32);
    function lzCompose(
        address from,
        address to,
        bytes32 guid,
        uint16 index,
        bytes calldata message,
        bytes calldata extraData
    ) external payable;
}

interface IERC20Decimals {
    function balanceOf(address) external view returns (uint256);
    function decimals() external view returns (uint8);
}

/// @notice End-to-end validation against the REAL USDT0 OFT + LayerZero endpoint on an Optimism
///         fork — the OFT analogue of the CCTP burn fork test that de-risked the live CCTP demo.
///
///         The load-bearing question is whether USDT0's OFT forwards an arbitrary `composeMsg` to
///         `lzCompose` (our whole settlement path depends on it). `test_inbound_*` proves it
///         end-to-end: we impersonate the endpoint and call the real `oft.lzReceive`, which mints
///         real USDT0 to our adapter and (if it forwards compose) queues a compose; we then drive
///         the real `endpoint.lzCompose` into our adapter and watch it settle. If USDT0 stripped
///         compose, the queue would be empty and `lzCompose` would revert — failing loudly.
contract OftForkE2ETest is ForkBase {
    using AddressCast for address;

    IUsdt0Oft internal oft = IUsdt0Oft(Addresses.USDT0_OFT_OPTIMISM);
    address internal token = Addresses.USDT0_TOKEN_OPTIMISM;
    ILzEndpointCompose internal lzEndpoint = ILzEndpointCompose(Addresses.LZ_ENDPOINT_V2);

    OftAdapter internal adapter;
    address internal srcAdapter = makeAddr("arbitrumSourceAdapter"); // the remote (Arbitrum) peer adapter
    address internal recipient = makeAddr("recipient");
    address internal relayer = makeAddr("relayer");

    uint256 internal constant ONE = 1e6; // 1 USDT0 (6 decimals)
    uint64 internal constant WINDOW = 600;

    function _setUpFork() internal returns (bool) {
        if (!_forkOrSkip(_opRpcUrl())) return false;
        // Deploy our adapter on the Optimism fork, wired to the REAL OFT + endpoint.
        adapter = new OftAdapter(address(this), 5e15, Addresses.LZ_ENDPOINT_V2, Addresses.USDT0_OFT_OPTIMISM);
        adapter.setEid(Addresses.CHAIN_OPTIMISM, Addresses.EID_OPTIMISM);
        adapter.setEid(Addresses.CHAIN_ARBITRUM, Addresses.EID_ARBITRUM);
        adapter.setRemoteAdapter(Addresses.CHAIN_ARBITRUM, srcAdapter.toBytes32());
        adapter.setRemoteOftToken(Addresses.CHAIN_ARBITRUM, Addresses.USDT0_TOKEN_ARBITRUM);
        return true;
    }

    // ---------------------------------------------------------------------------------------------
    // Sanity: the live contracts are what we expect
    // ---------------------------------------------------------------------------------------------

    function test_fork_op_usdt0_facts() external {
        if (!_setUpFork()) return;
        assertEq(oft.token(), token, "OFT.token() == USDT0 token");
        assertEq(oft.endpoint(), Addresses.LZ_ENDPOINT_V2, "OFT.endpoint() == canonical EndpointV2");
        assertEq(IERC20Decimals(token).decimals(), 6, "USDT0 has 6 decimals");
        assertFalse(oft.approvalRequired(), "mint/burn OFT does not require approval");
        assertTrue(oft.peers(Addresses.EID_ARBITRUM) != bytes32(0), "OFT has an Arbitrum peer");
    }

    // ---------------------------------------------------------------------------------------------
    // Source: the real OFT accepts the exact SendParam our adapter builds (with a composeMsg)
    // ---------------------------------------------------------------------------------------------

    function test_fork_op_quoteSend_acceptsOurComposeSendParam() external {
        if (!_setUpFork()) return;
        Order memory order = _order();
        SendParam memory sp = SendParam({
            dstEid: Addresses.EID_ARBITRUM,
            to: srcAdapter.toBytes32(),
            amountLD: ONE,
            minAmountLD: ONE,
            extraOptions: _composeOptions(80_000, 200_000),
            composeMsg: OrderLib.encode(order),
            oftCmd: ""
        });
        MessagingFee memory fee = oft.quoteSend(sp, false);
        assertGt(fee.nativeFee, 0, "real OFT quotes a native fee for our composed send");
    }

    // ---------------------------------------------------------------------------------------------
    // Destination: real lzReceive mints to our adapter + forwards compose; real lzCompose settles
    // ---------------------------------------------------------------------------------------------

    function test_fork_op_inbound_composeForwardsMintsAndSettles_unfilled() external {
        if (!_setUpFork()) return;
        Order memory order = _order();

        // 1) Real OFT delivery: impersonate the endpoint and call the real oft.lzReceive with a
        //    composed OFT message addressed to our adapter. This mints real USDT0 to the adapter
        //    and, if USDT0 forwards compose, queues a compose on the real endpoint.
        bytes32 guid = keccak256("fast-fill-oft-fork");
        bytes memory composedPayload = _deliverInbound(order, guid, ONE);

        assertEq(IERC20Decimals(token).balanceOf(address(adapter)), ONE, "real USDT0 minted to adapter");
        assertEq(
            lzEndpoint.composeQueue(address(oft), address(adapter), guid, 0),
            keccak256(composedPayload),
            "USDT0 forwarded our compose to the endpoint queue"
        );

        // 2) Real endpoint drives the compose into our adapter -> settlement (unfilled -> recipient).
        lzEndpoint.lzCompose(address(oft), address(adapter), guid, 0, composedPayload, "");

        assertEq(IERC20Decimals(token).balanceOf(recipient), ONE, "recipient received the full arrived amount");
        assertEq(IERC20Decimals(token).balanceOf(address(adapter)), 0, "adapter fully disbursed");
        assertEq(uint8(adapter.getOrder(OrderLib.hash(order)).status), uint8(FillStatus.Settled), "order settled");
    }

    function test_fork_op_inbound_filledThenSettle_reimbursesRelayer() external {
        if (!_setUpFork()) return;
        Order memory order = _order();
        bytes32 orderId = OrderLib.hash(order);

        // Relayer fills first (fronts the discounted payout to the recipient).
        (uint256 payout,) = adapter.quoteFill(order, block.timestamp);
        _mintViaOft(relayer, payout); // give the relayer USDT0 the honest way (real OFT mint)
        vm.startPrank(relayer);
        IERC20Decimals(token); // no-op to keep imports tidy
        _approve(token, address(adapter), payout);
        adapter.fill(order);
        vm.stopPrank();
        assertEq(IERC20Decimals(token).balanceOf(recipient), payout, "recipient paid at fill");

        // Bridge then delivers: real lzReceive mints to adapter, real lzCompose settles -> relayer.
        bytes32 guid = keccak256("fast-fill-oft-fork-filled");
        bytes memory composedPayload = _deliverInbound(order, guid, ONE);
        lzEndpoint.lzCompose(address(oft), address(adapter), guid, 0, composedPayload, "");

        assertEq(IERC20Decimals(token).balanceOf(relayer), order.outputAmount, "relayer reimbursed outputAmount");
        assertEq(uint8(adapter.getOrder(orderId).status), uint8(FillStatus.Settled), "settled");
    }

    // ---------------------------------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------------------------------

    function _order() internal view returns (Order memory) {
        return Order({
            bridgeType: OrderLib.BRIDGE_OFT,
            srcChainId: Addresses.CHAIN_ARBITRUM,
            dstChainId: Addresses.CHAIN_OPTIMISM,
            sender: srcAdapter.toBytes32(),
            recipient: recipient.toBytes32(),
            inputToken: Addresses.USDT0_TOKEN_ARBITRUM.toBytes32(),
            outputToken: token.toBytes32(), // delivered on Optimism = local USDT0
            inputAmount: ONE,
            outputAmount: ONE,
            nonce: 0,
            startTime: uint64(block.timestamp),
            expectedDeliveryTime: uint64(block.timestamp) + WINDOW,
            discountRate: 1e13
        });
    }

    /// @dev Craft a composed OFT receive message and feed it to the REAL oft.lzReceive as the
    ///      endpoint. Returns the exact composed payload the OFT forwards to the compose queue.
    function _deliverInbound(Order memory order, bytes32 guid, uint256 amount) internal returns (bytes memory) {
        bytes32 peer = oft.peers(Addresses.EID_ARBITRUM);
        require(peer != bytes32(0), "no arbitrum peer");
        Origin memory origin = Origin({srcEid: Addresses.EID_ARBITRUM, sender: peer, nonce: 1});

        // OFT message layout: sendTo(32) | amountSD(8) | composeFrom(32) | composeMsg.
        bytes memory composeBody = abi.encodePacked(srcAdapter.toBytes32(), OrderLib.encode(order));
        bytes memory oftMessage = abi.encodePacked(address(adapter).toBytes32(), uint64(amount), composeBody);

        vm.prank(Addresses.LZ_ENDPOINT_V2);
        oft.lzReceive(origin, guid, oftMessage, address(this), "");

        // What OFTCore re-encodes and hands to endpoint.sendCompose:
        //   nonce(8) | srcEid(4) | amountLD(32) | composeFrom(32) | composeMsg
        return OFTComposeMsgCodec.encode(origin.nonce, origin.srcEid, amount, composeBody);
    }

    /// @dev Mint USDT0 to `to` via the real OFT (a no-compose inbound delivery), so test actors get
    ///      tokens the honest way rather than via `deal` on a proxied token.
    function _mintViaOft(address to, uint256 amount) internal {
        bytes32 peer = oft.peers(Addresses.EID_ARBITRUM);
        Origin memory origin = Origin({srcEid: Addresses.EID_ARBITRUM, sender: peer, nonce: 2});
        bytes memory oftMessage = abi.encodePacked(to.toBytes32(), uint64(amount));
        vm.prank(Addresses.LZ_ENDPOINT_V2);
        oft.lzReceive(origin, keccak256(abi.encode("mint", to, amount)), oftMessage, address(this), "");
    }

    function _approve(address erc20, address spender, uint256 amount) internal {
        (bool ok,) = erc20.call(abi.encodeWithSelector(0x095ea7b3, spender, amount)); // approve(address,uint256)
        require(ok, "approve failed");
    }

    /// @dev Hand-encode LayerZero type-3 options: an executor lzReceive gas option + an executor
    ///      lzCompose gas option (index 0). Mirrors OptionsBuilder without importing the LZ libs.
    function _composeOptions(uint128 lzReceiveGas, uint128 composeGas) internal pure returns (bytes memory) {
        // worker id 1 = executor; option type 1 = lzReceive (16-byte gas); type 3 = lzCompose (index + gas).
        bytes memory recvOpt = abi.encodePacked(uint8(1), uint16(16 + 1), uint8(1), lzReceiveGas);
        bytes memory composeOpt = abi.encodePacked(uint8(1), uint16(18 + 1), uint8(3), uint16(0), composeGas);
        return abi.encodePacked(uint16(3), recvOpt, composeOpt);
    }
}
