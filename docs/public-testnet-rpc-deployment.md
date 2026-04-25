# Public Testnet RPC Deployment

This guide launches the first public INFI Testnet RPC endpoint from the current prototype node.

The goal is to make the RPC reachable over HTTPS so wallets and the website can test:

- `eth_chainId`
- `web3_clientVersion`
- `eth_blockNumber`
- `eth_getBalance`
- `eth_getTransactionCount`
- `eth_getTransactionReceipt`
- temporary dev-only `eth_sendRawTransaction`
- `GET /faucet/status?address=0x...`
- `POST /faucet/claim`

Important: this is still a prototype public RPC, not a decentralized production testnet. Real Ethereum signed transaction decoding, production database storage, validator networking, rate limiting, and INFI Scan indexing still need to be completed before a community testnet announcement.

## Network Metadata

```text
Network name: INFI Testnet
Chain ID: 98402
Chain ID hex: 0x18062
Native currency: test InvertX
Symbol: tINVX
Decimals: 18
Current public RPC URL: https://infi-testnet-rpc.onrender.com
Future custom RPC target: https://rpc.infi.infi
Health endpoint: /health
```

test InvertX is non-tradable testnet gas with no redeemable real-world value. The `1 test InvertX = 1.25 USDT` display is a testnet UI reference only.

## Local Public-Bind Test

Run the node locally but bind it like a server:

```bash
INFI_RPC_BIND=0.0.0.0:8545 cargo run -p infi-node
```

Check health:

```bash
curl -s http://127.0.0.1:8545/health
```

Expected:

```json
{"status":"ok","chain":"INFI Testnet","chainId":"0x18062","clientVersion":"infi-devnet/0.1.0"}
```

Check chain ID:

```bash
curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}'
```

Expected:

```json
{"jsonrpc":"2.0","id":1,"result":"0x18062"}
```

Check faucet status:

```bash
curl -s "http://127.0.0.1:8545/faucet/status?address=0x3333333333333333333333333333333333333333"
```

Claim 1,000 test InvertX:

```bash
curl -s -X POST http://127.0.0.1:8545/faucet/claim \
  -H "Content-Type: application/json" \
  -d '{"address":"0x3333333333333333333333333333333333333333","amount":"1000000000000000000000"}'
```

The faucet enforces:

- maximum single claim: `10,000 test InvertX`
- monthly wallet allowance: `100,000 test InvertX`
- warning: test InvertX is non-tradable and has no redeemable real-world value

## Docker Test

Build the image:

```bash
docker build -t infi-testnet-rpc .
```

Run it locally:

```bash
docker run --rm -p 8545:8545 -e INFI_RPC_BIND=0.0.0.0:8545 infi-testnet-rpc
```

Then test:

```bash
curl -s http://127.0.0.1:8545/health
```

## Render Deployment

This repository includes `render.yaml`, a Render Blueprint for a Docker web service.

The Blueprint uses:

- runtime: `docker`
- health check path: `/health`
- region: `frankfurt`
- plan: `free`
- data directory: `INFI_DATA_DIR=/home/infi/infi-data`

Render documentation confirms Docker services can build from a repo Dockerfile, Blueprint services use `runtime: docker`, and `healthCheckPath` defines the web-service health endpoint.

The node writes chain and faucet state to `INFI_DATA_DIR`. For state to survive host replacement or redeploys, the host must attach a real persistent disk to that path. A normal free ephemeral container can still lose data when the service is replaced.

Steps:

1. Push the latest repo to GitHub.
2. Open Render.
3. Create a new Blueprint.
4. Select `infiblockchain-dev/infi-blockchain`.
5. Confirm the service named `infi-testnet-rpc`.
6. Deploy.
7. Wait until the service health check passes.
8. Copy the Render HTTPS URL:

```text
https://infi-testnet-rpc.onrender.com
```

9. Test it:

```bash
curl -s https://YOUR_RENDER_URL/health
curl -s -X POST https://YOUR_RENDER_URL \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}'
```

Current verified public endpoint:

```bash
curl -s https://infi-testnet-rpc.onrender.com/health
curl -s -X POST https://infi-testnet-rpc.onrender.com \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}'
```

Expected:

```json
{"jsonrpc":"2.0","id":1,"result":"0x18062"}
```

## Custom RPC Domain

To use:

```text
https://rpc.infi.infi
```

configure the custom domain in the host provider, then add the DNS record requested by the provider.

Do not update the website wallet config to `https://rpc.infi.infi` as "live" until the domain responds with:

```json
{"jsonrpc":"2.0","id":1,"result":"0x18062"}
```

## Required Before Public Announcement

Before inviting outside users:

- attach a persistent disk or move state to managed database storage
- add real Ethereum signed transaction decoding
- add request rate limiting/proxy protection
- deploy INFI Scan testnet
- run at least three independent nodes
- publish a testnet status page
- publish known limitations and no-real-value warnings
