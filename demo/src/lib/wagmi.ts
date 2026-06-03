import { getDefaultConfig } from "@rainbow-me/rainbowkit";
import { http } from "wagmi";
import { arbitrum, base, optimism } from "wagmi/chains";
import { rpcUrl, wcProjectId } from "./config";

/**
 * wagmi + RainbowKit config over the three supported mainnets, with explicit per-chain transports
 * from env (public RPCs rate-limit the balance multicalls hard). `ssr: true` for the App Router.
 */
export const wagmiConfig = getDefaultConfig({
  appName: "fast-fill demo",
  projectId: wcProjectId,
  chains: [arbitrum, optimism, base],
  transports: {
    [arbitrum.id]: http(rpcUrl(42161)),
    [optimism.id]: http(rpcUrl(10)),
    [base.id]: http(rpcUrl(8453)),
  },
  ssr: true,
});
