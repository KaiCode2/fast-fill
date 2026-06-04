// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ForkBase} from "./ForkBase.sol";
import {CctpAdapter} from "../../src/adapters/CctpAdapter.sol";
import {CctpExecutor} from "../../src/CctpExecutor.sol";
import {FastFillConfig} from "../../src/config/FastFillConfig.sol";
import {ChainConfig} from "../../src/interfaces/IFastFillConfig.sol";
import {ITokenMessengerV2} from "../../src/interfaces/cctp/ITokenMessengerV2.sol";
import {IMessageTransmitterV2} from "../../src/interfaces/cctp/IMessageTransmitterV2.sol";

/// @notice Fork checks against the real CCTP v2 contracts AND the baked FastFillConfig. Self-skips
///         when no RPC is available, so the default/ci profiles run it as a no-op.
contract CctpForkTest is ForkBase {
    /// @notice The registry's Ethereum row must match the live chain — same domain the real
    ///         MessageTransmitter reports — and the adapter must construct against the real config.
    function test_fork_configMatchesLiveChain() external {
        if (!_forkMainnetOrSkip()) return; // Ethereum mainnet, chainid 1

        FastFillConfig config = new FastFillConfig();
        ChainConfig memory c = config.chainConfig(1);
        assertTrue(c.supported, "ethereum supported");
        assertGt(c.cctpTokenMessenger.code.length, 0, "TokenMessenger has code");
        assertGt(c.usdc.code.length, 0, "USDC has code");

        // The registry's domain must equal what the live MessageTransmitter reports.
        address transmitter = ITokenMessengerV2(c.cctpTokenMessenger).localMessageTransmitter();
        assertEq(IMessageTransmitterV2(transmitter).localDomain(), c.cctpDomain, "domain matches live chain");

        CctpExecutor executor = new CctpExecutor(address(config), address(this));
        CctpAdapter adapter = new CctpAdapter(address(config), address(this), 5e15, address(executor));
        assertEq(address(adapter.config()), address(config), "config wired");
        assertEq(adapter.cctpExecutor(), address(executor), "executor wired");
    }
}
