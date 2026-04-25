# Contributing to INFI

Contributions should make INFI safer, clearer, faster, or easier to use.

## Before You Start

Read:

- [Architecture](architecture.md)
- [Safety and transparency](safety-transparency.md)
- [Testing strategy](testing.md)
- [Developer quickstart](developer-quickstart.md)

## Development Workflow

1. create a focused branch
2. make a small change
3. add or update tests
4. update docs if behavior changes
5. run checks
6. open a pull request

## Code Principles

- prefer simple code
- avoid hidden global state
- avoid unsafe cryptography
- keep consensus behavior deterministic
- make errors clear
- keep public APIs stable
- document security-sensitive behavior

## Pull Request Checklist

- tests added or updated
- docs added or updated
- security impact considered
- backward compatibility considered
- no unrelated refactors
- no secrets committed

## Security Issues

Do not publish exploitable security issues publicly before maintainers can respond.

The project should create a dedicated security disclosure process before public testnet.

