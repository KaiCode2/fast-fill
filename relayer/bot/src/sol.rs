//! Generated-binding aliases, the canonical `Order`/`Execution` types, a small hand-written
//! `IERC20`, and the CCTP↔OFT `Order` conversion.
//!
//! `forge bind` emits a distinct `Order` struct per contract module (`cctp_adapter::CctpAdapter`
//! and `oft_adapter::OftAdapter`); they are structurally identical. We standardize the bot on the
//! CCTP adapter's `Order` and convert to the OFT one only at the OFT call site.

// Friendly aliases for the generated contract modules.
pub use fastfill_bindings::cctp_adapter::CctpAdapter as cctp;
pub use fastfill_bindings::fast_fill_config::FastFillConfig as cfg;
pub use fastfill_bindings::oft_adapter::OftAdapter as oft;
pub use fastfill_bindings::oft_adapter_factory::OftAdapterFactory as factory;

/// The canonical order type used throughout the bot (identical layout to `src/libraries/OrderLib.sol`).
pub type Order = cctp::Order;

/// Convert the canonical (CCTP) order into the structurally-identical OFT order for OFT adapter calls.
pub fn to_oft_order(o: &Order) -> oft::Order {
    oft::Order {
        bridgeType: o.bridgeType,
        srcChainId: o.srcChainId,
        dstChainId: o.dstChainId,
        sender: o.sender,
        recipient: o.recipient,
        inputToken: o.inputToken,
        outputToken: o.outputToken,
        inputAmount: o.inputAmount,
        outputAmount: o.outputAmount,
        nonce: o.nonce,
        startTime: o.startTime,
        expectedDeliveryTime: o.expectedDeliveryTime,
        discountRate: o.discountRate,
        baseFee: o.baseFee,
        callbackGasLimit: o.callbackGasLimit,
        hookData: o.hookData.clone(),
    }
}

alloy::sol! {
    #[sol(rpc)]
    #[allow(missing_docs)]
    interface IERC20 {
        function balanceOf(address owner) external view returns (uint256);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 value) external returns (bool);
        function decimals() external view returns (uint8);
    }
}
