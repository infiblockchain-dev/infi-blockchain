# INFI Node Operator Guide

This guide is for people running INFI infrastructure.

## Node Types

### Full Node

A full node verifies chain state and serves RPC.

### Validator Node

A validator participates in consensus and block finality.

### Archive Node

An archive node stores complete historical state and is useful for explorers and analytics.

### Indexer Node

An indexer reads chain data and writes query-friendly data for INFI Scan.

## Current Prototype

The current prototype runs a local in-process devnet node only.

## Future Node Requirements

A production node should support:

- config file
- genesis file
- data directory
- RPC bind address
- peer-to-peer bind address
- validator key path
- logging config
- metrics endpoint
- graceful shutdown
- database recovery

## Example Future Command

```bash
infi-node \
  --chain ./genesis.mainnet.json \
  --data-dir /var/lib/infi \
  --rpc 0.0.0.0:8545 \
  --p2p 0.0.0.0:30303
```

## Operator Safety

- keep validator keys offline where possible
- use firewalls
- expose public RPC through a proxy
- rate-limit public RPC
- monitor disk usage
- monitor memory usage
- monitor peer count
- monitor block height
- monitor missed blocks
- keep backups
- test restore procedure

## Public RPC Rules

Public RPC endpoints should:

- use HTTPS
- rate-limit requests
- block abusive clients
- avoid exposing debug/admin methods
- publish privacy policy
- publish uptime status

