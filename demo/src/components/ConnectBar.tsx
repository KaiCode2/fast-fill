"use client";

import { ConnectButton } from "@rainbow-me/rainbowkit";
import Link from "next/link";
import type { Address } from "viem";
import { useEnsAvatar, useEnsName } from "@/hooks/useEns";

/** Connected account pill: shows the mainnet ENS name + avatar when available, else the address. */
function AccountPill({ address, fallbackLabel, onClick }: { address: Address; fallbackLabel: string; onClick: () => void }) {
  const ensName = useEnsName(address);
  const ensAvatar = useEnsAvatar(ensName);
  return (
    <button onClick={onClick} className="btn-ghost flex items-center gap-1.5 text-sm" title={address}>
      {ensAvatar && (
        // eslint-disable-next-line @next/next/no-img-element
        <img src={ensAvatar} alt="" width={18} height={18} className="rounded-full" />
      )}
      <span className="font-medium">{ensName ?? fallbackLabel}</span>
    </button>
  );
}

export function ConnectBar() {
  return (
    <header className="sticky top-0 z-20 border-b border-edge bg-ink/70 backdrop-blur">
      <div className="mx-auto flex max-w-5xl items-center justify-between gap-4 px-4 py-3">
        <Link href="/" className="flex items-center gap-2">
          <span className="grid h-8 w-8 place-items-center rounded-lg bg-accent font-bold text-ink">f</span>
          <div className="leading-tight">
            <div className="text-sm font-semibold text-slate-100">fast-fill</div>
            <div className="text-[11px] text-slate-500">optimistic cross-chain transfers</div>
          </div>
        </Link>
        <div className="flex items-center gap-4 sm:gap-6">
          <nav>
            <Link href="/docs" className="text-sm font-medium text-slate-400 transition hover:text-slate-200">
              Docs
            </Link>
          </nav>
          {/* Custom button so we can show a mainnet-resolved ENS name without adding Ethereum to the
              network switcher (the bridge only supports Arbitrum / Optimism / Base). */}
          <ConnectButton.Custom>
            {({ account, chain, openAccountModal, openChainModal, openConnectModal, mounted }) => {
              const ready = mounted;
              const connected = ready && account && chain;
              return (
                <div
                  className="flex items-center gap-2"
                  {...(!ready && { "aria-hidden": true, style: { opacity: 0, pointerEvents: "none", userSelect: "none" } })}
                >
                  {!connected ? (
                    <button onClick={openConnectModal} className="btn-primary text-sm">
                      Connect Wallet
                    </button>
                  ) : chain.unsupported ? (
                    <button onClick={openChainModal} className="btn-ghost text-sm text-bad">
                      Wrong network
                    </button>
                  ) : (
                    <>
                      <button
                        onClick={openChainModal}
                        className="btn-ghost flex items-center px-2 text-sm"
                        title={chain.name ?? "Switch network"}
                      >
                        {chain.hasIcon &&
                          (chain.iconUrl ? (
                            // eslint-disable-next-line @next/next/no-img-element
                            <img
                              src={chain.iconUrl}
                              alt={chain.name ?? "chain"}
                              width={18}
                              height={18}
                              className="rounded-full"
                              style={{ background: chain.iconBackground }}
                            />
                          ) : (
                            <span className="h-[18px] w-[18px] rounded-full" style={{ background: chain.iconBackground }} />
                          ))}
                      </button>
                      <AccountPill address={account.address as Address} fallbackLabel={account.displayName} onClick={openAccountModal} />
                    </>
                  )}
                </div>
              );
            }}
          </ConnectButton.Custom>
        </div>
      </div>
    </header>
  );
}
