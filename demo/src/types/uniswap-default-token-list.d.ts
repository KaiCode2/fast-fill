/** Ambient types for the untyped `@uniswap/default-token-list` JSON package. */
declare module "@uniswap/default-token-list" {
  interface UniswapTokenListEntry {
    chainId: number;
    address: string;
    name: string;
    symbol: string;
    decimals: number;
    logoURI?: string;
    tags?: string[];
    extensions?: Record<string, unknown>;
  }
  interface UniswapTokenList {
    name: string;
    timestamp: string;
    version: { major: number; minor: number; patch: number };
    tokens: UniswapTokenListEntry[];
  }
  const list: UniswapTokenList;
  export default list;
}
