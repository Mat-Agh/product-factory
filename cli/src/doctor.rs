//! `pf doctor`: read-only health check of this machine, the factory checkout and,
//! optionally, a product repo. The exit code is the number of failures.

use std::fs;
use std::path::Path;

use crate::{diagrams, docs_map, guard, install, util};

#[derive(Default)]
struct Doc {
    fails: i32,
}

impl Doc {
    fn ok(&self, m: impl AsRef<str>) {
        println!("  ok    {}", m.as_ref());
    }
    fn bad(&mut self, m: impl AsRef<str>) {
        println!("  FAIL  {}", m.as_ref());
        self.fails += 1;
    }
    fn note(&self, m: impl AsRef<str>) {
        println!("  note  {}", m.as_ref());
    }
    fn check(&mut self, cond: bool, ok: impl AsRef<str>, bad: impl AsRef<str>) {
        if cond { self.ok(ok) } else { self.bad(bad) }
    }
    fn soft(&self, cond: bool, ok: impl AsRef<str>, note: impl AsRef<str>) {
        if cond { self.ok(ok) } else { self.note(note) }
    }
}

pub fn main(factory: &Path, project: Option<&Path>) -> i32 {
    let mut d = Doc::default();
    let kit = factory.join("kit");

    println!("Machine");
    d.check(util::has_cmd("git"), "git", "git missing");
    d.check(
        util::has_cmd("pf"),
        "pf on PATH (the guard hook calls it)",
        format!(
            "pf not on PATH: cargo install --path {}",
            factory.join("cli").display()
        ),
    );
    for t in ["gh", "glab", "claude", "node"] {
        d.soft(util::has_cmd(t), t, format!("{t} not installed (optional)"));
    }
    if util::has_cmd("gh") {
        d.soft(
            util::output("gh", &["auth", "status"], None).is_some(),
            "gh authenticated",
            "gh not logged in (gh auth login)",
        );
    }
    let (passed, total, _) = guard::self_test();
    d.check(
        passed == total,
        format!("guard policy self-test {passed}/{total}"),
        format!("guard policy self-test {passed}/{total} (cargo test)"),
    );
    let dm = docs_map::run(&docs_map::Options {
        root: factory.to_path_buf(),
        docs: "docs".into(),
        fix: false,
        strict: true,
        allow: vec!["kit/".into()],
    });
    d.check(
        dm.errors.is_empty(),
        format!("factory {}", dm.summary()),
        "factory docs map stale or broken (pf docs-map --strict --allow kit/)",
    );
    let (n, df) = diagrams::check(factory);
    d.check(
        df.is_empty(),
        format!("{n} diagrams match their sources"),
        "diagrams stale (pf diagrams render)",
    );

    println!("Factory repo");
    let remotes = util::output("git", &["remote"], Some(factory))
        .map(|o| o.lines().count())
        .unwrap_or(0);
    d.soft(
        remotes >= 2,
        format!("{remotes} remotes (mirrored)"),
        format!("only {remotes} remote; add a mirror: pf mirror add <name> <url>"),
    );
    d.soft(
        util::output("git", &["status", "--porcelain"], Some(factory))
            .is_some_and(|o| o.is_empty()),
        "clean",
        "uncommitted changes in the factory repo",
    );
    if util::output(
        "git",
        &["rev-parse", "-q", "--verify", "@{u}"],
        Some(factory),
    )
    .is_some()
    {
        let _ = util::output("git", &["fetch", "-q"], Some(factory));
        let behind = util::output("git", &["rev-list", "--count", "HEAD..@{u}"], Some(factory))
            .unwrap_or_default();
        d.soft(
            behind == "0",
            "up to date with upstream",
            format!("{behind} commit(s) behind upstream; git pull"),
        );
    } else {
        d.note("no upstream branch set");
    }
    let home = util::claude_home();
    d.soft(
        home.join("skills/factory")
            .symlink_metadata()
            .is_ok_and(|m| m.file_type().is_symlink()),
        "user-level /factory linked",
        "user-level not linked (pf install user)",
    );

    if let Some(p) = project {
        let p = util::absolute(p);
        println!("Project {}", p.display());
        for f in [
            ".factory/factory.yaml",
            ".factory/backlog.md",
            ".claude/skills/factory/SKILL.md",
            ".claude/skills/factory/CONSTITUTION.md",
            "AGENTS.md",
        ] {
            d.check(
                p.join(f).exists(),
                f,
                format!("{f} missing (pf install project {})", p.display()),
            );
        }
        let read = |f: &str| fs::read_to_string(p.join(f)).unwrap_or_default();
        d.check(
            read(".gitignore").lines().any(|l| l == ".factory/jobs/"),
            ".factory/jobs/ ignored",
            ".factory/jobs/ not in .gitignore",
        );
        let settings = read(".claude/settings.json");
        d.check(
            settings.contains("pf guard"),
            "guard hook wired (pf guard)",
            "guard hook not in .claude/settings.json (pf install project)",
        );
        if settings.contains("factory-guard.py") {
            d.bad("legacy Python guard still wired (re-run pf install project)");
        }
        let agents_md = read("AGENTS.md");
        d.check(
            agents_md
                .lines()
                .any(|l| l.starts_with("## Product Factory")),
            "AGENTS.md factory section",
            "AGENTS.md lacks the Product Factory section",
        );
        for s in ["Setup", "Dev server", "Gates"] {
            let found = agents_md
                .lines()
                .any(|l| l.starts_with('#') && l.to_lowercase().contains(&s.to_lowercase()));
            d.soft(
                found,
                format!("AGENTS.md has a {s} heading"),
                format!("AGENTS.md: no '{s}' heading yet (the factory needs it)"),
            );
        }
        let mut drift = false;
        let mut compare = |src: &Path, rel: &str, expected: Option<String>| {
            let want = expected.or_else(|| fs::read_to_string(src).ok());
            if want != fs::read_to_string(p.join(rel)).ok() {
                drift = true;
                d.note(format!("drift: {rel} differs from factory kit"));
            }
        };
        for a in install::agent_files(factory) {
            let rel = format!(
                ".claude/agents/{}",
                a.file_name().unwrap().to_string_lossy()
            );
            compare(&a, &rel, None);
        }
        for f in ["SKILL.md", "protocol.md"] {
            compare(
                &kit.join(".claude/skills/factory").join(f),
                &format!(".claude/skills/factory/{f}"),
                None,
            );
        }
        let cons = kit.join(".claude/skills/factory/CONSTITUTION.md");
        compare(
            &cons,
            ".claude/skills/factory/CONSTITUTION.md",
            fs::read_to_string(&cons)
                .ok()
                .map(|t| install::shipped_constitution(&t)),
        );
        compare(
            &kit.join(".factory/dashboard.html"),
            ".factory/dashboard.html",
            None,
        );
        d.soft(
            !drift,
            format!("kit matches factory @ {}", install::revision(factory)),
            format!(
                "re-run pf install project {} to update (review the diff first)",
                p.display()
            ),
        );
        if p.join("docs").is_dir() {
            let r = docs_map::run(&docs_map::Options {
                root: util::git_root(&p),
                docs: "docs".into(),
                fix: false,
                strict: false,
                allow: vec![],
            });
            d.soft(
                r.errors.is_empty(),
                r.summary(),
                format!("docs: {} (pf docs-map --fix)", r.summary()),
            );
        } else {
            d.note("no docs/ folder yet (pf docs-map --fix creates it)");
        }
        let is_github = util::output("git", &["remote", "get-url", "origin"], Some(&p))
            .is_some_and(|u| u.contains("github"));
        if util::has_cmd("gh") && is_github {
            let base = read(".factory/factory.yaml")
                .lines()
                .find_map(|l| {
                    l.trim_start()
                        .strip_prefix("base_branch:")
                        .map(|v| v.split('#').next().unwrap_or("").trim().to_string())
                })
                .unwrap_or_default();
            let repo = util::output(
                "gh",
                &[
                    "repo",
                    "view",
                    "--json",
                    "nameWithOwner",
                    "-q",
                    ".nameWithOwner",
                ],
                Some(&p),
            )
            .unwrap_or_default();
            if !repo.is_empty() && !base.is_empty() {
                let protected = util::output(
                    "gh",
                    &["api", &format!("repos/{repo}/branches/{base}/protection")],
                    None,
                )
                .is_some();
                d.soft(
                    protected,
                    format!("{base} is protected on GitHub"),
                    format!(
                        "{base} has no branch protection on GitHub (or no permission to read it)"
                    ),
                );
            }
        }
    }
    println!();
    if d.fails == 0 {
        println!("Healthy.")
    } else {
        println!("{} failure(s).", d.fails)
    }
    d.fails
}
