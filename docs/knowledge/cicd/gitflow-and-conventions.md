# GitFlow and conventions

**Status:** Active
**Last updated:** 2026-10-20

## Branches and refs

| Branch or ref | Role |
|---|---|
| Work branch | One change; merge request into the integration branch |
| `develop` | Integration. Builds and signs artifacts; does not deploy to production |
| `main` | Release line. May deploy to a stage environment |
| `release/X.Y.Z`, `hotfix/X.Y.Z` | Optional release and hotfix lines |
| Protected tag `vX.Y.Z` (or `vX.Y.Z-rc.N`, `vX.Y.Z-beta.N`) | The only trigger for production artifacts; must match the version declared in the build |

`develop`, `main` and `v*` tags are protected; direct pushes are blocked.

## Naming rules (enforced fail-closed by hooks and CI)

| Rule | Pattern |
|---|---|
| Work branch | `(docs\|test\|refactor\|ci\|build\|chore\|perf)/[a-z0-9]+(-[a-z0-9]+)*` |
| Ticket branch | `(feature\|fix)/[a-z0-9]+(-[a-z0-9]+)+` |
| Agent branches | A dedicated, documented prefix per agent system, or the normal kinds above |
| Commit subject | `type(scope)?: description` — `feat\|fix\|docs\|test\|refactor\|perf\|build\|ci\|chore\|style\|revert`, lowercase after the colon, at most 72 characters; merge commits skipped |
| Author and committer | The organisation's email domain where policy requires it |

Keep one pattern per rule across all repos of a product; differences between
sibling repos (for example `.codex/` vs `codex/`) are bugs to fix.

## Hooks and bypasses

- Repo hooks live in the repo (for example `.githooks/`) with an install script;
  CI re-checks everything they check.
- No `--no-verify`, no skip-CI variables, no `core.hooksPath` overrides. CI does not
  honour skip switches.

## Shipping a change

1. Branch from `develop` with an allowed name.
2. Follow the handbook and run the local equivalents of the CI gates and
   `git diff --check`.
3. Open a merge request into `develop`.
4. Wait for validation and notifications to pass.
5. A human approves; merge happens per the [approval flow](merge-requests-and-approval.md).
6. Promote `develop` → `main` the same way; tag only when stage is healthy.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [CI/CD](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
