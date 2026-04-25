# INFI Safety and Transparency Requirements

INFI must prioritize safety, transparency, and a good experience for new users.

## Security Position

No serious blockchain can promise that it cannot be hacked.

INFI should instead promise:

- open code
- simple architecture where possible
- strong tests
- external audits
- transparent risks
- fast incident response
- no hidden admin powers
- public monitoring
- responsible launch process

## Safety Principles

- minimize trusted parties
- minimize privileged keys
- separate validator, RPC, indexer, bridge, and governance permissions
- require multisig or governance approval for privileged actions before full decentralization
- remove emergency powers or put them under transparent governance before mainnet maturity
- use reproducible builds
- publish signed releases
- publish checksums
- run bug bounties before mainnet

## Core Threats

Track these from the beginning:

- consensus failure
- double signing
- validator key compromise
- bridge compromise
- RPC phishing
- fake explorer URLs
- fake or lookalike INFI domains
- malicious chain metadata
- wallet-draining dapps
- contract bugs
- governance capture
- database corruption
- denial-of-service attacks

## Required Testing

- unit tests
- integration tests
- EVM compatibility tests
- JSON-RPC compatibility tests
- transaction replay tests
- consensus safety tests
- validator restart tests
- database recovery tests
- explorer indexer consistency tests
- wallet import tests
- WalletConnect QR and mobile deep-link tests
- bridge tests before any bridge launch

## Required Audits

Before mainnet:

- consensus audit
- EVM execution integration audit
- staking and slashing audit
- bridge audit, if bridge exists at launch
- governance audit
- wallet connection and dapp security review
- infrastructure review

## Transparency Requirements

Publish:

- chain ID
- genesis file
- validator set at genesis
- native token information
- RPC endpoints
- explorer URL
- node release checksums
- audit reports
- known risks
- upgrade process
- incident response process
- governance process

## New User Experience

New users should be able to:

- open INFI Scan without connecting a wallet
- search a transaction hash
- add INFI to a wallet with one button
- connect with QR code on mobile
- see what Invertx is used for
- understand gas fees before signing
- see clear transaction status
- see clear warnings for risky actions
- find manual wallet setup instructions

## Product Rule

Do not make users guess.

Every important action should answer:

- what is happening
- why it is needed
- what it costs
- what can go wrong
- how to verify it
