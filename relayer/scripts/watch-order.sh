#!/usr/bin/env bash
# Poll a destination CCTP order until it Settles (status 2) or times out.
# Prints `RESULT=SETTLED` (exit 0) or `RESULT=TIMEOUT` (exit 1).
#
#   watch-order.sh DST ORDER_ID [TIMEOUT_SECS=1200] [POLL_SECS=10]
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"
# shellcheck disable=SC1091
source "$HERE/lib.sh"

DST="${1:?dst chain id}"
ORDER_ID="${2:?order id}"
TIMEOUT="${3:-1200}"
POLL="${4:-10}"

deadline=$(( $(date +%s) + TIMEOUT ))
last=""
echo ">> watching $ORDER_ID on $(name_for "$DST") (timeout ${TIMEOUT}s)" >&2
while [ "$(date +%s)" -lt "$deadline" ]; do
  st="$(order_status "$DST" "$ORDER_ID" || true)"
  st="${st:-0}"
  if [ "$st" != "$last" ]; then
    label="None"; [ "$st" = "1" ] && label="Filled"; [ "$st" = "2" ] && label="Settled"
    echo "   status=$st ($label) filler=$(order_filler "$DST" "$ORDER_ID")" >&2
    last="$st"
  fi
  if [ "$st" = "2" ]; then
    echo "RESULT=SETTLED"
    exit 0
  fi
  sleep "$POLL"
done
echo "RESULT=TIMEOUT"
exit 1
