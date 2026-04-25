# INFI Decentralization Requirements

INFI must be fully decentralized before mainnet.

## Decentralization Goals

- independent validators
- public node software
- permissionless node operation
- transparent validator set
- open RPC access
- public explorer
- open governance
- auditable upgrades
- reproducible builds

## Validator Decentralization

INFI needs:

- many validators across different operators
- validators in different countries and data centers
- no single operator with majority voting power
- documented hardware requirements
- documented key management
- staking and delegation
- slashing for malicious behavior

## Node Types

### Validator Node

Participates in consensus and finality.

### Full Node

Verifies blocks and serves RPC without proposing blocks.

### Archive Node

Stores complete history for explorers, analytics, and audits.

### INFI Scan Indexer Node

Indexes blocks, transactions, logs, contracts, validators, and addresses for explorer queries.

### SbSe Governance Engine Node

Future node type for governance logic execution.

## Mainnet Gate

Mainnet should not launch until:

- at least several independent validator operators are ready
- validator software is documented
- validator keys can be securely generated and rotated
- staking and slashing logic is tested
- public RPC infrastructure exists
- INFI Scan is live
- governance process is published
- upgrade process is tested

