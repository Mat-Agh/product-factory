# Acceptance

**Status:** Active
**Last updated:** {{DATE}}

What must be proven before a capability or release is called done, who supplies
the evidence, and which gate each claim satisfies. Gate definitions are in
[../development/01-product-documentation.md](../development/01-product-documentation.md).

## Ledger

| Capability | Contract | Runtime | Automated | Visual | RTL/A11y | E2E | Evidence |
|---|---|---|---|---|---|---|---|
| `<PRD-NNN R1>` | `<verified/pending>` | | | | | | `<link>` |

A row is `done` only when every applicable column is verified and the evidence is
a durable link (a merged MR, a CI run, a recorded acceptance session).

## Evidence owners

| Claim | Owner | Evidence |
|---|---|---|
| Product requirement | Product | PRD and this ledger |
| Visual | Design | Live design frames |
| Runtime per surface | The owning engineering repo | Its implementation status page (link, never copy) |
| Commercial | Business | The decision log |

## Gates

Name the release gate checklist and link it. A checked engineering task is not a
passed product gate.
