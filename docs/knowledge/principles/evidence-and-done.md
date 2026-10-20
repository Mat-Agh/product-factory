# Evidence and done

**Status:** Active
**Last updated:** 2026-10-20

## Compile is not done

A change is done when its quality gates pass **and** acceptance evidence for the
behaviour is recorded. A passing compile, an image build, contract validation or a
local development stack is never feature completion. A development environment
is not the production installer.

## What evidence says

Evidence states exactly what ran: commands, environment, versions or SHAs,
devices, timestamps, results and failures. Describe it precisely ("compiled,
statically validated, scanned and manually accepted on a fresh development
stack"); never inflate it ("fully tested", "security-proven", "production ready").

## Gates of a capability

A capability is done only when every applicable gate is verified with a durable
evidence link (a merged change, a CI run, a recorded acceptance session):

| Gate | Proves | Not sufficient |
|---|---|---|
| Contract | Requirements and design match the intended behaviour | Code presence |
| Runtime (per repo) | The owning runtime implements the contract | Compile or static analysis alone |
| Automated | A named suite or workflow ran for that surface | "Tests exist" without a run |
| Visual | The live design matches | A local screenshot |
| RTL and accessibility | Bidirectional and accessible behaviour | A theme screenshot |
| End to end | The cross-runtime journey on the intended topology | A development stack standing in for production |

## Phase or release close

Before a phase or release advances: upstream traceability, an architecture and
naming review, the threat-model delta, the API/event/schema diff, regenerated
docs, schema impact, acceptance evidence, and `git diff --check`.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Principles](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
