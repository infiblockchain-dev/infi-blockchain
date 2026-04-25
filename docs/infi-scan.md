# INFI Scan Explorer Plan

INFI Scan is the official explorer website for INFI Blockchain.

## Purpose

Users must be able to follow every transaction, block, address, validator, contract, token, and event on INFI.

INFI Scan should also support tokenized asset transparency and future decentralized VPN provider transparency.
INFI Scan should also support INFI Domains search and domain history.

## MVP Pages

Current static MVP page:

```text
https://infi-blockchain.pages.dev/scan
```

The current page reads directly from the public INFI Testnet RPC. It supports:

- latest block and RPC status
- transaction hash search
- block number search
- block hash search
- address balance and nonce search
- clear testnet-only warnings for test InvertX

This is not the final production INFI Scan indexer. A production explorer still needs a backend indexer, database, pagination, contract decoding, logs, token transfers, validator views, domains, RWA transparency, and decentralized VPN provider transparency.

### Home

- chain status
- latest block
- finalized block height
- transaction count
- average block time
- active validators
- gas token: Invertx

### Transactions

- latest transactions
- transaction hash
- from address
- to address
- value
- gas used
- status
- block number
- timestamp

### Transaction Detail

- transaction hash
- status
- block number
- timestamp
- confirmations/finality
- from
- to
- value in Invertx
- gas fee in Invertx
- input data
- logs

### Blocks

- latest blocks
- proposer
- transaction count
- timestamp
- gas used
- block hash

### Address Detail

- Invertx balance
- nonce
- transactions
- contract code, if contract address
- token balances, later

On testnet, INFI Scan must clearly label test balances:

```text
test InvertX is non-tradable and has no redeemable real-world value.
InvertX reference display for testnet: 1 test InvertX = 1.25 USDT.
```

INFI Scan should use the InvertX logo for both test InvertX and real InvertX:

```text
https://infi.infi/assets/tokens/invertx-icon-512.png
```

### Validators

- validator address
- voting power
- uptime
- produced blocks
- missed blocks
- stake, later

### Tokenized Assets

- asset overview
- issuer
- holders
- transfers
- mint/burn history
- metadata hash
- admin permissions
- risk warnings
- verification status

### INFI Domains

- domain name
- owner
- resolver
- wallet records
- content records
- cross-chain records
- history
- verified official status
- impersonation warnings

### VPN Providers

- provider address
- region metadata
- public key
- uptime
- price
- reputation
- stake
- slashing history
- terms hash
- aggregate network capacity

## Backend Indexer

INFI Scan needs an indexer service that reads from the node and stores query-friendly data.

Recommended components:

- INFI node JSON-RPC
- indexer worker
- PostgreSQL
- search API
- frontend website

## Search

The search bar should accept:

- transaction hash
- block number
- block hash
- address
- contract address
- token address, later
- tokenized asset name
- INFI domain name
- VPN provider address, later

## Wallet Support

INFI Scan should include wallet connection for:

- browser extension wallets
- WalletConnect QR code
- mobile deep links
- tablet and mobile browsers
- one-click add INFI network
- one-click switch to INFI

Wallet connection must never be required just to inspect public chain data.

## New User Experience

INFI Scan should make the chain understandable for first-time users:

- show clear transaction status
- explain gas fees in Invertx
- provide manual wallet setup instructions
- show verified RPC and explorer URLs
- warn users never to enter seed phrases
- make failed transactions understandable
- show confirmations/finality clearly

## Transparency

INFI Scan should publish:

- network status
- current chain ID
- active RPC endpoints
- validator set
- known incidents
- software version, when available
- governance activity, once SbSe Protocol exists
- tokenized asset risk flags
- VPN provider reputation, once the VPN function exists
