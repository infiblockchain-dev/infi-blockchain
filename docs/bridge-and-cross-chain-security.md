# Bridge and Cross-Chain Security

Cross-chain systems are high-risk. INFI should not launch bridges casually.

## Principle

Bridge security is chain security.

If a bridge is compromised, users can lose funds even if the base chain works correctly.

## Bridge Types

Possible models:

- lock-and-mint bridge
- burn-and-mint bridge
- liquidity bridge
- light-client bridge
- validator-attested bridge
- third-party bridge integration

## Requirements Before Launching a Bridge

- threat model
- external audit
- replay protection
- rate limits
- emergency pause process
- transparent signer or validator set
- monitoring
- incident response plan
- INFI Scan visibility

## Cross-Chain Message Data

INFI Scan should show:

- source chain
- destination chain
- source transaction
- destination transaction
- bridge status
- message ID
- finality status
- fees

## Non-EVM Adapters

Non-EVM support should be built with adapter modules.

Adapters should define:

- address format
- signing scheme
- finality assumptions
- transaction proof format
- message format
- failure handling

## User Warnings

Bridge UIs must explain:

- estimated time
- fees
- finality assumptions
- bridge risks
- destination address format
- what to do if a transfer is delayed

