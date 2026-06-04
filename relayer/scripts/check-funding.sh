#!/usr/bin/env bash
# Preflight: print the wallet's ETH/USDC balances and adapter allowance on each chain.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"
# shellcheck disable=SC1091
source "$HERE/lib.sh"

echo "Wallet: $WALLET"
printf "%-10s %-12s %-12s %-14s\n" "chain" "ETH" "USDC" "USDC->adapter"
for c in 42161 10 8453; do
  eth="$(eth_balance "$c" "$WALLET")"
  usdc="$(usdc_balance "$c" "$WALLET")"
  allw="$(allowance "$c" "$WALLET" "$CCTP_ADAPTER")"
  printf "%-10s %-12s %-12s %-14s\n" \
    "$(name_for "$c")" \
    "$(awk -v v="$eth" 'BEGIN{printf "%.5f", v/1e18}')" \
    "$(usdc_fmt "$usdc")" \
    "$(usdc_fmt "$allw")"
done
