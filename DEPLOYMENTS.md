# Deployments

Mainnet deployment of the current fast-fill contracts with canonical-recipient validation,
`callbackGasLimit` capped at 5,000,000, transient reentrancy guards, bounded hook returndata copying,
and deterministic per-OFT adapter isolation.

Deployed to **Base, Optimism, and Arbitrum**. Ethereum L1 is supported by the registry constants but
is intentionally not part of this deployment set.

> Prototype. Deployed for demonstration and testing.

## Addresses

All contracts below are CREATE2-deterministic from deployer/owner
`0xA06Bf163BC51A457D99C6283e78897727c4fDdF2` with `maxFeeRate = 5e15` (0.5%, WAD).

| Contract | Address | Chains |
|---|---:|---|
| `FastFillConfig` | `0xaec766479DB174110958Bc45D141A2C5eF693DF5` | Base · Optimism · Arbitrum |
| `CctpAdapter` (USDC) | `0xcceeB77d7E4FD660fFd8E501a29A58ec6133cF0E` | Base · Optimism · Arbitrum |
| `OftAdapterFactory` | `0x84Bb5d3142024da8d61CBEE0A4c722a1650fbFcb` | Base · Optimism · Arbitrum |
| `OftAdapter` (USDt0, `oftId = 0`) | `0x45165aD55c5FADa4AEeD968469dB6e8e85b943Bf` | Base · Optimism · Arbitrum |
| `OftAdapter` (USDe, `oftId = 1`) | `0x27989367741A6662daeFd5CabeC4f40323461778` | Base · Optimism · Arbitrum |
| `OftAdapter` (sUSDe, `oftId = 2`) | `0x58E5315Ab8B975737d2655e838De12a2c48b497D` | Base · Optimism · Arbitrum |
| `OftAdapter` (ENA, `oftId = 3`) | `0x3291098E6F0e7C206DfB64c54E6D4927e42262b8` | Base · Optimism · Arbitrum |

Notes:

- USDt0 is not configured on Base in `FastFillConfig`; the USDt0 adapter address is deployed there
  for cross-chain determinism but USDt0 operations on Base revert as unsupported.
- A deployed OFT adapter means the token has an adapter address, not that every chain pair is
  peer-enabled by the token issuer's live OFT. A USDe Arbitrum -> Base smoke was simulated and the
  live USDe OFT reverted with `NoPeer(30184)`; Arbitrum/Base USDe peers are currently zero.
- USDtb (`oftId = 4`) is present in the registry but was not deployed by the current factory script.
- The counterpart adapter for each bridge/token is the same address on every deployed chain.

## Deploy Transactions

### FastFillConfig

| Chain | Tx |
|---|---|
| Base | `0x9edfe2845739f8d92a6017879403e51955f99343df1e4d80a22602ecb1064e91` |
| Optimism | `0xb6c0005ce3d655c0e5b4adf6422029a4c0a7f9a1325179aca9011a0411c0c5c8` |
| Arbitrum | `0xc20b96945fc3a9fb8d56305fe7734469f0385eda492bba1e98ee58443136b0f0` |

### CctpAdapter

| Chain | Tx |
|---|---|
| Base | `0x88dfcee68f406728c82a74ebd9e5f63778c4abece1e1858bfad457b883eb2cbc` |
| Optimism | `0x7cc43793a6c827efdd24c4d35ddcee4c6d39a94295d160a2379b52e557a7ea41` |
| Arbitrum | `0xca380656c57a5bd092dc62788cc9b1ea1303a63bd8e2b2634bedfee94357385f` |

### OftAdapterFactory

| Chain | Tx |
|---|---|
| Base | `0x6a9657abb1042980c048594b3ef4529483df82e642d0bb5aae9fc79b1d15349f` |
| Optimism | `0xe36185c0c4b51a5b5f95132f3babbf1467027ed61cffb529edcb2eef5a1f1046` |
| Arbitrum | `0x74385782b0624567c7beb461d5add0dd03cc0bc70db7bf5947977d1a625d22db` |

### OFT Adapters

| Token | Chain | Tx |
|---|---|---|
| USDt0 | Base | `0x5f6cce8a93f14a9120982714991531b59829e774e69f23c59f62a4f3e769de4e` |
| sUSDe | Base | `0x14fb6b02e84976e4d38ad3c9b7b5b6821797ef8d330c777158904d88dc8a8f00` |
| USDe | Base | `0xeafd8dd4eb9c777339b4adf95fdb747d7af423a88e3bca75a57a21f18b1e3023` |
| ENA | Base | `0x80fb9179b5f9b71a14e6a0d62cb1a3b259ac783c141c00e719665dcc9ba2d159` |
| USDt0 | Optimism | `0x8e9abf9e5240565cecc864862c36ee6849228fb1bd8689c0aacc569fed185ba7` |
| sUSDe | Optimism | `0x15d85fea9ec294bd3bc367c443df00198a6563bf88eebead35640b55f3e08f82` |
| USDe | Optimism | `0x3c3330d1076d203189c9395c6467624ce9d9bf2f8f539b406ef775033a7be446` |
| ENA | Optimism | `0xc18b962ac7785c8cb8c5092c21f49e35458a2bf6d48e701f1d977c04c3439e80` |
| USDt0 | Arbitrum | `0x11d5e522d9a81c50bb63e40dad166f8fc00d48f1904469a0138cd40f2e849bdb` |
| sUSDe | Arbitrum | `0xdc64ae20a3620c1f72e27905389d1a34da0a22f48df4ed504638354ee85b29b4` |
| USDe | Arbitrum | `0x5931d02052cbc8c9a54596bb0759da065c8db24347d62782034f7d84b91cecb7` |
| ENA | Arbitrum | `0x2646d23ed79661d894ccb0a2de122ca03f72443a09191e15e8ae488bdf7f7f26` |

## Post-Deploy Checks

Confirmed on Base, Optimism, and Arbitrum:

- `CctpAdapter.config() == 0xaec766479DB174110958Bc45D141A2C5eF693DF5`
- `CctpAdapter.owner() == 0xA06Bf163BC51A457D99C6283e78897727c4fDdF2`
- `CctpAdapter.maxFeeRate() == 5e15`
- `OftAdapterFactory.config() == 0xaec766479DB174110958Bc45D141A2C5eF693DF5`
- `OftAdapterFactory.adapterOwner() == 0xA06Bf163BC51A457D99C6283e78897727c4fDdF2`
- `OftAdapterFactory.maxFeeRate() == 5e15`
- Each deployed OFT adapter reports the expected `oftId`, config, and owner.

## Smoke Tests

- USDC CCTP Base -> Arbitrum:
  - Source tx: `0x4267e424c580a7aa197eb428b5c8f9c7978fb532ccb09d73e0679150ac06943a`
  - Settle tx: `0xb1950a92b326325fc999b2b598aa256491680b30042cef07aa61981bc844518a`
  - Result: 1.000000 USDC burned on Base; 0.999870 USDC delivered on Arbitrum; destination adapter balance returned to zero.
- USDt0 OFT Optimism -> Arbitrum:
  - Source tx: `0xa7f16e39adba31f3f90878d8dafbd39cbee689029d0f9e4cc0cb6c0ffe5582a2`
  - LayerZero GUID: `0x1045d986d707c16043e3201192531e5fe388a872c989e211ab5edeb8dba7c4b4`
  - Status at submission time: in flight, waiting for the USDT0 pathway's 1000 source confirmations.
- USDe OFT Arbitrum -> Base:
  - Not broadcast. Simulation reverted during `quoteSend` with `NoPeer(30184)`.
  - Live USDe OFT checks: Arbitrum `peers(30184) == 0`, Base `peers(30110) == 0`.

## Reproduce

```bash
# 1. Config on each target chain.
forge script script/DeployFastFillConfig.s.sol --rpc-url $RPC --broadcast --private-key $KEY

# 2. CCTP adapter and OFT factory against the deterministic config.
CONFIG=0xaec766479DB174110958Bc45D141A2C5eF693DF5 OWNER=0xA06Bf163BC51A457D99C6283e78897727c4fDdF2 \
  forge script script/DeployCctpAdapter.s.sol --rpc-url $RPC --broadcast --private-key $KEY

CONFIG=0xaec766479DB174110958Bc45D141A2C5eF693DF5 OWNER=0xA06Bf163BC51A457D99C6283e78897727c4fDdF2 \
  forge script script/DeployOftAdapterFactory.s.sol --rpc-url $RPC --broadcast --private-key $KEY

# 3. OFT adapters via the factory.
FACTORY=0x84Bb5d3142024da8d61CBEE0A4c722a1650fbFcb \
  forge script script/DeployOftAdapter.s.sol:DeployOftAdapters --rpc-url $RPC --broadcast --private-key $KEY
```
