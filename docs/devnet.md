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

- starts INFI Devnet
- prints Invertx as the native gas token
- seeds two devnet accounts
- executes one native Invertx transfer
- finalizes block `#1`
- prints updated balances

## Devnet Accounts

```text
Alice: 0x1111111111111111111111111111111111111111
Bob:   0x2222222222222222222222222222222222222222
```

## Current Limitations

- No real signatures yet
- No JSON-RPC server yet
- No contract deployment yet
- No persistent database yet
- No validator networking yet
- No real EVM interpreter yet

These are deliberate first-pass limits. The repository now has the shape needed to add those pieces cleanly.

