# Deployment

**Status:** Active
**Last updated:** {{DATE}}

## Environments

| Environment | Build type | Flavor | Trigger | Who deploys |
|---|---|---|---|---|
| Local | development | `<beta>` | Developer | Developer |
| Stage | `<…>` | `<…>` | `<main branch>` | CI, after human merge |
| Production | production | release | Protected tag | A human |

Isolate environments: separate Compose project names, secret directories, volumes
and databases per flavor.

## Requirements

Record server sizing (CPU, memory, disk, IOPS), supported operating systems,
required ports and external dependencies, and how each was measured.

## Packaging

Record how images are built (JVM or native), how they are signed, where they are
stored, and how a deployment pins them by digest. Production artifacts come only
from the protected release pipeline.

## Helm, when this repository deploys with it

Record the chart path and version, every values file and environment overlay,
the release and namespace names, pinned image digests, secret references,
upgrade and rollback commands, and the CI job that applies them. Link the
actual chart files; this guide does not invent a chart for a repository that
deploys another way. Keep credentials outside values files committed to Git.
