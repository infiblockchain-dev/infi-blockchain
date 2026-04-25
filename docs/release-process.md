# INFI Release Process

INFI releases must be predictable, signed, and transparent.

## Release Types

### Devnet Release

For local development and early testing.

### Testnet Release

For public or private testnet validators.

### Mainnet Candidate

For final launch rehearsal.

### Mainnet Release

For production network use.

## Release Checklist

Before publishing:

- update version
- run tests
- run formatting
- run linting
- build binaries
- generate checksums
- sign release artifacts
- publish release notes
- document known issues
- document upgrade instructions

## Release Notes Must Include

- version
- date
- supported networks
- required upgrade or optional upgrade
- breaking changes
- security fixes
- database migration notes
- config changes
- checksum list

## Mainnet Release Rule

Mainnet releases must not be rushed. Security fixes can move quickly, but they still need verification, signed artifacts, and clear operator instructions.

