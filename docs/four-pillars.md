# INFI Four Pillars

INFI Blockchain is built on four pillars: **decentralization**, **safety**, **speed**, and **transparency**. Every protocol decision, software release, and operational practice is evaluated against them.

## 1. Decentralization

Decentralization is a design choice at every layer, not a marketing slogan.

- **Open validator set** — anyone meeting the technical requirements can run a validator, propose blocks, and earn from honest participation. No central operator, no permissioned gatekeepers.
- **Public node software** — validator, full, archive, and indexer node software is open-source. Anyone can build, audit, fork, or run their own.
- **Open RPC access** — public RPC endpoints are available, and operators are encouraged to host their own.
- **Open governance** — protocol upgrades, parameter changes, and treasury actions go through a public, auditable governance process.
- **No hidden admin powers** — privileged actions, where they exist during early testnet, are documented and removed or moved under transparent governance before mainnet maturity.

See [Decentralization Requirements](decentralization.md) for the full mainnet gate.

## 2. Safety

Safety is earned through proof, audits, and accountable processes—not promises.

- **Reproducible builds** — release binaries can be rebuilt from source by anyone, with matching checksums.
- **External audits** — consensus, EVM execution, staking/slashing, governance, and bridges (if any at launch) are all audited before mainnet.
- **Public bug bounties** — independent researchers are paid to find and report issues.
- **Transparent incident response** — when something goes wrong, INFI publishes a public post-mortem, mitigation steps, and timeline.
- **Honest security wording** — INFI does not claim to be unhackable. INFI commits to open code, strong tests, audits, and fast disclosure.

See [Safety and Transparency](safety-transparency.md) for the threat model, required testing, and required audits.

## 3. Speed

Speed without shortcuts. Fast where it matters, careful where it counts.

- **Short block targets** — block production targets a few seconds, keeping confirmation latency low for everyday transactions.
- **BFT-confirmed finality** — once a block is finalized by the validator set, it is final. No probabilistic reorg windows.
- **Lightweight Invertx gas** — the native gas unit (Invertx) is sized for everyday use. Cheap transactions, predictable fees.
- **EVM compatibility** — existing Solidity and Vyper contracts deploy without modification. Familiar tooling, no new toolchain to learn.
- **Efficient state layout** — the data layer (Merkle trie, headers, receipts) is structured for fast read/write paths in node software and indexers.

See [Protocol Specification](protocol-spec.md) and [Architecture](architecture.md) for the technical details.

## 4. Transparency

Transparency means the network's full state is public by default and verifiable by anyone.

- **INFI Scan** — every block, transaction, validator, contract, token, and INFI Domain is searchable, indexable, and verifiable through the public explorer.
- **Public chain metadata** — chain ID, genesis file, validator set at genesis, RPC endpoints, and node release checksums are all published.
- **Open audit trail** — audit reports, known risks, the upgrade process, governance decisions, and incident reports are all public.
- **Verifiable claims** — every public statement about the network can be checked against on-chain or open-source evidence.
- **Clear testnet labels** — while INFI is in testnet, this is communicated clearly in every relevant interface so no one mistakes test gas for real value.

See [INFI Scan](infi-scan.md) and [Safety and Transparency](safety-transparency.md) for the full transparency requirements.

## How These Pillars Map to the Codebase

| Pillar | Relevant code/docs |
| --- | --- |
| Decentralization | `crates/consensus`, `crates/node`, `docs/decentralization.md`, `docs/validator-guide.md` |
| Safety | `crates/primitives`, `docs/safety-transparency.md`, `docs/testing.md`, `docs/release-process.md` |
| Speed | `crates/evm`, `crates/mempool`, `crates/storage`, `docs/protocol-spec.md` |
| Transparency | `crates/rpc`, `docs/infi-scan.md`, `docs/rpc.md`, `docs/observability.md` |

## Pillar Trade-offs

The pillars are mutually reinforcing in the long run, but they trade off in the short term. INFI's design priorities, in order:

1. **Safety** before speed. A correct, audited slow path beats a fast broken path.
2. **Decentralization** before convenience. INFI accepts more operational complexity to keep the validator set, RPC layer, and governance open.
3. **Transparency** before polish. INFI publishes "in-progress" status, known limitations, and incident reports rather than hiding them.
4. **Speed** as the constant pressure. Within the constraints above, every release should be measurably faster, lighter, or more efficient than the last.

## Honest Status

INFI is in testnet. Not all four pillars are fully realized yet:

- Decentralization: prototype testnet runs on a small operator set; broad validator decentralization is a mainnet-launch requirement.
- Safety: external audits are scheduled before mainnet; bug bounty is in preparation.
- Speed: real EVM execution and production storage are being implemented; current performance numbers are not final.
- Transparency: INFI Scan is in development; current data is published from the testnet RPC and node software.

The mainnet launch gate in [`mainnet-launch.md`](mainnet-launch.md) describes what must be true before INFI moves out of testnet.
