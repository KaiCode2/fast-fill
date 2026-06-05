"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import type { SupportedChainId } from "@/lib/chains";
import { searchTokens, type TokenMeta } from "@/lib/tokenRegistry";

/** Token logo with a graceful letters-circle fallback for missing/broken images. */
function TokenIcon({ token, size = 18 }: { token: TokenMeta; size?: number }) {
  const [broken, setBroken] = useState(false);
  if (token.logoURI && !broken) {
    // Plain <img> (not next/image) so we don't need remote-domain config for arbitrary token CDNs.
    // eslint-disable-next-line @next/next/no-img-element
    return (
      <img
        src={token.logoURI}
        alt=""
        width={size}
        height={size}
        loading="lazy"
        className="shrink-0 rounded-full"
        onError={() => setBroken(true)}
      />
    );
  }
  return (
    <span
      className="flex shrink-0 items-center justify-center rounded-full bg-edge text-[8px] font-semibold text-slate-300"
      style={{ width: size, height: size }}
    >
      {token.symbol.slice(0, 3)}
    </span>
  );
}

/**
 * Combobox for picking the Uniswap swap-output token: a dropdown of supported tokens that filters as
 * the user types (symbol or name), plus arrow/enter keyboard nav. Pasting a raw 0x address still works
 * — the parent resolves it on-chain — so unlisted tokens remain reachable.
 */
export function TokenCombobox({
  chainId,
  value,
  onChange,
  resolved,
  excludeAddress,
  invalid,
  placeholder,
}: {
  chainId: SupportedChainId;
  value: string;
  onChange: (next: string) => void;
  resolved: TokenMeta | null;
  excludeAddress?: string; // hide a token from the list (e.g. the stable we're already bridging)
  invalid?: boolean;
  placeholder?: string;
}) {
  const [open, setOpen] = useState(false);
  const [highlight, setHighlight] = useState(0);
  const containerRef = useRef<HTMLDivElement>(null);

  // All matches (alphabetical, scrollable), minus the excluded token.
  const results = useMemo(() => {
    const ex = excludeAddress?.toLowerCase();
    return searchTokens(chainId, value).filter((t) => t.address.toLowerCase() !== ex);
  }, [chainId, value, excludeAddress]);

  // Close on click/tap outside the combobox.
  useEffect(() => {
    function onDocMouseDown(e: MouseEvent) {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) setOpen(false);
    }
    document.addEventListener("mousedown", onDocMouseDown);
    return () => document.removeEventListener("mousedown", onDocMouseDown);
  }, []);

  // Reset the keyboard highlight whenever the candidate list changes.
  useEffect(() => setHighlight(0), [value, chainId]);

  function select(t: TokenMeta) {
    onChange(t.symbol);
    setOpen(false);
  }

  function onKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setOpen(true);
      setHighlight((h) => Math.min(h + 1, results.length - 1));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setHighlight((h) => Math.max(h - 1, 0));
    } else if (e.key === "Enter") {
      if (open && results[highlight]) {
        e.preventDefault();
        select(results[highlight]);
      }
    } else if (e.key === "Escape") {
      setOpen(false);
    }
  }

  return (
    <div className="relative" ref={containerRef}>
      {resolved && (
        <span className="pointer-events-none absolute left-2 top-1/2 -translate-y-1/2">
          <TokenIcon token={resolved} />
        </span>
      )}
      <input
        className={`input font-mono ${resolved ? "pl-9" : ""} ${invalid ? "border-bad focus:border-bad" : ""}`}
        placeholder={placeholder ?? "Search symbol or paste address"}
        value={value}
        onChange={(e) => {
          onChange(e.target.value);
          setOpen(true);
        }}
        onFocus={() => setOpen(true)}
        onKeyDown={onKeyDown}
        spellCheck={false}
        autoCapitalize="off"
        autoComplete="off"
        autoCorrect="off"
        role="combobox"
        aria-expanded={open}
      />
      {open && (
        <ul className="absolute z-20 mt-1 max-h-60 w-full overflow-auto rounded-lg border border-edge bg-panel py-1 shadow-xl">
          {results.length === 0 ? (
            <li className="px-3 py-2 text-[12px] text-slate-500">
              No known tokens match — paste a contract address to use any token.
            </li>
          ) : (
            results.map((t, i) => (
              <li key={t.address}>
                <button
                  type="button"
                  onClick={() => select(t)}
                  onMouseEnter={() => setHighlight(i)}
                  className={`flex w-full items-center gap-2 px-3 py-2 text-left text-sm ${i === highlight ? "bg-ink" : ""}`}
                >
                  <TokenIcon token={t} />
                  <span className="font-medium text-slate-100">{t.symbol}</span>
                  <span className="truncate text-[11px] text-slate-500">{t.name}</span>
                  <span className="ml-auto shrink-0 font-mono text-[10px] text-slate-600">
                    {t.address.slice(0, 6)}…{t.address.slice(-4)}
                  </span>
                </button>
              </li>
            ))
          )}
        </ul>
      )}
    </div>
  );
}
