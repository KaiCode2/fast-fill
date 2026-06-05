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
  parseMintFee,
  suggestMaxFee,
  type BridgeParams,
} from "@/lib/bridge";
import { REGISTRY, SUPPORTED_CHAIN_IDS, tokenInfo, type SupportedChainId, type TokenSymbol } from "@/lib/chains";
import { cctpMintFeeUsd, contractsConfigured, maxUsdPerTransfer } from "@/lib/config";
import { fmtAmount } from "@/lib/format";
import { DEFAULT_MAX_FEE_RATE, linearDiscountRate } from "@/lib/pricing";
import { bridgeTypeForToken, chainsForToken, destinationsFor, getToken, isRouteSupported } from "@/lib/tokens";
import { BRIDGE_CCTP } from "@/lib/order";
import {
  encodeAaveHookData,
  encodeUniswapHookData,
  hookAddress,
  HOOK_CALLBACK_GAS,
  HOOK_LABEL,
  type HookKind,
} from "@/lib/hooks";
import { resolveToken, resolveTokenAsync, type TokenMeta } from "@/lib/tokenRegistry";
import { DEFAULT_SLIPPAGE_BPS } from "@/lib/uniswap";
import { useBalances } from "@/hooks/useBalances";
import { useInitiate } from "@/hooks/useInitiate";
import { useOftFee } from "@/hooks/useOftFee";
import { useSwapQuote } from "@/hooks/useSwapQuote";
import type { TransferRecord } from "@/hooks/useTransfers";
import { FeePreview } from "./FeePreview";
import { InfoTip } from "./InfoTip";
import { TokenCombobox } from "./TokenCombobox";

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

  // Destination action (optional hook). "none" = plain delivery; presets auto-fill recipient/hookData/gas.
  const [hookKind, setHookKind] = useState<HookKind>("none");
  const [tokenOutQuery, setTokenOutQuery] = useState(""); // swap target: symbol or 0x address
  const [tokenOut, setTokenOut] = useState<TokenMeta | null>(null); // resolved swap target
  const [tokenOutResolving, setTokenOutResolving] = useState(false);
  const [slippageBps, setSlippageBps] = useState<number>(DEFAULT_SLIPPAGE_BPS);

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

  // Resolve the swap target (symbol or address) against the destination chain. Symbols/known addresses
  // resolve synchronously; an unlisted address is read on-chain (debounced). Cleared when not swapping.
  useEffect(() => {
    if (hookKind !== "uniswap") {
      setTokenOut(null);
      setTokenOutResolving(false);
      return;
    }
    const q = tokenOutQuery.trim();
    const sync = q ? resolveToken(dst, q) : undefined;
    if (sync || !q || !isAddress(q)) {
      setTokenOut(sync ?? null);
      setTokenOutResolving(false);
      return;
    }
    let cancelled = false;
    setTokenOut(null);
    setTokenOutResolving(true);
    const handle = setTimeout(() => {
      resolveTokenAsync(dst, q)
        .then((t) => !cancelled && setTokenOut(t ?? null))
        .finally(() => !cancelled && setTokenOutResolving(false));
    }, 300);
    return () => {
      cancelled = true;
      clearTimeout(handle);
    };
  }, [tokenOutQuery, dst, hookKind]);

  const srcChains = chainsForToken(token);
  const dstChains = destinationsFor(token, src);

  const amount = tryParseUnits(amountStr, decimals);
  // The user's address: who ultimately receives the funds. With no hook it's the on-chain recipient;
  // with a hook it's encoded INTO hookData (the on-chain recipient becomes the hook contract).
  const beneficiary: Address | undefined = sendToSelf
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
  // When Relay Mint is on, the configured fee MUST parse to a positive amount. A malformed, empty, or
  // zero `NEXT_PUBLIC_CCTP_MINT_FEE_USD` would otherwise silently downgrade the order to the direct
  // settle path (mintFee = 0) while the UI still promised executor-guaranteed delivery — so catch it
  // and block submission rather than mislead. (Off → mintFee 0 is the intended optimistic-only path.)
  const configuredMintFee = relayMintActive ? parseMintFee(cctpMintFeeUsd, decimals) : 0n;
  const mintFeeMisconfigured = configuredMintFee === null;
  const mintFee = configuredMintFee ?? 0n;
  const forwarding = bridgeType === BRIDGE_CCTP ? relayMint : true;
  const baseFee = tryParseUnits(baseFeeStr, decimals) ?? 0n;

  // The amount that actually arrives at the recipient/hook (CCTP nets transport+mint fees; OFT is 1:1).
  // Computed from primitives so it can feed the swap quote BEFORE the order/params are built.
  const outputAmount = amount ? outputAmountOf({ bridgeType, amount, maxFee, mintFee }) : 0n;

  // Manual destination-execution payload (Advanced) — only used when no preset action is selected.
  const manualHookDataRaw = hookDataStr.trim() || "0x";
  const manualHookDataValid = HEX_RE.test(manualHookDataRaw);
  const manualHookData: Hex = manualHookDataValid ? (manualHookDataRaw as Hex) : "0x";
  const manualGas = gasLimitStr ? BigInt(gasLimitStr) : 0n;

  // Live "best pool" Uniswap quote for the swap hook: on the DESTINATION chain, swapping the arriving
  // stable (outputAmount) into the chosen token. Drives the amountOutMinimum baked into the order.
  const swapTokenIn = tokenInfo(dst, token)?.address;
  const { quote, loading: quoteLoading, error: quoteError, noPool } = useSwapQuote({
    dstChainId: dst,
    tokenIn: swapTokenIn,
    tokenOut: hookKind === "uniswap" ? tokenOut?.address : undefined,
    amountIn: outputAmount,
    slippageBps,
    enabled: hookKind === "uniswap",
  });

  // The on-chain triple. With a hook, recipient = the hook contract and the beneficiary + params live in
  // hookData; with no hook, recipient = beneficiary and the Advanced manual payload (if any) is used.
  const swapReady = hookKind === "uniswap" && !!tokenOut && !!quote;
  const effectiveRecipient: Address | undefined =
    hookKind === "none" ? beneficiary : hookAddress(hookKind, dst);
  const effectiveHookData: Hex =
    hookKind === "none"
      ? manualHookData
      : !beneficiary
        ? "0x"
        : hookKind === "aave"
          ? encodeAaveHookData(beneficiary)
          : swapReady
            ? encodeUniswapHookData({
                user: beneficiary,
                tokenOut: tokenOut!.address,
                poolFee: quote!.poolFee,
                amountOutMinimum: quote!.amountOutMin,
              })
            : "0x";
  const effectiveGas: bigint = hookKind === "none" ? manualGas : HOOK_CALLBACK_GAS[hookKind];
  const hasHook = hookKind !== "none" || manualHookData !== "0x";
  // Delivery window keys off the bridge speed (CCTP fast vs finalized); the discount rate is derived
  // so the premium decays linearly to zero across that window (no flat/capped tail). Both stay
  // overridable from Advanced.
  const autoWindow = deliveryWindowFor(bridgeType, finality);
  const deliveryWindow = windowStr ? BigInt(windowStr) : autoWindow;
  const autoDiscountRate = linearDiscountRate(deliveryWindow, DEFAULT_MAX_FEE_RATE);
  const discountRate = discountRateStr ? BigInt(discountRateStr) : autoDiscountRate;

  const params: BridgeParams | null =
    amount && effectiveRecipient
      ? { token, bridgeType, srcChainId: src, dstChainId: dst, amount, recipient: effectiveRecipient, maxFee, mintFee, minFinalityThreshold: finality, deliveryWindow, discountRate, baseFee, callbackGasLimit: effectiveGas, hookData: effectiveHookData }
      : null;

  // Live LayerZero native send-fee quote (OFT only) for the fee preview.
  const { fee: lzFee, loading: lzFeeLoading } = useOftFee(params);

  const amountUsd = amount ? Number(formatUnits(amount, decimals)) : 0;

  // Source balance for the MAX button.
  const srcBal = balances.find((b) => b.chainId === src)?.tokens.find((t) => t.symbol === token)?.amount;

  // Validation.
  const errors: string[] = [];
  if (!isConnected) errors.push("Connect a wallet");
  if (!contractsConfigured) errors.push("Contracts not configured (set env)");
  if (!amount || amount <= 0n) errors.push("Enter an amount");
  else if (amountUsd > maxUsdPerTransfer) errors.push(`Demo cap is ${maxUsdPerTransfer} (real money)`);
  if (!beneficiary) errors.push("Enter a valid recipient");
  if (bridgeType === BRIDGE_CCTP && amount && maxFee + mintFee >= amount) errors.push("CCTP fees must be < amount");
  if (mintFeeMisconfigured)
    errors.push(`Relay Mint is on but the mint fee "${cctpMintFeeUsd}" is invalid — set NEXT_PUBLIC_CCTP_MINT_FEE_USD to a positive amount, or turn off Relay Mint`);
  if (amount && baseFee >= outputAmount) errors.push("baseFee must be < output amount");
  if (hookKind === "none" && !manualHookDataValid) errors.push("hookData must be 0x-prefixed hex");
  if (hasHook && effectiveGas === 0n) errors.push("Set a callback gas limit for the hook");
  if (hookKind !== "none" && !hookAddress(hookKind, dst))
    errors.push(`${HOOK_LABEL[hookKind]} isn't available on ${REGISTRY[dst].shortName}`);
  if (hookKind === "uniswap") {
    if (!tokenOutQuery.trim()) errors.push("Enter a token to swap into");
    else if (tokenOutResolving) errors.push("Resolving the swap token…");
    else if (!tokenOut) errors.push("Unknown token — enter a symbol or paste a contract address");
    else if (swapTokenIn && tokenOut.address.toLowerCase() === swapTokenIn.toLowerCase())
      errors.push(`Choose a token other than ${token} to swap into`);
    else if (quoteLoading) errors.push("Fetching the best swap quote…");
    else if (noPool) errors.push(`No Uniswap pool for ${token}→${tokenOut.symbol} on ${REGISTRY[dst].shortName}`);
    else if (quoteError) errors.push("Swap quote failed — try again");
    else if (!quote) errors.push("Waiting for a swap quote");
  }
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
        recipient: beneficiary!,
        srcTxHash: res.srcTxHash,
        forwarding,
        hookKind,
        swapTokenSymbol: hookKind === "uniswap" ? tokenOut?.symbol : undefined,
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
          <label className="label">{hookKind === "none" ? "Recipient" : "Beneficiary"} on {REGISTRY[dst].shortName}</label>
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

      {/* Destination action (optional hook) */}
      <div>
        <label className="label flex items-center gap-1">
          <span>Destination action</span>
          <InfoTip label="What is a destination action?">
            Run a contract the instant funds arrive on {REGISTRY[dst].shortName}: deposit the bridged{" "}
            {token} into Aave, or swap it into another token via Uniswap. If the action reverts, the
            original {token} is delivered to the beneficiary instead — funds are never stuck.
          </InfoTip>
        </label>
        <div className="flex gap-2 rounded-lg border border-edge bg-ink p-1">
          {(["none", "aave", "uniswap"] as HookKind[]).map((k) => (
            <button key={k} onClick={() => setHookKind(k)} className={`seg ${hookKind === k ? "seg-on" : "seg-off"}`}>
              {HOOK_LABEL[k]}
            </button>
          ))}
        </div>

        {hookKind === "aave" && (
          <p className="mt-2 rounded-lg border border-edge bg-ink/60 p-3 text-[12px] text-slate-400">
            Supplies the delivered {token} into Aave V3 on {REGISTRY[dst].shortName}; aTokens are minted to{" "}
            {sendToSelf ? "you" : "the beneficiary"}.
          </p>
        )}

        {hookKind === "uniswap" && (
          <div className="mt-2 space-y-2">
            <div>
              <label className="label">Swap into (symbol or address on {REGISTRY[dst].shortName})</label>
              <TokenCombobox
                chainId={dst}
                value={tokenOutQuery}
                onChange={setTokenOutQuery}
                resolved={tokenOut}
                excludeAddress={swapTokenIn}
                invalid={!!tokenOutQuery.trim() && !tokenOut && !tokenOutResolving}
                placeholder="WETH, DAI, or 0x…"
              />
              {tokenOut ? (
                <p className="mt-1 text-[11px] text-slate-500">
                  {tokenOut.symbol} · {tokenOut.decimals}dp ·{" "}
                  <span className="font-mono">
                    {tokenOut.address.slice(0, 6)}…{tokenOut.address.slice(-4)}
                  </span>
                </p>
              ) : tokenOutResolving ? (
                <p className="mt-1 text-[11px] text-slate-500">Resolving…</p>
              ) : tokenOutQuery.trim() ? (
                <p className="mt-1 text-[11px] text-bad">Unknown token — enter a known symbol or paste a contract address.</p>
              ) : null}
            </div>

            {/* Live best-pool quote */}
            <div className="rounded-lg border border-edge bg-ink/60 p-2 text-[12px]">
              {!tokenOut ? (
                <span className="text-slate-500">Enter a token to fetch the best quote.</span>
              ) : quoteLoading ? (
                <span className="text-slate-500">Finding the best pool…</span>
              ) : noPool ? (
                <span className="text-warn">
                  No Uniswap V3 pool for {token} → {tokenOut.symbol} on {REGISTRY[dst].shortName}.
                </span>
              ) : quoteError ? (
                <span className="text-bad">Quote failed — retry.</span>
              ) : quote ? (
                <span className="text-slate-300">
                  ≈ {fmtAmount(quote.amountOut, tokenOut.decimals, 6)} {tokenOut.symbol}{" "}
                  <span className="text-slate-500">
                    · best tier {(quote.poolFee / 10_000).toFixed(2)}% · min{" "}
                    {fmtAmount(quote.amountOutMin, tokenOut.decimals, 6)} @ {(slippageBps / 100).toFixed(2)}% slippage
                  </span>
                </span>
              ) : (
                <span className="text-slate-500">Waiting for a quote…</span>
              )}
            </div>
          </div>
        )}
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
                {!relayMint
                  ? "optimistic only"
                  : mintFeeMisconfigured
                    ? "fee misconfigured"
                    : `guaranteed · $${Number(cctpMintFeeUsd).toFixed(2)}`}
              </span>
            </label>
            {mintFeeMisconfigured && (
              <p className="mt-1 text-[11px] text-bad">
                Invalid mint fee config (<code>NEXT_PUBLIC_CCTP_MINT_FEE_USD</code> = &quot;{cctpMintFeeUsd}&quot;).
              </p>
            )}
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
            {hookKind === "uniswap" && (
              <div>
                <label className="label flex items-center gap-1">
                  <span>swap slippage</span>
                  <InfoTip label="What is swap slippage?">
                    The minimum output is locked into the order when you sign and the swap runs on
                    delivery. A little room absorbs price drift before the relayer fills; too tight and
                    the swap reverts (you then receive the original {token}).
                  </InfoTip>
                </label>
                <select className="input" value={slippageBps} onChange={(e) => setSlippageBps(Number(e.target.value))}>
                  <option value={10}>0.1%</option>
                  <option value={50}>0.5%</option>
                  <option value={100}>1.0%</option>
                  <option value={300}>3.0%</option>
                </select>
              </div>
            )}
            <div>
              <label className="label flex items-center gap-1">
                <span>callback gas limit</span>
                <InfoTip label="What is the callback gas limit?">
                  Gas budgeted to run the recipient&apos;s <code>onFastFill</code> hook on delivery.
                  Leave at 0 to just deliver funds.
                </InfoTip>
              </label>
              <input
                className="input font-mono disabled:opacity-50"
                placeholder="0 (deliver funds only)"
                value={hookKind === "none" ? gasLimitStr : effectiveGas.toString()}
                onChange={(e) => setGasLimitStr(e.target.value.replace(/\D/g, ""))}
                inputMode="numeric"
                disabled={hookKind !== "none"}
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
                className={`input font-mono disabled:opacity-50 ${hookKind === "none" && !manualHookDataValid ? "border-bad focus:border-bad" : ""}`}
                placeholder="0x"
                value={hookKind === "none" ? hookDataStr : effectiveHookData}
                onChange={(e) => setHookDataStr(e.target.value)}
                disabled={hookKind !== "none"}
              />
              {hookKind !== "none" && (
                <p className="mt-1 text-[11px] text-slate-500">
                  Auto-encoded by the “{HOOK_LABEL[hookKind]}” action above (recipient ={" "}
                  <span className="font-mono">
                    {effectiveRecipient ? `${effectiveRecipient.slice(0, 6)}…${effectiveRecipient.slice(-4)}` : "—"}
                  </span>
                  ).
                </p>
              )}
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
        {busy
          ? (state.message ?? "Working…")
          : hookKind === "aave"
            ? `Send ${amountStr || ""} ${token} → Aave`
            : hookKind === "uniswap"
              ? `Swap ${amountStr || ""} ${token} → ${tokenOut?.symbol ?? "token"}`
              : `Send ${amountStr || ""} ${token}`}
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
