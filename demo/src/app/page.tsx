import { ConnectBar } from "@/components/ConnectBar";
import { DemoApp } from "@/components/DemoApp";
import { SetupBanner } from "@/components/SetupBanner";
import { DocLink } from "@/components/docs/DocLink";

export default function Page() {
  return (
    <>
      <ConnectBar />
      <main className="mx-auto max-w-5xl px-4 py-8">
        <section className="mb-8">
          <h1 className="text-2xl font-bold text-slate-50">Optimistic cross-chain transfers</h1>
          <p className="mt-2 max-w-2xl text-sm text-slate-400">
            Bridge USDC (Circle CCTP) and USD₮0 (LayerZero OFT) across Arbitrum, Optimism and Base. A
            relayer optimistically <span className="text-slate-200">fills your transfer on the destination in seconds</span>{" "}
            for a small, user-priced premium — then is reimbursed when the bridge settles. Sign gasless
            intents (EIP-2612 / Permit2) or submit yourself.
          </p>
          <p className="mt-3 text-sm">
            <DocLink href="/docs">Read the docs</DocLink>
          </p>
        </section>
        <SetupBanner />
        <DemoApp />
      </main>
    </>
  );
}
