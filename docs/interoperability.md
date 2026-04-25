# INFI Cross-Chain Support

INFI should support all major blockchain ecosystems over time, including EVM and non-EVM chains.

## Principle

INFI is EVM-compatible at the execution layer, but cross-chain support must not be limited to EVM chains.

## EVM Support

EVM compatibility should include:

- Ethereum-style addresses
- Solidity contracts
- Ethereum JSON-RPC
- EIP-1559-style transactions
- Foundry support
- Hardhat support
- MetaMask support
- WalletConnect support

## Non-EVM Support

Future adapter targets:

- Solana
- Bitcoin
- Cosmos/IBC ecosystems
- Polkadot/Substrate ecosystems
- other chains through adapter modules

## Architecture

Cross-chain support should be built through adapters, not hardcoded into the core node.

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

## Bridge Requirements

Any bridge or cross-chain messaging system must be:

- audited
- rate-limited
- observable
- upgrade-aware
- protected against replay attacks
- protected against validator or signer compromise
- transparent in INFI Scan

## Explorer Requirements

INFI Scan should eventually show:

- source chain
- destination chain
- bridge transaction status
- cross-chain message status
- finality status on both sides
- bridge fees

