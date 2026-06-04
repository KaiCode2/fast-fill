#!/usr/bin/env bash
# End-to-end live test of the relayer against mainnet. Starts the relayer (low caps), then initiates
# real CCTP orders the relayer must fill + settle:
#   Scenario A — optimistic fill + DIRECT settle (mintFee = 0)       : exercises fill() + CctpAdapter.settle
#   Scenario B — optimistic fill + EXECUTOR mint relay (mintFee > 0) : exercises fill() + CctpExecutor.execute
#
# The harness only INITIATES + observes; all fills/settles/executes are done by the relayer process.
# Uses the demo wallet from .env for both roles, so it runs on different chains than the relayer acts
# on (harness sends on the source chain; relayer sends on the destination) to avoid nonce conflicts.
#
# Env overrides: AMOUNT, MAXFEE, MINTFEE, RELAYER_MAX_BASE_UNITS, WATCH_TIMEOUT, SCENARIOS=AB|A|B
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"
# shellcheck disable=SC1091
source "$HERE/lib.sh"
RELAYER_DIR="$(cd "$HERE/.." && pwd)"

AMOUNT="${AMOUNT:-100000}"     # 0.100000 USDC
MAXFEE="${MAXFEE:-500}"        # 0.000500 USDC bridge fee cap
MINTFEE="${MINTFEE:-1000}"     # 0.001000 USDC mint-relay fee (Scenario B, filled)
MINTFEE_C="${MINTFEE_C:-30000}" # 0.030000 USDC mint-relay fee (Scenario C, unfilled — must exceed gas)
MAXCAP="${RELAYER_MAX_BASE_UNITS:-300000}"  # 0.3 USDC per-fill cap (safety)
WATCH_TIMEOUT="${WATCH_TIMEOUT:-1200}"
SCENARIOS="${SCENARIOS:-AB}"
LOG="${LOG:-$HERE/.live-test-relayer.log}"

# Scenario C tests the PURE mint relay of an order we do NOT fill, so it runs the relayer with
# optimistic filling disabled (no token enabled). A/B need USDC filling enabled.
if [[ "$SCENARIOS" == *A* || "$SCENARIOS" == *B* ]]; then ENABLED_TOKENS="USDC"; else ENABLED_TOKENS="NONE"; fi

echo "==================== PREFLIGHT ===================="
bash "$HERE/check-funding.sh"

echo ""; echo "==================== BUILD ===================="
(cd "$RELAYER_DIR" && cargo build -q)
RELAYER_BIN="$RELAYER_DIR/target/debug/fastfill-relayer"

echo ""; echo "==================== START RELAYER (live, fills=$ENABLED_TOKENS cap=$(usdc_fmt "$MAXCAP") USDC) ===================="
: > "$LOG"
RELAYER_PRIVATE_KEY="$DEMO_PRIVATE_KEY" ALCHEMY_API_KEY="$ALCHEMY_API_KEY" \
  RELAYER_ENABLED_TOKENS="$ENABLED_TOKENS" RELAYER_MAX_BASE_UNITS="$MAXCAP" RELAYER_MIN_FEE=0 \
  RELAYER_MIN_MINT_FEE=0 RELAYER_SRC_CONFIRMATIONS=1 RUST_LOG="info" \
  "$RELAYER_BIN" >"$LOG" 2>&1 &
RELAYER_PID=$!
cleanup() { kill "$RELAYER_PID" 2>/dev/null || true; }
trap cleanup EXIT

for _ in $(seq 1 90); do
  grep -q "relayer running" "$LOG" && break
  kill -0 "$RELAYER_PID" 2>/dev/null || { echo "relayer exited early:"; tail -30 "$LOG"; exit 1; }
  sleep 2
done
grep -q "relayer running" "$LOG" || { echo "relayer did not start in time:"; tail -30 "$LOG"; exit 1; }
echo "relayer up (pid $RELAYER_PID), startup approvals done; log: $LOG"

# --- initiate the selected scenarios up front, then watch them concurrently ---
names=(); oids=(); dsts=(); filledseen=(); settledseen=()

initiate() {
  local name="$1" src="$2" dst="$3" mintfee="$4"
  echo ""; echo "-------- initiate: $name --------"
  local out oid
  out="$(bash "$HERE/initiate-cctp.sh" "$src" "$dst" "$AMOUNT" "$MAXFEE" "$mintfee")"
  oid="$(printf '%s\n' "$out" | sed -n 's/^ORDER_ID=//p')"
  if [ -z "$oid" ]; then echo "FAIL: could not initiate $name"; return 1; fi
  echo "$name → order $oid (dst $(name_for "$dst"))"
  names+=("$name"); oids+=("$oid"); dsts+=("$dst"); filledseen+=("0"); settledseen+=("0")
}

echo ""; echo "==================== INITIATE ORDERS ===================="
case "$SCENARIOS" in *A*) initiate "Scenario A (optimistic fill + direct settle, mintFee=0)" 10 8453 0 ;; esac
case "$SCENARIOS" in *B*) initiate "Scenario B (optimistic fill + executor mint relay, mintFee>0)" 42161 8453 "$MINTFEE" ;; esac
case "$SCENARIOS" in *C*) initiate "Scenario C (UNFILLED pure mint relay, mintFee>gas)" 10 8453 "$MINTFEE_C" ;; esac
[ "${#oids[@]}" -gt 0 ] || { echo "no orders initiated"; exit 1; }

echo ""; echo "==================== WATCH (timeout ${WATCH_TIMEOUT}s) ===================="
deadline=$(( $(date +%s) + WATCH_TIMEOUT ))
while [ "$(date +%s)" -lt "$deadline" ]; do
  alldone=1
  for i in "${!oids[@]}"; do
    [ "${settledseen[$i]}" = "1" ] && continue
    st="$(order_status "${dsts[$i]}" "${oids[$i]}" || true)"; st="${st:-0}"
    if [ "$st" -ge 1 ] && [ "${filledseen[$i]}" = "0" ]; then
      f="$(order_filler "${dsts[$i]}" "${oids[$i]}")"
      filledseen[$i]=1
      if [ "$f" != "0x0000000000000000000000000000000000000000" ]; then
        echo "[$(date +%H:%M:%S)] FILLED  ${names[$i]}  (filler=$f)"
      else
        echo "[$(date +%H:%M:%S)] (no optimistic fill — pure mint relay)  ${names[$i]}"
      fi
    fi
    if [ "$st" = "2" ]; then
      settledseen[$i]=1
      echo "[$(date +%H:%M:%S)] SETTLED ${names[$i]}"
    else
      alldone=0
    fi
  done
  [ "$alldone" = "1" ] && break
  sleep 10
done

echo ""; echo "==================== SUMMARY ===================="
rc=0
for i in "${!oids[@]}"; do
  echo ""; echo "### ${names[$i]}"
  echo "order: ${oids[$i]}  dst: $(name_for "${dsts[$i]}")  final status: $(order_status "${dsts[$i]}" "${oids[$i]}")"
  echo "relayer log:"; grep -F "${oids[$i]}" "$LOG" | sed 's/^/   /' || true
  if [ "${settledseen[$i]}" = "1" ]; then echo "PASS — order Settled by the relayer"; else echo "FAIL — not settled within ${WATCH_TIMEOUT}s"; rc=1; fi
done

echo ""; echo "==================== RESULT (rc=$rc) ===================="
[ "$rc" = 0 ] && echo "ALL SCENARIOS PASSED" || echo "SOME SCENARIOS FAILED — see above and $LOG"
exit "$rc"
