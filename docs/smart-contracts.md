# Smart Contract Developer Guide

INFI is designed to support Solidity and EVM-compatible contracts.

## Current Status

The current prototype does not yet execute real smart contracts. Contract support will arrive when the execution layer integrates a real EVM engine such as `revm`.

## Target Tooling

INFI should support:

- Solidity
- Foundry
- Hardhat
- Remix
- ethers.js
- viem
- OpenZeppelin contracts

## Native Token

Invertx is the native gas token. It behaves like ETH on Ethereum.

This means:

- users pay gas in Invertx
- account balances include native Invertx
- contracts can receive native Invertx
- wrapped Invertx can be added later as an ERC-20

## Contract Deployment Flow

Once JSON-RPC and EVM execution are implemented:

1. add INFI network to wallet
2. configure Foundry or Hardhat with the INFI RPC URL
3. compile contracts
4. deploy through RPC
5. verify transaction in INFI Scan
6. verify contract source, once verification exists

## Foundry Example

```toml
[rpc_endpoints]
infi_devnet = "http://127.0.0.1:8545"
```

```bash
forge create \
  --rpc-url infi_devnet \
  --private-key "$PRIVATE_KEY" \
  src/MyContract.sol:MyContract
```

## Hardhat Example

```js
module.exports = {
  networks: {
    infiDevnet: {
      url: "http://127.0.0.1:8545",
      chainId: 98401,
      accounts: [process.env.PRIVATE_KEY]
    }
  },
  solidity: "0.8.24"
};
```

## Contract Safety

Before deploying contracts with real value:

- write tests
- run static analysis
- avoid unaudited upgrade logic
- avoid hidden owner powers
- use timelocks for privileged actions
- publish source code
- verify contracts in INFI Scan when available

