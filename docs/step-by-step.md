# INFI Step-by-Step Build Guide

This is the practical order for building INFI without losing the plot.

## Step 1: Local Tooling

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart your terminal, then verify:

```bash
rustc --version
cargo --version
```

Then check the workspace:

```bash
cargo check
```

## Step 2: Run the First Local Node

Run:

```bash
cargo run -p infi-node
```

Goal:

- INFI Devnet starts locally
- Invertx is shown as the gas token
- a sample Invertx transfer executes
- block `#1` finalizes

## Step 3: Add Real JSON-RPC

Build the first RPC server.

Required first methods:

- `web3_clientVersion`
- `eth_chainId`
- `eth_blockNumber`
- `eth_getBalance`

Goal:

- a browser, script, or wallet can ask INFI for basic chain state

## Step 4: Add Transaction RPC

Add:

- `eth_getTransactionCount`
- `eth_sendRawTransaction`
- `eth_getTransactionReceipt`
- `eth_getBlockByNumber`

Goal:

- users can submit transactions
- transaction hashes can be followed
- receipts can be queried

## Step 5: Build INFI Scan

Create the INFI Scan explorer website.

Minimum pages:

- home dashboard
- latest blocks
- latest transactions
- transaction detail page
- address detail page
- block detail page
- validator page

Goal:

- every INFI transaction can be followed by hash
- every address balance and nonce can be inspected
- every block can be inspected

## Step 6: Add WalletConnect and Wallet UX

Implement wallet connection for all major device types:

- desktop browser extension wallets
- mobile wallets through QR code
- mobile deep links
- tablet browsers
- laptop browsers

Use WalletConnect/Reown AppKit on dapps such as INFI Scan, and later use WalletKit if INFI ships its own wallet.

Goal:

- users can connect from any supported wallet
- QR code works from desktop to mobile
- mobile deep linking works on phone and tablet

## Step 7: Add Real EVM Execution

Replace the transfer-only placeholder with a real EVM engine.

Recommended:

- `revm`

Goal:

- deploy Solidity contracts
- call contracts
- run ERC-20 transfers
- support Foundry and Hardhat

## Step 8: Add Persistent State

Replace memory-only state with a database.

Recommended first database:

- RocksDB

Goal:

- node can restart without losing chain state
- blocks, receipts, accounts, logs, and transactions are indexed

## Step 9: Decentralize the Network

Move from one local validator to many independent validators.

Required:

- validator keys
- peer discovery
- block gossip
- transaction gossip
- finality voting
- staking
- slashing
- node operator guide

Goal:

- anyone who meets requirements can run an INFI node
- validators are not controlled by one party

## Step 10: Add Cross-Chain Support

INFI should support interoperability with EVM and non-EVM ecosystems.

Targets:

- EVM chains through standard bridge and RPC compatibility
- Solana
- Bitcoin
- Cosmos/IBC-style ecosystems
- any future chain through adapter modules

Goal:

- INFI becomes a multichain execution and settlement environment

## Step 11: Add SbSe Protocol Governance Engine

After Invertx and the base chain are stable, add SbSe Protocol as a governance logic engine node.

Goal:

- SbSe becomes the main governance logic engine
- governance is auditable, decentralized, and upgrade-aware

## Step 12: Public Testnet

Before mainnet:

- publish public RPC endpoints
- publish INFI Scan
- publish faucet
- enforce the 100,000 test InvertX monthly faucet cap
- show that test InvertX is non-tradable and has no redeemable real-world value
- display the InvertX reference value only where clearly labeled for testnet
- publish validator docs
- publish wallet setup docs
- run a long-lived public testnet

## Step 13: Mainnet Readiness

Do not launch mainnet until:

- EVM compatibility tests pass
- consensus is audited
- bridge design is audited
- explorer indexing is stable
- validator software is stable
- governance is defined
- incident recovery is tested
- decentralization requirements are met
