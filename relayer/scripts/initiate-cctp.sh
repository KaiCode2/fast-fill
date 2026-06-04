#!/usr/bin/env bash
# Initiate ONE CCTP order from the demo wallet (does NOT fill or settle — that's the relayer's job).
# Prints `ORDER_ID=0x..` and `SRC_TX=0x..` on stdout for capture by the orchestrator.
#
#   initiate-cctp.sh SRC DST AMOUNT MAXFEE MINTFEE [RECIPIENT]
#     SRC/DST  chain ids (42161 Arbitrum, 10 Optimism, 8453 Base)
#     AMOUNT/MAXFEE/MINTFEE  USDC base units (6-dp). MINTFEE>0 ⇒ executor-routed.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"
# shellcheck disable=SC1091
source "$HERE/lib.sh"

SRC="${1:?usage: initiate-cctp.sh SRC DST AMOUNT MAXFEE MINTFEE [RECIPIENT]}"
DST="${2:?dst chain id}"
AMOUNT="${3:?amount (USDC base units)}"
MAXFEE="${4:?maxFee}"
MINTFEE="${5:?mintFee (0 = direct, >0 = executor-routed)}"
RECIPIENT="${6:-$WALLET}"

rpc="$(rpc_for "$SRC")"
usdc="$(usdc_for "$SRC")"
recip_b32="$(to_b32 "$RECIPIENT")"

echo ">> initiate CCTP $(name_for "$SRC")->$(name_for "$DST") amount=$(usdc_fmt "$AMOUNT") maxFee=$(usdc_fmt "$MAXFEE") mintFee=$(usdc_fmt "$MINTFEE") recipient=$RECIPIENT" >&2

# Ensure the adapter can pull inputAmount (no-op once the relayer has set a max allowance).
cur="$(allowance "$SRC" "$WALLET" "$CCTP_ADAPTER")"
if int_lt "$cur" "$AMOUNT"; then
  echo "   approving USDC -> adapter on $(name_for "$SRC")…" >&2
  cast send --rpc-url "$rpc" --private-key "$DEMO_PRIVATE_KEY" \
    "$usdc" "approve(address,uint256)" "$CCTP_ADAPTER" "$AMOUNT" >/dev/null
fi

# initiateCCTP(dstChainId, recipient, inputAmount, maxFee, mintFee, minFinalityThreshold,
#              deliveryWindow, discountRate, baseFee, exec(gasLimit,data))
# FINALITY_FAST=1000, deliveryWindow=3600s, discountRate=0, baseFee=0, no destination execution.
receipt="$(cast send --rpc-url "$rpc" --private-key "$DEMO_PRIVATE_KEY" --json "$CCTP_ADAPTER" \
  "initiateCCTP(uint32,bytes32,uint256,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes))" \
  "$DST" "$recip_b32" "$AMOUNT" "$MAXFEE" "$MINTFEE" 1000 3600 0 0 '(0,0x)')"

src_tx="$(echo "$receipt" | jq -r '.transactionHash')"
order_id="$(echo "$receipt" | jq -r \
  --arg a "$(echo "$CCTP_ADAPTER" | tr 'A-Z' 'a-z')" \
  --arg sig "$(echo "$ORDER_CREATED_SIG" | tr 'A-Z' 'a-z')" \
  '.logs[] | select((.address|ascii_downcase)==$a and (.topics[0]|ascii_downcase)==$sig) | .topics[1]' \
  | head -1)"

if [ -z "$order_id" ] || [ "$order_id" = "null" ]; then
  echo "ERROR: no OrderCreated log found in $src_tx" >&2
  exit 1
fi
echo "   src tx: $src_tx" >&2
echo "SRC_TX=$src_tx"
echo "ORDER_ID=$order_id"
