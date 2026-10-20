# Deployment and environments

**Status:** Active
**Last updated:** 2026-10-20

## Two axes

| Axis | Values | Changes |
|---|---|---|
| Build type | `development`, `production` | Packaging (JVM vs native, local vs installer-generated keys) |
| Deployment flavor | `beta`, `release` | Environment identity only: hostname, secrets, databases, buckets, queues, backups |

A flavor never disables a security control. Each flavor has its own hostname,
certificate, install paths, Compose project name, keys, credentials, storage and
backups; never point two flavors at shared state.

## Channels

| Channel | Trigger | Who |
|---|---|---|
| Local development | Developer | Developer; development Compose is not the installer |
| Stage | `main` after a human merge | CI over SSH into a dedicated stage directory |
| Customer or production | Protected tag | A human runs the signed installer or rollout |

Keep channels apart: separate Compose projects, networks, volumes and directories;
a stage reset is scoped to its own project and never touches another product or
the installer-owned path. Every launch report says which channel it exercised.

## Deploying over SSH

- Verify host keys (`known_hosts` variable); the key is a file variable.
- Copy only manifests and Compose files; pull signed images from the registry and
  `cosign verify` on the host.
- Host-managed configuration (domains, image pins, subnets) lives in a host env
  file that CI only overlays for image keys. A renamed `${VAR:?}` in Compose needs
  an operator note, because CI cannot fix host drift.
- Root-owned secrets directories must still be traversable by the deploy user
  (`chmod g+x`/`o+x` on the directory only); POSIX applies group bits when the user
  is in the owning group.
- Self-healing health checks (`test -s token || kill 1` with a restart policy)
  recover sidecars whose state file disappears.
- Network subnet changes need a one-time `compose down` in a maintenance window;
  `up -d` does not recreate existing networks.

## Kubernetes alignment

Moving Compose estates to Kubernetes keeps the contracts: one namespace and chart
per product boundary (never one chart that makes a vendor plane a customer
option), digest-pinned images with admission, secrets mounted at the same paths
(`/run/secrets`), NetworkPolicies equal to the Compose network rules, TLS
passthrough where mTLS routes need client certificates, datastores moved first, and
GitOps per namespace. A working dry-run does not satisfy production gates.

## Release evidence

Static checks and a development stack prove wiring, not production. A release is
usable after its signed artifacts pass installation on disposable supported hosts,
key custody, backup and restore, and the security ledger.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [CI/CD](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
