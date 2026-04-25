# INFI Blockchain

INFI is a high-throughput EVM-compatible Layer 1 blockchain prototype.

The native gas token is **Invertx**.

This repository starts with a local devnet skeleton:

- deterministic genesis
- native Invertx balances
- blocks and signed-transaction placeholders
- mempool
- storage abstraction
- consensus placeholder
- EVM/RPC placeholders
- a runnable node binary once Rust is installed
- INFI Scan explorer roadmap
- WalletConnect/AppKit integration roadmap
- one-click wallet import roadmap
- free INFI domains roadmap
- cross-chain interoperability roadmap
- tokenized asset decentralization roadmap
- decentralized VPN function roadmap
- safety, transparency, and new-user experience requirements
- future SbSe Protocol governance engine roadmap

## Current Stage

This is an early public-testnet prototype. It is not mainnet software and does not yet include real EVM execution, validator networking, cryptographic signatures, staking, slashing, or production-grade database storage.

INFI must become fully decentralized before any mainnet launch. That means independent validators, permissionless node operation, staking, slashing, open-source clients, public RPCs, public explorer infrastructure, governance, and reproducible builds.

## Install Rust

Install Rust from:

```text
https://rustup.rs
```

Then run:

```bash
cargo check
cargo run -p infi-node
```

## Target Architecture

```text
crates/
  primitives/    shared chain types
  storage/       state and block storage
  mempool/       transaction pool
  consensus/     block proposal/finality logic
  evm/           EVM execution adapter
  rpc/           Ethereum JSON-RPC adapter
  node/          devnet node binary
```

Documentation:

- [Documentation home](docs/index.md)
- [Full concept overview](docs/full-concept-overview.md)
- [Developer quickstart](docs/developer-quickstart.md)
- [Dapp developer guide](docs/dapp-developer-guide.md)
- [Smart contract guide](docs/smart-contracts.md)
- [Node operator guide](docs/node-operator-guide.md)
- [API examples](docs/api-examples.md)
- [Step-by-step build guide](docs/step-by-step.md)
- [INFI Scan explorer plan](docs/infi-scan.md)
- [Indexer guide](docs/indexer-guide.md)
- [WalletConnect plan](docs/wallet-connect.md)
- [Wallet import plan](docs/wallet-import.md)
- [Safety and transparency requirements](docs/safety-transparency.md)
- [Decentralization requirements](docs/decentralization.md)
- [Cross-chain support](docs/interoperability.md)
- [Protocol specification](docs/protocol-spec.md)
- [Token economics](docs/token-economics.md)
- [INFI Domains](docs/infi-domains.md)
- [Tokenized assets](docs/tokenized-assets.md)
- [Decentralized VPN function](docs/decentralized-vpn.md)
- [Bridge and cross-chain security](docs/bridge-and-cross-chain-security.md)
- [Genesis and configuration](docs/genesis-and-config.md)
- [Website launch guide](docs/website-launch.md)
- [Public testnet RPC deployment](docs/public-testnet-rpc-deployment.md)
- [Persistent storage](docs/persistent-storage.md)
- [Testnet launch guide](docs/testnet-launch.md)
- [Testnet faucet](docs/testnet-faucet.md)
- [Testing strategy](docs/testing.md)
- [Release process](docs/release-process.md)
- [Observability and status](docs/observability.md)
- [Mainnet launch guide](docs/mainnet-launch.md)
- [Governance guide](docs/governance.md)
- [SbSe Protocol governance engine](docs/sbse-protocol.md)
- [Contributing](docs/contributing.md)
- [Glossary](docs/glossary.md)
- [FAQ](docs/faq.md)

## Native Token

```text
Name: Invertx
Symbol: TBD
Decimals: 18
Role: native gas token
```

Native Invertx is tracked directly in account state, like ETH on Ethereum. A wrapped ERC-20 version can be added later for DeFi integrations.
