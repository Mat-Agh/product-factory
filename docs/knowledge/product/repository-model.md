# Multi-repository product model

**Status:** Active
**Last updated:** 2026-10-20

A product split across repositories keeps one authority per domain and a clear
contract direction.

| Repository | Owns | Must not redefine |
|---|---|---|
| Product (business, product, design) | Commercial truth, requirements, logical data, roadmap, brand, tokens, screens, copy | Runtime API or schema details, implementation status |
| Web front-end | Browser applications, shared JS/TS packages, web delivery images | Product policy, backend persistence, client implementation |
| Backend services | APIs, physical schema, auth and security, realtime, providers, operations, installer | Product scope, commercial policy, client visual behaviour |
| Mobile client | The app, shared multiplatform logic, local state, platform adapters | Product policy, physical backend schema |
| Vendor control plane (if any) | Vendor-operated services that are never shipped to customers | Customer installers and customer data |

## Contract direction

```text
Business policy ──> Product contract <── Design contract
                          │
                          ├──> Web applications
                          ├──> Backend services
                          ├──> Mobile client
                          └──> Cross-repository acceptance evidence
```

Product requirements win when behaviour is ambiguous. The backend owns physical
persistence and API detail; clients own their implementations.

## Synchronisation rule

A product change to entities, permissions, navigation or integration policy
updates every affected repo's handoff in the same change (through each repo's
`cross-project/` docs). Business changes to packaging or entitlement name the
product and backend capability impact. Design changes map to product UX contracts
and to each consuming design system.

## Workspace

A workspace repository that pins the children (git submodules or gitlinks) holds
no product code. Children publish first; the workspace pin moves after. Names
matter: call each repo by its agreed name (a mobile "Client" is not a "Frontend").

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Product](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
