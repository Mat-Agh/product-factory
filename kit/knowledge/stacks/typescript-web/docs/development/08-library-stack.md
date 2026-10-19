# 08 — Library stack

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (typescript-web)

Record every choice with its reason and the thing we never do instead.

| Concern | Choice | Why | What we never do |
|---|---|---|---|
| App framework | Next.js App Router, static export | Layouts, metadata, chunking, SRI | Server actions, route handlers, a second session layer |
| Bundler | The one SRI requires | SRI support | Swap bundlers without re-validating SRI |
| UI | React client components at interactive boundaries | Stateful consoles | `dangerouslySetInnerHTML` |
| Crypto | Web Crypto | Native, audited | Shipping private keys |
| i18n and CSS | Local message files and global tokens | Small surface, RTL control | Heavy frameworks without need |
| Package manager | pnpm, frozen lockfile | Exact versions | Install scripts in container builds |
| Unit tests | Vitest | Neutral logic | Claiming it covers browser security |
| Browser tests | Playwright | Real workflows on desktop and mobile | Treating unit tests as end-to-end |
| Edge | A hardened web server, non-root, generated CSP | Same-origin static hosting | Node serving production assets |

Upgrade a dependency only after reading its release notes and re-running the full
gate. Coupled versions (for example TypeScript and the lint stack) move together.
