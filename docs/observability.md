# INFI Observability and Status

INFI infrastructure needs public and private monitoring.

## Node Metrics

Track:

- latest block
- finalized block
- peer count
- mempool size
- RPC request count
- RPC error count
- database size
- disk usage
- memory usage
- CPU usage
- restart count

## Validator Metrics

Track:

- blocks proposed
- blocks missed
- voting participation
- validator uptime
- validator power
- slashing events
- peer count
- consensus round time

## Explorer Metrics

Track:

- latest indexed block
- indexer lag
- failed indexing jobs
- API latency
- search latency
- database errors

## Public Status Page

INFI should publish a status page with:

- chain status
- RPC status
- INFI Scan status
- indexer lag
- known incidents
- maintenance windows

## Alerts

Alert on:

- block production halt
- finality halt
- high missed-block rate
- validator offline
- RPC error spike
- indexer lag
- disk almost full
- database corruption
- suspicious bridge activity

