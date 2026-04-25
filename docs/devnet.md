# INFI Devnet Guide

## Prerequisites

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart the terminal, then verify:

```bash
rustc --version
cargo --version
```

## Check the Workspace

```bash
cargo check
```

## Run the Prototype Node

```bash
cargo run -p infi-node
```

Expected behavior:

- starts INFI Testnet prototype mode
- prints test InvertX as the native gas token
- loads state from `INFI_DATA_DIR`, or seeds two devnet accounts if no state exists
- executes one native test InvertX transfer only when seeding the first prototype block
- finalizes block `#1` on first seed
- prints updated balances
- starts JSON-RPC on `127.0.0.1:8545`

## Persistent Data Directory

The node persists prototype chain state to:

```text
.infi-data
```

Override it with:

```bash
INFI_DATA_DIR=/path/to/infi-data cargo run -p infi-node
```

The directory contains:

- `accounts.tsv`
- `blocks.tsv`
- `faucet_claims.tsv`

These files are prototype storage, not the final production database.

To reset local prototype state, stop the node and delete the selected data directory.

## Devnet Accounts

```text
Alice: 0x1111111111111111111111111111111111111111
Bob:   0x2222222222222222222222222222222222222222
```

Bob is also the prototype faucet source account.

## Public Bind Mode

For local server-style testing:

```bash
INFI_RPC_BIND=0.0.0.0:8545 cargo run -p infi-node
```

For hosts that provide a `PORT` environment variable, the node automatically binds to:

```text
0.0.0.0:$PORT
```

Health check:

```bash
curl -s http://127.0.0.1:8545/health
```

## Current Limitations

- No real signatures yet
- No contract deployment yet
- No production database yet
- No validator networking yet
- No real EVM interpreter yet

These are deliberate first-pass limits. The repository now has the shape needed to add those pieces cleanly.
