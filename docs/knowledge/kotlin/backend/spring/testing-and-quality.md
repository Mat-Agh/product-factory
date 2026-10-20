# Spring testing and quality

**Status:** Active
**Last updated:** 2026-10-20

## Testing policy

The testing policy is a product decision. Record the one that applies to
the repository here and in `AGENTS.md` (`Gates`), and keep `tests:` in
`.factory/factory.yaml` consistent with it.

| Policy | What it means | When it fits |
|---|---|---|
| **Test pyramid** | Unit tests for use cases, policies and mappers; slice tests for persistence and controllers; a few integration tests against real dependencies (Testcontainers) | The default for most services |
| **Static gates and manual acceptance** | No unit or in-process test sources. Verification is compile, lint, static and security gates, contract validation, and recorded manual acceptance on a fresh environment. Narrow exceptions (for example cryptographic known-answer vectors) are named explicitly | Products that chose it deliberately; record the ADR |

Whichever policy applies, agents never add tests that the policy forbids, and
never skip tests that the policy requires.

## Required gates

- Compile every Kotlin module and every Boot artifact.
- ktlint, detekt, architecture verification, contract/schema/documentation
  validation, dependency and secret scans, container scanning.
- The tests the policy requires.
- For a release candidate: start a fresh environment and inspect Flyway completion,
  health and readiness, TLS identities, logs (redaction), schema and representative
  query plans.

## Evidence

Evidence states exactly what ran: commands, timestamps, environment, results. Say
"compiled, statically validated, scanned and manually accepted" when that is what
happened; never call it "fully tested" or "security-proven".

## Phase close

Before a phase or release advances: upstream traceability, architecture and naming
review, threat-model delta, API/event/schema diff, generated docs, schema impact,
acceptance evidence, and `git diff --check`.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Spring Boot services](README.md)
- Documentation home: [Product Factory documentation](../../../../README.md)
- Agent guide: [AGENTS.md](../../../../../AGENTS.md)
<!-- documentation-routing:end -->
