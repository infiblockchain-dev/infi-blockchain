# INFI Ethereum RPC Plan

## First RPC Methods

Implement these first:

- `web3_clientVersion`
- `eth_chainId`
- `eth_blockNumber`
- `eth_getBalance`
- `eth_getTransactionCount`
- `eth_sendRawTransaction`
- `eth_getTransactionReceipt`

Current implemented prototype methods:

- `web3_clientVersion`
- `eth_chainId`
- `net_version`
- `eth_blockNumber`
- `eth_getBalance`
- `eth_getTransactionCount`
- `eth_getTransactionReceipt`

Prototype placeholder:

- `eth_sendRawTransaction` returns a clear not-implemented error until signed Ethereum transaction decoding is added.

Current local RPC URL:

```text
http://127.0.0.1:8545
```

## Local Test Commands

Start the node:

```bash
cargo run -p infi-node
```

Check client version:

```bash
curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"web3_clientVersion","params":[]}'
```

Check chain ID:

```bash
curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}'
```

Check block number:

```bash
curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_blockNumber","params":[]}'
```

Check balance:

```bash
curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_getBalance","params":["0x1111111111111111111111111111111111111111","latest"]}'
```

Check transaction count:

```bash
curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_getTransactionCount","params":["0x1111111111111111111111111111111111111111","latest"]}'
```

Check raw transaction placeholder:

```bash
curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_sendRawTransaction","params":["0x00"]}'
```

Expected placeholder error:

```json
{"jsonrpc":"2.0","id":1,"error":{"code":-32000,"message":"Raw Ethereum transaction decoding is not implemented yet"}}
```

## MetaMask Requirements

MetaMask needs:

- chain ID as hex
- RPC URL
- native currency name
- native currency symbol
- native currency decimals
- block explorer URL, once available

Current native currency:

```text
Name: Invertx
Symbol: TBD
Decimals: 18
```

## Later RPC Methods

- `eth_call`
- `eth_estimateGas`
- `eth_getBlockByNumber`
- `eth_getBlockByHash`
- `eth_getLogs`
- `eth_newFilter`
- `eth_getFilterChanges`
- `net_version`
- `net_peerCount`
