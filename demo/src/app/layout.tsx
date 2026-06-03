import type { Metadata } from "next";
import type { ReactNode } from "react";
import "./globals.css";
import { Providers } from "./providers";

export const metadata: Metadata = {
  title: "fast-fill — optimistic cross-chain transfers",
  description:
    "Demo: bridge USDC (CCTP) and USD₮0 (LayerZero OFT) across Arbitrum, Optimism and Base with instant optimistic fills, gasless intents, and live settlement.",
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body className="min-h-screen bg-ink text-slate-200 antialiased">
        <Providers>{children}</Providers>
      </body>
    </html>
  );
}
