# INFI Testnet Faucet

The INFI Testnet faucet distributes free test InvertX tokens.

## Token Identity

```text
Name: test InvertX
Symbol: tInvertX
Decimals: 18
Network: INFI Testnet only
Real value: none
Logo: https://infi.infi/assets/tokens/invertx-icon-512.png
```

## Displayed Reference Value

Testnet UIs may display:

```text
1 test InvertX = 1.25 USDT
```

This is the InvertX reference value displayed in testnet wallets and explorers.

For test InvertX, it is not:

- a market price
- a redemption guarantee
- an exchange rate
- a financial promise
- a mainnet token valuation
- permission to trade test tokens

Required UI label:

```text
InvertX reference display for testnet only. test InvertX is non-tradable and has no redeemable real-world value.
```

## Monthly Mint Limit

Maximum per wallet:

```text
100,000 test InvertX per calendar month
```

This applies only to testnet.

## Recommended Faucet Claims

The maximum is monthly. The default claim should be smaller.

Suggested defaults:

```text
Default claim: 1,000 test InvertX
Large claim: 5,000 test InvertX
Monthly max: 100,000 test InvertX
```

## Faucet Requirements

The faucet must:

- distribute only test InvertX
- enforce the monthly wallet cap
- prevent or discourage trading, transfers for sale, and exchange listings
- rate-limit by wallet
- rate-limit by IP or session where appropriate
- show the no-real-value warning
- show remaining monthly allowance
- link every claim to INFI Scan
- log claims for abuse review
- expose faucet health status

## Anti-Abuse Rules

To keep tokens free without letting one user drain the faucet:

- one claim per wallet per time window
- monthly cap per wallet
- optional captcha
- optional GitHub/Discord/community verification later
- deny obvious scripted abuse
- publish transparent faucet limits

## Faucet Data Model

Track:

- wallet address
- claimed amount
- claim timestamp
- calendar month
- transaction hash
- claim source metadata

Never store unnecessary sensitive user data.

## Smart Contract Policy

If faucet minting is handled by a contract:

- faucet minter permission must exist only on testnet
- minter permissions must not exist in mainnet genesis
- mint events must be indexed by INFI Scan
- max monthly allowance must be enforceable or auditable

## Mainnet Separation

Mainnet Invertx must not be minted through the testnet faucet.

The faucet UI must clearly identify the connected network. If the wallet is on mainnet, faucet minting must be disabled.

## Wallet Display

Wallet and dapp UIs should show:

- test InvertX name
- tInvertX symbol
- InvertX logo
- no-real-world-value warning
- InvertX reference value display only where clearly labeled
- non-tradable testnet-only warning
