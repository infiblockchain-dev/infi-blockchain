# INFI Testnet Launch Guide

This is the step-by-step guide for launching INFI Blockchain on testnet first.

The testnet is for development, testing, wallet integration, INFI Scan testing, validator testing, and community feedback. It is not mainnet.

## Testnet Token Policy

The testnet gas token should be displayed as:

```text
Name: test InvertX
Symbol: tInvertX
Decimals: 18
Network: INFI Testnet only
Real value: none
Wallet/explorer display reference: 1 test InvertX = 1.25 USDT
```

Important: the displayed `1 test InvertX = 1.25 USDT` value is the InvertX reference value shown for testnet UI only.

For test InvertX, it must not mean:

- redeemable value
- tradable value
- exchange listing value
- investment value
- legal value
- mainnet value
- promise of future value

Every faucet, wallet, explorer, and testnet page must clearly show:

```text
test InvertX is a non-tradable testnet token with no redeemable real-world value.
```

## Monthly Faucet Limit

The faucet may mint a maximum of:

```text
100,000 test InvertX per wallet per calendar month
```

This limit applies only to testnet.

Mainnet Invertx must not use this faucet minting policy.

## Phase 1: Finish Local Devnet

Before public testnet, local devnet must support:

- node starts reliably
- blocks are produced
- account balances update correctly
- gas token metadata is correct
- JSON-RPC server works
- basic transactions work
- logs are readable

Commands after Rust is installed:

```bash
cargo check
cargo run -p infi-node
```

Exit criteria:

- local node runs without crashing
- sample transaction executes
- block height increases once block production is implemented

## Phase 2: Add JSON-RPC

Implement first RPC methods:

- `web3_clientVersion`
- `eth_chainId`
- `eth_blockNumber`
- `eth_getBalance`
- `eth_getTransactionCount`
- `eth_sendRawTransaction`
- `eth_getTransactionReceipt`

Current prototype progress:

- `web3_clientVersion`: implemented
- `eth_chainId`: implemented
- `net_version`: implemented
- `eth_blockNumber`: implemented
- `eth_getBalance`: implemented
- `eth_getTransactionCount`: implemented
- `eth_getTransactionReceipt`: implemented for indexed prototype transactions
- `eth_sendRawTransaction`: implemented for temporary dev-only transfer payloads

Important: real Ethereum signed transaction decoding is still pending. The dev-only payload format must be replaced before public testnet wallet support.

Exit criteria:

- wallet or script can query chain ID
- wallet or script can query balance
- transaction hash can be returned
- receipt can be queried
- dev-only transfer transaction can be submitted locally

## Phase 3: Add Persistent Storage

Add a database before public testnet.

Recommended:

- RocksDB or MDBX

Required data:

- blocks
- accounts
- transactions
- receipts
- logs
- validator data, later

Exit criteria:

- node can restart without losing state
- block history remains available
- INFI Scan can index from stored data

## Phase 4: Add Real EVM Execution

Replace the transfer-only execution placeholder with real EVM execution.

Recommended:

- `revm`

Exit criteria:

- Solidity contracts deploy
- ERC-20 test contract works
- contract calls work
- gas accounting works
- receipts and logs work

## Phase 5: Create Testnet Genesis

Create:

```text
specs/genesis.testnet.json
```

It should include:

- chain name: `INFI Testnet`
- chain ID: final testnet chain ID
- native token: `test InvertX`
- faucet allocation or faucet minter permission
- initial validator set
- initial system contracts, if any

Exit criteria:

- every validator uses identical genesis
- genesis hash is published
- chain ID is unique

## Phase 6: Build the Faucet

The faucet must:

- mint or distribute only test InvertX
- enforce monthly limit of 100,000 test InvertX per wallet
- show no-real-value warning
- show InvertX reference value only with testnet/non-tradable labels
- rate-limit requests
- prevent obvious abuse
- log claims for transparency
- expose faucet status

Current prototype:

- static faucet page exists at `site/faucet.html`
- local dev faucet can submit dev-only transfers through `eth_sendRawTransaction`
- server-side monthly cap enforcement is still pending

Recommended claim policy:

```text
Maximum: 100,000 test InvertX per wallet per calendar month
Default claim: smaller amount, such as 1,000 or 5,000 test InvertX
Network: INFI Testnet only
```

Exit criteria:

- wallet can claim test tokens
- monthly cap works
- warning is visible
- faucet cannot mint mainnet tokens

## Phase 7: Launch INFI Scan Testnet

INFI Scan testnet must show:

- latest blocks
- latest transactions
- transaction search
- address search
- test InvertX balances
- faucet links
- no-real-value warnings
- InvertX reference value label

Display balances like:

```text
1,000 test InvertX
InvertX reference display for testnet: 1 test InvertX = 1.25 USDT
Non-tradable. No redeemable real-world value.
```

Exit criteria:

- every testnet transaction can be followed
- faucet transactions are searchable
- no-real-value warning is visible

## Phase 8: Add Wallet Import

Testnet dapps should support:

- one-click add INFI Testnet
- one-click switch to INFI Testnet
- WalletConnect QR
- mobile deep links
- manual setup fallback

Wallet metadata:

```text
Network name: INFI Testnet
Chain ID: 98402
Chain ID hex: 0x18062
Native currency: test InvertX
Symbol: tInvertX
Decimals: 18
RPC URL: https://rpc.infi.infi
Explorer URL: https://scan.infi.infi
Token icon: https://infi.infi/assets/tokens/invertx-icon-512.png
```

The chain ID is provisional until the public testnet genesis is finalized.

Exit criteria:

- users can add INFI Testnet to wallet
- users can connect from desktop and mobile
- users can claim faucet tokens
- users can send a test transaction
- wallet shows test InvertX metadata correctly

## Phase 9: Multi-Validator Testnet

Start with internal validators, then add independent operators.

Required:

- validator onboarding docs
- validator keys
- peer discovery
- block gossip
- transaction gossip
- finality voting
- monitoring
- restart procedure

Exit criteria:

- at least 4 validators
- at least 2 independent operators before wider public testing
- network survives one validator going offline
- validator status is visible in INFI Scan

## Phase 10: Public Testnet Announcement

Publish:

- testnet website
- testnet RPC URL
- testnet INFI Scan URL
- faucet URL
- wallet setup guide
- validator guide
- known limitations
- no-real-value token warning
- bug report process

Announcement must say:

```text
INFI Testnet tokens are free test tokens. They are non-tradable, have no redeemable real-world value, and are not exchange-listed assets.
```

## Phase 11: Testnet Monitoring

Monitor:

- block production
- finality
- validator uptime
- RPC errors
- faucet abuse
- indexer lag
- wallet connection failures
- failed transactions
- suspicious activity

Publish testnet health status regularly.

## Phase 12: Testnet Upgrade Process

Testnet upgrades should be rehearsals for mainnet.

Every upgrade should include:

- release notes
- version number
- validator instructions
- rollback notes
- known risks
- upgrade window
- post-upgrade report

## Testnet Launch Blockers

Do not launch public testnet if:

- JSON-RPC is missing
- transactions cannot be followed
- faucet can mint unlimited tokens
- no-real-value warning is missing
- monthly faucet cap is missing
- chain ID is not final
- genesis is not reproducible
- validator restart fails
- INFI Scan cannot index
- wallet import does not work
