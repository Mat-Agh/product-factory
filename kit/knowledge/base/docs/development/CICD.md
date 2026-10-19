# CI/CD

**Status:** Active
**Last updated:** {{DATE}}

The pipeline contract for {{MODULE}}. Agents follow it exactly and never bypass
it. The branch template, commit convention and merge rules here must match
`.factory/factory.yaml` (`vcs`), the git hooks and the CI configuration.

## Branches and commits

| Branch or ref | Role |
|---|---|
| Work branch (`feature/…`, `fix/…`, `docs/…`, `chore/…`, …) | One change; opens a merge request into the integration branch |
| Integration branch (for example `develop`) | Merged work; builds and signs artifacts; does not deploy to production |
| Release line (for example `main`) | Release candidates; may deploy to a stage environment |
| Protected tag `vX.Y.Z` | The only trigger for production artifacts |

Direct pushes to protected branches are blocked.

Commit subjects follow the convention in `vcs.commit_convention` (for example
Conventional Commits: `type(scope): lowercase subject`, at most 72 characters).
Record the exact branch-name pattern and allowed types here.

| Rule | Pattern |
|---|---|
| Work branch | `<pattern>` |
| Commit subject | `<pattern>` |
| Tag | `vX.Y.Z` |

## Pipeline stages

| Trigger | Stages |
|---|---|
| Merge request | Conventions check, lint, static analysis, build, tests per policy, docs checks |
| Integration branch | Build, scan and sign artifacts |
| Release line | The above, plus deploy to stage when configured |
| Protected tag | Release build, signed artifacts, release notes, published release |

Skip-CI variables and hook bypasses are not honoured.

## Approval and merge

A merge request needs a human approval. Record whether merge is a human click or
automatic after approval and a green pipeline. Either way, an agent opens the
merge request and stops.

## Release and deploy (human-only steps)

Tagging, deploying to production and publishing are done by people. Record who,
and the exact steps, here.

## Local equivalents

| CI job | Local command |
|---|---|
| `<job>` | `<command>` |
