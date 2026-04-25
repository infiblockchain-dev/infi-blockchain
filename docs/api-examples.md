# INFI API Examples

These examples describe the target JSON-RPC behavior for INFI.

## Chain ID

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_chainId",
  "params": []
}
```

Devnet response:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x18061"
}
```

## Block Number

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_blockNumber",
  "params": []
}
```

## Get Balance

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_getBalance",
  "params": [
    "0x1111111111111111111111111111111111111111",
    "latest"
  ]
}
```

## Send Raw Transaction

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_sendRawTransaction",
  "params": ["0x..."]
}
```

## Get Transaction Receipt

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "eth_getTransactionReceipt",
  "params": ["0xTRANSACTION_HASH"]
}
```

## Error Rules

RPC errors should be:

- deterministic
- Ethereum-compatible where possible
- clear enough for dapp developers
- safe for public exposure

Do not leak private validator, node, database, or infrastructure details through RPC errors.

