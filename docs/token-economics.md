# INFI Token Economics

This document describes native-token design requirements.

## Native Token

Invertx is the native gas token of INFI.

Current metadata:

```text
Name: Invertx
Symbol: TBD
Decimals: 18
```

## Testnet Token

INFI Testnet should use `test InvertX`.

```text
Name: test InvertX
Symbol: tINVX
Decimals: 18
Real value: none
Wallet/explorer display reference: 1 test InvertX = 1.25 USDT
Monthly faucet limit: 100,000 test InvertX per wallet
Logo: https://infi.infi/assets/tokens/invertx-icon-512.png
Trading: disabled / not tradable
```

The displayed USDT reference mirrors the intended InvertX reference value in testnet UIs. test InvertX itself is not tradable, redeemable, or a promise of value.

## Role of Invertx

Invertx is used for:

- gas fees
- validator staking, later
- delegation, later
- governance participation, if approved
- bridge fees, if applicable

## Native Token vs ERC-20

Native Invertx is not an ERC-20.

It lives directly in account state and pays gas.

A wrapped ERC-20 version can be added later:

```text
Wrapped Invertx: wINVERTX or final name TBD
```

## Fee Policy

INFI should define:

- base fee behavior
- priority fee behavior
- fee burning or distribution
- validator rewards
- spam resistance
- minimum gas price

## Supply Policy

Mainnet cannot launch until supply rules are final:

- genesis supply
- allocations
- vesting
- inflation, if any
- validator rewards
- treasury, if any
- burn rules, if any

## Transparency Requirements

Publish before mainnet:

- initial supply
- genesis allocations
- unlock schedule
- foundation/team allocations, if any
- validator reward rules
- treasury rules
- fee distribution rules
