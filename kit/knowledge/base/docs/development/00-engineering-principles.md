# 00 — Engineering principles

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (generic)

The invariants for {{MODULE}}. Every other chapter applies these; when they
disagree, this one wins. Exceptions are written down in
[../architecture/CONFORMANCE_LEDGER.md](../architecture/CONFORMANCE_LEDGER.md).
Add the stack-specific rules (module topology, layers, naming) as sections below.

## Module topology

Split the code into deployables, reusable core modules and vertical features.
Core never depends on features. A feature belongs to exactly one deployable.
Dependencies point one way and are checked by a build-time rule, not by review alone.

## Layers and naming

Business rules live in a layer with no framework, transport or storage types.
Adapters (HTTP, database, broker, platform) depend on that layer, never the
reverse. One public type per file. Names say what a thing is (`*Dto` for wire
shapes, `*Repository` for persistence interfaces, plain names for domain models).

## Contracts

- Contracts (APIs, events, schemas) are written first and change in the same change
  as the code.
- Versions are explicit; a breaking change is a new version.
- One error shape everywhere, with a stable code and a correlation ID.

## Security

- Fail closed. A missing, malformed or ambiguous identity, permission or
  configuration denies the operation.
- Identity comes from the authenticated principal, never from a request body.
- Secrets never enter git, logs, metrics, errors or analytics.
- Prefer platform cryptography behind one module; never hand-roll crypto.

## Design discipline (review-blocking)

| Principle | Meaning |
|---|---|
| KISS | The smallest construct that satisfies the contract; no speculative abstraction |
| DRY | One owner for every rule, shape and constant; coincidental similarity is not duplication |
| SOLID | One reason to change per type; depend on interfaces, not implementations |
| YAGNI | Only what a current contract requires; delete dead code in the change that orphans it |

## Quality gates

The commands are in [09-tools-scripts-and-commands.md](09-tools-scripts-and-commands.md).
Compile is not done: done means the gates pass and the acceptance evidence is
recorded.

## Anti-patterns reviewers reject on sight

- Business rules in controllers, views or adapters.
- Versions hard-coded outside the single version catalog.
- Silent fallbacks on security or decryption failure.
- Copied docs instead of links.
- Unused parameters, flags, endpoints or fields "for later".
