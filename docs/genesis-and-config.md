# Genesis and Configuration

Genesis defines the first state of the INFI chain.

## Devnet Genesis

The current devnet genesis file is:

```text
specs/genesis.devnet.json
```

It contains:

- chain name
- chain ID
- native token metadata
- initial accounts
- validator list

## Genesis Rules

Genesis must be:

- deterministic
- public
- versioned
- signed before mainnet
- verified by validators before launch
- referenced by hash in release notes

## Mainnet Genesis Requirements

Mainnet genesis must include:

- chain ID
- native token metadata
- initial account allocations
- initial validator set or validator activation rules
- staking parameters
- slashing parameters
- governance parameters
- upgrade parameters

## Config File Requirements

Future node config should include:

```toml
[chain]
genesis = "./genesis.mainnet.json"

[storage]
data_dir = "/var/lib/infi"

[rpc]
enabled = true
bind = "127.0.0.1:8545"

[p2p]
enabled = true
bind = "0.0.0.0:30303"

[metrics]
enabled = true
bind = "127.0.0.1:9090"
```

## Chain ID Safety

The chain ID protects users from replay attacks across EVM chains.

Wallets and nodes must agree on the same chain ID.

Mainnet chain ID must be unique before launch.

