"use client";

import { useEffect, useId, useRef, useState } from "react";

/**
 * Renders a Mermaid diagram to SVG on the client. `mermaid` is imported dynamically inside the
 * effect so its browser-only internals never execute during SSR. Falls back to showing the raw
 * diagram source if rendering fails.
 */
export function Mermaid({ code }: { code: string }) {
  // mermaid.render needs a DOM-id-safe string; useId() contains colons.
  const id = `mmd-${useId().replace(/[^a-zA-Z0-9]/g, "")}`;
  const ref = useRef<HTMLDivElement>(null);
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const mermaid = (await import("mermaid")).default;
        mermaid.initialize({ startOnLoad: false, theme: "dark", securityLevel: "strict" });
        const { svg } = await mermaid.render(id, code);
        if (!cancelled && ref.current) {
          ref.current.innerHTML = svg;
          setFailed(false);
        }
      } catch {
        if (!cancelled) setFailed(true);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [code, id]);

  if (failed) {
    return (
      <pre className="overflow-x-auto rounded-lg border border-edge bg-ink p-4 text-xs text-slate-400">
        {code}
      </pre>
    );
  }

  return <div ref={ref} className="my-4 flex justify-center overflow-x-auto" role="img" aria-label="diagram" />;
}
