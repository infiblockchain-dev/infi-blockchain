# INFI Website Launch Guide

This guide explains how to launch the INFI website.

The website lives in:

```text
site/
```

## Current Website Files

- `site/index.html`
- `site/styles.css`
- `site/site.webmanifest`
- `site/robots.txt`
- `site/sitemap.xml`
- `site/assets/brand/infi-logo.png`
- `site/assets/brand/infi-social-preview.png`
- `site/assets/icons/favicon-16.png`
- `site/assets/icons/favicon-32.png`
- `site/assets/icons/apple-touch-icon.png`
- `site/assets/icons/icon-192.png`
- `site/assets/icons/icon-512.png`
- `site/assets/tokens/invertx-logo.png`
- `site/assets/tokens/invertx-icon-512.png`
- `site/assets/chains/infi-testnet.json`
- `site/testnet.html`

## Step 1: Confirm the Official Domains

The requested official URLs are:

```text
Website: https://infi.infi/
Explorer: https://scan.infi.infi/
```

Future service URLs can follow the same pattern:

```text
RPC: https://rpc.infi.infi/
Docs: https://docs.infi.infi/
```

Important: `.infi` is not a normal public DNS top-level domain unless it is officially delegated or supported by a resolver/gateway/browser integration.

For normal browser access, choose one of these strategies:

- obtain an ICANN-recognized `.infi` TLD, if possible
- provide an INFI gateway on a normal public domain
- ship an INFI browser extension or resolver
- support wallet/app resolution inside INFI dapps

## Step 2: Verify Production URLs

These files now use:

```text
https://infi.infi/
https://scan.infi.infi/
```

Check:

- `site/index.html`
- `site/robots.txt`
- `site/sitemap.xml`
- `site/README.md`

## Step 3: Preview Locally

From the repository root:

```bash
cd site
python3 -m http.server 8080
```

Open:

```text
http://127.0.0.1:8080
```

Check:

- logo displays
- favicon appears in browser tab
- mobile layout works
- links work
- text is readable
- no placeholder text remains

## Step 4: Check SEO Metadata

Confirm:

- page title is correct
- meta description is correct
- canonical URL uses the real domain
- Open Graph URL uses the real domain
- Open Graph image uses the real domain
- Twitter image uses the real domain
- sitemap URL uses the real domain
- robots.txt points to the real sitemap

## Step 5: Deploy

Any static host can serve this site.

Good options:

- Cloudflare Pages
- Vercel
- Netlify
- GitHub Pages
- your own Nginx server

For a static host, set the publish directory to:

```text
site
```

## Step 6: Configure DNS

Point your domain to the hosting provider.

Typical DNS records:

```text
A / CNAME record for root domain
CNAME record for www
CNAME record for scan, later
CNAME record for docs, later
CNAME record for rpc, later
```

## Step 7: Enable HTTPS

Do not launch without HTTPS.

Wallets, SEO crawlers, and users expect secure HTTPS URLs.

## Step 8: Submit to Search Engines

After launch:

1. create Google Search Console property
2. submit `sitemap.xml`
3. create Bing Webmaster Tools property
4. submit `sitemap.xml`
5. test Open Graph preview
6. test Twitter/X card preview

## Step 9: Production Checklist

Before announcement:

- real domain is configured
- HTTPS works
- favicon works
- social preview image works
- sitemap loads
- robots.txt loads
- mobile layout works
- no placeholder domains remain
- no false mainnet claims are present
- security wording is honest

## Important

The website can launch before the blockchain mainnet.

But the website must clearly say when INFI is still in devnet/testnet and not yet mainnet.
