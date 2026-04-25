# INFI WalletConnect Plan

INFI needs proper wallet support across desktop, mobile, tablet, and laptop.

## Goals

- support browser extension wallets
- support QR code connection
- support mobile deep linking
- support one-click wallet import
- support one-click network switching
- support tablet browsers
- support desktop and laptop browsers
- support EVM-compatible wallets first
- support non-EVM wallet namespaces later through cross-chain adapters

## Recommended Integration

Use WalletConnect/Reown AppKit for dapps such as:

- INFI Scan
- bridge UI
- governance UI
- staking UI
- faucet UI

Use WalletKit later if INFI ships its own wallet application.

For EVM wallet import, use:

- `wallet_addEthereumChain`
- `wallet_switchEthereumChain`

See [wallet import plan](wallet-import.md).

## Required User Flows

### Desktop Browser Extension

1. User opens INFI Scan.
2. User clicks connect wallet.
3. User selects browser wallet.
4. Wallet prompts user to switch/add INFI network.
5. User approves connection.

### Desktop to Mobile QR

1. User opens INFI Scan on desktop or laptop.
2. User clicks connect wallet.
3. User selects WalletConnect.
4. INFI Scan shows a QR code.
5. User scans QR code with mobile wallet.
6. User approves the session on mobile.

### Mobile Deep Link

1. User opens INFI Scan on a mobile browser.
2. User clicks connect wallet.
3. User selects mobile wallet.
4. The browser opens the wallet through a deep link.
5. User approves.
6. User returns to INFI Scan.

## Chain Metadata

Wallet integrations need:

```text
Chain name: INFI
Chain ID: 98401 for devnet
Native currency name: Invertx
Native currency symbol: TBD
Native currency decimals: 18
RPC URL: TBD
Explorer URL: INFI Scan URL, TBD
```

## Non-EVM Support

INFI is EVM-compatible first, but the wallet layer should be designed for multichain support.

Future namespaces:

- EVM
- Solana
- Bitcoin
- Cosmos-style chains
- additional non-EVM chains through adapters

## Security Requirements

- never ask users for seed phrases
- never promise that a transaction is risk-free
- verify domain identity
- show clear signing prompts
- separate read-only explorer usage from wallet-connected actions
- require explicit user approval for transactions
- make chain switching obvious
- show manual setup instructions when automatic import fails
