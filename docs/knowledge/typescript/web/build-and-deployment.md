# Web build and deployment

**Status:** Active
**Last updated:** 2026-10-20

## Production artifact

The production build writes a static `out/` directory. Post-build steps write an
integrity manifest (SHA-384 and byte length for every file) and generate hashes
for any inline scripts the framework emits, so the CSP needs no `unsafe-inline`.

## Container

A multi-stage Dockerfile:

1. installs the frozen lockfile with lifecycle scripts disabled;
2. builds only the target app;
3. copies `out/` into a hardened web-server image;
4. runs as a non-root UID on an unprivileged port, compatible with
   `no-new-privileges` and `cap_drop: ALL`.

The web server applies CSP, HSTS, COOP, Permissions-Policy, no-referrer, nosniff,
immutable caching for hashed assets, and static-route fallbacks.

```bash
docker build -f apps/<app>/Dockerfile -t <app>:dev .
docker run --rm -p 8080:8080 <app>:dev
```

## Release signing

One release pipeline owns image naming, SBOM and provenance, vulnerability gates,
signatures and digest pins. Do not add a second signing authority or commit a
signing key. A locally built image is development evidence only.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Web apps](README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
<!-- documentation-routing:end -->
