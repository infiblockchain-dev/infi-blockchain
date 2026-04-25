# INFI Decentralized VPN Function

INFI should include a future decentralized VPN function.

## Purpose

The INFI VPN function should let users access privacy-preserving network relay services through decentralized infrastructure.

The goal is:

- user privacy
- censorship resistance
- transparent node reputation
- decentralized provider discovery
- payment in Invertx or approved assets
- safety controls against abuse

## Important Safety Note

A VPN function can be misused.

INFI should design this feature with:

- abuse prevention
- legal awareness
- node operator controls
- transparent terms
- user privacy
- no secret backdoors
- no false anonymity claims

INFI should not promise perfect anonymity.

## Architecture

The VPN function should be separate from core consensus.

```text
INFI Chain
  registry contracts
  payment settlement
  reputation records
  staking/slashing hooks

VPN Network
  provider nodes
  relay sessions
  encrypted tunnels
  bandwidth metering
  client apps
```

This keeps validator consensus from depending on VPN traffic.

## Node Roles

### VPN Provider Node

Offers bandwidth to users.

### VPN Client

Connects to provider nodes.

### Registry Contract

Tracks provider metadata, stake, region, pricing, uptime, and reputation.

### Payment Contract

Handles escrow, streaming payments, refunds, and provider settlement.

## Provider Registry

The registry should track:

- provider address
- endpoint metadata
- public key
- supported regions
- price per bandwidth or time
- uptime
- reputation score
- stake
- slashing history
- terms of service hash

## Payments

Possible payment models:

- pay per session
- pay per bandwidth
- prepaid escrow
- subscription NFT or pass
- streaming micropayments

Payments should be transparent but should avoid leaking unnecessary browsing details on-chain.

## Privacy Requirements

The VPN function should:

- encrypt traffic between client and provider
- avoid publishing browsing destinations on-chain
- minimize logs
- make provider logging policies visible
- support rotating providers
- avoid linking wallet identity to browsing activity where possible

## Abuse Prevention

Provider nodes should be able to define acceptable-use policies.

The protocol should support:

- provider opt-in
- provider region choice
- provider policy disclosure
- staking or reputation penalties for malicious providers
- reporting and dispute mechanisms
- rate limits
- denial-of-service protections

## INFI Scan Requirements

INFI Scan should show VPN network data without exposing user browsing activity:

- provider list
- provider reputation
- uptime
- price
- region
- stake
- slashing events
- public terms hash
- aggregate network capacity

INFI Scan must not show private user traffic details.

## Mainnet Rule

Do not launch the VPN function until:

- threat model is complete
- privacy review is complete
- abuse prevention design is complete
- provider legal/compliance review is complete
- client security review is complete
- payment contracts are audited
- user warnings are clear

