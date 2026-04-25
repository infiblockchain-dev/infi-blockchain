# INFI Mainnet Launch Guide

This is the step-by-step path to launch INFI Mainnet responsibly.

Do not treat this as a shortcut. Mainnet launch is the final step after testnets, audits, monitoring, documentation, and decentralization work.

## Phase 0: Freeze the Mainnet Requirements

Finalize:

- chain name
- chain ID
- Invertx symbol
- Invertx decimals
- genesis allocation
- validator requirements
- staking model
- slashing rules
- governance model
- upgrade process
- public RPC policy
- INFI Scan domain
- faucet policy for testnet only

Before mainnet, confirm that testnet faucet minting cannot affect mainnet Invertx.

Output:

- final chain parameter document
- final genesis allocation draft
- public risk document

## Phase 1: Build a Stable Devnet

Required:

- node starts reliably
- blocks finalize
- transactions execute
- balances persist after restart
- RPC works
- INFI Scan indexes blocks and transactions
- wallet import works
- WalletConnect QR and deep links work

Exit criteria:

- devnet runs for at least 7 days without state corruption
- all critical bugs fixed

## Phase 2: Build Real EVM Compatibility

Required:

- real EVM execution through `revm` or equivalent
- contract deployment
- contract calls
- logs and events
- receipts
- gas accounting
- EIP-155 chain ID replay protection
- Foundry tests
- Hardhat tests

Exit criteria:

- ERC-20 deployment and transfer works
- common Solidity contracts work
- JSON-RPC behavior matches wallet expectations

## Phase 3: Multi-Validator Testnet

Required:

- independent validator nodes
- peer discovery
- transaction gossip
- block gossip
- finality voting
- validator key management
- validator restart recovery
- node snapshots or state sync

Exit criteria:

- at least 4 independent validators
- network continues after one validator stops
- no single validator controls finality

## Phase 4: INFI Scan Public Testnet

Required:

- public INFI Scan
- transaction search
- block search
- address search
- validator pages
- contract pages
- API for frontend queries
- indexer recovery process

Exit criteria:

- every public testnet transaction can be followed
- explorer data matches node RPC data

## Phase 5: Wallet and User Experience Testnet

Required:

- one-click add INFI network
- one-click switch to INFI
- manual setup instructions
- WalletConnect QR connection
- mobile deep linking
- tablet layout
- desktop layout
- clear signing warnings

Exit criteria:

- non-technical users can add INFI and find a transaction without help

## Phase 6: Decentralization Testnet

Required:

- documented validator onboarding
- multiple independent operators
- public full-node guide
- public archive-node guide
- public RPC guide
- validator monitoring
- slashing test cases

Exit criteria:

- several independent teams can run nodes from docs alone
- validator distribution is published

## Phase 7: Security Reviews

Required:

- internal review
- external audit
- threat model
- bug bounty
- incident response drill
- disaster recovery drill
- bridge audit if any bridge exists
- governance review
- tokenized asset module review, if enabled
- INFI Domains contract review, if enabled
- VPN module review, if enabled

Exit criteria:

- no known critical or high severity issues
- medium issues are fixed or publicly accepted with rationale

## Phase 8: Mainnet Candidate

Prepare:

- mainnet genesis file
- signed node release
- checksums
- Docker image, if used
- validator configs
- RPC configs
- INFI Scan production deployment
- status page
- monitoring dashboards
- public docs

Exit criteria:

- mainnet candidate runs as a rehearsal network
- launch team can reproduce deployment from docs

## Phase 9: Genesis Ceremony

Steps:

1. Publish final genesis file.
2. Publish final chain ID and wallet metadata.
3. Publish validator list and signatures.
4. Publish node release checksums.
5. Validators independently verify genesis.
6. Validators start nodes.
7. Confirm peer connectivity.
8. Produce first block.
9. Confirm finality.
10. Confirm INFI Scan indexing.
11. Confirm wallet import.
12. Confirm public RPC health.

Exit criteria:

- chain finalizes blocks
- explorer works
- wallets can add the network
- public RPC responds correctly

## Phase 10: Post-Launch Safety Window

First 72 hours:

- monitor block production
- monitor validator participation
- monitor RPC errors
- monitor indexer lag
- monitor chain reorgs
- monitor suspicious transactions
- publish status updates
- avoid unnecessary upgrades

First 30 days:

- publish network health reports
- publish known issues
- continue bug bounty
- onboard more validators
- prepare first governance process

## Mainnet Launch Blockers

Do not launch if:

- Rust/node client cannot run reliably
- EVM execution is incomplete
- RPC is incompatible with wallets
- INFI Scan cannot track transactions
- wallet import does not work
- validator set is centralized
- slashing is untested
- audits are missing
- known critical bugs exist
- bridge is unaudited
- tokenized asset permission visibility is missing while assets are enabled
- INFI Domains reserved names are not protected while domains are enabled
- VPN threat model is missing while VPN function is enabled
- genesis allocations are unclear
- governance rules are unclear
- incident response is untested
