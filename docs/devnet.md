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
- seeds two devnet accounts
- executes one native test InvertX transfer
- finalizes block `#1`
- prints updated balances
- starts JSON-RPC on `127.0.0.1:8545`

## Devnet Accounts

```text
Alice: 0x1111111111111111111111111111111111111111
Bob:   0x2222222222222222222222222222222222222222
```

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
- No persistent database yet
- No validator networking yet
- No real EVM interpreter yet

These are deliberate first-pass limits. The repository now has the shape needed to add those pieces cleanly.
