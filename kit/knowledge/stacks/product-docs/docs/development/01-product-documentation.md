# 01 — Product documentation

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (product-docs)

## Ownership

| Domain | Owns | Never owns |
|---|---|---|
| Product | Behaviour, requirements, logical data, priorities, acceptance intent | Physical schema, wire formats, code |
| Business | Pricing, packaging, entitlement policy, go-to-market | Implementation |
| Design | Brand, design system, screen blueprints, copy | Behaviour not in a PRD |
| Engineering repos | APIs, physical schema, implementation, technical evidence | Product scope |

Product leads behaviour. Engineering repos implement approved contracts and report
evidence back; they never redefine scope.

## Writing rules

- One home per fact. Link, do not copy. An engineering doc that needs a product
  rule links to its PRD section.
- Every document starts with a title, `Status`, and `Last updated`.
- Active scope and history are separate. Superseded text moves to history or is
  deleted; it is not left beside the current rule.
- Decisions are ADRs with a status (Proposed, Accepted, Superseded).
- Open questions are named, owned and dated; never silently assumed.

## PRD conventions

- `PRD-NNN-<slug>.md`, one capability each, from the template in `prd/_templates/`.
- Requirements are numbered (`R1`, `R2`, …) and testable.
- Acceptance criteria name the evidence that will prove them.
- Out of scope is explicit.

## Delivery packets

A packet hands one slice of a PRD to one team: the PRD sections, design frames,
data model changes, acceptance criteria and the questions engineering must answer.
Packets are versioned; a changed packet is a new version with a changelog line.

## Evidence model: compile is not done

A delivery-ledger row may be marked done only when every column is verified with a
durable evidence link:

| Gate | Proves | Not sufficient |
|---|---|---|
| Contract | PRD and design match the intended behaviour | Code presence |
| Runtime (per repo) | The owning runtime implements the contract | Compile or static analysis alone |
| Automated | A named suite or workflow ran for that surface | "Tests exist" without a run |
| Visual | The live design pair matches | A local screenshot |
| RTL / accessibility | Bidirectional and accessible behaviour | A theme screenshot |
| End to end | The cross-runtime journey on the intended topology | A development stack standing in for production |
