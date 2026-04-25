# INFI Blockchain Full Concept Overview

This document summarizes the complete INFI Blockchain concept captured in this repository.
It is designed as a quick handoff for developers, infrastructure partners, reviewers, and future contributors.

## Vision

INFI Blockchain is planned as an EVM-compatible Layer 1 blockchain focused on decentralization, transparency, new-user experience, and cross-chain usability.

The long-term goal is to combine:

- EVM-compatible smart contracts
- wallet-friendly onboarding
- public transaction transparency through INFI Scan
- support for EVM and non-EVM blockchain ecosystems
- tokenized asset infrastructure
- DeFi, DePIN, AI, and decentralized RWA infrastructure
- a future decentralized VPN function
- free INFI Domains
- future SbSe Protocol governance logic engine node

The project must become fully decentralized before mainnet. Mainnet should not launch until validator networking, independent node operation, staking or validator economics, security review, monitoring, public docs, and production infrastructure are complete.

## Current Repository Status

The repository currently contains an early public-testnet prototype, not mainnet software.

Implemented prototype pieces include:

- Rust workspace with chain modules
- primitive chain/account/block/transaction types
- memory-backed storage with prototype file persistence
- mempool placeholder
- consensus placeholder
- EVM execution placeholder
- Ethereum JSON-RPC compatibility layer
- public RPC deployment config for Render
- static INFI website
- INFI Testnet wallet setup page
- faucet UI
- faucet RPC endpoints
- token/logo/favicon assets
- developer and launch documentation

Important limitations:

- no real Ethereum signed transaction decoding yet
- no production EVM execution yet
- no production validator networking yet
- no staking, delegation, or slashing yet
- no production database or indexer yet
- no public INFI Scan explorer backend yet
- no production WalletConnect QR session until a Reown/WalletConnect project is configured
- no mainnet readiness yet

## Chain Identity

Current public testnet target:

```text
Network name: INFI Testnet
Chain ID: 98402
Chain ID hex: 0x18062
Native test token: test InvertX
Wallet symbol: tINVX
Decimals: 18
Public RPC: https://infi-testnet-rpc.onrender.com
Explorer target: https://scan.infi.infi
Public setup page: https://infi-blockchain.pages.dev/testnet
```

Planned official domains:

```text
Website: https://infi.infi/
Explorer: https://scan.infi.infi/
```

Important: `.infi` browser resolution requires a real DNS, gateway, browser extension, wallet browser, or resolver strategy before normal browsers can open those names publicly.

## InvertX and test InvertX

InvertX is planned as the native gas token for INFI mainnet.

Current testnet token:

```text
Name: test InvertX
Symbol: tINVX
Decimals: 18
Purpose: free INFI Testnet gas only
```

Testnet policy:

- test InvertX is non-tradable
- test InvertX has no redeemable real-world value
- faucet minting applies only to testnet
- mainnet InvertX must never use testnet faucet minting logic
- testnet UI may display `1 test InvertX = 1.25 USDT` as a test UI reference only

Faucet policy:

```text
Maximum per claim: 10,000 test InvertX
Maximum per wallet per calendar month: 100,000 test InvertX
```

## Website and User Experience

The static website lives in `site/`.

Current pages:

- `site/index.html` - INFI Blockchain public homepage
- `site/testnet.html` - wallet setup page
- `site/faucet.html` - faucet page

The website includes:

- SEO metadata
- Open Graph and Twitter metadata
- favicons and app icons
- brand assets
- animated blockchain-style background
- public disclosure boundaries
- testnet warnings
- wallet setup information
- faucet information

Public page:

```text
https://infi-blockchain.pages.dev/
```

## Wallet Support

Current wallet setup page supports:

- injected browser wallets on desktop and laptop
- mobile and tablet wallet browsers
- one-click `wallet_addEthereumChain`
- one-click `wallet_switchEthereumChain`
- public RPC readiness check
- MetaMask mobile deep link
- Coinbase Wallet mobile deep link
- Trust Wallet mobile deep link
- manual setup fallback
- copyable setup fields
- chain metadata JSON fallback
- WalletConnect QR status button

WalletConnect/Reown AppKit is still a required next milestone for real QR sessions. A production WalletConnect QR flow needs a Reown/WalletConnect project ID and a real dapp connection flow.

## RPC Compatibility

The current prototype exposes wallet-friendly Ethereum JSON-RPC methods so wallets can recognize and inspect INFI Testnet.

Implemented prototype RPC methods include:

- `web3_clientVersion`
- `eth_chainId`
- `net_version`
- `eth_syncing`
- `eth_accounts`
- `eth_blockNumber`
- `eth_gasPrice`
- `eth_maxPriorityFeePerGas`
- `eth_feeHistory`
- `eth_estimateGas`
- `eth_getBalance`
- `eth_getBlockByNumber`
- `eth_getBlockByHash`
- `eth_getBlockTransactionCountByNumber`
- `eth_getBlockTransactionCountByHash`
- `eth_getTransactionCount`
- `eth_getTransactionByHash`
- `eth_getTransactionByBlockNumberAndIndex`
- `eth_getTransactionByBlockHashAndIndex`
- `eth_getTransactionReceipt`
- `eth_getCode`
- `eth_getStorageAt`
- `eth_call`
- `eth_getLogs`
- `eth_sendRawTransaction`

Prototype HTTP endpoints:

- `GET /health`
- `GET /faucet/status?address=0x...`
- `POST /faucet/claim`

Important: some RPC responses are prototype compatibility responses. Production mainnet requires full EVM state execution, contract storage, logs/events, signed transaction decoding, fee logic, and indexed explorer data.

## INFI Scan

INFI Scan is the planned explorer website where users can follow:

- transactions
- blocks
- addresses
- contracts
- validators
- faucet transfers
- gas usage
- future cross-chain activity
- future domains

The final explorer should not query raw node state for every page. It should use:

- INFI JSON-RPC
- indexer worker
- production database
- search API
- responsive frontend

## INFI Domains

INFI should include free on-chain domains.

Initial direction:

- one free domain per wallet
- domain ownership stored on-chain
- wallet address records
- dapp/service records
- cross-chain records later
- wallet/dapp-level resolution before normal browser resolution

Official requested domains:

```text
https://infi.infi/
https://scan.infi.infi/
```

## Cross-Chain Direction

INFI is EVM-compatible first, but the architecture should support EVM and non-EVM ecosystems over time.

Future adapters can target:

- EVM chains
- Solana
- Bitcoin
- Cosmos-style chains
- additional non-EVM networks

Security principle: cross-chain support must be adapter-based and audited. Bridges and wrapped asset systems are high risk and must not be launched casually.

## InvertX Cross-Chain Role

The concept positions InvertX as an important protocol unit for instant cross-chain transaction experience.

Long-term design direction:

- reduce bridge dependency
- reduce wrapped asset complexity
- reduce liquidity fragmentation
- support DeFi, DePIN, AI, and RWA infrastructure
- avoid APY/staking promises in decentralized environments unless a future audited economic model explicitly supports them

This remains a protocol design goal, not a completed implementation.

## Tokenized Assets and RWA

INFI should support decentralized tokenized asset infrastructure.

Design goals:

- transparent asset metadata
- issuer and attestation records
- ownership and transfer history
- compliance-aware architecture without compromising core decentralization
- auditability
- cross-chain records later

Real-world asset infrastructure requires legal, oracle, custody, and compliance design before production use.

## DePIN, AI, and VPN Direction

INFI should support DePIN and AI infrastructure as ecosystem use cases.

The decentralized VPN function should be a separate provider-node/service layer coordinated by on-chain registry, payments, reputation, and transparent rules.

Important privacy boundary:

- avoid publishing browsing destinations on-chain
- avoid linking wallet identity to browsing activity where possible
- keep VPN provider logic separate from validator consensus

## Governance and SbSe Protocol

SbSe Protocol is planned as a future governance logic engine node.

It should be added only after:

- base chain is stable
- public testnet is running
- INFI Scan indexing exists
- governance requirements are known
- safety and decentralization assumptions are reviewed

Validators still finalize chain state. SbSe should support governance logic, not bypass consensus.

## Safety and Transparency

No blockchain can honestly promise it "cannot be hacked." INFI should instead focus on:

- threat modeling
- transparent limits
- public docs
- audits
- reproducible builds
- secure key handling
- conservative launch stages
- explicit wallet confirmations
- no seed phrase requests
- visible RPC and chain metadata
- public status pages
- incident response process

Public website copy should focus on what the network does, how anyone can verify it, and how to participate. The protocol, source code, audit results, and chain state are public. Operational security details like incident-response playbooks and key custody procedures are documented internally—standard practice for any security-conscious project—but the protocol itself remains fully open.

## Testnet Launch Path

Current testnet steps:

1. Run local node and verify JSON-RPC.
2. Deploy public RPC to Render.
3. Deploy website to Cloudflare Pages.
4. Add INFI Testnet wallet setup page.
5. Add faucet UI and faucet endpoints.
6. Add prototype persistence for faucet/account/block state.
7. Improve wallet compatibility for MetaMask-style RPC calls.
8. Verify public RPC, wallet import, faucet, and explorer targets.

Next important testnet work:

- redeploy Render after latest RPC compatibility commit
- verify MetaMask no longer shows RPC warning
- add production database for faucet and indexer
- add real Ethereum signed transaction decoding
- add real EVM execution
- build INFI Scan indexer and explorer backend
- add WalletConnect/Reown AppKit with QR sessions
- add validator networking
- add observability/status page
- run private multi-node testnet before public community testnet

## Mainnet Launch Gate

Do not launch mainnet until at least:

- real EVM execution exists
- signed transactions are implemented
- validator networking is implemented
- independent validators can run nodes
- staking/validator economics are finalized
- slashing or equivalent validator accountability is designed
- production storage is implemented
- INFI Scan is live and indexing
- wallet import is stable
- WalletConnect QR and mobile flows are tested
- public RPC redundancy exists
- security review and audits are complete
- incident response plan exists
- chain ID and genesis are final
- testnet faucet cannot affect mainnet
- operational security playbooks (key custody, incident response procedures) are documented internally and not in marketing copy

## Key Files

- `README.md` - project introduction
- `INFI_BLOCKCHAIN_BLUEPRINT.md` - original architecture blueprint
- `docs/index.md` - documentation home
- `docs/testnet-launch.md` - testnet launch guide
- `docs/mainnet-launch.md` - mainnet launch guide
- `docs/rpc.md` - RPC reference
- `docs/wallet-import.md` - wallet import plan
- `docs/wallet-connect.md` - WalletConnect plan
- `docs/infi-scan.md` - explorer plan
- `docs/infi-domains.md` - domains concept
- `docs/interoperability.md` - cross-chain concept
- `docs/tokenized-assets.md` - tokenized asset concept
- `docs/decentralized-vpn.md` - VPN concept
- `docs/sbse-protocol.md` - future governance logic node
- `site/` - public website and testnet/faucet pages
- `crates/` - Rust prototype modules
- `specs/` - chain parameters and genesis files
- `render.yaml` - public RPC deployment blueprint
- `Dockerfile` - RPC container build

## Final Note

This repository is a serious foundation and concept package, but it is not a completed production blockchain. Treat it as the INFI concept, testnet prototype, documentation base, and launch roadmap. The next engineering phase is to replace prototype internals with production-grade EVM execution, networking, storage, indexing, wallet sessions, security controls, and decentralized validator operation.
