# Artifacts and supply chain

**Status:** Active
**Last updated:** 2026-10-20

## Images

- Push service images to a registry under a product prefix
  (`<registry>/<product>/<repo>/<service>`), tagged with the commit SHA or the
  release version.
- Sign with Cosign directly against the registry reference (`cosign sign`,
  `cosign verify`), and verify again on the deploy host before running.
- Production images are digest-pinned (`repo@sha256:…`), non-root, minimal
  (distroless for native binaries), and labelled with build type and flavor.
- Scan every image (for example Trivy) and fail on the agreed severity.

## Other artifacts

- Installer kits, mobile builds and CI handoff binaries go to a raw binary
  repository with write-once release paths (`{product}/installers/{version}/`).
- Produce an SBOM and provenance for release artifacts; keep signing in one
  release pipeline and one signing authority.

## Variables and secrets

| Kind | Protection |
|---|---|
| Needed by merge-request pipelines (notifications, read tokens) | Masked, **unprotected** (MR pipelines run on unprotected refs) |
| Signing keys, production URLs, deploy SSH keys, store credentials | Masked and **protected**; file variables for keys |
| Pipeline YAML | Never pins feature switches to `false` (YAML variables override project variables on some versions) |

- Never commit private keys, keystores, `.env` files or service tokens. The public
  half (for example `cosign.pub`) may live in the repo.
- Keep an operator-owned, encrypted backup of keys outside the product repos (a
  private secrets repo or the platform's secure files), never copied into children.

## Dependency resolution

- Pin the build tool wrapper and verify its checksum where the mirror supports it.
- Route dependency downloads through one mirror or proxy and set
  `repositoriesMode = FAIL_ON_PROJECT_REPOS` (Gradle) so modules cannot add their
  own.
- Pin CI action revisions and base images by digest.
- Dependency locking and verification metadata add churn behind a single mirror;
  decide deliberately and record the decision.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [CI/CD](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
