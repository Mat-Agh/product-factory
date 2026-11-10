//! `pf audit`: inventory a repository's existing agentic setup before the factory adopts
//! it (CONSTITUTION §8), and flag what would clash with the factory.
//!
//! It covers agent instructions and IDE rules, Claude Code config, git hooks, CI and
//! auto-merge, branch and commit conventions (it extracts the regexes and tests the
//! factory's job branch against them), attribution rules, other agent automations,
//! secrets in agent config files (never printing a value), and docs routing markers.
//! It is read-only.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;
use serde_json::{Value, json};

use crate::{descriptor, util};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Severity {
    Blocker,
    Conflict,
    Warn,
    Info,
}

impl Severity {
    fn label(self) -> &'static str {
        match self {
            Severity::Blocker => "BLOCKER",
            Severity::Conflict => "CONFLICT",
            Severity::Warn => "WARN",
            Severity::Info => "INFO",
        }
    }
}

#[derive(Debug)]
pub struct Finding {
    pub severity: Severity,
    pub area: &'static str,
    pub message: String,
    pub action: String,
}

const INSTRUCTION_FILES: &[&str] = &[
    "AGENTS.md",
    "CLAUDE.md",
    "CLAUDE.local.md",
    ".claude/CLAUDE.md",
    "GEMINI.md",
    ".github/copilot-instructions.md",
    ".cursorrules",
    ".windsurfrules",
    ".clinerules",
    "HERMES.md",
    "CONTRIBUTING.md",
];
const AGENT_DIRS: &[&str] = &[
    ".cursor/rules",
    ".codex/rules",
    ".codex/agents",
    ".agents/rules",
    ".agents/skills",
    ".junie",
    ".claude/skills",
    ".claude/agents",
    ".claude/commands",
    ".claude/rules",
    ".claude/hooks",
];
const SECRET_DIRS: &[&str] = &[
    ".cursor", ".codex", ".claude", ".agents", ".vscode", ".junie", ".gemini",
];
const FACTORY_AGENTS: &[&str] = &[
    "product-manager",
    "designer",
    "tech-lead",
    "engineer",
    "reviewer-code",
    "reviewer-security",
    "reviewer-ux",
    "reviewer-ui",
    "tester",
    "release-manager",
];

static QUOTED: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(r?)"(\^[^"]+)""#).unwrap());
static SLASHED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[=(:,]\s*/(\^.+?)/[a-z]*\s*[;,)]?\s*$").unwrap());
static SKIP_VAR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b([A-Z][A-Z0-9_]*SKIP[A-Z0-9_]*|HUSKY|LEFTHOOK)\b").unwrap());
static SECRET: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(glpat-[A-Za-z0-9_-]{10,}|gh[pousr]_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|sk-ant-[A-Za-z0-9_-]{10,}|xox[bpa]-[A-Za-z0-9-]{10,}|"[A-Z_]*(TOKEN|SECRET|PASSWORD|API_KEY)"\s*[:=]\s*"[^"$\{][^"]{11,}")"#).unwrap()
});
static AUTOMERGE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"merge_when_pipeline_succeeds|auto_merge|automerge|gh pr merge [^\n]*--auto|enable-auto-merge").unwrap()
});
static NO_AI_ATTRIBUTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?is)(never|no|must not|do not|don't)[^.\n]{0,120}(co-authored-by|attribution|name any ai|ai model|generated with)").unwrap()
});

fn read(p: &Path) -> String {
    fs::metadata(p)
        .ok()
        .filter(|m| m.len() < 512 * 1024)
        .and_then(|_| fs::read_to_string(p).ok())
        .unwrap_or_default()
}

fn tracked_files(root: &Path) -> Vec<String> {
    util::output(
        "git",
        &[
            "-C",
            &root.to_string_lossy(),
            "ls-files",
            "-co",
            "--exclude-standard",
        ],
        None,
    )
    .map(|o| o.lines().map(str::to_string).collect())
    .unwrap_or_default()
}

fn is_tracked(root: &Path, rel: &str) -> bool {
    util::output(
        "git",
        &[
            "-C",
            &root.to_string_lossy(),
            "ls-files",
            "--error-unmatch",
            rel,
        ],
        None,
    )
    .is_some()
}

fn walk_files(dir: &Path, out: &mut Vec<PathBuf>, depth: usize) {
    if depth > 4 {
        return;
    }
    for e in fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            if e.file_name() != "node_modules" {
                walk_files(&p, out, depth + 1);
            }
        } else {
            out.push(p);
        }
    }
}

/// Branch-name regexes found in hooks, CI and build logic, unescaped and compiled.
pub fn branch_patterns(root: &Path, files: &[String]) -> Vec<(String, Regex)> {
    let mut out: Vec<(String, Regex)> = Vec::new();
    let candidates = files.iter().filter(|f| {
        let l = f.to_lowercase();
        (l.starts_with(".ci/")
            || l.starts_with(".gitlab/")
            || l.starts_with(".github/")
            || l.starts_with(".githooks/")
            || l.starts_with(".husky/")
            || l.contains("build-logic/")
            || l.contains("tooling/ci/")
            || l.starts_with("scripts/"))
            && [
                ".kt", ".kts", ".py", ".ts", ".js", ".mjs", ".sh", ".yml", ".yaml",
            ]
            .iter()
            .any(|x| l.ends_with(x))
    });
    for f in candidates {
        for line in read(&root.join(f)).lines() {
            if !line.to_lowercase().contains("branch") {
                continue;
            }
            let mut raw: Option<String> = None;
            if let Some(c) = QUOTED.captures(line) {
                let s = c[2].to_string();
                raw = Some(if &c[1] == "r" {
                    s
                } else {
                    s.replace("\\\\", "\\")
                });
            } else if let Some(c) = SLASHED.captures(line) {
                raw = Some(c[1].replace("\\/", "/"));
            }
            let Some(src) = raw.filter(|s| s.contains('/')) else {
                continue;
            };
            if let Ok(re) = Regex::new(&src)
                && !out.iter().any(|(s, _)| *s == src)
            {
                out.push((src, re));
            }
        }
    }
    out
}

/// A sample job branch built from the descriptor's template.
pub fn sample_branch(d: Option<&descriptor::Descriptor>) -> String {
    let template = d
        .and_then(|d| d.get("vcs.branch_template"))
        .unwrap_or("factory/{job-id}");
    let kind = d
        .and_then(|d| d.get("vcs.branch_kinds.feature"))
        .unwrap_or("feature");
    template
        .replace("{kind}", kind)
        .replace("{job-id}", "001-sample-change")
}

pub fn audit(factory: &Path, root: &Path) -> (Vec<Finding>, Value) {
    let mut f: Vec<Finding> = Vec::new();
    let mut add = |severity, area, message: String, action: &str| {
        f.push(Finding {
            severity,
            area,
            message,
            action: action.into(),
        })
    };
    let files = tracked_files(root);
    let desc = descriptor::find(root);
    let mut inventory = serde_json::Map::new();

    // 1. Agent instructions and IDE rules.
    let present: Vec<&str> = INSTRUCTION_FILES
        .iter()
        .copied()
        .filter(|p| root.join(p).exists())
        .collect();
    let dirs: Vec<Value> = AGENT_DIRS
        .iter()
        .filter(|d| root.join(d).is_dir())
        .map(|d| {
            let mut v = Vec::new();
            walk_files(&root.join(d), &mut v, 0);
            json!({"dir": d, "files": v.len()})
        })
        .collect();
    inventory.insert("instruction_files".into(), json!(present));
    inventory.insert("agent_dirs".into(), json!(dirs));
    add(
        Severity::Info,
        "instructions",
        format!(
            "{} instruction files ({}) and {} agent/rule folders",
            present.len(),
            present.join(", "),
            dirs.len()
        ),
        "Read every one before installing. Mirror the factory rule into each rule folder that follows a sync convention.",
    );
    let claude_md = read(&root.join("CLAUDE.md"));
    if !claude_md.is_empty() && root.join("AGENTS.md").exists() && !claude_md.contains("@AGENTS.md")
    {
        add(
            Severity::Warn,
            "instructions",
            "CLAUDE.md exists but does not import @AGENTS.md, so Claude Code never auto-loads AGENTS.md".into(),
            "Propose adding an @AGENTS.md line to CLAUDE.md (and to its mirrors if the repo keeps them in sync).",
        );
    }
    let mirror_dirs: Vec<&str> = [".cursor/rules", ".codex/rules", ".agents/rules"]
        .into_iter()
        .filter(|d| root.join(d).is_dir())
        .collect();
    let all_instr: String = present
        .iter()
        .map(|p| read(&root.join(p)))
        .collect::<Vec<_>>()
        .join("\n");
    if mirror_dirs.len() >= 2 || all_instr.to_lowercase().contains("in sync") {
        add(
            Severity::Conflict,
            "instructions",
            format!(
                "rule mirrors kept in sync by convention: {}",
                mirror_dirs.join(", ")
            ),
            "Adoption must add the factory rule to every mirror, not only .claude/rules.",
        );
    }

    // 2. Existing Claude Code config that the installer touches.
    for a in FACTORY_AGENTS {
        let p = root.join(format!(".claude/agents/{a}.md"));
        if p.exists() && !read(&p).contains("description: Factory") {
            add(
                Severity::Conflict,
                "claude",
                format!(".claude/agents/{a}.md exists and is not a factory agent"),
                "pf install keeps it as .claude/agents/<name>.pre-factory.md. Decide whether to merge its content into the product's AGENTS.md.",
            );
        }
    }
    let skill = root.join(".claude/skills/factory/SKILL.md");
    if skill.exists() && !read(&skill).contains("Product Factory") {
        add(
            Severity::Conflict,
            "claude",
            ".claude/skills/factory exists and is not the Product Factory skill".into(),
            "pf install keeps it as .claude/skills/factory.pre-factory.",
        );
    }
    for s in [".claude/settings.json", ".claude/settings.local.json"] {
        let Ok(v) = serde_json::from_str::<Value>(&read(&root.join(s))) else {
            continue;
        };
        if let Some(h) = v.get("hooks").and_then(|h| h.as_object()) {
            let n: usize = h
                .values()
                .filter_map(|x| x.as_array())
                .map(|a| a.len())
                .sum();
            if n > 0 {
                add(
                    Severity::Info,
                    "claude",
                    format!(
                        "{s} already defines {n} hook entr{}",
                        if n == 1 { "y" } else { "ies" }
                    ),
                    "Kept. The factory merges its guard next to them.",
                );
            }
        }
        let allow: Vec<String> = v
            .pointer("/permissions/allow")
            .and_then(|a| a.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        let risky: Vec<&String> = allow
            .iter()
            .filter(|p| {
                let l = p.to_lowercase();
                l == "bash"
                    || l == "bash(*)"
                    || l.contains("git push")
                    || l.contains("merge")
                    || l.contains("glab api")
                    || l.contains("gh api")
            })
            .collect();
        if !risky.is_empty() {
            add(
                Severity::Warn,
                "claude",
                format!(
                    "{s} pre-approves broad commands: {}",
                    risky
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                "The guard still blocks merges, tags and protected pushes, but review whether these approvals are still wanted.",
            );
        }
    }

    // 3. Git hooks.
    let hooks_path = util::output(
        "git",
        &[
            "-C",
            &root.to_string_lossy(),
            "config",
            "--get",
            "core.hooksPath",
        ],
        None,
    );
    let hook_dirs: Vec<&str> = [
        ".githooks",
        ".husky",
        "lefthook.yml",
        ".pre-commit-config.yaml",
    ]
    .into_iter()
    .filter(|d| root.join(d).exists())
    .collect();
    if !hook_dirs.is_empty() {
        let active = hooks_path.clone().unwrap_or_default();
        add(
            if active.is_empty() {
                Severity::Warn
            } else {
                Severity::Info
            },
            "hooks",
            format!(
                "git hooks in {} ({})",
                hook_dirs.join(", "),
                if active.is_empty() {
                    "NOT active: core.hooksPath unset".to_string()
                } else {
                    format!("active via core.hooksPath={active}")
                }
            ),
            "Factory commits and pushes run these hooks. They are the repo's gates, and the factory must pass them, never bypass them.",
        );
        let mut skips: Vec<String> = Vec::new();
        for d in &hook_dirs {
            let mut v = Vec::new();
            let p = root.join(d);
            if p.is_dir() {
                walk_files(&p, &mut v, 0)
            } else {
                v.push(p)
            }
            for file in v {
                for c in SKIP_VAR.captures_iter(&read(&file)) {
                    if !skips.contains(&c[1].to_string()) {
                        skips.push(c[1].to_string());
                    }
                }
            }
        }
        if !skips.is_empty() {
            add(
                Severity::Info,
                "hooks",
                format!("hook bypass switches exist: {}", skips.join(", ")),
                "Agents must never set these. The guard blocks them before git commands.",
            );
        }
    }
    inventory.insert(
        "hooks".into(),
        json!({"core.hooksPath": hooks_path, "dirs": hook_dirs}),
    );

    // 4. CI and auto-merge.
    let ci: Vec<&String> = files
        .iter()
        .filter(|f| {
            f.as_str() == ".gitlab-ci.yml"
                || f.starts_with(".github/workflows/")
                || f.starts_with(".ci/")
                || f.starts_with(".gitlab/ci/")
        })
        .collect();
    if !ci.is_empty() {
        add(
            Severity::Info,
            "ci",
            format!("{} CI files", ci.len()),
            "Kept unchanged. Changes to CI paths always escalate (risk.escalate_paths).",
        );
        let auto: Vec<&String> = ci
            .iter()
            .copied()
            .filter(|p| AUTOMERGE.is_match(&read(&root.join(p))))
            .collect();
        if !auto.is_empty() {
            add(
                Severity::Info,
                "ci",
                format!(
                    "approval-triggered auto-merge in {}",
                    auto.iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                "Compatible: the human approval is the merge decision, and the guard blocks agents from approving. Escalated (draft) MRs merge only after a human marks them ready.",
            );
        }
    }

    // 5. Branch and commit conventions.
    let patterns = branch_patterns(root, &files);
    let sample = sample_branch(desc.as_ref());
    if !patterns.is_empty() {
        let accepted = patterns.iter().any(|(_, re)| re.is_match(&sample));
        add(
            if accepted {
                Severity::Info
            } else {
                Severity::Blocker
            },
            "conventions",
            format!(
                "branch conventions found ({} patterns); factory job branch `{sample}` is {}",
                patterns.len(),
                if accepted { "accepted" } else { "REJECTED" }
            ),
            if accepted {
                "No change needed."
            } else {
                "Set vcs.branch_template and vcs.branch_kinds in the descriptor to a form these patterns accept (for example `{kind}/{job-id}` with feature/fix/docs/chore), or propose adding a factory pattern to the repo's conventions."
            },
        );
        inventory.insert(
            "branch_patterns".into(),
            json!(patterns.iter().map(|(s, _)| s).collect::<Vec<_>>()),
        );
    }
    let commit_rules: Vec<&String> = files
        .iter()
        .filter(|p| {
            let t = read(&root.join(p.as_str()));
            (p.contains("build-logic/")
                || p.starts_with(".ci/")
                || p.contains("tooling/ci/")
                || p.contains("commitlint"))
                && (t.contains("commitSubject")
                    || t.contains("COMMIT_SUBJECT")
                    || t.contains("commitlint")
                    || t.contains("conventional"))
        })
        .collect();
    if !commit_rules.is_empty() {
        let configured = desc
            .as_ref()
            .and_then(|d| d.get("vcs.commit_convention"))
            .is_some_and(|c| c != "none");
        add(
            if configured {
                Severity::Info
            } else {
                Severity::Conflict
            },
            "conventions",
            format!(
                "commit-message rules in {}",
                commit_rules
                    .iter()
                    .map(|s| s.as_str())
                    .take(3)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            "Set vcs.commit_convention (for example `conventional`) so agents write subjects the hooks accept.",
        );
    }

    // 6. Attribution.
    if NO_AI_ATTRIBUTION.is_match(&all_instr) {
        let settings = read(&root.join(".claude/settings.json"));
        let off = settings.contains("\"attribution\"");
        add(
            if off {
                Severity::Info
            } else {
                Severity::Conflict
            },
            "attribution",
            "instructions forbid AI attribution in commits or PRs".into(),
            if off {
                "Claude Code attribution already configured."
            } else {
                "Set vcs.attribution: none (pf install writes the setting)."
            },
        );
    }

    // 7. Other automations.
    let hermes = all_instr.to_lowercase().matches("hermes").count();
    if hermes > 0 || root.join("HERMES.md").exists() {
        add(
            Severity::Conflict,
            "automation",
            format!("Hermes automation is referenced ({hermes} mentions in instruction files)"),
            "Decide its role: pause its intake, or keep it for intake only. Update the instruction files to name the factory as the delivery pipeline. Never let both take the same issue.",
        );
    }
    for (dir, name) in [
        (".codex", "Codex"),
        (".cursor", "Cursor"),
        (".junie", "Junie"),
        (".gemini", "Gemini"),
        (".agents/skills", "agent skills"),
    ] {
        if root.join(dir).exists() {
            add(
                Severity::Info,
                "automation",
                format!("{name} configuration in {dir}"),
                "Kept. Mirror the factory rule where it holds rules.",
            );
        }
    }

    // 8. Secrets in agent config (tracked or not). Values are never printed.
    for d in SECRET_DIRS {
        let mut v = Vec::new();
        walk_files(&root.join(d), &mut v, 0);
        for p in v {
            if SECRET.is_match(&read(&p)) {
                let rel = util::rel(root, &p);
                let tracked = is_tracked(root, &rel);
                add(
                    if tracked {
                        Severity::Blocker
                    } else {
                        Severity::Warn
                    },
                    "secrets",
                    format!(
                        "{rel} contains a credential-like value ({})",
                        if tracked {
                            "TRACKED in git"
                        } else {
                            "untracked or ignored"
                        }
                    ),
                    "Rotate it, move it to an environment variable or secret store, and reference the variable. Agents must not read or copy it.",
                );
            }
        }
    }

    // 9. Docs routing markers in a foreign format.
    let docs_root = ["docs", "doc"].into_iter().find(|d| root.join(d).is_dir());
    if let Some(dr) = docs_root {
        let mut v = Vec::new();
        walk_files(&root.join(dr), &mut v, 0);
        let marked: Vec<PathBuf> = v
            .into_iter()
            .filter(|p| read(p).contains("<!-- documentation-routing:start -->"))
            .collect();
        let foreign = marked
            .iter()
            .filter(|p| !read(p).contains("- Agent guide: [AGENTS.md]"))
            .count();
        if foreign > 0 {
            add(
                Severity::Conflict,
                "docs",
                format!(
                    "{foreign} docs in {dr}/ use documentation-routing markers in another format"
                ),
                "Do not run pf docs-map --fix blindly: it would rewrite them. Decide per repo, and fix on a branch with review.",
            );
        }
        if dr != "docs" {
            add(
                Severity::Info,
                "docs",
                format!("docs live in {dr}/, not docs/"),
                "Use pf docs-map --docs doc, or propose a move.",
            );
        }
    }

    // 10. Knowledge structure for the module's stack.
    let t = crate::knowledge::target(root, None, None);
    if let Ok(p) = crate::knowledge::plan(factory, &t) {
        let gaps = p.present.iter().filter(|x| !x.gaps.is_empty()).count();
        if !p.missing.is_empty() || !p.conflicts.is_empty() || gaps > 0 {
            add(
                Severity::Warn,
                "knowledge",
                format!(
                    "knowledge structure ({} stack, {}/): {} present, {} missing, {} missing sections, {} numbering conflicts",
                    t.stack,
                    t.docs_root,
                    p.present.len(),
                    p.missing.len(),
                    gaps,
                    p.conflicts.len()
                ),
                "Initialize: pf knowledge plan shows the gaps; pf knowledge init imports missing files and appends missing sections for review (never overwrites); then adapt them in a docs job.",
            );
        }
    }

    // 11. Factory state.
    let installed = root.join(".factory/installed.lock").exists()
        || root.join(".claude/rules/product-factory.md").exists();
    add(
        Severity::Info,
        "factory",
        if installed {
            "factory already installed here".into()
        } else {
            "factory not installed yet".to_string()
        },
        "",
    );

    f.sort_by_key(|x| x.severity);
    let report = json!({
        "root": root,
        "inventory": inventory,
        "findings": f.iter().map(|x| json!({"severity": x.severity.label(), "area": x.area, "message": x.message, "action": x.action})).collect::<Vec<_>>(),
    });
    (f, report)
}

pub fn main(factory: &Path, path: Option<PathBuf>, as_json: bool, modules: bool) -> i32 {
    let start =
        path.unwrap_or_else(|| util::git_root(&std::env::current_dir().unwrap_or_default()));
    let (roots, missing) = crate::discovery::roots(&start, modules);
    for path in &missing {
        util::warn(format!("module unavailable: {}", path.display()));
    }
    let mut blockers = missing.len();
    let mut reports = Vec::new();
    for r in &roots {
        let (f, report) = audit(factory, r);
        blockers += f.iter().filter(|x| x.severity == Severity::Blocker).count();
        if !as_json {
            println!("Audit {}", r.display());
            for x in &f {
                println!("  {:<8} {:<12} {}", x.severity.label(), x.area, x.message);
                if !x.action.is_empty() {
                    println!("  {:<8} {:<12} → {}", "", "", x.action);
                }
            }
            println!();
        }
        reports.push(report);
    }
    if as_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!(reports)).unwrap_or_default()
        );
    } else {
        println!(
            "{blockers} blocker(s). Record the reconciliation in .factory/adoption.md (/factory adopt)."
        );
    }
    i32::from(blockers > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_kotlin_python_and_js_branch_regexes() {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let d = std::env::temp_dir().join(format!("pf-audit-{n}"));
        fs::create_dir_all(d.join("build-logic")).unwrap();
        fs::create_dir_all(d.join(".ci")).unwrap();
        fs::create_dir_all(d.join("tooling/ci")).unwrap();
        fs::write(d.join("build-logic/R.kt"), "    val ticketBranch: Regex = Regex(\"^(feature|fix)/[a-z0-9]+(-[a-z0-9]+)+$\")\n    val hermesBranch: Regex = Regex(\"^\\\\.hermes/[0-9]+-[a-z0-9]+$\")\n").unwrap();
        fs::write(
            d.join(".ci/c.py"),
            "WORK_BRANCH = r\"^(docs|chore)/[a-z0-9]+(-[a-z0-9]+)*$\"\n",
        )
        .unwrap();
        fs::write(
            d.join("tooling/ci/g.ts"),
            "const CODEX_BRANCH = /^codex\\/[a-z0-9]+$/;\n",
        )
        .unwrap();
        let files = vec![
            "build-logic/R.kt".to_string(),
            ".ci/c.py".to_string(),
            "tooling/ci/g.ts".to_string(),
        ];
        let p = branch_patterns(&d, &files);
        let srcs: Vec<&str> = p.iter().map(|(s, _)| s.as_str()).collect();
        assert!(
            srcs.contains(&"^(feature|fix)/[a-z0-9]+(-[a-z0-9]+)+$"),
            "{srcs:?}"
        );
        assert!(srcs.contains(&"^\\.hermes/[0-9]+-[a-z0-9]+$"), "{srcs:?}");
        assert!(
            srcs.contains(&"^(docs|chore)/[a-z0-9]+(-[a-z0-9]+)*$"),
            "{srcs:?}"
        );
        assert!(srcs.contains(&"^codex/[a-z0-9]+$"), "{srcs:?}");
        assert!(
            !p.iter()
                .any(|(_, re)| re.is_match("factory/001-sample-change"))
        );
        assert!(
            p.iter()
                .any(|(_, re)| re.is_match("feature/001-sample-change"))
        );
        fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn sample_branch_follows_the_template() {
        let d = descriptor::parse(
            "vcs:\n  branch_template: \"{kind}/{job-id}\"\n  branch_kinds: { feature: feature, bug: fix }\n",
        );
        assert_eq!(sample_branch(Some(&d)), "feature/001-sample-change");
        assert_eq!(sample_branch(None), "factory/001-sample-change");
    }
}
