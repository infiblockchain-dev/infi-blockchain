# INFI Governance Guide

Governance controls how INFI changes over time.

## Current Status

Governance is not implemented yet.

SbSe Protocol is planned as the future governance logic engine node.

## Governance Goals

- transparent proposals
- public voting
- clear upgrade rules
- parameter change process
- treasury rules, if a treasury exists
- validator governance hooks
- emergency process with public accountability

## Proposal Lifecycle

Future lifecycle:

1. draft proposal
2. public discussion
3. formal proposal
4. voting period
5. execution delay
6. on-chain execution
7. INFI Scan governance record

## Governance Safety

- avoid hidden admin powers
- use timelocks
- publish all proposal data
- make emergency actions visible
- document who can do what before full decentralization
- reduce privileged control over time

## SbSe Role

SbSe Protocol should evaluate governance logic, but validators still finalize chain state.

```text
users propose
holders or validators vote
SbSe evaluates governance rules
validators finalize governance outcome
INFI Scan displays the result
```

