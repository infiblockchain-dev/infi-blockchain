# INFI Protocol Specification

This document defines the intended protocol shape for INFI.

## Chain Identity

```text
Chain: INFI
Native gas token: Invertx
Address format: Ethereum-style 20-byte addresses
Execution target: EVM-compatible
```

## Account Model

INFI uses an Ethereum-style account model.

Each account has:

- address
- native Invertx balance
- nonce
- contract code, if it is a contract account
- storage, if it is a contract account

## Transaction Model

INFI should support Ethereum transaction types:

- legacy transactions
- EIP-155 replay-protected transactions
- EIP-1559-style fee transactions
- access-list transactions
- contract creation
- contract calls

## Block Model

Each block should contain:

- parent hash
- block number
- state root
- transaction root
- receipt root
- logs bloom
- proposer
- timestamp
- gas used
- gas limit
- transactions

## State Transition

For every valid block:

1. verify parent block
2. verify proposer
3. verify transaction ordering
4. execute transactions deterministically
5. charge gas in Invertx
6. update account state
7. produce receipts
8. calculate roots
9. finalize through consensus

## Finality

INFI should use BFT-style finality with validator voting.

Target properties:

- deterministic finality
- clear finalized height
- no ambiguous finality UI
- explorer displays finality status

## Parallel Execution

INFI should support safe parallel execution where transaction dependencies are known.

Initial strategy:

- parallelize simple transfers
- parallelize access-list transactions without conflicts
- fall back to sequential execution on conflicts

## Compatibility

INFI should preserve EVM behavior even when optimizing execution.

Performance optimizations must not change:

- transaction results
- gas behavior
- event logs
- receipt status
- contract storage
- block state root

## Special Protocol Modules

INFI should support special modules over time.

### Tokenized Asset Module

Purpose:

- decentralized tokenized asset issuance
- asset registry
- issuer transparency
- verifiable metadata
- permission and risk visibility

This module should be implemented through EVM contracts first, with protocol/indexer support where needed.

### INFI Domains Module

Purpose:

- free on-chain domain creation
- wallet address resolution
- reverse resolution
- content and website records
- cross-chain address records
- official name verification

The domain system should use registry and resolver contracts.

### Decentralized VPN Module

Purpose:

- decentralized VPN provider registry
- provider reputation
- payment settlement
- staking and slashing hooks
- privacy-preserving user experience

This module must remain separate from validator consensus so VPN traffic cannot affect block production or finality.
