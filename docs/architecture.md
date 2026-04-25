# INFI Architecture

## Identity

INFI is the chain.

Invertx is the native gas token.

## Current Prototype Flow

```text
node -> mempool -> evm executor -> storage -> consensus -> block
```

The first devnet path is intentionally simple:

1. Seed genesis balances.
2. Accept transactions into the mempool.
3. Execute simple native Invertx transfers.
4. Propose a block with the devnet consensus engine.
5. Store the finalized block in memory.

## Crates

### `infi-primitives`

Shared domain types:

- chain config
- native token metadata
- addresses
- amounts
- accounts
- transactions
- blocks
- hashes

### `infi-storage`

State and chain storage.

The first implementation is in-memory. Later this should gain:

- RocksDB or MDBX persistence
- Merkle Patricia Trie state roots
- block indexes
- receipt indexes
- log indexes

### `infi-mempool`

Pending transaction queue.

Later this should gain:

- sender nonce ordering
- fee prioritization
- replacement rules
- spam limits
- peer gossip

### `infi-evm`

Execution adapter.

Today it handles only native Invertx transfers. Later this should wrap `revm` for real EVM execution.

### `infi-consensus`

Devnet block proposal.

Today it is a single-validator finality placeholder. Later this should become validator consensus with voting, finality certificates, networking, staking, and slashing.

### `infi-rpc`

Ethereum RPC compatibility layer.

Today it exposes metadata helpers. Later this should expose a JSON-RPC server with Ethereum-compatible methods.

### `infi-node`

Runnable node binary.

The node wires all other crates together.

## Next Engineering Milestones

1. Install Rust and verify the workspace with `cargo check`.
2. Add real JSON-RPC server support.
3. Implement `eth_chainId`, `eth_blockNumber`, and `eth_getBalance`.
4. Add signed Ethereum transaction decoding.
5. Add `eth_sendRawTransaction`.
6. Replace transfer-only execution with `revm`.
7. Add persistent storage.
8. Add multi-validator devnet networking.

