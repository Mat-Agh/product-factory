# 02 — TypeScript conventions

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (typescript-web)

- `strict: true`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes` where the
  codebase allows.
- No `any`. Use `unknown` at boundaries and narrow it with a parser.
- No unbounded type assertions (`as Foo`) on network data; validate the shape first.
- Discriminated unions for view state (`{ kind: "loading" } | { kind: "ready"; … }`).
- Domain names for types (`Workspace`, `MemberRole`), not transport names.
- One component per file; colocate its styles and tests.
- Exhaustive `switch` over unions with a `never` check.
- Dates on the wire are ISO-8601 strings; format them only for display.
- Pin the TypeScript version the lint stack supports; upgrade them together.
