# INFI Persistent Storage

INFI Testnet prototype nodes persist restart-safe state to a local data directory.

Default:

```text
.infi-data
```

Override:

```bash
INFI_DATA_DIR=/path/to/infi-data cargo run -p infi-node
```

## Files

The prototype writes dependency-free TSV files:

```text
accounts.tsv
blocks.tsv
faucet_claims.tsv
```

`accounts.tsv` stores:

- address
- native test InvertX balance
- nonce

`blocks.tsv` stores:

- block headers
- transactions

Receipts and transaction indexes are rebuilt from stored blocks when the node starts.

`faucet_claims.tsv` stores:

- wallet address
- UTC month key
- claimed amount

This allows the `100,000 test InvertX` monthly faucet cap to survive a node restart when the data directory is preserved.

## Render Deployment

The Docker image sets:

```text
INFI_DATA_DIR=/home/infi/infi-data
```

For real restart safety on Render, attach persistent disk storage to that path or move state into managed database storage. A normal ephemeral container can still lose state if the service is replaced.

## Local Reset

Stop the node, then remove the selected data directory.

Example:

```bash
rm -rf .infi-data
```

Only do this for local development. Removing public testnet state will reset balances, blocks, receipts, and faucet claim history.

## Production Direction

This file-backed layer is a prototype milestone, not final chain infrastructure.

Before a wider public testnet:

- replace TSV files with RocksDB, MDBX, or another audited embedded store
- add crash-recovery tests
- add database backups
- add schema migration/versioning
- index addresses, transactions, receipts, and logs for INFI Scan
- persist validator/node metadata once multi-node networking is added
