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
- `eth_syncing`
- `eth_accounts`
- `eth_blockNumber`
- `eth_gasPrice`
- `eth_maxPriorityFeePerGas`
- `eth_feeHistory`
- `eth_estimateGas`
- `eth_getBalance`
- `eth_getBlockByNumber`
- `eth_getBlockByHash`
- `eth_getBlockTransactionCountByNumber`
- `eth_getBlockTransactionCountByHash`
- `eth_getTransactionCount`
- `eth_getTransactionByHash`
- `eth_getTransactionByBlockNumberAndIndex`
- `eth_getTransactionByBlockHashAndIndex`
- `eth_getTransactionReceipt`
- `eth_getCode`
- `eth_getStorageAt`
- `eth_call`
- `eth_getLogs`
- `GET /health`
- `GET /faucet/status?address=0x...`
- `GET /faucet/history?limit=100`
- `GET /faucet/history?limit=100&address=0x...`
- `POST /faucet/claim`

The wallet-compatibility methods above are prototype responses so wallets such as MetaMask can recognize and inspect INFI Testnet. They are not a replacement for full production EVM execution, signed Ethereum transaction decoding, contract storage, event indexing, or finalized fee-market logic.

Prototype dev transaction format:

- `eth_sendRawTransaction` accepts a temporary dev-only hex payload.
- Real Ethereum signed transaction decoding is still pending.
- Do not use the dev payload format on public testnet or mainnet.

Current local RPC URL:

```text
http://127.0.0.1:8545
```

Current public-testnet target:

```text
Network: INFI Testnet
Chain ID: 98402
Chain ID hex: 0x18062
Native currency: test InvertX (tINVX)
Current public RPC: https://infi-testnet-rpc.onrender.com
Future custom RPC target: https://rpc.infi.infi
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

Submit a dev transfer transaction:

The temporary raw payload format is:

```text
infi:transfer:<from>:<to>:<value_wei>:<nonce>
```

Example payload:

```text
infi:transfer:0x2222222222222222222222222222222222222222:0x1111111111111111111111111111111111111111:1000000000000000000:0
```

Convert it to hex:

```bash
RAW_TX=$(printf 'infi:transfer:0x2222222222222222222222222222222222222222:0x1111111111111111111111111111111111111111:1000000000000000000:0' | xxd -p -c 256)
```

Submit it:

```bash
curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_sendRawTransaction\",\"params\":[\"0x$RAW_TX\"]}"
```

Expected result:

```json
{"jsonrpc":"2.0","id":1,"result":"0xTRANSACTION_HASH"}
```

Then query the receipt with that hash using `eth_getTransactionReceipt`.

Check faucet allowance:

```bash
curl -s "http://127.0.0.1:8545/faucet/status?address=0x3333333333333333333333333333333333333333"
```

List current and previous faucet mint transactions:

```bash
curl -s "http://127.0.0.1:8545/faucet/history?limit=100"
```

Filter faucet history by wallet address:

```bash
curl -s "http://127.0.0.1:8545/faucet/history?limit=100&address=0x3333333333333333333333333333333333333333"
```

Claim 1,000 test InvertX from the faucet:

```bash
curl -s -X POST http://127.0.0.1:8545/faucet/claim \
  -H "Content-Type: application/json" \
  -d '{"address":"0x3333333333333333333333333333333333333333","amount":"1000000000000000000000"}'
```

The faucet endpoint enforces a `10,000 test InvertX` maximum per claim and a `100,000 test InvertX` monthly cap per wallet in the running RPC process. Faucet claim and history responses expose transaction hash, wallet address, minting time, and block number for public transparency.

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
Name: test InvertX
Symbol: tINVX
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
