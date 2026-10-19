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
