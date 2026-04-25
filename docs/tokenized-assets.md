# INFI Tokenized Assets

INFI should include special functions for decentralizing tokenized assets.

## Purpose

Tokenized assets represent real or digital value on-chain.

Examples:

- real-world assets
- securities-like assets, where legally allowed
- invoices
- carbon credits
- property records
- loyalty points
- game assets
- identity-bound credentials
- digital collectibles
- cross-chain wrapped assets

## Design Principle

INFI should not make tokenized assets depend on one centralized issuer, server, database, or admin wallet.

The goal is:

- transparent ownership
- auditable issuance
- clear transfer rules
- decentralized custody where possible
- verifiable metadata
- compliance hooks where required
- no hidden asset manipulation

## Asset Standards

INFI should support common EVM token standards:

- ERC-20-style fungible assets
- ERC-721-style non-fungible assets
- ERC-1155-style multi-token assets
- account-bound or soulbound credentials, where needed
- wrapped cross-chain asset representations

INFI can later define INFI-native asset extensions for stronger decentralization and transparency.

## Required Asset Registry

INFI should eventually provide an on-chain asset registry.

The registry should track:

- asset contract address
- asset name
- symbol
- decimals, if fungible
- issuer address
- metadata URI
- verification status
- risk flags
- legal/compliance category, if applicable
- bridge origin, if cross-chain

## Decentralized Metadata

Asset metadata should avoid depending only on centralized servers.

Preferred metadata options:

- IPFS
- Arweave
- content-addressed storage
- hash-committed HTTPS fallback

Every asset should expose metadata hashes so users can verify that important asset details were not silently changed.

## Issuer Transparency

INFI Scan should show:

- issuer address
- mint history
- burn history
- admin permissions
- upgrade permissions
- pause/freeze permissions
- verification documents, where applicable
- metadata change history

## Custody and Control

Tokenized assets should clearly show who controls what.

INFI Scan should display warnings when an asset has:

- mint authority
- freeze authority
- blacklist authority
- upgradeable proxy admin
- centralized oracle dependency
- centralized metadata dependency
- bridge custody dependency

## Real-World Asset Safety

For real-world assets, INFI should separate technical ownership from legal claims.

Docs and user interfaces must explain:

- what the token represents
- who the issuer is
- what rights the token gives
- what legal jurisdiction applies
- whether redemption exists
- what happens if the issuer fails
- what off-chain dependency exists

## Developer Requirements

Developers building tokenized assets should:

- publish contract source
- publish metadata schema
- disclose admin permissions
- disclose oracle dependencies
- disclose upgradeability
- disclose issuer identity where legally required
- use audited token contracts
- provide emergency procedures

## INFI Scan Requirements

INFI Scan should include tokenized asset pages:

- asset overview
- holders
- transfers
- mint/burn history
- metadata
- issuer
- permissions
- verification status
- risk warnings

## Mainnet Rule

Tokenized asset functionality should not be marketed as safe until:

- asset contracts are audited
- INFI Scan can display permissions and risks
- metadata verification works
- issuer transparency rules are published
- legal/compliance requirements are reviewed

