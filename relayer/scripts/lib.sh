#!/usr/bin/env bash
# Shared helpers for the live-test harness. Sourced by the other scripts.
# Reads the repo-root .env (ALCHEMY_API_KEY, DEMO_PRIVATE_KEY).
set -euo pipefail

# Resolve repo root from this file's location (works whether sourced under bash or zsh).
_lib_src="${BASH_SOURCE[0]:-${(%):-%x}}"
REPO="$(cd "$(dirname "$_lib_src")/../.." && pwd)"
# shellcheck disable=SC1091
set -a; source "$REPO/.env"; set +a

export FOUNDRY_DISABLE_NIGHTLY_WARNING=1

: "${ALCHEMY_API_KEY:?set ALCHEMY_API_KEY in $REPO/.env}"
: "${DEMO_PRIVATE_KEY:?set DEMO_PRIVATE_KEY in $REPO/.env}"

# OrderCreated(bytes32 indexed orderId, uint8, address indexed sender, uint32, bytes32, uint256, uint64)
export ORDER_CREATED_SIG="0x$(cast keccak 'OrderCreated(bytes32,uint8,address,uint32,bytes32,uint256,uint64)' | sed 's/^0x//')"

# Deployed mainnet contracts (CREATE2-identical on every chain). Source: DEPLOYMENTS.md.
export CCTP_ADAPTER="0x9FA37faBfA1Fd31Afe5A5F93e1c4Cd986b27bA75" # executor-enabled
export CCTP_EXECUTOR="0xAFc7bBc0B5fD7A4d9b936349cfE991e5bC6E2a80"

WALLET="$(cast wallet address --private-key "$DEMO_PRIVATE_KEY")"
export WALLET

# chain_id -> alchemy slug / usdc / cctp domain / short name
rpc_for() {
  local slug
  case "$1" in
    42161) slug="arb-mainnet" ;;
    10) slug="opt-mainnet" ;;
    8453) slug="base-mainnet" ;;
    *) echo "unknown chain $1" >&2; return 1 ;;
  esac
  echo "https://${slug}.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
}
usdc_for() {
  case "$1" in
    42161) echo "0xaf88d065e77c8cC2239327C5EDb3A432268e5831" ;;
    10) echo "0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85" ;;
    8453) echo "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913" ;;
    *) echo "unknown chain $1" >&2; return 1 ;;
  esac
}
name_for() {
  case "$1" in 42161) echo Arbitrum ;; 10) echo Optimism ;; 8453) echo Base ;; *) echo "chain$1" ;; esac
}

# 0x-prefixed, left-padded bytes32 for an address.
to_b32() { local a="${1#0x}"; echo "0x000000000000000000000000${a}"; }

usdc_balance() { cast call --rpc-url "$(rpc_for "$1")" "$(usdc_for "$1")" "balanceOf(address)(uint256)" "$2" | awk '{print $1}'; }
eth_balance()  { cast balance --rpc-url "$(rpc_for "$1")" "$2"; }
allowance()    { cast call --rpc-url "$(rpc_for "$1")" "$(usdc_for "$1")" "allowance(address,address)(uint256)" "$2" "$3" | awk '{print $1}'; }

# getOrder(orderId).status on the CCTP adapter of chain $1 (0 None, 1 Filled, 2 Settled).
order_status() {
  cast call --rpc-url "$(rpc_for "$1")" "$CCTP_ADAPTER" \
    "getOrder(bytes32)((address,uint8,uint40))" "$2" 2>/dev/null \
    | tr -d '()' | awk -F',' '{gsub(/ /,"",$2); print $2}'
}
order_filler() {
  cast call --rpc-url "$(rpc_for "$1")" "$CCTP_ADAPTER" \
    "getOrder(bytes32)((address,uint8,uint40))" "$2" 2>/dev/null \
    | tr -d '()' | awk -F',' '{gsub(/ /,"",$1); print $1}'
}

# Format 6-dp USDC base units as a decimal for logs.
usdc_fmt() { awk -v v="$1" 'BEGIN{printf "%.6f", v/1000000}'; }

# Non-negative big-integer less-than (no bash 64-bit overflow): compare by length, then lexically.
int_lt() {
  local a="${1:-0}" b="${2:-0}"
  if [ "${#a}" -ne "${#b}" ]; then [ "${#a}" -lt "${#b}" ]; return; fi
  [[ "$a" < "$b" ]]
}
