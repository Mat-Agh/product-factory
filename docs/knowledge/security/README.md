# Security

**Status:** Active
**Last updated:** 2026-10-20

Cross-stack security knowledge. Implementation per stack:
[Spring cryptography by layer](../kotlin/backend/spring/cryptography-by-layer.md),
[client cryptography](../kotlin/client/security-and-cryptography.md),
[web security](../typescript/web/security.md).

| Page | Covers |
|---|---|
| [data-classification.md](data-classification.md) | Classes, storage and logging treatment, what never enters operational output |
| [cryptography.md](cryptography.md) | Approved suites, key custody, rotation, forbidden patterns |
| [transport-and-identity.md](transport-and-identity.md) | TLS and mTLS, authorization checklist, facts vs decisions, credentials |
| [threat-modeling.md](threat-modeling.md) | The threat table every repo keeps |

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Development knowledge](../README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
- In this folder:
  - [Cryptography policy](cryptography.md)
  - [Data classification](data-classification.md)
  - [Threat modeling](threat-modeling.md)
  - [Transport and identity](transport-and-identity.md)
<!-- documentation-routing:end -->
