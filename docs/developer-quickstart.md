# Developer Quickstart

This guide gets a developer from zero to a local INFI prototype.

## 1. Install Rust

Install Rust from:

```text
https://rustup.rs
```

Then verify:

```bash
rustc --version
cargo --version
```

## 2. Check the Workspace

From the repository root:

```bash
cargo check
```

## 3. Run the Devnet Node

```bash
cargo run -p infi-node
```

The current node prototype:

- starts INFI Devnet
- uses Invertx as the native gas token
- seeds two accounts
- executes a sample transfer
- finalizes one block
- prints updated balances
- starts JSON-RPC at `http://127.0.0.1:8545`

## 4. Test JSON-RPC

```bash
curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}'
```

Check Alice's nonce:

```bash
curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_getTransactionCount","params":["0x1111111111111111111111111111111111111111","latest"]}'
```

Submit a temporary dev transfer from Bob to Alice:

```bash
RAW_TX=$(printf 'infi:transfer:0x2222222222222222222222222222222222222222:0x1111111111111111111111111111111111111111:1000000000000000000:0' | xxd -p -c 256)

curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_sendRawTransaction\",\"params\":[\"0x$RAW_TX\"]}"
```

This is not a real Ethereum signed transaction yet. It is a local development bridge so the RPC/faucet/explorer flow can progress.

## 5. Learn the Crates

```text
crates/primitives  shared protocol types
crates/storage     state and block storage
crates/mempool     pending transactions
crates/evm         execution adapter
crates/consensus   block proposal and finality
crates/rpc         Ethereum JSON-RPC adapter
crates/node        node binary
```

## 6. Developer Priorities

The first implementation priorities are:

1. real Ethereum signed transaction decoding
2. production-safe `eth_sendRawTransaction`
3. persistent state
4. real EVM execution
5. INFI Scan indexer
6. wallet import and WalletConnect support
7. multi-validator networking

## 7. Safety Rule

Do not use this prototype with real funds. It is development software.
