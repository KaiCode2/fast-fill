import type { Address, Hex } from "viem";
import { erc20Abi } from "./abis/erc20";
import { erc20PermitAbi } from "./abis/erc20Permit";
import type { SupportedChainId } from "./chains";
import { publicClientFor } from "./viemClients";

/**
 * Read the EIP-712 domain (name + version) an ERC-2612 `permit` signature must be bound to. Prefers
 * EIP-5267 `eip712Domain()`, falling back to `name()` + optional `version()` (default "1"). Shared by
 * the initiate (`selfPermit` + initiate) and self-relay (`selfPermit` + fill) flows.
 */
export async function readTokenDomain(
  token: Address,
  chainId: SupportedChainId,
): Promise<{ name: string; version: string }> {
  const client = publicClientFor(chainId);
  try {
    // EIP-5267 returns (fields, name, version, chainId, verifyingContract, salt, extensions).
    const d = (await client.readContract({
      address: token,
      abi: erc20PermitAbi,
      functionName: "eip712Domain",
    })) as readonly [Hex, string, string, bigint, Address, Hex, readonly bigint[]];
    return { name: d[1], version: d[2] };
  } catch {
    /* token doesn't implement EIP-5267 */
  }
  const name = (await client.readContract({ address: token, abi: erc20Abi, functionName: "name" })) as string;
  let version = "1";
  try {
    version = (await client.readContract({ address: token, abi: erc20PermitAbi, functionName: "version" })) as string;
  } catch {
    /* default to "1" */
  }
  return { name, version };
}
