"use client";

import { useEffect, useMemo, useState } from "react";
import { formatUnits, isAddress, parseUnits, type Address, type Hex } from "viem";
import { useAccount } from "wagmi";
import type { SubmitMode } from "@/lib/api";
import {
  deliveryWindowFor,
  FINALITY_FAST,
  FINALITY_FINALIZED,
  outputAmountOf,
  suggestMaxFee,
  type BridgeParams,
} from "@/lib/bridge";
import { REGISTRY, SUPPORTED_CHAIN_IDS, type SupportedChainId, type TokenSymbol } from "@/lib/chains";
import { cctpMintFeeUsd, contractsConfigured, maxUsdPerTransfer } from "@/lib/config";
import { fmtAmount } from "@/lib/format";
import { DEFAULT_MAX_FEE_RATE, linearDiscountRate } from "@/lib/pricing";
import { bridgeTypeForToken, chainsForToken, destinationsFor, getToken, isRouteSupported } from "@/lib/tokens";
import { BRIDGE_CCTP } from "@/lib/order";
import { useBalances } from "@/hooks/useBalances";
import { useInitiate } from "@/hooks/useInitiate";
import { useOftFee } from "@/hooks/useOftFee";
import type { TransferRecord } from "@/hooks/useTransfers";
import { FeePreview } from "./FeePreview";
import { InfoTip } from "./InfoTip";

/** hookData must be a 0x-prefixed, whole-byte hex string ("0x" = deliver funds only, no hook). */
const HEX_RE = /^0x([0-9a-fA-F]{2})*$/;

const MODES: { key: SubmitMode; label: string; hint: string }[] = [
  { key: "standard", label: "Approval Tx", hint: "Approve + send yourself" },
  { key: "permit2612", label: "Permit (ERC-2612)", hint: "Permit + send in one tx" },
  { key: "permit2", label: "Permit2", hint: "Sign only — relayer submits" },
];

function tryParseUnits(s: string, decimals: number): bigint | null {
  if (!s || !/^\d*\.?\d*$/.test(s)) return null;
  try {
    return parseUnits(s as `${number}`, decimals);
  } catch {
    return null;
  }
}

export function BridgeForm({ onStarted }: { onStarted: (t: TransferRecord) => void }) {
  const { address, isConnected } = useAccount();
  const { balances } = useBalances();
  const { submit, state } = useInitiate();

  const [token, setToken] = useState<TokenSymbol>("USDC");
  const [src, setSrc] = useState<SupportedChainId>(8453);
  const [dst, setDst] = useState<SupportedChainId>(42161);
  const [amountStr, setAmountStr] = useState("1");
  const [sendToSelf, setSendToSelf] = useState(true);
  const [recipientStr, setRecipientStr] = useState("");
  const [finality, setFinality] = useState<number>(FINALITY_FAST);
  const [relayMint, setRelayMint] = useState(true); // CCTP "Relay Mint" — guarantee delivery via the executor
  const [mode, setMode] = useState<SubmitMode>("permit2612");
  const [advanced, setAdvanced] = useState(false);
  const [maxFeeStr, setMaxFeeStr] = useState(""); // empty → auto
  const [windowStr, setWindowStr] = useState(""); // empty → auto (from fast/slow)
  const [discountRateStr, setDiscountRateStr] = useState(""); // empty → auto (linear over the window)
  const [baseFeeStr, setBaseFeeStr] = useState("0");
  const [hookDataStr, setHookDataStr] = useState("0x"); // destination-execution payload ("0x" = none)
  const [gasLimitStr, setGasLimitStr] = useState(""); // callback gas for the recipient's onFastFill hook

  const bridgeType = bridgeTypeForToken(token);
  const decimals = getToken(src, token).decimals;

  // Switching token must also move the route onto chains that support it (USD₮0 is Arb/Op only).
  // Done in the click handler — batched with setToken — so we never render an unsupported
  // (src, token) pair, which would throw in getToken() before any effect could correct it.
  function onSelectToken(t: TokenSymbol) {
    if (!isRouteSupported(t, src, dst)) {
      const chains = chainsForToken(t);
      const ns = chains.includes(src) ? src : chains[0];
      const nd = destinationsFor(t, ns)[0] ?? chains.find((c) => c !== ns)!;
      setSrc(ns);
      setDst(nd);
    }
    setToken(t);
  }

  useEffect(() => {
    if (src === dst || !isRouteSupported(token, src, dst)) {
      const opts = destinationsFor(token, src);
      if (opts.length) setDst(opts[0]);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [src]);

  const srcChains = chainsForToken(token);
  const dstChains = destinationsFor(token, src);

  const amount = tryParseUnits(amountStr, decimals);
  const recipient: Address | undefined = sendToSelf
    ? (address as Address | undefined)
    : isAddress(recipientStr)
      ? (recipientStr as Address)
      : undefined;

  const maxFee = useMemo(() => {
    if (bridgeType !== BRIDGE_CCTP || finality === FINALITY_FINALIZED) return 0n;
    if (maxFeeStr) return tryParseUnits(maxFeeStr, decimals) ?? 0n;
    return amount ? suggestMaxFee(amount) : 0n;
  }, [bridgeType, finality, maxFeeStr, decimals, amount]);

  // "Relay Mint" (CCTP only): a non-zero mintFee routes settlement through the CctpExecutor so the
  // relayer is paid to guarantee delivery whether or not an optimistic filler shows up. When the
  // toggle is off we fall back to the legacy direct-settle path (no executor fee). For OFT the LZ
  // executor always auto-delivers settlement, so forwarding stays on.
  const relayMintActive = bridgeType === BRIDGE_CCTP && relayMint;
  const mintFee = relayMintActive ? (tryParseUnits(cctpMintFeeUsd, decimals) ?? 0n) : 0n;
  const forwarding = bridgeType === BRIDGE_CCTP ? relayMint : true;
  const baseFee = tryParseUnits(baseFeeStr, decimals) ?? 0n;

  // Optional destination execution: a hook payload + the gas budget to run it on the recipient.
  const hookDataRaw = hookDataStr.trim() || "0x";
  const hookDataValid = HEX_RE.test(hookDataRaw);
  const hookData: Hex = hookDataValid ? (hookDataRaw as Hex) : "0x";
  const hasHook = hookData !== "0x";
  const callbackGasLimit = gasLimitStr ? BigInt(gasLimitStr) : 0n;
  // Delivery window keys off the bridge speed (CCTP fast vs finalized); the discount rate is derived
  // so the premium decays linearly to zero across that window (no flat/capped tail). Both stay
  // overridable from Advanced.
  const autoWindow = deliveryWindowFor(bridgeType, finality);
  const deliveryWindow = windowStr ? BigInt(windowStr) : autoWindow;
  const autoDiscountRate = linearDiscountRate(deliveryWindow, DEFAULT_MAX_FEE_RATE);
  const discountRate = discountRateStr ? BigInt(discountRateStr) : autoDiscountRate;

  const params: BridgeParams | null =
    amount && recipient
      ? { token, bridgeType, srcChainId: src, dstChainId: dst, amount, recipient, maxFee, mintFee, minFinalityThreshold: finality, deliveryWindow, discountRate, baseFee, callbackGasLimit, hookData }
      : null;

  // Live LayerZero native send-fee quote (OFT only) for the fee preview.
  const { fee: lzFee, loading: lzFeeLoading } = useOftFee(params);

  const outputAmount = params ? outputAmountOf(params) : 0n;
  const amountUsd = amount ? Number(formatUnits(amount, decimals)) : 0;

  // Source balance for the MAX button.
  const srcBal = balances.find((b) => b.chainId === src)?.tokens.find((t) => t.symbol === token)?.amount;

  // Validation.
  const errors: string[] = [];
  if (!isConnected) errors.push("Connect a wallet");
  if (!contractsConfigured) errors.push("Contracts not configured (set env)");
  if (!amount || amount <= 0n) errors.push("Enter an amount");
  else if (amountUsd > maxUsdPerTransfer) errors.push(`Demo cap is ${maxUsdPerTransfer} (real money)`);
  if (!recipient) errors.push("Enter a valid recipient");
  if (bridgeType === BRIDGE_CCTP && amount && maxFee + mintFee >= amount) errors.push("CCTP fees must be < amount");
  if (amount && baseFee >= outputAmount) errors.push("baseFee must be < output amount");
  if (!hookDataValid) errors.push("hookData must be 0x-prefixed hex");
  if (hasHook && callbackGasLimit === 0n) errors.push("Set a callback gas limit for the hook");
  if (params && !isRouteSupported(token, src, dst)) errors.push("Unsupported route");
  const canSubmit = errors.length === 0 && state.phase !== "submitting" && state.phase !== "confirming";

  const busy = ["switching", "approving", "quoting", "signing", "submitting", "confirming", "handoff"].includes(state.phase);

  async function onSubmit() {
    if (!params) return;
    try {
      // `forwarding` follows the Relay Mint toggle: on → the relayer settles (via the executor when
      // mintFee > 0); off → the recipient is left to settle the bridge themselves if nobody fills.
      const res = await submit(params, mode, forwarding);
      onStarted({
        orderId: res.orderId,
        mode,
        bridgeType,
        token,
        srcChainId: src,
        dstChainId: dst,
        amount: amount!.toString(),
        outputAmount: outputAmount.toString(),
        recipient: recipient!,
        srcTxHash: res.srcTxHash,
        forwarding,
        createdAt: Date.now(),
      });
    } catch {
      /* state.error is surfaced below */
    }
  }

  return (
    <div className="card space-y-4">
      {/* Token */}
      <div className="flex gap-2 rounded-lg border border-edge bg-ink p-1">
        {(["USDC", "USDT0"] as TokenSymbol[]).map((t) => (
          <button key={t} onClick={() => onSelectToken(t)} className={`seg ${token === t ? "seg-on" : "seg-off"}`}>
            {t}
          </button>
        ))}
      </div>

      {/* Route */}
      <div className="grid grid-cols-[1fr_auto_1fr] items-end gap-2">
        <div>
          <label className="label">From</label>
          <select className="input" value={src} onChange={(e) => setSrc(Number(e.target.value) as SupportedChainId)}>
            {srcChains.map((id) => (
              <option key={id} value={id}>
                {REGISTRY[id].shortName}
              </option>
            ))}
          </select>
        </div>
        <button
          className="btn-ghost mb-0.5 px-2"
          title="Swap"
          onClick={() => {
            if (isRouteSupported(token, dst, src)) {
              setSrc(dst);
              setDst(src);
            }
          }}
        >
          ⇄
        </button>
        <div>
          <label className="label">To</label>
          <select className="input" value={dst} onChange={(e) => setDst(Number(e.target.value) as SupportedChainId)}>
            {dstChains.map((id) => (
              <option key={id} value={id}>
                {REGISTRY[id].shortName}
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* Amount */}
      <div>
        <div className="flex items-center justify-between">
          <label className="label">Amount</label>
          <button
            className="text-[11px] text-slate-400 hover:text-slate-200"
            onClick={() => srcBal !== undefined && setAmountStr(formatUnits(srcBal, decimals))}
          >
            balance: {fmtAmount(srcBal, decimals, 4)} {token}
          </button>
        </div>
        <div className="flex items-center gap-2">
          <input className="input font-mono" value={amountStr} onChange={(e) => setAmountStr(e.target.value)} inputMode="decimal" />
          <span className="pill border-edge bg-panel">{token}</span>
        </div>
      </div>

      {/* Recipient */}
      <div>
        <div className="flex items-center justify-between">
          <label className="label">Recipient on {REGISTRY[dst].shortName}</label>
          <label className="flex items-center gap-1 text-[11px] text-slate-400">
            <input type="checkbox" checked={sendToSelf} onChange={(e) => setSendToSelf(e.target.checked)} />
            send to myself
          </label>
        </div>
        <input
          className="input font-mono disabled:opacity-50"
          placeholder="0x…"
          value={sendToSelf ? (address ?? "") : recipientStr}
          onChange={(e) => setRecipientStr(e.target.value)}
          disabled={sendToSelf}
        />
      </div>

      {/* CCTP-only: speed + (relayer-managed) settlement */}
      {bridgeType === BRIDGE_CCTP && (
        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="label">Bridge speed</label>
            <div className="flex gap-2 rounded-lg border border-edge bg-ink p-1">
              <button onClick={() => setFinality(FINALITY_FAST)} className={`seg ${finality === FINALITY_FAST ? "seg-on" : "seg-off"}`}>
                Fast
              </button>
              <button onClick={() => setFinality(FINALITY_FINALIZED)} className={`seg ${finality === FINALITY_FINALIZED ? "seg-on" : "seg-off"}`}>
                Finalized
              </button>
            </div>
          </div>
          <div>
            <label className="label flex items-center gap-1">
              <span>Relay Mint</span>
              <InfoTip label="What is Relay Mint?">
                Uses the CCTP executor contract to ensure your funds are delivered on the destination
                whether or not an optimistic filler shows up — for a flat ${Number(cctpMintFeeUsd).toFixed(2)}{" "}
                mint fee. Off: settlement falls back to the optimistic filler, and if none fills you
                settle the bridge yourself.
              </InfoTip>
            </label>
            <label className="flex h-[38px] cursor-pointer items-center gap-2 rounded-lg border border-edge bg-ink px-3 text-sm transition hover:border-accent/60">
              <input type="checkbox" checked={relayMint} onChange={(e) => setRelayMint(e.target.checked)} />
              <span className="text-slate-300">
                {relayMint ? `guaranteed · $${Number(cctpMintFeeUsd).toFixed(2)}` : "optimistic only"}
              </span>
            </label>
          </div>
        </div>
      )}

      {/* Fee preview */}
      <FeePreview
        outputAmount={outputAmount}
        deliveryWindow={deliveryWindow}
        discountRate={discountRate}
        baseFee={baseFee}
        decimals={decimals}
        symbol={token}
        mintFee={mintFee}
        lzFeeWei={bridgeType === BRIDGE_CCTP ? undefined : lzFee}
        lzFeeLoading={lzFeeLoading}
      />

      {/* Advanced */}
      <div>
        <button onClick={() => setAdvanced((v) => !v)} className="text-xs text-slate-400 hover:text-slate-200">
          {advanced ? "▾" : "▸"} Advanced parameters
        </button>
        {advanced && (
          <div className="mt-2 grid grid-cols-2 gap-3">
            {bridgeType === BRIDGE_CCTP && (
              <div>
                <label className="label">maxFee ({token}, fast)</label>
                <input
                  className="input font-mono"
                  placeholder={amount ? formatUnits(suggestMaxFee(amount), decimals) : "auto"}
                  value={maxFeeStr}
                  onChange={(e) => setMaxFeeStr(e.target.value)}
                  disabled={finality === FINALITY_FINALIZED}
                />
              </div>
            )}
            <div>
              <label className="label">delivery window (s)</label>
              <input
                className="input font-mono"
                placeholder={`${autoWindow} (auto: ${finality === FINALITY_FINALIZED ? "finalized" : "fast"})`}
                value={windowStr}
                onChange={(e) => setWindowStr(e.target.value.replace(/\D/g, ""))}
              />
            </div>
            <div>
              <label className="label">discountRate (WAD/s)</label>
              <input
                className="input font-mono"
                placeholder={`${autoDiscountRate} (auto: linear)`}
                value={discountRateStr}
                onChange={(e) => setDiscountRateStr(e.target.value.replace(/\D/g, ""))}
              />
            </div>
            <div>
              <label className="label">baseFee ({token})</label>
              <input className="input font-mono" value={baseFeeStr} onChange={(e) => setBaseFeeStr(e.target.value)} />
            </div>
            <div>
              <label className="label flex items-center gap-1">
                <span>callback gas limit</span>
                <InfoTip label="What is the callback gas limit?">
                  Gas budgeted to run the recipient&apos;s <code>onFastFill</code> hook on delivery.
                  Leave at 0 to just deliver funds.
                </InfoTip>
              </label>
              <input
                className="input font-mono"
                placeholder="0 (deliver funds only)"
                value={gasLimitStr}
                onChange={(e) => setGasLimitStr(e.target.value.replace(/\D/g, ""))}
                inputMode="numeric"
              />
            </div>
            <div className="col-span-2">
              <label className="label flex items-center gap-1">
                <span>hook data (hex)</span>
                <InfoTip label="What is hook data?">
                  Payload passed to the recipient&apos;s <code>onFastFill</code> hook for destination
                  execution. Requires a callback gas limit. <code>0x</code> = deliver funds only.
                </InfoTip>
              </label>
              <input
                className={`input font-mono ${hookDataValid ? "" : "border-bad focus:border-bad"}`}
                placeholder="0x"
                value={hookDataStr}
                onChange={(e) => setHookDataStr(e.target.value)}
              />
            </div>
          </div>
        )}
      </div>

      {/* Approval method */}
      <div>
        <label className="label">Approval method</label>
        <div className="flex gap-2 rounded-lg border border-edge bg-ink p-1">
          {MODES.map((m) => (
            <button key={m.key} onClick={() => setMode(m.key)} className={`seg ${mode === m.key ? "seg-on" : "seg-off"}`} title={m.hint}>
              {m.label}
            </button>
          ))}
        </div>
      </div>

      {/* Submit */}
      <button className="btn-primary w-full" disabled={!canSubmit || busy} onClick={onSubmit}>
        {busy ? state.message ?? "Working…" : `Send ${amountStr || ""} ${token}`}
      </button>

      {state.phase === "error" && state.error && (
        <p className="text-xs text-bad">{state.error}</p>
      )}
      {!canSubmit && errors.length > 0 && state.phase !== "error" && (
        <p className="text-xs text-slate-500">{errors[0]}</p>
      )}
    </div>
  );
}
