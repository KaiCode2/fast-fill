// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {console2} from "forge-std/console2.sol";
import {Fixtures} from "../utils/Fixtures.sol";
import {Order, OrderLib, Execution} from "../../src/libraries/OrderLib.sol";
import {CctpMessageBuilder} from "../utils/CctpMessageBuilder.sol";
import {SendParam, MessagingFee} from "../../src/interfaces/layerzero/IOFT.sol";

/// @notice Gas benchmarks for the fast-fill adapters. Two things are reported:
///
///  • ABSOLUTE gas for each fast-fill op (initiate / fill / settle), measured against the in-EVM
///    mocks. These are NOT mainnet-representative — the real CCTP/LayerZero contracts cost far more
///    (their numbers are captured by the fork tests). Use them only to compare ops to each other.
///
///  • OVERHEAD: how much gas fast-fill adds versus calling the bridge directly. This IS faithful even
///    on mocks, because the measured bridge call (`depositForBurnWithHook` / `oft.send` /
///    `receiveMessage`) is byte-identical in both the adapter path and the bare path, so the mock's
///    own cost cancels in the subtraction. The remainder is exactly what fast-fill adds: pulling the
///    user's funds into the adapter, the order build + hash + nonce, the live cross-checks, the
///    OrderCreated event, and — on settle — re-parsing the authenticated message and disbursing.
///
/// Run: `forge test --match-path test/gas/GasBench.t.sol -vv` (the table prints via console2), or
/// `forge test --match-path test/gas/GasBench.t.sol --gas-report` for the per-function view.
contract GasBenchTest is Fixtures {
    using OrderLib for Order;

    uint256 constant INPUT = 1_000e6;
    uint256 constant MAX_FEE = 1e6;
    uint256 constant RATE = 1e13;
    uint64 constant WINDOW = 100;

    uint256 constant OFT_INPUT = 1_000e18;
    uint256 constant OFT_MIN = 999e18;

    uint256 bareNonce; // distinct CCTP bridge nonces for the bare-path messages

    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp();
        _setUpOft();
    }

    // ---------------------------------------------------------------------------------------------
    // CCTP
    // ---------------------------------------------------------------------------------------------

    function _gasCctpInitiate() internal returns (uint256 used) {
        usdc.mint(user, INPUT);
        vm.chainId(SRC_CHAIN);
        vm.prank(user);
        usdc.approve(address(srcCctp), INPUT);
        vm.prank(user);
        uint256 g0 = gasleft();
        srcCctp.initiateCCTP(DST_CHAIN, _b32(recipient), INPUT, MAX_FEE, 1000, WINDOW, RATE, 0, Execution(0, ""));
        used = g0 - gasleft();
    }

    /// @dev Bare baseline: what a user pays calling CCTP directly (approve messenger, burn-with-hook
    ///      carrying an identically-sized order payload). Uses the same mock messenger as the adapter.
    function _gasCctpBareBurn() internal returns (uint256 used) {
        Order memory o =
            _cctpOrder(INPUT, MAX_FEE, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, 0);
        bytes memory hookData = OrderLib.encode(o);
        usdc.mint(user, INPUT);
        vm.chainId(SRC_CHAIN);
        vm.prank(user);
        usdc.approve(address(messengerSrc), INPUT);
        vm.prank(user);
        uint256 g0 = gasleft();
        messengerSrc.depositForBurnWithHook(
            INPUT, DST_DOMAIN, _b32(recipient), address(usdc), _b32(recipient), MAX_FEE, 1000, hookData
        );
        used = g0 - gasleft();
    }

    function _gasCctpFill(Order memory order) internal returns (uint256 used) {
        vm.chainId(DST_CHAIN);
        (uint256 payout,) = dstCctp.quoteFill(order, block.timestamp);
        usdc.mint(relayer, payout);
        vm.prank(relayer);
        usdc.approve(address(dstCctp), payout);
        vm.prank(relayer);
        uint256 g0 = gasleft();
        dstCctp.fill(order);
        used = g0 - gasleft();
    }

    /// @dev fast-fill settle (unfilled path: recipient is paid). Wraps receiveMessage + auth + payout.
    function _gasCctpSettle(Order memory order) internal returns (uint256 used) {
        vm.chainId(DST_CHAIN);
        bytes memory message = _cctpMessage(order, 4e5, bytes32(++bareNonce));
        uint256 g0 = gasleft();
        dstCctp.settle(message, "");
        used = g0 - gasleft();
    }

    /// @dev Bare baseline: the raw receiveMessage a direct CCTP user calls (mints to an EOA, open
    ///      destinationCaller). Same mock transmitter as the adapter's settle.
    function _gasCctpBareReceive(Order memory order) internal returns (uint256 used) {
        bytes memory message = CctpMessageBuilder.build(
            CctpMessageBuilder.Msg({
                sourceDomain: SRC_DOMAIN,
                destinationDomain: DST_DOMAIN,
                nonce: bytes32(++bareNonce),
                destinationCaller: bytes32(0), // open: anyone may relay a direct transfer
                burnToken: _b32(address(usdc)),
                mintRecipient: _b32(recipient), // mint straight to an EOA
                amount: order.inputAmount,
                messageSender: _b32(address(srcCctp)),
                maxFee: MAX_FEE,
                feeExecuted: 4e5,
                hookData: OrderLib.encode(order)
            })
        );
        vm.chainId(DST_CHAIN);
        uint256 g0 = gasleft();
        transmitter.receiveMessage(message, "");
        used = g0 - gasleft();
    }

    // ---------------------------------------------------------------------------------------------
    // OFT
    // ---------------------------------------------------------------------------------------------

    function _gasOftInitiate() internal returns (uint256 used) {
        oftToken.mint(user, OFT_INPUT);
        vm.chainId(SRC_CHAIN);
        vm.prank(user);
        oftToken.approve(address(srcOft), OFT_INPUT);
        vm.prank(user);
        uint256 g0 = gasleft();
        srcOft.initiateOFT(DST_CHAIN, _b32(recipient), OFT_INPUT, OFT_MIN, "", WINDOW, RATE, 0, Execution(0, ""));
        used = g0 - gasleft();
    }

    /// @dev Bare baseline: a direct OFT user calling `send` with an identically-sized compose payload.
    function _gasOftBareSend() internal returns (uint256 used) {
        Order memory o =
            _oftOrder(OFT_INPUT, OFT_MIN, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, 0);
        SendParam memory sp = SendParam({
            dstEid: DST_EID,
            to: _b32(address(dstOft)),
            amountLD: OFT_INPUT,
            minAmountLD: OFT_MIN,
            extraOptions: "",
            composeMsg: OrderLib.encode(o),
            oftCmd: ""
        });
        oftToken.mint(user, OFT_INPUT);
        vm.chainId(SRC_CHAIN);
        vm.prank(user);
        oftToken.approve(address(oftToken), OFT_INPUT);
        vm.prank(user);
        uint256 g0 = gasleft();
        oftToken.send(sp, MessagingFee({nativeFee: 0, lzTokenFee: 0}), user);
        used = g0 - gasleft();
    }

    /// @dev fast-fill OFT settle = the lzCompose callback (the OFT credited the adapter during
    ///      _lzReceive; the executor then drives this). There is no bare baseline — compose IS the
    ///      settle on OFT — so this is reported as an absolute cost only.
    function _gasOftCompose(Order memory order) internal returns (uint256 used) {
        vm.chainId(DST_CHAIN);
        oftToken.mint(address(dstOft), OFT_INPUT);
        uint256 g0 = gasleft();
        lzEndpoint.deliver(
            address(dstOft), address(oftToken), SRC_EID, _b32(address(srcOft)), OFT_INPUT, OrderLib.encode(order)
        );
        used = g0 - gasleft();
    }

    function _mkCctpOrder(uint64 nonce) internal view returns (Order memory) {
        return _cctpOrder(INPUT, MAX_FEE, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, nonce);
    }

    function _mkOftOrder(uint64 nonce) internal view returns (Order memory) {
        return _oftOrder(OFT_INPUT, OFT_MIN, uint64(block.timestamp), uint64(block.timestamp) + WINDOW, RATE, 0, nonce);
    }

    // ---------------------------------------------------------------------------------------------
    // Report (asserts overheads stay within sane regression bounds, and prints the table)
    // ---------------------------------------------------------------------------------------------

    function test_gas_report() public {
        // A test function is one transaction, so the FIRST touch of any storage slot pays the cold
        // surcharge. Run every path once as a throwaway to warm the shared slots (mock bridge state,
        // token balances), so the MEASURED runs below reflect steady-state gas and the bridge call —
        // identical in the adapter and bare paths — cancels cleanly in the overhead. Distinct order
        // nonces are used so the measured fill/settle/compose act on fresh orderIds.
        _gasCctpBareBurn();
        _gasCctpInitiate();
        _gasCctpFill(_mkCctpOrder(100));
        _gasCctpSettle(_mkCctpOrder(101));
        _gasCctpBareReceive(_mkCctpOrder(102));
        _gasOftBareSend();
        _gasOftInitiate();
        _gasOftCompose(_mkOftOrder(100));

        // --- measured (warm) ---
        uint256 cctpBareBurn = _gasCctpBareBurn();
        uint256 cctpInit = _gasCctpInitiate();
        uint256 cctpFill = _gasCctpFill(_mkCctpOrder(200));
        uint256 cctpSettle = _gasCctpSettle(_mkCctpOrder(201)); // unfilled path: pays the recipient
        uint256 cctpBareRecv = _gasCctpBareReceive(_mkCctpOrder(202));
        uint256 oftBareSend = _gasOftBareSend();
        uint256 oftInit = _gasOftInitiate();
        uint256 oftCompose = _gasOftCompose(_mkOftOrder(200));

        console2.log("=== fast-fill gas (mock bridges; absolute is illustrative, OVERHEAD is faithful) ===");
        console2.log("--- CCTP ---");
        console2.log("initiateCCTP (fast-fill)        :", cctpInit);
        console2.log("depositForBurnWithHook (direct) :", cctpBareBurn);
        console2.log("  => initiate OVERHEAD          :", cctpInit - cctpBareBurn);
        console2.log("fill (fast-fill only)           :", cctpFill);
        console2.log("settle unfilled (fast-fill)     :", cctpSettle);
        console2.log("receiveMessage (direct)         :", cctpBareRecv);
        console2.log("  => settle OVERHEAD            :", cctpSettle - cctpBareRecv);
        console2.log("--- OFT ---");
        console2.log("initiateOFT (fast-fill)         :", oftInit);
        console2.log("oft.send (direct)               :", oftBareSend);
        console2.log("  => initiate OVERHEAD          :", oftInit - oftBareSend);
        console2.log("lzCompose settle (fast-fill)    :", oftCompose);

        // Regression guards: the adapter overhead must stay modest (well under a typical bridge tx).
        assertGt(cctpInit, cctpBareBurn, "CCTP adapter should cost more than the bare bridge");
        assertGt(oftInit, oftBareSend, "OFT adapter should cost more than the bare bridge");
        assertLt(cctpInit - cctpBareBurn, 150_000, "CCTP initiate overhead regressed");
        assertLt(oftInit - oftBareSend, 150_000, "OFT initiate overhead regressed");
        assertLt(cctpSettle - cctpBareRecv, 150_000, "CCTP settle overhead regressed");
    }
}
