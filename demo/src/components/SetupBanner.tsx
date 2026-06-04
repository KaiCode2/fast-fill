import { contractsConfigured } from "@/lib/config";
import { DocLink } from "@/components/docs/DocLink";

/** Shown until the deployed adapter addresses are present in env. */
export function SetupBanner() {
  if (contractsConfigured) return null;
  return (
    <div className="mb-6 rounded-xl border border-warn/40 bg-warn/10 p-4 text-sm text-warn">
      <p className="font-semibold">Contracts not configured</p>
      <p className="mt-1 text-warn/80">
        Set <code className="rounded bg-ink px-1">NEXT_PUBLIC_CCTP_ADAPTER</code>,{" "}
        <code className="rounded bg-ink px-1">NEXT_PUBLIC_OFT_ADAPTER</code> and{" "}
        <code className="rounded bg-ink px-1">NEXT_PUBLIC_FASTFILL_CONFIG</code> in{" "}
        <code className="rounded bg-ink px-1">demo/.env.local</code> after deploying. See{" "}
        <code className="rounded bg-ink px-1">.env.example</code>. Balances still work without them.
      </p>
      <p className="mt-2">
        <DocLink href="/docs/deployments">Deployment guide &amp; addresses</DocLink>
      </p>
    </div>
  );
}
