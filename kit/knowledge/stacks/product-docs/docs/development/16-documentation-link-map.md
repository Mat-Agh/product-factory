# 16 — Documentation link map

**Status:** Active — agent routing authority
**Last updated:** {{DATE}}

## Knowledge order (mandatory)

1. This map, [../README.md](../README.md) and `AGENTS.md`.
2. The routed docs below.
3. Implementation repos, through [../cross-project/README.md](../cross-project/README.md),
   for evidence only; they never override product scope.
4. External research, cited, only for what is still missing.
5. Durable findings go into the docs.

## Router

| Task type | Read first | Then, if needed |
|---|---|---|
| What the product is now | `CURRENT_STATE.md` | `docs/feature-matrix/README.md` |
| Why the product exists | `docs/vision/README.md` | `docs/metrics/README.md` |
| A capability's behaviour | `docs/prd/README.md` → the PRD | `docs/data/README.md`, `docs/design/README.md` |
| Writing or changing a PRD | [01](01-product-documentation.md) | `docs/prd/_templates/` |
| Handing work to engineering | `docs/delivery/README.md` | The PRD, acceptance |
| Is it done? | `docs/acceptance/README.md` | The delivery ledger |
| What is next | `docs/roadmap/README.md` | `docs/risks/README.md` |
| Who decides | `docs/governance/README.md` | `docs/decisions/README.md` |
| An implementation repo | `docs/cross-project/README.md` | That repo's docs |
| History | `docs/history/DEVELOPMENT_LOG.md` | `git log` |
| Requirements and delivery flow | [02](02-requirements-and-delivery.md) | [01](01-product-documentation.md) |
| Repositories and authorities | [03](03-repository-model.md) | `docs/cross-project/README.md` |
| General principles, evidence, documentation | [knowledge/principles/](knowledge/principles/README.md) | |
| CI/CD and workflows | [knowledge/cicd/](knowledge/cicd/README.md) | `CICD.md` if present |
| Agentic workflows | [knowledge/agentic/](knowledge/agentic/README.md) | |
