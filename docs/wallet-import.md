# INFI Wallet Import Plan

INFI should be easy to add to any compatible wallet.

## Goals

- one-click add network button
- one-click switch network button
- QR and mobile wallet connection through WalletConnect/Reown AppKit
- clear chain metadata
- clear RPC endpoint warnings
- no seed phrase requests
- beginner-friendly fallback instructions

## EVM Wallet Import

EVM wallets should use:

- `wallet_addEthereumChain` to suggest adding INFI
- `wallet_switchEthereumChain` to request switching to INFI

Wallets may reject either request. INFI dapps must handle rejection politely and show manual setup instructions.

## Devnet Metadata

```json
{
  "chainId": "0x18061",
  "chainName": "INFI Devnet",
  "nativeCurrency": {
    "name": "Invertx",
    "symbol": "TBD",
    "decimals": 18
  },
  "rpcUrls": ["http://127.0.0.1:8545"],
  "blockExplorerUrls": ["http://127.0.0.1:3000"]
}
```

The devnet chain ID `98401` is `0x18061` in hexadecimal.

## Testnet Metadata

Provisional testnet metadata:

```json
{
  "chainId": "0x18062",
  "chainName": "INFI Testnet",
  "nativeCurrency": {
    "name": "test InvertX",
    "symbol": "tInvertX",
    "decimals": 18
  },
  "rpcUrls": ["https://infi-testnet-rpc.onrender.com"],
  "blockExplorerUrls": ["https://scan.infi.infi"],
  "iconUrls": [
    "https://infi-blockchain.pages.dev/assets/tokens/invertx-icon-512.png",
    "https://infi-blockchain.pages.dev/assets/tokens/invertx-icon-256.png"
  ]
}
```

The provisional testnet chain ID `98402` is `0x18062` in hexadecimal.

Wallets should display:

```text
Network name: INFI Testnet
Native currency name: test InvertX
Native currency symbol: tInvertX
Native currency decimals: 18
RPC URL: https://infi-testnet-rpc.onrender.com
Explorer target: https://scan.infi.infi
Token icon: https://infi-blockchain.pages.dev/assets/tokens/invertx-icon-512.png
```

Testnet wallet flows must show that `test InvertX` is non-tradable and has no redeemable real-world value.

Wallets and dapps may show:

```text
1 test InvertX = 1.25 USDT
```

as the InvertX reference value for testnet UI only.

Testnet setup page:

```text
https://infi.infi/testnet.html
```

## Mainnet Metadata

Mainnet metadata must be finalized before launch.

```json
{
  "chainId": "TBD_HEX",
  "chainName": "INFI Mainnet",
  "nativeCurrency": {
    "name": "Invertx",
    "symbol": "TBD",
    "decimals": 18
  },
  "rpcUrls": ["https://rpc.infi.infi"],
  "blockExplorerUrls": ["https://scan.infi.infi"]
}
```

## Dapp Button Behavior

The connect flow should:

1. Detect whether an injected wallet is available.
2. Request account connection only after the user clicks connect.
3. Check `eth_chainId`.
4. If INFI is not active, call `wallet_switchEthereumChain`.
5. If the wallet reports INFI is unknown, call `wallet_addEthereumChain`.
6. If the user rejects, show manual setup instructions.
7. Never block public explorer reads behind wallet connection.

## Safety Requirements

- use HTTPS RPC URLs for public networks
- verify `eth_chainId` matches the configured chain ID
- show the RPC URL before adding the chain
- show the explorer URL before adding the chain
- warn users that RPC endpoints can see IP address and request metadata
- never auto-submit transactions after switching chains
- require explicit approval for every signing or transaction request

## Manual Setup Fallback

INFI Scan should include a manual setup panel:

```text
Network name: INFI Mainnet
RPC URL: TBD
Chain ID: TBD
Currency symbol: TBD
Block explorer URL: TBD
```

For devnet:

```text
Network name: INFI Devnet
RPC URL: http://127.0.0.1:8545
Chain ID: 98401
Currency symbol: TBD
Block explorer URL: http://127.0.0.1:3000
```
