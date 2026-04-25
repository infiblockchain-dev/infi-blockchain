# INFI Scan Indexer Guide

INFI Scan needs an indexer because explorer pages should not depend on expensive raw node queries.

## Purpose

The indexer turns chain data into searchable data.

It should index:

- blocks
- transactions
- receipts
- logs
- addresses
- contracts
- validators
- token transfers
- tokenized asset metadata
- tokenized asset permissions
- domain records
- domain ownership history
- cross-chain messages later
- governance activity later
- VPN provider records later

## Recommended Components

```text
INFI node JSON-RPC
  -> indexer worker
  -> PostgreSQL
  -> explorer API
  -> INFI Scan frontend
```

## Indexing Flow

1. read latest finalized block
2. fetch block data
3. fetch receipts
4. decode logs
5. update address balances and counters
6. write indexed records
7. expose search API

## Consistency Rules

- index only finalized blocks when possible
- detect reorgs if non-finalized data is indexed
- store block hash with every indexed record
- make indexing idempotent
- track indexer height
- expose indexer lag

## Search Types

INFI Scan search should detect:

- transaction hash
- block hash
- block number
- address
- contract
- token
- tokenized asset
- INFI domain
- VPN provider, later

## Operational Metrics

Track:

- latest indexed block
- latest finalized block
- indexer lag
- failed RPC calls
- database write errors
- queue depth
- average indexing time per block
