# INFI Blockchain Blueprint

## Goal

Build INFI as a high-throughput, low-fee, EVM-compatible Layer 1 blockchain.

The target is:

- EVM smart contracts and Ethereum tooling support
- Fast finality
- Parallel transaction execution where safe
- Low validator hardware friction at first, with room to scale
- Native gas token: `Invertx`
- RPC compatibility with wallets, explorers, bridges, and dapps
- INFI Scan explorer for tracking transactions, blocks, addresses, validators, contracts, and cross-chain activity
- WalletConnect QR/deep-link support across mobile, tablet, desktop, and laptop
- one-click wallet import and network switching where wallets support it
- free INFI Domains for wallet, dapp, asset, and service names
- Cross-chain support for EVM and non-EVM ecosystems through adapters
- decentralized tokenized asset functions
- decentralized VPN function through separate provider nodes and on-chain coordination
- safety, transparency, and new-user experience as product requirements
- Future SbSe Protocol governance logic engine node

## Important Reality Check

Solana and the EVM use very different execution models.

Solana gets much of its speed from explicit account access lists and parallel execution. Ethereum-style EVM transactions do not naturally declare all touched state up front, so full Solana-style parallelism is harder.

The practical INFI approach is:

- Keep EVM compatibility for developers
- Use an existing battle-tested EVM engine
- Add parallel execution where dependency analysis is safe
- Optimize networking, mempool, block production, storage, and finality

## Recommended Architecture

### 1. Execution Layer

Use an existing EVM implementation instead of writing a new EVM from scratch.

Good options:

- `revm` if building in Rust
- `go-ethereum` EVM if building in Go
- Polygon Edge / Erigon-style components if prioritizing existing infra

Recommendation: Rust with `revm`.

Why:

- Strong performance
- Modern modular EVM
- Good fit for parallel execution experiments
- Easier to build custom chain components around it

### 2. Consensus

Start with Proof of Stake using BFT-style finality.

Good options:

- CometBFT-style consensus
- HotStuff-inspired consensus
- Tendermint-style validator voting

MVP recommendation:

- Fixed validator set in genesis
- 1 block proposer per slot
- Validators vote on blocks
- Finality after 2/3+ voting power signs

Later:

- Dynamic staking
- Delegation
- Slashing
- Validator rotation

### 3. Transaction Model

Support normal Ethereum transaction types:

- Legacy transactions
- EIP-1559 transactions
- Access-list transactions
- Contract creation
- Contract calls

Add INFI-specific optimization:

- Encourage access-list transactions
- Use access lists to schedule non-conflicting transactions in parallel
- Fall back to sequential execution when conflicts are unknown

### 4. Parallel Execution

Use a staged strategy:

1. Execute simple transfers in parallel
2. Execute access-list transactions in parallel when read/write sets do not conflict
3. Add speculative execution later
4. Re-run conflicted transactions sequentially

This preserves EVM behavior while improving throughput.

### 5. Networking

Use libp2p or a similar peer-to-peer networking stack.

Network roles:

- Validator gossip
- Transaction gossip
- Block propagation
- Peer discovery
- State sync

### 6. Storage

Use a Merkle Patricia Trie-compatible state root for EVM compatibility.

Potential storage engines:

- RocksDB
- MDBX
- Redb for simpler early prototypes

Recommendation:

- RocksDB for MVP
- Abstract storage behind traits/interfaces so it can be replaced later

### 7. RPC Compatibility

Expose Ethereum JSON-RPC methods:

- `eth_chainId`
- `eth_blockNumber`
- `eth_getBalance`
- `eth_getTransactionCount`
- `eth_sendRawTransaction`
- `eth_call`
- `eth_estimateGas`
- `eth_getTransactionReceipt`
- `eth_getBlockByNumber`
- `eth_getLogs`

Wallet target:

- MetaMask-compatible from day one

Tooling target:

- Foundry
- Hardhat
- Remix
- ethers.js
- viem

### 8. INFI Scan Explorer

INFI needs its own explorer website: INFI Scan.

INFI Scan should support:

- transaction search by hash
- block search by number or hash
- address lookup
- contract lookup
- validator tracking
- gas fee visibility in Invertx
- logs and events
- cross-chain message status later

The explorer should use an indexer service instead of querying raw node state for every page.

Recommended explorer stack:

- INFI node JSON-RPC
- indexer worker
- PostgreSQL
- search API
- responsive web frontend

### 9. Wallet Connectivity

INFI dapps should support all major device classes:

- mobile
- tablet
- desktop
- laptop

Wallet flows:

- browser extension connection
- WalletConnect QR code connection
- mobile deep-link connection
- manual network add/switch flow
- one-click wallet import through `wallet_addEthereumChain`
- one-click wallet switching through `wallet_switchEthereumChain`

Use WalletConnect/Reown AppKit for INFI dapps such as INFI Scan, bridge, faucet, governance, and staking.

If INFI later ships its own wallet, use WalletKit for wallet-side WalletConnect support.

Public wallet import must display:

- chain name
- chain ID
- native currency metadata
- RPC URL
- INFI Scan URL
- a warning that users should never enter seed phrases

### 9.1 Safety and Transparency

INFI cannot honestly promise to be impossible to hack.

INFI should instead commit to:

- open-source clients
- reproducible builds
- public genesis
- public validator data
- public audits
- bug bounty before mainnet
- public incident reports
- minimal privileged keys
- transparent upgrade process
- clear user warnings in every dapp

### 10. Cross-Chain Interoperability

INFI should be EVM-compatible first, but not EVM-only.

Long-term supported ecosystems:

- Ethereum and EVM chains
- Solana
- Bitcoin
- Cosmos/IBC-style chains
- Polkadot/Substrate-style chains
- additional chains through adapter modules

Cross-chain support should be modular:

```text
INFI Core
  EVM execution
  consensus
  storage
  RPC

Interoperability Layer
  EVM adapter
  Solana adapter
  Bitcoin adapter
  Cosmos adapter
  custom adapter SDK
```

### 11. SbSe Protocol Governance Engine

SbSe Protocol should be added later as a specialized governance logic engine node.

SbSe should not replace validator consensus. Instead:

- validators finalize chain state
- SbSe evaluates governance logic
- approved governance outcomes are written on-chain
- INFI Scan displays governance actions

Potential SbSe responsibilities:

- proposal validation
- voting rule execution
- upgrade authorization
- parameter change authorization
- treasury rules
- validator governance hooks

### 12. Tokenized Asset Decentralization

INFI should include protocol-level support for decentralized tokenized assets.

This includes:

- EVM token standards
- asset registry
- issuer transparency
- verifiable metadata
- mint/burn history
- permission visibility
- risk labels in INFI Scan
- bridge origin tracking for wrapped assets

INFI should make it obvious when an asset has centralized controls such as mint authority, freeze authority, proxy upgrade authority, or centralized metadata.

### 13. INFI Domains

INFI should include an on-chain domain system.

Users should be able to create domains for free.

INFI Domains should support:

- wallet name resolution
- reverse address resolution
- website/content records
- cross-chain address records
- tokenized asset records
- VPN provider records
- verified official names
- INFI Scan domain search

Official names requested:

```text
https://infi.infi/
https://scan.infi.infi/
```

Important: `.infi` requires a browser/DNS resolution strategy before normal browsers can open those URLs publicly.

### 14. Decentralized VPN Function

INFI should include a future decentralized VPN function.

The VPN function should not be part of validator consensus. It should use separate provider nodes coordinated by on-chain registries, staking, reputation, and payment contracts.

The VPN design should include:

- provider registry
- provider reputation
- encrypted tunnels
- privacy-preserving payment design
- user safety warnings
- abuse prevention
- provider terms disclosure
- INFI Scan provider transparency

INFI should not promise perfect anonymity. The product should clearly explain what the VPN protects and what it does not protect.

## Chain Parameters

Suggested first testnet values:

- Chain name: `INFI Testnet`
- Native gas token: `Invertx`
- Token symbol: `TBD`
- Decimals: `18`
- Chain ID: choose a unique number before launch
- Block time: `400ms - 2s` depending on consensus stability
- Finality target: under `3s`
- Gas token: `Invertx`
- Address format: Ethereum `0x...`

## MVP Roadmap

### Phase 1: Local Devnet

- Create genesis file
- Start one local validator
- Execute signed EVM transactions
- Maintain account balances and contract state
- Expose basic Ethereum JSON-RPC
- Deploy and call Solidity contracts

Success test:

- Connect MetaMask
- Deploy an ERC-20 contract
- Transfer tokens
- Query transactions through RPC

### Phase 2: Multi-Validator Testnet

- Add validator networking
- Add block proposal and voting
- Add finality certificates
- Add peer discovery
- Add state sync
- Add block explorer support

Success test:

- Run 4 validators locally
- Stop 1 validator and continue finalizing blocks
- Deploy contracts through Foundry or Hardhat

### Phase 3: Performance Layer

- Add transaction batching
- Add access-list based parallel execution
- Add optimized block propagation
- Add mempool prioritization
- Add fee market tuning

Success test:

- Benchmark simple transfers
- Benchmark ERC-20 transfers
- Benchmark contract-heavy transactions
- Measure TPS, latency, finality, and failed transaction rate

### Phase 4: Public Testnet

- Faucet
- INFI Scan explorer
- Bridge plan
- WalletConnect QR/deep-link support
- Validator docs
- RPC provider docs
- Dapp deployment guide
- Security monitoring
- Free test InvertX faucet
- Monthly faucet cap of 100,000 test InvertX per wallet
- Clear warning that test InvertX is non-tradable and has no redeemable real-world value
- Wallet/explorer display reference of `1 test InvertX = 1.25 USDT`

### Phase 5: Cross-Chain and Governance

- EVM bridge adapter
- non-EVM adapter design
- cross-chain message tracking in INFI Scan
- SbSe Protocol governance engine design
- governance UI
- staking UI

### Phase 6: Mainnet Readiness

- Full decentralization requirements met
- External audits
- Bridge audits
- Slashing and staking economics
- Governance
- Upgrade process
- Incident response plan
- Load testing
- Long-running testnet

## Suggested Initial Repo Structure

```text
infi/
  crates/
    node/          # node binary
    evm/           # EVM execution wrapper
    consensus/     # validator consensus
    storage/       # chain/state database
    rpc/           # Ethereum JSON-RPC
    mempool/       # transaction pool
    primitives/    # shared types
  specs/
    genesis.devnet.json
    chain-params.md
  docs/
    architecture.md
    decentralization.md
    infi-scan.md
    interoperability.md
    validator-guide.md
    wallet-connect.md
    rpc.md
    sbse-protocol.md
    step-by-step.md
  tests/
    evm-compat/
    devnet/
```

## First Build Decision

The most realistic first implementation path is:

```text
Rust + revm + RocksDB + libp2p + Ethereum JSON-RPC
```

This gives INFI a serious foundation while still letting you build toward a Solana-like developer and performance experience.

## Non-Negotiables

Do not launch mainnet until:

- EVM compatibility tests pass
- Consensus has been audited
- State transitions are deterministic
- Reorg behavior is well-defined
- Validator keys are secure
- Chain upgrades are planned
- RPC behavior matches Ethereum expectations
- There is a tested recovery path for network failure
- INFI Scan can reliably track transactions and blocks
- WalletConnect works across desktop, mobile, tablet, and laptop
- Validator participation is decentralized
- Governance is transparent and auditable

## Short Version

INFI should not be "Solana copied with EVM pasted on top."

INFI should be:

- EVM-compatible like Ethereum
- Fast-finality like modern PoS chains
- Parallel where the EVM safely allows it
- Cheap and developer-friendly
- Built from serious components, not toy cryptography
