# INFI Chain Parameters

## Devnet

- Chain name: INFI Devnet
- Chain ID: 98401
- Native gas token name: Invertx
- Native gas token symbol: TBD
- Native gas token decimals: 18
- Address format: Ethereum-style `0x` addresses
- Block time target: 1 second
- Finality target: local instant finality for MVP
- Explorer: INFI Scan
- Wallet connection: browser wallets plus WalletConnect QR and mobile deep links
- Wallet import: `wallet_addEthereumChain` and `wallet_switchEthereumChain`
- Official website: `https://infi.infi/`
- Official explorer: `https://scan.infi.infi/`
- Domain system: free INFI Domains, pending resolver/browser strategy
- Governance engine: SbSe Protocol later, not active in devnet

## Testnet Token Policy

- Testnet token name: test InvertX
- Testnet token symbol: tINVX
- Testnet token decimals: 18
- Testnet chain ID: `98402`
- Testnet chain ID hex: `0x18062`
- Current testnet RPC URL: `https://infi-testnet-rpc.onrender.com`
- Future custom testnet RPC URL: `https://rpc.infi.infi`
- Testnet explorer target: `https://scan.infi.infi`
- Current testnet token logo: `https://infi-blockchain.pages.dev/assets/tokens/invertx-icon-512.png`
- Testnet token real value: none
- Testnet token trading: disabled / not tradable
- Wallet/explorer display reference: `1 test InvertX = 1.25 USDT`
- Faucet monthly cap: `100,000 test InvertX per wallet per calendar month`
- Scope: testnet only

The USDT display is the intended InvertX reference value shown in testnet wallets and explorers. test InvertX itself is not tradable, redeemable, or a promise of value.

## Compatibility Goals

- Ethereum JSON-RPC compatibility
- Solidity contract deployment
- MetaMask support
- WalletConnect support
- Foundry and Hardhat support
- EVM execution compatibility
- INFI Scan transaction tracking
- EVM and non-EVM cross-chain adapter support over time

## Mainnet Decentralization Targets

- permissionless full nodes
- independent validators
- staking and delegation
- slashing
- archive nodes
- public RPC providers
- public INFI Scan explorer
- transparent governance
- future SbSe Protocol governance engine node
- public safety and transparency documentation
- one-click wallet import from INFI Scan and official dapps
- free INFI Domains with reserved official names

## Mainnet Metadata Requirements

Mainnet cannot launch until these are final:

- chain ID in decimal and hex
- chain name
- native currency name
- native currency symbol
- native currency decimals
- official HTTPS RPC URLs
- official INFI Scan HTTPS URL
- public genesis file
- validator set or validator onboarding process
