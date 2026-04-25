# INFI Website

This folder contains the static INFI website for the current prototype and
testnet launch flow.

## Pages

- `index.html` - investor and developer landing page with animated INFI multichain hero
- `testnet.html` - one-click wallet import and network switching page
- `faucet.html` - public testnet faucet UI for free test InvertX

## Brand Assets

The supplied logo image was converted into:

- `assets/brand/infi-logo.png`
- `assets/brand/infi-logo-transparent.png`
- `assets/brand/infi-social-preview.png`
- `assets/icons/favicon-16.png`
- `assets/icons/favicon-32.png`
- `assets/icons/apple-touch-icon.png`
- `assets/icons/icon-192.png`
- `assets/icons/icon-512.png`
- `assets/tokens/invertx-logo.png`
- `assets/tokens/invertx-logo-transparent.png`
- `assets/tokens/invertx-icon-512.png`
- `assets/tokens/invertx-icon-256.png`
- `assets/tokens/invertx-icon-128.png`
- `assets/tokens/invertx-icon-64.png`
- `assets/tokens/invertx-favicon-32.png`
- `assets/tokens/invertx-favicon-16.png`

## SEO Setup

The site includes:

- page title
- meta description
- canonical URL
- robots directives
- Open Graph tags
- Twitter Card tags
- JSON-LD organization schema
- JSON-LD website schema
- web app manifest
- favicon links
- `robots.txt`
- `sitemap.xml`

The public canonical URLs are currently written for the planned official
domains. Until `.infi` resolution is live in normal browsers, Cloudflare Pages
will remain the practical hosted preview/deployment URL.

## Testnet Wallet Page

The testnet wallet setup page is:

- `testnet.html`
- `faucet.html`

It includes:

- one-click `wallet_addEthereumChain`
- one-click `wallet_switchEthereumChain`
- desktop and laptop injected wallet support
- mobile and tablet wallet browser support
- mobile links for MetaMask, Coinbase Wallet, and Trust Wallet
- copyable full setup and manual fields
- chain JSON fallback at `assets/chains/infi-testnet.json`
- test InvertX logo
- InvertX reference value display for testnet
- no-real-world-value warning
- manual wallet setup details

The faucet page calls the public INFI Testnet RPC faucet endpoints:

- `GET /faucet/status?address=0x...`
- `POST /faucet/claim`

The prototype enforces a `100,000 test InvertX` monthly wallet cap in the RPC process and limits each claim to `10,000 test InvertX`. Faucet claim history persists through `INFI_DATA_DIR` when the host preserves that directory. Stronger abuse controls are still required before a broader community testnet announcement.

## Official Domains

- `https://infi.infi/`
- `https://scan.infi.infi/`

Important: `.infi` requires a real DNS/resolution strategy before it will work in normal public browsers.

## Public Disclosure Boundary

The public website should focus on decentralization, transparency, wallet
onboarding, testnet safety, and user experience. Internal core concepts,
security playbooks, private operational details, and sensitive data should not
be published in public website copy.
