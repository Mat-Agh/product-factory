//! `pf guard`: Claude Code PreToolUse hook for Bash.
//!
//! It blocks the actions the constitution reserves for humans or forbids outright:
//! merging, tagging, releasing, force-pushing, pushing anything but `factory/*`,
//! skipping hooks, and discarding work. Exit code 2 blocks the call and shows the
//! reason to the model. Remote branch protection is the real control. This hook is
//! the second line in front of it.
//!
//! Override for a human-driven session only: start Claude with `FACTORY_GUARD=off`.
//! An inline `FACTORY_GUARD=off git …` inside a command has no effect, because the hook
//! reads its own environment.

use std::io::Read;
use std::sync::LazyLock;

use regex::Regex;

const PROTECTED: &[&str] = &[
    "main",
    "master",
    "develop",
    "trunk",
    "production",
    "prod",
    "release",
];
const PROTECTED_PREFIXES: &[&str] = &["release/", "hotfix/"];
const GIT_GLOBAL_OPTS_WITH_ARG: &[&str] = &[
    "-C",
    "-c",
    "--git-dir",
    "--work-tree",
    "--namespace",
    "--exec-path",
];
const WRAPPERS: &[&str] = &[
    "env", "command", "builtin", "exec", "nice", "nohup", "time", "sudo", "xargs",
];

static HEREDOC: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<<-?\s*['"]?([A-Za-z_][A-Za-z0-9_]*)['"]?"#).unwrap());
static SPLIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\|\||&&|[;|\n]|\$\(|`").unwrap());
static ASSIGN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*=").unwrap());
static RM_RF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-\w*r\w*f|-\w*f\w*r").unwrap());
static RM_TARGET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(^|\s)(/|~|\$HOME|\.\.?)(\s|/?$)").unwrap());
static API_WRITE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"-X\s*(PUT|PATCH|DELETE)|--method\s+(PUT|PATCH|DELETE)").unwrap());
static API_TARGET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"/(merge|protection|releases|git/refs|actions/secrets)").unwrap());

static HOOK_BYPASS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?:^|[\s;&|(])(?:export\s+|env\s+)?([A-Z][A-Z0-9_]*SKIP[A-Z0-9_]*|HUSKY|LEFTHOOK|SKIP)=",
    )
    .unwrap()
});
static GIT_WRITE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\bgit\b[^\n;&|]*\b(commit|push|merge|rebase|am|cherry-pick|revert)\b").unwrap()
});

/// Which branches count as the factory's own job branches in this project.
pub struct Policy {
    pub job_branch: Regex,
    pub label: String,
}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            job_branch: Regex::new(r"^factory/").unwrap(),
            label: "factory/*".into(),
        }
    }
}

impl Policy {
    /// Build from `vcs.branch_template` (placeholders `{kind}` and `{job-id}`) and
    /// `vcs.branch_kinds`. A job id is `NNN-slug`.
    pub fn from_template(template: &str, kinds: &[String]) -> Self {
        let mut src = String::from("^");
        let mut rest = template;
        while !rest.is_empty() {
            if let Some(r) = rest.strip_prefix("{kind}") {
                let alts = if kinds.is_empty() {
                    "[a-z]+".to_string()
                } else {
                    format!(
                        "({})",
                        kinds
                            .iter()
                            .map(|k| regex::escape(k))
                            .collect::<Vec<_>>()
                            .join("|")
                    )
                };
                src.push_str(&alts);
                rest = r;
            } else if let Some(r) = rest.strip_prefix("{job-id}") {
                src.push_str("[0-9]{3}-[a-z0-9]+(-[a-z0-9]+)*");
                rest = r;
            } else {
                let c = rest.chars().next().unwrap();
                src.push_str(&regex::escape(&c.to_string()));
                rest = &rest[c.len_utf8()..];
            }
        }
        src.push('$');
        match Regex::new(&src) {
            Ok(re) => Policy {
                job_branch: re,
                label: template.to_string(),
            },
            Err(_) => Policy::default(),
        }
    }

    pub fn from_descriptor(d: &crate::descriptor::Descriptor) -> Self {
        let Some(t) = d
            .get("vcs.branch_template")
            .filter(|t| *t != "factory/{job-id}")
        else {
            return Policy::default();
        };
        let mut kinds: Vec<String> = d
            .values
            .iter()
            .filter(|(k, _)| k.starts_with("vcs.branch_kinds."))
            .map(|(_, v)| v.clone())
            .collect();
        kinds.sort();
        kinds.dedup();
        Policy::from_template(t, &kinds)
    }

    /// The project's policy: descriptor from $CLAUDE_PROJECT_DIR, else the payload's cwd.
    pub fn for_payload(payload: &str) -> Self {
        let cwd = serde_json::from_str::<serde_json::Value>(payload)
            .ok()
            .and_then(|v| {
                v.get("cwd")
                    .and_then(|c| c.as_str())
                    .map(std::path::PathBuf::from)
            });
        let start = std::env::var_os("CLAUDE_PROJECT_DIR")
            .map(std::path::PathBuf::from)
            .or(cwd);
        start
            .and_then(|s| crate::descriptor::find(&s))
            .map(|d| Policy::from_descriptor(&d))
            .unwrap_or_default()
    }
}

fn is_protected(r: &str) -> bool {
    let r = r.rsplit(':').next().unwrap_or(r).trim_start_matches('+');
    let r = r.strip_prefix("refs/heads/").unwrap_or(r);
    PROTECTED.contains(&r) || PROTECTED_PREFIXES.iter().any(|p| r.starts_with(p))
}

/// Split a shell command into simple commands on `;`, `&&`, `||`, `|`, newlines, `$(` and
/// backticks. Heredoc bodies are dropped, so text inside them is never parsed as a command.
fn split_segments(command: &str) -> Vec<String> {
    let mut out: Vec<&str> = Vec::new();
    let mut skip_until: Option<String> = None;
    for line in command.split('\n') {
        if let Some(end) = &skip_until {
            if line.trim() == end {
                skip_until = None;
            }
            continue;
        }
        let mut line = line;
        if let Some(c) = HEREDOC.captures(line) {
            skip_until = Some(c[1].to_string());
            line = &line[..c.get(0).unwrap().start()];
        }
        out.push(line);
    }
    let text = out.join("\n");
    SPLIT
        .split(&text)
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
        .collect()
}

fn tokens_of(segment: &str) -> Vec<String> {
    let mut toks = shlex::split(segment)
        .unwrap_or_else(|| segment.split_whitespace().map(str::to_string).collect());
    while let Some(head) = toks.first().cloned() {
        if !(ASSIGN.is_match(&head) || WRAPPERS.contains(&head.as_str())) {
            break;
        }
        toks.remove(0);
        if matches!(head.as_str(), "sudo" | "nice" | "xargs") {
            while toks.first().is_some_and(|t| t.starts_with('-')) {
                toks.remove(0);
            }
        }
    }
    if let Some(first) = toks.first_mut() {
        *first = first.rsplit('/').next().unwrap_or(first).to_string();
    }
    toks
}

fn git_sub(toks: &[String]) -> Option<(&str, &[String])> {
    let mut i = 1;
    while i < toks.len() {
        let t = toks[i].as_str();
        if GIT_GLOBAL_OPTS_WITH_ARG.contains(&t) {
            i += 2;
        } else if t.starts_with('-') {
            i += 1;
        } else {
            return Some((t, &toks[i + 1..]));
        }
    }
    None
}

fn short_flags(args: &[String]) -> String {
    args.iter()
        .filter(|a| a.starts_with('-') && !a.starts_with("--"))
        .map(|a| &a[1..])
        .collect()
}

fn has(args: &[String], v: &str) -> bool {
    args.iter().any(|a| a == v)
}

fn check_git(toks: &[String], pol: &Policy) -> Option<String> {
    if toks
        .windows(2)
        .any(|w| w[0] == "-c" && w[1].to_lowercase().starts_with("core.hookspath"))
    {
        return Some("Overriding core.hooksPath bypasses the repo's hooks. Forbidden.".into());
    }
    let (sub, args) = git_sub(toks)?;
    if sub == "config"
        && args
            .iter()
            .any(|a| a.eq_ignore_ascii_case("core.hooksPath"))
        && !args
            .iter()
            .any(|a| a == "--get" || a == "--list" || a == "-l")
    {
        return Some("Changing core.hooksPath bypasses the repo's hooks. Ask the human.".into());
    }
    let flags = short_flags(args);
    if has(args, "--no-verify") || (flags.contains('n') && sub == "commit") {
        return Some("Skipping hooks (--no-verify) is forbidden.".into());
    }
    if sub == "merge" && !args.iter().any(|a| a == "--abort" || a == "--quit") {
        return Some("Merging is human-only. The factory opens a PR and stops.".into());
    }
    if sub == "merge-tree" {
        return None;
    }
    let listing = args.iter().any(|a| {
        matches!(a.as_str(), "-l" | "--list" | "--contains" | "--points-at") || a.starts_with("-n")
    });
    if sub == "tag" && !args.is_empty() && !listing {
        return Some("Tagging is human-only.".into());
    }
    if sub == "push" {
        if has(args, "--force")
            || has(args, "--force-with-lease")
            || flags.contains('f')
            || args
                .iter()
                .any(|a| a.starts_with("--force") || a.starts_with('+') || a.contains(":+"))
        {
            return Some("Force-push is forbidden.".into());
        }
        if ["--tags", "--follow-tags", "--mirror", "--all"]
            .iter()
            .any(|f| has(args, f))
        {
            return Some("Pushing tags, --all or --mirror is human-only.".into());
        }
        if has(args, "--delete") || flags.contains('d') || args.iter().any(|a| a.starts_with(':')) {
            return Some("Deleting remote refs is human-only.".into());
        }
        let refs: Vec<&String> = args
            .iter()
            .filter(|a| !a.starts_with('-'))
            .skip(1)
            .collect();
        if refs.is_empty() {
            return Some(format!(
                "Name the branch explicitly: git push <remote> <job branch> ({}).",
                pol.label
            ));
        }
        for r in refs {
            if is_protected(r) || r.contains("refs/tags/") {
                return Some(format!("Pushing to protected ref '{r}' is human-only."));
            }
            if !pol.job_branch.is_match(r.rsplit(':').next().unwrap_or(r)) {
                return Some(format!(
                    "The factory pushes only its job branches ({}), not '{r}'.",
                    pol.label
                ));
            }
        }
        return None;
    }
    if sub == "reset" && has(args, "--hard") {
        return Some("git reset --hard discards work. Ask the human.".into());
    }
    if sub == "clean" && (flags.contains('f') || has(args, "--force")) {
        return Some("git clean -f discards work. Ask the human.".into());
    }
    if sub == "stash"
        && args.first().is_none_or(|a| {
            matches!(
                a.as_str(),
                "push" | "save" | "drop" | "clear" | "-u" | "--include-untracked"
            )
        })
    {
        return Some("Stashing or dropping someone's work is forbidden. Ask the human.".into());
    }
    if matches!(sub, "checkout" | "restore") && has(args, "--") && has(args, ".") {
        return Some(
            "Discarding working-tree changes wholesale is forbidden. Ask the human.".into(),
        );
    }
    if sub == "branch"
        && (flags.contains('D') || flags.contains('d') || has(args, "--delete"))
        && args
            .iter()
            .filter(|a| !a.starts_with('-'))
            .any(|t| !pol.job_branch.is_match(t))
    {
        return Some(format!(
            "Only the factory's own job branches ({}) may be deleted by the factory.",
            pol.label
        ));
    }
    if matches!(sub, "update-ref" | "filter-branch" | "filter-repo") {
        return Some(format!("git {sub} rewrites refs. Human-only."));
    }
    None
}

fn check_gh(toks: &[String]) -> Option<String> {
    if toks.len() < 3 {
        return None;
    }
    let (area, verb) = (toks[1].as_str(), toks[2].as_str());
    match (area, verb) {
        ("pr", "merge") => Some("Merging is human-only.".into()),
        ("pr", "review") if has(toks, "--approve") || has(toks, "-a") => {
            Some("Approving PRs is human-only.".into())
        }
        ("release", "create" | "edit" | "delete" | "upload") => {
            Some("Releases are human-only.".into())
        }
        ("repo", "delete" | "archive" | "rename" | "edit") => {
            Some(format!("gh repo {verb} is human-only."))
        }
        ("secret" | "variable", _) => Some("Secrets and variables are operator-only.".into()),
        ("workflow", "run" | "enable" | "disable") => {
            Some("Running or toggling workflows is human-only.".into())
        }
        ("api", _) => {
            let joined = toks.join(" ");
            (API_WRITE.is_match(&joined) && API_TARGET.is_match(&joined))
                .then(|| "This API call merges, releases or changes protection. Human-only.".into())
        }
        _ => None,
    }
}

fn check_glab(toks: &[String]) -> Option<String> {
    if toks.len() < 3 {
        return None;
    }
    match (toks[1].as_str(), toks[2].as_str()) {
        ("mr", "merge" | "approve") => Some("Merging and approving MRs is human-only.".into()),
        ("release", "create") => Some("Releases are human-only.".into()),
        _ => None,
    }
}

/// The reason a command is blocked, or None when it is allowed.
pub fn check(command: &str, pol: &Policy) -> Option<String> {
    if let Some(c) = HOOK_BYPASS.captures(command)
        && GIT_WRITE.is_match(command)
    {
        return Some(format!(
            "Setting {} bypasses the repo's hooks or gates. Forbidden.",
            &c[1]
        ));
    }
    for seg in split_segments(command) {
        let toks = tokens_of(&seg);
        let Some(name) = toks.first() else { continue };
        let reason = match name.as_str() {
            "git" => check_git(&toks, pol),
            "gh" => check_gh(&toks),
            "glab" => check_glab(&toks),
            "rm" if RM_RF.is_match(&seg) && RM_TARGET.is_match(&seg) => {
                Some("Recursive delete of a root, home or parent directory is forbidden.".into())
            }
            _ => None,
        };
        if reason.is_some() {
            return reason;
        }
    }
    None
}

/// Decide on one hook payload. Returns the process exit code and the stderr message.
pub fn decide(payload: &str, guard_off: bool, pol: &Policy) -> (i32, Option<String>) {
    if guard_off {
        return (0, None);
    }
    let Ok(v) = serde_json::from_str::<serde_json::Value>(payload) else {
        return (
            2,
            Some("factory-guard: unreadable hook input; blocking to fail closed.".into()),
        );
    };
    if v.get("tool_name").and_then(|t| t.as_str()) != Some("Bash") {
        return (0, None);
    }
    let command = v
        .pointer("/tool_input/command")
        .and_then(|c| c.as_str())
        .unwrap_or("");
    match check(command, pol) {
        Some(r) => (
            2,
            Some(format!(
                "factory-guard: {r} (see .claude/skills/factory/CONSTITUTION.md §5)"
            )),
        ),
        None => (0, None),
    }
}

pub fn main() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);
    let off = std::env::var("FACTORY_GUARD")
        .map(|v| v.eq_ignore_ascii_case("off"))
        .unwrap_or(false);
    let pol = Policy::for_payload(&input);
    let (code, msg) = decide(&input, off, &pol);
    if let Some(m) = msg {
        eprintln!("{m}");
    }
    code
}

/// Commands the guard must block. Shared by `cargo test` and `pf doctor`.
pub const BLOCK: &[&str] = &[
    "DARIA_HOOK_SKIP_GRADLE=1 git push origin factory/001-x",
    "export HUSKY=0; git commit -m x",
    "git -c core.hooksPath=/dev/null commit -m x",
    "git config core.hooksPath /dev/null",
    "SKIP=lint git commit -m x",
    "git merge factory/001-x",
    "git -C /repo merge factory/001-x",
    "cd /w && git merge --no-ff factory/001-x -m 'm'",
    "GIT_DIR=x git merge foo",
    "env git merge foo",
    "command git merge foo",
    "/usr/bin/git merge foo",
    "git push origin main",
    "git push origin HEAD:main",
    "git push origin factory/001-x:main",
    "git push origin develop",
    "git push origin release/1.2",
    "git push -f origin factory/001-x",
    "git push -fu origin factory/001-x",
    "git push --force-with-lease origin factory/001-x",
    "git push origin +factory/001-x",
    "git push --tags",
    "git push origin --delete factory/001-x",
    "git push origin :factory/001-x",
    "git push",
    "git push origin",
    "git push origin HEAD",
    "git push origin feature/other",
    "git tag v1.0.0",
    "git tag -a v1 -m x",
    "git commit --no-verify -m x",
    "git commit -nm x",
    "git reset --hard HEAD~1",
    "git clean -fd",
    "git stash",
    "git stash drop",
    "git checkout -- .",
    "git branch -D main",
    "gh pr merge 12 --squash",
    "gh pr review 12 --approve",
    "gh release create v1",
    "gh repo delete Mat-Agh/x --yes",
    "gh secret set TOKEN",
    "gh workflow run deploy.yml",
    "gh api -X PUT repos/o/r/pulls/1/merge",
    "glab mr merge 3",
    "glab mr approve 3",
    "rm -rf ~",
    "rm -rf /",
    "true && git merge x",
    "echo $(git merge x)",
];

/// Commands the guard must allow.
pub const ALLOW: &[&str] = &[
    "git config --get core.hooksPath",
    "SKIP_TESTS=1 npm test",
    "git status --porcelain",
    "git log --oneline main..HEAD",
    "git diff main...HEAD",
    "git merge-tree --write-tree origin/main HEAD",
    "git merge --abort",
    "git worktree add -b factory/001-x ../r-factory/001-x main",
    "cd /w && git commit -m 'Add reading time'",
    "git push -u origin factory/001-x",
    "git push origin factory/001-x",
    "git branch -d factory/001-x",
    "git branch --show-current",
    "git tag --list",
    "git tag -l 'v*'",
    "git stash list",
    "git fetch origin main",
    "gh pr create --base main --head factory/001-x --title t --body-file pr.md",
    "gh pr create --draft --base main --head factory/001-x --title t --body-file pr.md",
    "gh pr view 1 --json state",
    "gh issue view 3 --json title,body",
    "gh issue comment 3 --body-file -",
    "glab mr create --target-branch main --source-branch factory/001-x",
    "rm -rf node_modules",
    "npm test",
    "cat <<'FACTORY_COMMENT'\ngit merge main\ngit push origin main\nFACTORY_COMMENT",
    "python3 -m http.server",
];

fn payload(cmd: &str) -> String {
    serde_json::json!({"tool_name": "Bash", "tool_input": {"command": cmd}}).to_string()
}

/// Run the shared policy cases in-process. Returns (passed, total, failures).
pub fn self_test() -> (usize, usize, Vec<String>) {
    let mut fails = Vec::new();
    for c in BLOCK {
        if decide(&payload(c), false, &Policy::default()).0 != 2 {
            fails.push(format!("should BLOCK: {c:?}"));
        }
    }
    for c in ALLOW {
        if decide(&payload(c), false, &Policy::default()).0 != 0 {
            fails.push(format!("should ALLOW: {c:?}"));
        }
    }
    if decide(&payload("git merge x"), true, &Policy::default()).0 != 0 {
        fails.push("FACTORY_GUARD=off should allow".into());
    }
    if decide("not json", false, &Policy::default()).0 != 2 {
        fails.push("bad input should fail closed".into());
    }
    let total = BLOCK.len() + ALLOW.len() + 2;
    (total - fails.len(), total, fails)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_cases() {
        let (passed, total, fails) = self_test();
        assert!(fails.is_empty(), "{passed}/{total}:\n{}", fails.join("\n"));
        assert_eq!(total, 80);
    }

    #[test]
    fn project_branch_template() {
        let kinds: Vec<String> = ["chore", "docs", "feature", "fix"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let pol = Policy::from_template("{kind}/{job-id}", &kinds);
        let push = |b: &str| decide(&payload(&format!("git push -u origin {b}")), false, &pol).0;
        assert_eq!(push("feature/004-org-name"), 0);
        assert_eq!(push("fix/012-empty-password"), 0);
        assert_eq!(push("feature/someone-elses-branch"), 2);
        assert_eq!(push("factory/004-org-name"), 2);
        assert_eq!(push("develop"), 2);
        let d = crate::descriptor::parse(
            "vcs:\n  branch_template: \"{kind}/{job-id}\"\n  branch_kinds: { feature: feature, bug: fix }\n",
        );
        let p2 = Policy::from_descriptor(&d);
        assert!(p2.job_branch.is_match("fix/001-a-b"));
        assert!(!p2.job_branch.is_match("docs/001-a-b"));
    }

    #[test]
    fn non_bash_tools_pass() {
        assert_eq!(
            decide(
                r#"{"tool_name":"Read","tool_input":{}}"#,
                false,
                &Policy::default()
            )
            .0,
            0
        );
    }
}
