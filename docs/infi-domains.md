# INFI Domains

INFI should include an on-chain domain function.

Users should be able to create INFI domains for free.

## Purpose

INFI Domains should make blockchain addresses and resources easier to use.

Examples:

- `alice.infi`
- `wallet.infi`
- `dao.infi`
- `token.infi`
- `vpn-provider.infi`
- `scan.infi`

## Official Domains

Requested official domains:

```text
https://infi.infi/
https://scan.infi.infi/
```

Important: `.infi` names can exist on-chain before they work in normal public web browsers.

Normal browsers will not resolve `.infi` through public DNS unless one of these exists:

- an officially delegated `.infi` top-level domain
- a DNS gateway
- a browser extension
- an operating-system resolver
- wallet or dapp-level name resolution

## Free Domain Creation

INFI Domains should allow free domain creation.

To prevent spam and squatting while keeping creation free, the protocol should consider:

- one free domain per wallet at first
- optional proof-of-personhood later
- rate limits
- minimum name length
- reserved official names
- anti-phishing review for sensitive names
- expiration or renewal rules, if needed
- transparent dispute process

Free should mean no purchase price. Users may still need to pay normal gas in Invertx unless governance later subsidizes domain transactions.

## Domain Records

Each domain should support records such as:

- owner address
- resolver address
- wallet address records
- content hash
- website URL
- avatar/logo
- text records
- social links
- tokenized asset links
- VPN provider links
- cross-chain addresses

## Resolver Design

INFI should use resolver contracts so domains can point to different resource types.

Suggested model:

```text
Domain Registry
  owns names
  tracks expiration/reservation
  points each name to a resolver

Resolver Contract
  stores wallet records
  stores content records
  stores text records
  stores cross-chain records
```

## Reserved Names

Reserve official and safety-sensitive names before public launch.

Examples:

- `infi.infi`
- `scan.infi`
- `rpc.infi`
- `docs.infi`
- `wallet.infi`
- `bridge.infi`
- `governance.infi`
- `support.infi`
- `security.infi`
- `admin.infi`

## INFI Scan Support

INFI Scan should support:

- domain search
- domain owner display
- resolver records
- domain history
- expiration status, if expiration exists
- warnings for impersonation risks
- verified official domain badges

## Wallet Support

Wallets and dapps should be able to:

- resolve `name.infi` to an address
- reverse-resolve an address to a primary name
- show verified names
- warn on suspicious lookalike names
- support cross-chain address records later

## Website Resolution

For websites like `https://infi.infi/`, INFI needs a resolution strategy.

Options:

- public DNS delegation for `.infi`
- gateway such as `https://gateway-domain/infi.infi`
- browser extension that resolves `.infi`
- custom resolver app
- wallet browser support

## Security Requirements

- protect official names
- prevent invisible ownership changes
- show full domain in wallet prompts
- protect against lookalike names
- support recovery process
- publish resolver contract addresses
- make all domain changes visible in INFI Scan

## Mainnet Rule

Do not launch public free domains until:

- reserved names are protected
- domain contracts are audited
- INFI Scan can search names
- wallet resolution behavior is tested
- anti-phishing warnings exist
- official domain policy is published

