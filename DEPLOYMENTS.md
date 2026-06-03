# Deployments

Mainnet deployment of the fast-fill contracts — the current version with **base-fee pricing**,
**on-chain timing**, **user-signed bridge mode**, and **destination executions**. Deployed to
**Base, Optimism, and Arbitrum**. Ethereum L1 is intentionally not deployed (it has no funded
deployer and is not part of the current demo set).

> ⚠️ Prototype, **unaudited**. Deployed for demonstration only.

## Addresses

All three contracts are **CREATE2-deterministic** — deployed via the canonical deterministic factory
(`0x4e59b44847b379578588920cA78FbF26c0B4956C`) with a fixed salt, so each has the **same address on
every chain**. The adapters take the `FastFillConfig` address as their only chain-specific argument;
because the config address is identical everywhere, the adapters are deterministic too, and the
counterpart on every chain is simply `address(this)`. There is no per-chain wiring.

| Contract | Address | Chains |
|---|---|---|
| `FastFillConfig` | [`0xE66dF457F18B4a8a30a251750BFfA21D843c749D`](#fastfillconfig) | Base · Optimism · Arbitrum |
| `CctpAdapter` (USDC) | [`0xDde8F1F1f68fefc016Deb99B01A31AD474f1f626`](#cctpadapter) | Base · Optimism · Arbitrum |
| `OftAdapter` (USD₮0) | [`0x7E9F7540218E4F4e8F58D57ca54E54438bC39cBC`](#oftadapter) | Optimism · Arbitrum |

- **CctpAdapter** is deployed on all three chains (USDC is everywhere).
- **OftAdapter** is deployed only on **Optimism + Arbitrum** — USD₮0 has no deployment on Base, so the
  OFT path there would be inert.
- **Owner:** `0xA06Bf163BC51A457D99C6283e78897727c4fDdF2` (also the deployer).
- **Adapter constructor args:** `config = 0xE66dF457F18B4a8a30a251750BFfA21D843c749D`,
  `owner = 0xA06Bf163BC51A457D99C6283e78897727c4fDdF2`, `maxFeeRate = 5e15` (0.5% cap, WAD).
- **CREATE2 salts:** `keccak256("fast-fill.config.v1")`, `…cctp.v1`, `…oft.v1`.

### FastFillConfig
`0xE66dF457F18B4a8a30a251750BFfA21D843c749D`

| Chain | Explorer | Deploy tx |
|---|---|---|
| Base | [address](https://basescan.org/address/0xE66dF457F18B4a8a30a251750BFfA21D843c749D) | [`0x460879…d18bbe`](https://basescan.org/tx/0x460879d1b7522a89c142fc4e8822343265c236bd41bead061bf62e1b06d18bbe) |
| Optimism | [address](https://optimistic.etherscan.io/address/0xE66dF457F18B4a8a30a251750BFfA21D843c749D) | [`0x6fd57a…f05808`](https://optimistic.etherscan.io/tx/0x6fd57a32244c501f6ee697a1954588a5a4d37a49f873cedd62e39683c3f05808) |
| Arbitrum | [address](https://arbiscan.io/address/0xE66dF457F18B4a8a30a251750BFfA21D843c749D) | [`0x819e6e…ad8b24`](https://arbiscan.io/tx/0x819e6e2d936fd841611a2b695bdd332688edddc453328fe0c0c754f20cad8b24) |

### CctpAdapter
`0xDde8F1F1f68fefc016Deb99B01A31AD474f1f626`

| Chain | Explorer | Deploy tx |
|---|---|---|
| Base | [address](https://basescan.org/address/0xDde8F1F1f68fefc016Deb99B01A31AD474f1f626) | [`0x7afe53…f4437e`](https://basescan.org/tx/0x7afe5342dedd899d0abffa8c550305cdc076ae0c54f8b9d2f083afae55f4437e) |
| Optimism | [address](https://optimistic.etherscan.io/address/0xDde8F1F1f68fefc016Deb99B01A31AD474f1f626) | [`0xcf33d8…29af65e`](https://optimistic.etherscan.io/tx/0xcf33d85125351ec163e6e26ee292e37967b59e7770656e407cea162de29af65e) |
| Arbitrum | [address](https://arbiscan.io/address/0xDde8F1F1f68fefc016Deb99B01A31AD474f1f626) | [`0x1fa9a0…0c4100`](https://arbiscan.io/tx/0x1fa9a0551ecf795e142082abbd3e156d50bd433178fe4a5d70ec731a270c4100) |

### OftAdapter
`0x7E9F7540218E4F4e8F58D57ca54E54438bC39cBC`

| Chain | Explorer | Deploy tx |
|---|---|---|
| Optimism | [address](https://optimistic.etherscan.io/address/0x7E9F7540218E4F4e8F58D57ca54E54438bC39cBC) | [`0xc922ed…ad06ba`](https://optimistic.etherscan.io/tx/0xc922ed330f1520f6450388193ff75388bdc19e6cadd99e29e94d129a5bad06ba) |
| Arbitrum | [address](https://arbiscan.io/address/0x7E9F7540218E4F4e8F58D57ca54E54438bC39cBC) | [`0xd50d04…a352cdf`](https://arbiscan.io/tx/0xd50d048dcd5fa428d745fc5cd09fa2c484511b99446a4ba674a39aa91a352cdf) |

## Post-deploy verification (on-chain)

Confirmed after deployment:

- `CctpAdapter.config() == 0xE66dF457F18B4a8a30a251750BFfA21D843c749D`, `owner() ==
  0xA06Bf163BC51A457D99C6283e78897727c4fDdF2`, `maxFeeRate() == 5e15` on each chain; both adapters
  resolve the same config on Base/OP/ARB.
- `FastFillConfig.chainConfig(8453)` returns Base USDC `0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913`,
  CCTP domain `6`, LZ eid `30184`, TokenMessenger `0x28b5a0e9C621a5BadaA536219b3a228C8168cf5d`.
- Deployed runtime sizes match the compiled bytecode (CctpAdapter 17,245 B; OftAdapter 17,397 B).

## Reproduce

```bash
# 1. Config (same address on every chain)
forge script script/DeployFastFillConfig.s.sol --rpc-url $RPC --broadcast --private-key $KEY

# 2. Adapters against it (CONFIG identical on every chain ⇒ adapters deterministic)
CONFIG=0xE66dF457F18B4a8a30a251750BFfA21D843c749D OWNER=0xA06Bf163BC51A457D99C6283e78897727c4fDdF2 \
  forge script script/DeployCctpAdapter.s.sol --rpc-url $RPC --broadcast --private-key $KEY
CONFIG=0xE66dF457F18B4a8a30a251750BFfA21D843c749D OWNER=0xA06Bf163BC51A457D99C6283e78897727c4fDdF2 \
  forge script script/DeployOftAdapter.s.sol  --rpc-url $RPC --broadcast --private-key $KEY   # OP/ARB only
```
