# Runners

**Status:** Active
**Last updated:** 2026-10-20

## Versions

- Keep the GitLab Runner binary and its helper image on the same minor version.
  A helper from one major with a runner from another fails after a green script
  (unknown flags on artifact upload). Pin the helper in `config.toml` and hold the
  runner package (`apt-mark hold gitlab-runner`).
- The runner pulls the helper before the job image; job YAML cannot change it.
  Log the runner's service user in to the private registry so helper pulls
  authenticate.

## Tags

A tag that routes every job of every pipeline must be registered on **exactly
one** runner. A second runner carrying it silently takes a random share of all
jobs. Audit tags when bringing a runner back online.

## Docker in CI

- A runner that bind-mounts the host Docker socket cannot also start a
  `docker:dind` sidecar (socket busy). Either use the host daemon
  (`DOCKER_HOST=unix:///var/run/docker.sock`, SHA-tagged images, periodic prune) or
  a privileged runner with a DinD sidecar and TLS certs. Fail fast with
  `docker version` in `before_script`.
- Give every Compose network an explicit subnet; Docker's fallback pool can collide
  with office LAN or VPN ranges.

## Proxy

Behind an office proxy, export `HTTP_PROXY`/`HTTPS_PROXY` for jobs, keep the
platform host, `localhost`, internal domains and the DinD hostname (`docker`) in
`NO_PROXY`, pass the proxy into DinD and `docker build`, and verify each new
artifact host (registry, binary repo) explicitly. Java tunnelling through an
authenticating proxy may need `jdk.http.auth.tunneling.disabledSchemes`.

## Caches

A single-host runner needs no S3 cache backend; its default local cache volume is
shared by every job. Use a shared S3 cache only when several hosts must share
caches.

## Sizing

| Workload | Need |
|---|---|
| GraalVM native compile | ≥32 GB per job cgroup; one at a time on 64 GB, two on 96 GB+ |
| Android assemble | 8–16 GB, several workers |
| Web build, validation, notifications | Small |

A good single host: 16+ cores, 64–128 GB RAM, 512 GB–1 TB NVMe, `concurrent`
matched to memory. Or split a heavy (native, Android) runner from a light one.
Raise native threads and heap only after measuring resident memory.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [CI/CD](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
