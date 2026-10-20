# Rust CLI tooling

**Status:** Active
**Last updated:** 2026-10-20

Any script, CLI, hook, generator or helper an agent writes and keeps is written in
Rust (a cargo crate, or a subcommand of an existing Rust CLI) unless the operator
says otherwise. Product code follows the product's own stack. Short throwaway
shell one-liners used only to inspect something may stay in shell.

## Shape

- One binary with subcommands (`clap` derive) instead of many scripts.
- Edition 2024; `cargo fmt`, `cargo clippy` with zero warnings, `cargo test` in CI.
- Few, mainstream dependencies (`clap`, `regex`, `serde_json`, `sha2`, `shlex`);
  shell out to `git`, `gh` or `glab` rather than re-implementing them.
- Exit codes are the contract: `0` success, non-zero failures; a hook that blocks
  exits `2` with the reason on stderr.
- Restore default `SIGPIPE` on Unix so `tool … | head` exits quietly instead of
  panicking.
- Paths: normalise, compute relative paths explicitly, never assume the current
  directory; find the repo root with `git rev-parse`.
- Non-ASCII text: compare and slice by `chars()`, never by byte index.

## Safety

- A guard used as a hook fails closed: malformed input blocks.
- Writers never overwrite user files silently; they back up (`*.pre-<tool>`),
  merge (settings JSON), or append in marked blocks.
- Idempotent: a second run changes nothing.

## Tests

- Unit tests next to the code; table-driven policy cases (allow and block lists)
  for guards.
- Integration tests create a temporary git repo, run the command, and assert on the
  files, including that a rerun is a no-op.

## Distribution

`cargo install --path cli` from a checkout, or `cargo install --git <repo> <bin>`.
Binaries land in `~/.cargo/bin`, which must be on `PATH`. Reinstall after pulling.

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Rust](README.md)
- Documentation home: [Product Factory documentation](../../README.md)
- Agent guide: [AGENTS.md](../../../AGENTS.md)
<!-- documentation-routing:end -->
