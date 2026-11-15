# 05 — Releases and versioning

**Status:** Active
**Last updated:** 2026-11-15

## Versions

The factory uses semantic versions internally. The published production release is the immutable `v1` tag; its bundled CLI reports its tested internal version.

| Bump | When |
|---|---|
| MAJOR | Any breaking change ([03](03-changing-the-factory.md#breaking-changes)): artifact or verdict format, `job.json`, descriptor schema, agent names, commands |
| MINOR | New lane, agent, lens, command, descriptor key with a default, script |
| PATCH | Wording, checklists, bug fixes, docs |

The current release is [`v1`](https://github.com/Mat-Agh/product-factory/releases/tag/v1), containing CLI 1.2.0. It adds `pf graph view --modules` for a combined local multi-repository network while preserving the thought-sphere and brain-cloud views. Existing product docs and SQL migrations remain in their owning repositories. The Linux x86-64 binary and SHA-256 checksum are attached to the signed release. See the [implementation status](../status/IMPLEMENTATION_STATUS.md) for verified features and remaining live runtime checks.

## Releasing the factory

Tagging is human-only, here as everywhere.

1. All gates pass ([04](04-testing-and-verification.md)).
2. Review the release PR. The CLI version, implementation status and
   evolution log must match the intended tag.
3. Build from the reviewed commit with
   `cargo build --release --locked --manifest-path cli/Cargo.toml`. Copy
   the binary to a clean environment without the source checkout, verify
   `pf --version`, install a scratch project and run its knowledge and
   doctor checks. Repeat for every platform whose binary will be attached.
4. After merge, a human signs and pushes the tag with
   `git tag -s vX.Y.Z -m "Product Factory vX.Y.Z"` and
   `git push origin vX.Y.Z`. Publish the GitHub release from that tag
   with the verified binaries, their SHA-256 checksums, and install
   instructions. Use the tagged checkout for source builds and user links.
5. Run `pf mirror push` so the tag reaches every mirror, then
   `pf mirror bundle`.

## Updating products

Each product records the factory revision it runs in `.factory/installed.lock`.

```bash
cd ~/Work/ProductFactory && git pull && cargo install --path cli
pf doctor ~/Work/MyApp            # shows drift
pf install project ~/Work/MyApp   # keeps factory.yaml and backlog.md
cd ~/Work/MyApp && git diff               # review, then commit
```

For a MAJOR update, follow the migration steps in that version's evolution-log
entry, and finish in-flight jobs first.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Factory development handbook](README.md)
- Documentation home: [Product Factory documentation](../README.md)
- Agent guide: [AGENTS.md](../../AGENTS.md)
<!-- documentation-routing:end -->
