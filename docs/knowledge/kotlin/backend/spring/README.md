# Spring Boot services

**Status:** Active
**Last updated:** 2026-10-20

Knowledge for independently deployable Spring Boot (Kotlin) services with
practical clean architecture.

| Page | Covers |
|---|---|
| [principles.md](principles.md) | Topology, layers, naming, use-case contract, invariants checklist |
| [service-architecture.md](service-architecture.md) | Service ownership, the feature-to-service compile rule |
| [layers.md](layers.md) | `domain` / `data` / `presentation` / `di`, folder tree, core modules |
| [persistence.md](persistence.md) | Flyway, `*JpaSource` vs `*Repository`, tenant scope, migrations |
| [events-outbox-inbox.md](events-outbox-inbox.md) | Transactional outbox and inbox |
| [security.md](security.md) | API surfaces, authorization checklist, credentials, errors |
| [cryptography-by-layer.md](cryptography-by-layer.md) | Envelope encryption, keys, hashing, blind indexes, transport by layer |
| [testing-and-quality.md](testing-and-quality.md) | Testing policies, gates, evidence, phase close |
| [build-and-ci.md](build-and-ci.md) | Baseline, Boot BOM as a platform, convention plugins, CI gates |
| [tools.md](tools.md) | Everyday loop, generators, contract surfaces |
| [code-samples.md](code-samples.md) | Templates by task and by file type |

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Kotlin backend](../README.md)
- Documentation home: [Product Factory documentation](../../../../README.md)
- Agent guide: [AGENTS.md](../../../../../AGENTS.md)
- In this folder:
  - [Spring build and CI](build-and-ci.md)
  - [Spring code samples](code-samples.md)
  - [Cryptography by layer](cryptography-by-layer.md)
  - [Events: outbox and inbox](events-outbox-inbox.md)
  - [Spring clean architecture layers](layers.md)
  - [Persistence: source vs repository](persistence.md)
  - [Spring service engineering principles](principles.md)
  - [Spring security contracts](security.md)
  - [Service architecture](service-architecture.md)
  - [Spring testing and quality](testing-and-quality.md)
  - [Spring tools and commands](tools.md)
<!-- documentation-routing:end -->
