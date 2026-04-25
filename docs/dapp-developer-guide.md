# INFI Dapp Developer Guide

This guide is for developers building applications on INFI.

## Compatibility Goal

INFI should feel familiar to Ethereum developers:

- Ethereum-style addresses
- EVM-compatible smart contracts
- Ethereum JSON-RPC
- Solidity support
- Foundry support
- Hardhat support
- MetaMask and WalletConnect support
- Invertx as the native gas token

## Network Metadata

Devnet:

```text
Chain name: INFI Devnet
Chain ID: 98401
Chain ID hex: 0x18061
Native currency: Invertx
Decimals: 18
RPC URL: http://127.0.0.1:8545
Explorer URL: http://127.0.0.1:3000
```

Mainnet metadata is not final yet.

## Connecting Wallets

Dapps should support:

- injected browser wallets
- WalletConnect QR code
- mobile deep links
- manual wallet setup fallback
- INFI Domains resolution once the domain module is available

Never ask for seed phrases.

## Adding INFI to a Wallet

Use `wallet_addEthereumChain` for EVM wallets.

```js
await window.ethereum.request({
  method: "wallet_addEthereumChain",
  params: [
    {
      chainId: "0x18061",
      chainName: "INFI Devnet",
      nativeCurrency: {
        name: "Invertx",
        symbol: "TBD",
        decimals: 18
      },
      rpcUrls: ["http://127.0.0.1:8545"],
      blockExplorerUrls: ["http://127.0.0.1:3000"]
    }
  ]
});
```

## Switching to INFI

Use `wallet_switchEthereumChain`.

```js
await window.ethereum.request({
  method: "wallet_switchEthereumChain",
  params: [{ chainId: "0x18061" }]
});
```

## Transaction UX

Every dapp should show:

- connected chain name
- connected address
- estimated gas fee in Invertx
- transaction hash after submission
- INFI Scan link
- final transaction status
- resolved INFI domain name, when available

## User Safety Checklist

- never request unlimited permissions unless necessary
- avoid blind signing
- explain dangerous actions
- show token and contract addresses clearly
- link to INFI Scan for verification
- handle wallet rejection without breaking the app
