//! Reconstruct the full `Order` from a source `initiate*` tx and prove its authenticity.
//!
//! Port of `demo/src/lib/server/verify.ts`. The `OrderCreated` event carries only 7 of 16 fields;
//! the rest come from the tx calldata (unwrapping `multicall([selfPermit, initiate])`), the source
//! block timestamp (`startTime`), and the token registry. The recomputed `orderId` MUST equal the
//! emitted one — fill against a wrong order and the bot is never reimbursed.

use alloy::consensus::Transaction as _;
use alloy::primitives::{Address, Bytes, FixedBytes, B256, U256};
use alloy::providers::Provider;
use alloy::rpc::types::BlockNumberOrTag;
use alloy::sol_types::SolCall;
use eyre::{eyre, Result, WrapErr};

use crate::app::App;
use crate::config::BRIDGE_CCTP;
use crate::order::{addr_to_b32, order_id};
use crate::sol::{cctp, oft, Order};
use crate::watcher::Discovered;

#[derive(Clone)]
#[allow(dead_code)] // src_block_number retained for provenance
pub struct VerifiedOrder {
    pub order: Order,
    pub order_id: B256,
    pub bridge_type: u8,
    pub oft_id: Option<u8>,
    pub adapter: Address,
    pub src_chain_id: u64,
    pub dst_chain_id: u64,
    pub src_tx_hash: B256,
    pub src_block_number: u64,
    /// CCTP `mintFee` (USDC base units). `> 0` ⇒ executor-routed (settle via `CctpExecutor.execute`,
    /// the mint-relay path that pays this fee). `0` ⇒ direct (settle via `CctpAdapter.settle`). OFT: 0.
    pub mint_fee: U256,
}

/// The leading `initiate*` args we need (positions are identical for the `*For` variants).
struct InitiateArgs {
    dst_chain_id: u32,
    recipient: FixedBytes<32>,
    input_amount: U256,
    fourth: U256,   // maxFee (CCTP) or minAmountLD (OFT)
    mint_fee: U256, // CCTP executor mint-relay fee (0 for OFT / direct CCTP)
    delivery_window: u64,
    discount_rate: U256,
    base_fee: U256,
    gas_limit: u64,
    hook_data: Bytes,
}

pub async fn reconstruct_and_verify(app: &App, d: &Discovered) -> Result<VerifiedOrder> {
    let meta = app
        .registry
        .adapter_meta
        .get(&d.adapter)
        .ok_or_else(|| eyre!("OrderCreated from unknown adapter {}", d.adapter))?;
    let bridge_type = meta.bridge_type;
    let oft_id = meta.oft_id;
    if bridge_type != d.event.bridge_type {
        return Err(eyre!(
            "bridgeType mismatch: adapter={} event={}",
            bridge_type,
            d.event.bridge_type
        ));
    }

    let provider = app.chains.provider(d.src_chain_id)?;
    let tx = provider
        .get_transaction_by_hash(d.tx_hash)
        .await?
        .ok_or_else(|| eyre!("source tx {} not found", d.tx_hash))?;
    let block = provider
        .get_block_by_number(BlockNumberOrTag::Number(d.block_number))
        .await?
        .ok_or_else(|| eyre!("source block {} not found", d.block_number))?;
    let start_time = block.header.timestamp;

    let input = tx.input().clone();
    let args = decode_initiate(bridge_type, input.as_ref())?;
    let dst_chain_id = args.dst_chain_id as u64;

    let in_info = app
        .registry
        .output_token(d.src_chain_id, bridge_type, oft_id)
        .ok_or_else(|| eyre!("no input token for src chain {}", d.src_chain_id))?;
    let out_info = app
        .registry
        .output_token(dst_chain_id, bridge_type, oft_id)
        .ok_or_else(|| eyre!("no output token for dst chain {}", dst_chain_id))?;

    // CCTP reserves both the bridge fee (maxFee) and the executor mint-relay fee (mintFee); OFT uses
    // the signed slippage floor (minAmountLD).
    let output_amount = if bridge_type == BRIDGE_CCTP {
        args.input_amount - args.fourth - args.mint_fee
    } else {
        args.fourth
    };

    let order = Order {
        bridgeType: bridge_type,
        srcChainId: d.src_chain_id as u32,
        dstChainId: args.dst_chain_id,
        sender: addr_to_b32(d.event.sender),
        recipient: args.recipient,
        inputToken: addr_to_b32(in_info.token),
        outputToken: addr_to_b32(out_info.token),
        inputAmount: args.input_amount,
        outputAmount: output_amount,
        nonce: d.event.nonce,
        startTime: start_time,
        expectedDeliveryTime: start_time + args.delivery_window,
        discountRate: args.discount_rate,
        baseFee: args.base_fee,
        callbackGasLimit: args.gas_limit,
        hookData: args.hook_data,
    };

    let id = order_id(&order);
    if id != d.event.order_id {
        return Err(eyre!(
            "authenticity check failed: recomputed {} != emitted {}",
            id,
            d.event.order_id
        ));
    }
    if addr_to_b32(out_info.token) != d.event.output_token {
        return Err(eyre!("output token mismatch vs OrderCreated event"));
    }

    Ok(VerifiedOrder {
        order,
        order_id: id,
        bridge_type,
        oft_id,
        adapter: d.adapter,
        src_chain_id: d.src_chain_id,
        dst_chain_id,
        src_tx_hash: d.tx_hash,
        src_block_number: d.block_number,
        mint_fee: args.mint_fee,
    })
}

/// Decode the `initiate*` call from tx input, unwrapping a `multicall([selfPermit, initiate])`.
fn decode_initiate(bridge_type: u8, input: &[u8]) -> Result<InitiateArgs> {
    if input.len() >= 4 {
        let sel: [u8; 4] = input[0..4].try_into().unwrap();
        if sel == cctp::multicallCall::SELECTOR {
            let mc = cctp::multicallCall::abi_decode(input).wrap_err("decode multicall")?;
            for call in &mc.data {
                if let Ok(a) = decode_initiate_inner(bridge_type, call.as_ref()) {
                    return Ok(a);
                }
            }
            return Err(eyre!("no initiate call found inside multicall"));
        }
    }
    decode_initiate_inner(bridge_type, input)
}

fn decode_initiate_inner(bridge_type: u8, data: &[u8]) -> Result<InitiateArgs> {
    if data.len() < 4 {
        return Err(eyre!("calldata too short"));
    }
    let sel: [u8; 4] = data[0..4].try_into().unwrap();

    if bridge_type == BRIDGE_CCTP {
        if sel == cctp::initiateCCTPCall::SELECTOR {
            let c = cctp::initiateCCTPCall::abi_decode(data)?;
            Ok(InitiateArgs {
                dst_chain_id: c.dstChainId,
                recipient: c.recipient,
                input_amount: c.inputAmount,
                fourth: c.maxFee,
                mint_fee: c.mintFee,
                delivery_window: c.deliveryWindow,
                discount_rate: c.discountRate,
                base_fee: c.baseFee,
                gas_limit: c.exec.gasLimit,
                hook_data: c.exec.data,
            })
        } else if sel == cctp::initiateCCTPForCall::SELECTOR {
            let c = cctp::initiateCCTPForCall::abi_decode(data)?;
            Ok(InitiateArgs {
                dst_chain_id: c.dstChainId,
                recipient: c.recipient,
                input_amount: c.inputAmount,
                fourth: c.maxFee,
                mint_fee: c.mintFee,
                delivery_window: c.deliveryWindow,
                discount_rate: c.discountRate,
                base_fee: c.baseFee,
                gas_limit: c.exec.gasLimit,
                hook_data: c.exec.data,
            })
        } else {
            Err(eyre!("calldata is not a CCTP initiate call"))
        }
    } else if sel == oft::initiateOFTCall::SELECTOR {
        let c = oft::initiateOFTCall::abi_decode(data)?;
        Ok(InitiateArgs {
            dst_chain_id: c.dstChainId,
            recipient: c.recipient,
            input_amount: c.inputAmount,
            fourth: c.minAmountLD,
            mint_fee: U256::ZERO,
            delivery_window: c.deliveryWindow,
            discount_rate: c.discountRate,
            base_fee: c.baseFee,
            gas_limit: c.exec.gasLimit,
            hook_data: c.exec.data,
        })
    } else if sel == oft::initiateOFTForCall::SELECTOR {
        let c = oft::initiateOFTForCall::abi_decode(data)?;
        Ok(InitiateArgs {
            dst_chain_id: c.dstChainId,
            recipient: c.recipient,
            input_amount: c.inputAmount,
            fourth: c.minAmountLD,
            mint_fee: U256::ZERO,
            delivery_window: c.deliveryWindow,
            discount_rate: c.discountRate,
            base_fee: c.baseFee,
            gas_limit: c.exec.gasLimit,
            hook_data: c.exec.data,
        })
    } else {
        Err(eyre!("calldata is not an OFT initiate call"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{BRIDGE_CCTP, CCTP_ADAPTER, FASTFILL_CONFIG};
    use crate::order::order_id;
    use crate::sol::cfg;
    use alloy::providers::ProviderBuilder;
    use alloy::sol_types::SolEvent;

    /// Reconstruct a REAL mainnet CCTP order from its source tx and assert the recomputed `orderId`
    /// equals the emitted one — the load-bearing authenticity gate, including the `mintFee` term in
    /// `outputAmount = inputAmount - maxFee - mintFee`.
    async fn check_golden(rpc: &str, src_chain_id: u64, tx_hash_str: &str) {
        let provider = ProviderBuilder::new()
            .connect_http(rpc.parse().unwrap())
            .erased();
        let tx_hash: B256 = tx_hash_str.parse().unwrap();

        let receipt = provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .expect("receipt");
        let tx = provider
            .get_transaction_by_hash(tx_hash)
            .await
            .unwrap()
            .expect("tx");
        let block = provider
            .get_block_by_number(BlockNumberOrTag::Number(receipt.block_number.unwrap()))
            .await
            .unwrap()
            .unwrap();
        let start_time = block.header.timestamp;

        let ev = receipt
            .inner
            .logs()
            .iter()
            .find_map(|l| {
                if l.inner.address != CCTP_ADAPTER {
                    return None;
                }
                cctp::OrderCreated::decode_log(&l.inner).ok()
            })
            .expect("OrderCreated from CctpAdapter");
        let e = ev.data;

        let args = decode_initiate(BRIDGE_CCTP, tx.input().as_ref()).unwrap();
        let dst = args.dst_chain_id as u64;

        // FastFillConfig is a registry of ALL chains, readable from any chain's deployment.
        let conf = cfg::new(FASTFILL_CONFIG, provider.clone());
        let src_usdc = conf
            .chainConfig(U256::from(src_chain_id))
            .call()
            .await
            .unwrap()
            .usdc;
        let dst_usdc = conf.chainConfig(U256::from(dst)).call().await.unwrap().usdc;

        let order = Order {
            bridgeType: BRIDGE_CCTP,
            srcChainId: src_chain_id as u32,
            dstChainId: args.dst_chain_id,
            sender: addr_to_b32(e.sender),
            recipient: args.recipient,
            inputToken: addr_to_b32(src_usdc),
            outputToken: addr_to_b32(dst_usdc),
            inputAmount: args.input_amount,
            outputAmount: args.input_amount - args.fourth - args.mint_fee,
            nonce: e.nonce,
            startTime: start_time,
            expectedDeliveryTime: start_time + args.delivery_window,
            discountRate: args.discount_rate,
            baseFee: args.base_fee,
            callbackGasLimit: args.gas_limit,
            hookData: args.hook_data,
        };

        let id = order_id(&order);
        assert_eq!(
            id, e.orderId,
            "reconstructed orderId must equal the emitted OrderCreated.orderId"
        );
    }

    /// Direct CCTP order (`mintFee = 0`), Base → Arbitrum. Source: DEPLOYMENTS.md smoke.
    /// Network test — run with `cargo test -- --ignored` (set RPC_URL_8453 to override).
    #[tokio::test]
    #[ignore = "network: requires a Base RPC; run with --ignored"]
    async fn golden_orderid_base_cctp_direct() {
        let rpc =
            std::env::var("RPC_URL_8453").unwrap_or_else(|_| "https://mainnet.base.org".into());
        check_golden(
            &rpc,
            8453,
            "0x4c6b282cde6fc03bcc0c3bc75b40a601aaadf232e789db0643039052a7b790bc",
        )
        .await;
    }

    /// Executor-routed CCTP order (`mintFee > 0`), Arbitrum → Base. Source: DEPLOYMENTS.md smoke.
    /// Exercises the `outputAmount = inputAmount - maxFee - mintFee` reconstruction.
    #[tokio::test]
    #[ignore = "network: requires an Arbitrum RPC; run with --ignored"]
    async fn golden_orderid_arbitrum_cctp_routed() {
        let rpc = std::env::var("RPC_URL_42161")
            .unwrap_or_else(|_| "https://arbitrum-one.publicnode.com".into());
        check_golden(
            &rpc,
            42161,
            "0x83eef9a71fae6f0057ae954cc9c22221866c469f97deef8c9b398c733aa98504",
        )
        .await;
    }
}
