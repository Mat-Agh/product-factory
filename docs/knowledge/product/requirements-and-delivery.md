# Requirements and delivery

**Status:** Active
**Last updated:** 2026-10-20

## From idea to merged change

1. An idea becomes a PRD (or an amendment) and is accepted by the product owner.
2. Product writes one delivery packet per receiving team: backend, client, web,
   design.
3. Engineering answers the packet's open questions, then implements on job
   branches, one repo per change.
4. Evidence flows back into the acceptance ledger
   ([evidence and done](../principles/evidence-and-done.md)).
5. Humans merge, tag and release.

## PRDs

- `PRD-NNN-<slug>.md`, one capability each.
- Numbered, testable requirements (`R1`, `R2`, …).
- Acceptance criteria name the evidence that will prove them.
- Data touched, design frames, out of scope, open decisions with owners and dates.

## Packets

A packet hands one slice of a PRD to one team: the PRD sections, design frames,
data model changes, acceptance criteria, and the questions engineering must
answer. Packets are versioned; a changed packet is a new version with a changelog
line.

## Ledgers

- **Delivery ledger:** one row per requirement × gate, with evidence links.
- **Feature matrix:** capabilities × repos × status (`planned`, `in progress`,
  `implemented`, `verified`).
- **Current confirmed state:** one page saying what is confirmed in stage or
  production today, with evidence.

## Product-first vs code-first

During research and development, confirmed code may update product docs. Once a
product says product-first is in force (record it as an ADR), product, design and
acceptance docs change first and implementation follows. Do not assume either; read
the product's governance.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Product](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
