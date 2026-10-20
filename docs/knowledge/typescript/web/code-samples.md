# Web code samples

**Status:** Active
**Last updated:** 2026-10-20

Point at living code, not copies. Keep the canonical samples exemplary.

| I need to… | Look at |
|---|---|
| An app's state machine (setup, login, console) | `apps/<app>/src/features/application/` |
| A console panel | `apps/<app>/src/features/<area>/panels/` |
| The typed API client with CSRF | `packages/core/src/client.ts` |
| Contract types | `packages/core/src/types.ts` |
| A browser test with intercepted endpoints | `apps/<app>/e2e/` |

## Typed endpoint function

```ts
export async function updateMemberRole(
  memberId: string,
  role: MemberRole,
): Promise<Member> {
  const res = await request(`/admin/v1/members/${encodeURIComponent(memberId)}`, {
    method: "PATCH",
    body: JSON.stringify({ role }),
  });
  return parseMember(await res.json());
}
```

## View state as a union

```ts
type PanelState =
  | { kind: "loading" }
  | { kind: "error"; message: string }
  | { kind: "ready"; members: Member[] };
```

## Asserting a mutation body in Playwright

```ts
const req = page.waitForRequest((r) => r.method() === "PATCH" && r.url().includes("/members/"));
await page.getByRole("button", { name: "Save" }).click();
expect((await req).postDataJSON()).toEqual({ role: "admin" });
```

<!-- documentation-routing:start -->
## Documentation routing

- Parent: [Web apps](README.md)
- Documentation home: [Product Factory documentation](../../../README.md)
- Agent guide: [AGENTS.md](../../../../AGENTS.md)
<!-- documentation-routing:end -->
