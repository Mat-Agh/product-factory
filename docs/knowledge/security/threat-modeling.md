# Threat modeling

**Status:** Active
**Last updated:** 2026-10-20

Every repo keeps one threat-model table in its `docs/security/` area and every
security-relevant change carries a threat-model delta in its merge request.

| Asset | Threat | Control | Residual risk |
|---|---|---|---|
| Credentials | Theft, replay | Hashing, rotation, device binding | Compromised endpoint |
| Restricted data | Database or backup theft | Envelope encryption, encrypted backups | An authorized live service can still decrypt |
| Tenant boundary | Cross-tenant access | Principal-derived scope, RLS, non-enumerating errors | Bugs in policy code |
| Browser | XSS, injected dependencies | Strict CSP, SRI, no third-party scripts | Compromised browser |
| Supply chain | Tampered artifacts | Pinned versions, signed images, SBOM, provenance | Compromised upstream before pinning |

Write down what the system does **not** protect against. A compromised host root,
an authorized live process, or a compromised client endpoint are typical
residual risks; saying so keeps claims honest.

Review it at every release gate together with the data classification
([data-classification.md](data-classification.md)).

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Security](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
