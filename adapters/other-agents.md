# Codex, Cursor, Gemini CLI and other agents

The role files are plain markdown. The Claude Code frontmatter (`name`, `tools`,
`model`) is metadata that other agents can ignore. Any agent that reads
`AGENTS.md` sees the Product Factory section the installer appended, and that
section points it to the roles.

## One role at a time

There is no orchestrator outside Claude Code, so you act as the coordinator:

1. Create the job by hand:
   ```bash
   id=001-my-change; mkdir -p .factory/jobs/$id
   git worktree add -b factory/$id ../$(basename $PWD)-factory/$id main
   ```
2. Prompt the agent once per stage:
   > You are the Product Factory `product-manager`. Read
   > `.claude/skills/factory/protocol.md` and `.claude/agents/product-manager.md`.
   > Job folder: `<abs>/.factory/jobs/001-my-change/`. Worktree: `<abs worktree>`.
   > Branch: `factory/001-my-change`. Descriptor: `<abs>/.factory/factory.yaml`.
   > Stage: spec. Request: "…".
3. Read the artifact's first line, and pick the next stage from the lane table
   in `.claude/skills/factory/SKILL.md`.

## Codex as a cross-vendor reviewer

After a Claude Code build round, run the code lens with a different model family:

```bash
codex exec "You are the Product Factory reviewer-code. Read .claude/skills/factory/protocol.md and .claude/agents/reviewer-code.md. Job folder: … Worktree: … Branch: … Review round 1. Write review-code-codex.md instead of review-code.md."
```

## Guard outside Claude Code

The hook works only in Claude Code. Elsewhere, branch protection on the remote
is the control, and the agent's own approval mode (for example Codex
`--ask-for-approval`) is the second line.
