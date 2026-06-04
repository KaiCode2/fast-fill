"use client";

import type { ReactNode } from "react";

/**
 * A small "?" affordance that reveals an explanatory tooltip on hover or keyboard focus. Purely
 * presentational — pass the explanation as children. Sits above the trigger (z above the sticky
 * header) and is focusable so it's reachable without a pointer.
 */
export function InfoTip({ children, label = "More info" }: { children: ReactNode; label?: string }) {
  return (
    <span className="group relative inline-flex align-middle">
      <span
        tabIndex={0}
        role="img"
        aria-label={label}
        className="grid h-3.5 w-3.5 cursor-help place-items-center rounded-full border border-edge text-[9px] font-bold leading-none text-slate-400 transition hover:border-accent hover:text-slate-200 focus:border-accent focus:outline-none"
      >
        ?
      </span>
      <span
        role="tooltip"
        className="pointer-events-none absolute bottom-full left-1/2 z-30 mb-1.5 w-56 -translate-x-1/2 rounded-md border border-edge bg-panel px-2.5 py-1.5 text-[11px] font-normal normal-case leading-relaxed tracking-normal text-slate-300 opacity-0 shadow-lg shadow-black/40 transition-opacity duration-150 group-hover:opacity-100 group-focus-within:opacity-100"
      >
        {children}
      </span>
    </span>
  );
}
