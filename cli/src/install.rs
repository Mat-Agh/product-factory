//! `pf install`: put the factory into a product repo (a committed copy) or into a user's
//! Claude Code config (symlinks that follow `git pull`). Never commits, never overwrites a
//! project's settings.json (it merges), and never overwrites an existing descriptor or backlog.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;
use serde_json::{Value, json};

use crate::descriptor;
use crate::util::{self, say, warn};

/// The hook command. It fails closed: without `pf` on PATH every Bash call is blocked
/// with an install hint, rather than silently running unguarded.
pub const HOOK_CMD: &str = "command -v pf >/dev/null 2>&1 || { echo 'factory-guard: the pf CLI is not installed, so Bash is blocked (fail closed). Install it: cargo install --git ssh://git@github.com/Mat-Agh/product-factory.git pf' >&2; exit 2; }; pf guard";
const LEGACY_HOOK_MARK: &str = "factory-guard.py";

static ROUTING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?s)\n*<!-- documentation-routing:start -->.*?<!-- documentation-routing:end -->\n?",
    )
    .unwrap()
});

pub fn revision(factory: &Path) -> String {
    util::output("git", &["rev-parse", "--short", "HEAD"], Some(factory))
        .unwrap_or_else(|| "unknown".into())
}

fn kit(factory: &Path) -> PathBuf {
    factory.join("kit")
}

pub fn agent_files(factory: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = fs::read_dir(kit(factory).join(".claude/agents"))
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "md"))
        .collect();
    v.sort();
    v
}

/// The constitution as shipped to products: the factory-repo routing block removed,
/// because its links only resolve inside the factory repo.
pub fn shipped_constitution(text: &str) -> String {
    ROUTING.replace_all(text, "\n").into_owned()
}

/// Copy a directory tree, following symlinks (the kit links the constitution from docs/).
fn copy_tree(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for e in fs::read_dir(src)? {
        let e = e?;
        let from = e.path();
        let to = dst.join(e.file_name());
        if fs::metadata(&from)?.is_dir() {
            copy_tree(&from, &to)?
        } else {
            fs::copy(&from, &to).map(|_| ())?
        }
    }
    Ok(())
}

/// Add the guard hook to a settings.json, removing legacy Python-guard entries. Returns a message.
pub fn merge_hook(settings: &Path, cmd: &str) -> Result<String, String> {
    let mut v: Value = match fs::read_to_string(settings) {
        Ok(s) if !s.trim().is_empty() => {
            serde_json::from_str(&s).map_err(|e| format!("{}: {e}", settings.display()))?
        }
        _ => json!({}),
    };
    if !v.is_object() {
        return Err(format!("{} is not a JSON object", settings.display()));
    }
    let hooks = v
        .as_object_mut()
        .unwrap()
        .entry("hooks")
        .or_insert_with(|| json!({}));
    let pre = hooks
        .as_object_mut()
        .ok_or("hooks is not an object")?
        .entry("PreToolUse")
        .or_insert_with(|| json!([]));
    let list = pre
        .as_array_mut()
        .ok_or("hooks.PreToolUse is not an array")?;
    let before = list.len();
    list.retain(|entry| {
        !entry["hooks"].as_array().is_some_and(|hs| {
            hs.iter().any(|h| {
                h["command"]
                    .as_str()
                    .is_some_and(|c| c.contains(LEGACY_HOOK_MARK))
            })
        })
    });
    let removed = before - list.len();
    let present = list.iter().any(|e| {
        e["hooks"]
            .as_array()
            .is_some_and(|hs| hs.iter().any(|h| h["command"] == cmd))
    });
    let msg = if present {
        format!("guard hook already wired in {}", settings.display())
    } else {
        list.push(json!({"matcher": "Bash", "hooks": [{"type": "command", "command": cmd}]}));
        format!("wired guard hook (pf guard) into {}", settings.display())
    };
    if !present || removed > 0 {
        if let Some(d) = settings.parent() {
            fs::create_dir_all(d).map_err(|e| e.to_string())?;
        }
        let body = serde_json::to_string_pretty(&v).map_err(|e| e.to_string())? + "\n";
        fs::write(settings, body).map_err(|e| e.to_string())?;
    }
    Ok(if removed > 0 {
        format!(
            "{msg} (removed {removed} legacy Python guard entr{})",
            if removed == 1 { "y" } else { "ies" }
        )
    } else {
        msg
    })
}

fn remove_legacy(path: &Path, what: &str) {
    if path.symlink_metadata().is_ok() && fs::remove_file(path).is_ok() {
        say(format!("removed legacy {what}"));
    }
}

/// The settings value that hides all AI attribution, readable by old and new Claude Code.
pub fn no_attribution_value() -> Value {
    json!({"commit": "", "pr": "", "sessionUrl": false})
}

/// Set `attribution` to hide AI trailers and PR lines. Returns a message.
pub fn merge_attribution(settings: &Path) -> Result<String, String> {
    let mut v: Value = match fs::read_to_string(settings) {
        Ok(s) if !s.trim().is_empty() => {
            serde_json::from_str(&s).map_err(|e| format!("{}: {e}", settings.display()))?
        }
        _ => json!({}),
    };
    let obj = v
        .as_object_mut()
        .ok_or_else(|| format!("{} is not a JSON object", settings.display()))?;
    if obj.get("attribution") == Some(&no_attribution_value()) {
        return Ok(format!(
            "AI attribution already off in {}",
            settings.display()
        ));
    }
    obj.insert("attribution".into(), no_attribution_value());
    let body = serde_json::to_string_pretty(&v).map_err(|e| e.to_string())? + "\n";
    fs::write(settings, body).map_err(|e| e.to_string())?;
    Ok(format!(
        "turned AI attribution off in {} (vcs.attribution: none)",
        settings.display()
    ))
}

/// The Claude Code side of an install: the /factory skill with the constitution, the
/// agents, the always-on factory rule, the guard hook and the attribution policy.
fn install_claude_kit(factory: &Path, root: &Path, attribution_none: bool) -> Result<(), String> {
    let kit = kit(factory);
    let skill = root.join(".claude/skills/factory");
    if skill.exists() {
        fs::remove_dir_all(&skill).map_err(|e| e.to_string())?;
    }
    copy_tree(&kit.join(".claude/skills/factory"), &skill).map_err(|e| e.to_string())?;
    let constitution = skill.join("CONSTITUTION.md");
    let text = fs::read_to_string(&constitution).map_err(|e| e.to_string())?;
    fs::write(&constitution, shipped_constitution(&text)).map_err(|e| e.to_string())?;
    let agents_dir = root.join(".claude/agents");
    fs::create_dir_all(&agents_dir).map_err(|e| e.to_string())?;
    let agents = agent_files(factory);
    for a in &agents {
        fs::copy(a, agents_dir.join(a.file_name().unwrap())).map_err(|e| e.to_string())?;
    }
    let rules_dir = root.join(".claude/rules");
    fs::create_dir_all(&rules_dir).map_err(|e| e.to_string())?;
    fs::copy(
        kit.join(RULE),
        root.join(".claude")
            .join(RULE.trim_start_matches(".claude/")),
    )
    .map_err(|e| e.to_string())?;
    say(format!(
        "copied the /factory skill (with the constitution), {} agents and the always-on factory rule",
        agents.len()
    ));
    remove_legacy(
        &root.join(".claude/hooks/factory-guard.py"),
        "Python guard .claude/hooks/factory-guard.py",
    );
    let settings = root.join(".claude/settings.json");
    say(merge_hook(&settings, HOOK_CMD)?);
    if attribution_none {
        say(merge_attribution(&settings)?);
    }
    Ok(())
}

/// Install into a module repo of a multi-repo workspace: the Claude Code kit only, so a
/// session opened inside the module also runs under the factory. The workspace keeps the
/// descriptor, backlog and job state; the module's own AGENTS.md and CLAUDE.md are untouched.
pub fn module(factory: &Path, target: &Path, workspace: &Path, no_attribution: bool) -> i32 {
    let root = util::absolute(target);
    let ws = util::absolute(workspace);
    if util::output("git", &["rev-parse", "--show-toplevel"], Some(&root)).as_deref()
        != Some(root.to_string_lossy().as_ref())
    {
        warn(format!("{} is not the top of a git repo", root.display()));
        return 1;
    }
    let Some(d) = descriptor::find(&ws).filter(|d| d.root == ws) else {
        warn(format!(
            "no .factory/factory.yaml in {} (pf install project it first)",
            ws.display()
        ));
        return 1;
    };
    if !d.multi_repo() {
        warn(format!(
            "{}/.factory/factory.yaml is not layout: multi-repo",
            ws.display()
        ));
        return 1;
    }
    let rel = util::rel(&ws, &root);
    if !d.modules.iter().any(|m| {
        m.get("path")
            .is_some_and(|p| p.trim_end_matches('/') == rel)
    }) {
        warn(format!(
            "{rel} is not listed under modules: in the workspace descriptor; add it first"
        ));
        return 1;
    }
    println!(
        "Installing Product Factory @ {} into module {rel} of {}",
        revision(factory),
        ws.display()
    );
    if let Err(e) = install_claude_kit(factory, &root, d.attribution_none() || no_attribution) {
        warn(e);
        return 1;
    }
    println!(
        "Done. Nothing was committed. Sessions opened in {rel} now use the workspace factory."
    );
    0
}

const RULE: &str = ".claude/rules/product-factory.md";

pub fn project(factory: &Path, target: &Path, no_attribution: bool) -> i32 {
    let root = util::absolute(target);
    if util::output("git", &["rev-parse", "--show-toplevel"], Some(&root)).is_none() {
        warn(format!("{} is not a git repo", root.display()));
        return 1;
    }
    let kit = kit(factory);
    println!(
        "Installing Product Factory @ {} into {}",
        revision(factory),
        root.display()
    );
    let step = || -> Result<(), String> {
        let attribution_none = descriptor::find(&root)
            .filter(|d| d.root == root)
            .is_some_and(|d| d.attribution_none());
        install_claude_kit(factory, &root, attribution_none || no_attribution)?;

        let fdir = root.join(".factory");
        fs::create_dir_all(&fdir).map_err(|e| e.to_string())?;
        for f in ["factory.yaml", "backlog.md"] {
            if fdir.join(f).exists() {
                say(format!("kept existing .factory/{f}"));
            } else {
                fs::copy(kit.join(".factory").join(f), fdir.join(f)).map_err(|e| e.to_string())?;
                say(format!(
                    "created .factory/{f} (run /factory init to fill it)"
                ));
            }
        }
        fs::copy(
            kit.join(".factory/dashboard.html"),
            fdir.join("dashboard.html"),
        )
        .map_err(|e| e.to_string())?;
        remove_legacy(
            &fdir.join("docs-map.py"),
            "Python .factory/docs-map.py (now: pf docs-map)",
        );

        let gi = root.join(".gitignore");
        let ignore = fs::read_to_string(&gi).unwrap_or_default();
        if !ignore.lines().any(|l| l == ".factory/jobs/") {
            let sep = if ignore.is_empty() || ignore.ends_with('\n') {
                ""
            } else {
                "\n"
            };
            fs::write(&gi, format!("{ignore}{sep}.factory/jobs/\n")).map_err(|e| e.to_string())?;
            say("added .factory/jobs/ to .gitignore");
        }

        let agents_md = root.join("AGENTS.md");
        let current = fs::read_to_string(&agents_md).unwrap_or_default();
        if current.lines().any(|l| l.starts_with("## Product Factory")) {
            say("AGENTS.md already has the Product Factory section");
        } else {
            let section =
                fs::read_to_string(kit.join("AGENTS.factory.md")).map_err(|e| e.to_string())?;
            let section: String = section.lines().skip(1).map(|l| format!("{l}\n")).collect();
            let sep = if current.is_empty() { "" } else { "\n" };
            fs::write(&agents_md, format!("{current}{sep}{section}")).map_err(|e| e.to_string())?;
            say("appended the Product Factory section to AGENTS.md");
        }
        if !root.join("CLAUDE.md").exists() {
            fs::write(root.join("CLAUDE.md"), "@AGENTS.md\n").map_err(|e| e.to_string())?;
            say("created CLAUDE.md -> @AGENTS.md");
        }
        let source = util::output("git", &["remote", "get-url", "origin"], Some(factory))
            .unwrap_or_else(|| factory.display().to_string());
        fs::write(
            fdir.join("installed.lock"),
            format!(
                "source: {source}\nrevision: {}\ninstalled: {}\n",
                revision(factory),
                util::now_utc_iso()
            ),
        )
        .map_err(|e| e.to_string())
    };
    if let Err(e) = step() {
        warn(e);
        return 1;
    }
    println!(
        "Done. Nothing was committed. Next: open Claude Code in {}, run /factory init, review, commit.",
        root.display()
    );
    println!("Docs map: pf docs-map (add --fix to generate README links and the map).");
    if !util::has_cmd("pf") {
        warn(
            "pf is not on PATH: the guard hook will block Bash in this project until it is (cargo install --path cli).",
        );
    }
    0
}

#[cfg(unix)]
fn link(src: &Path, dst: &Path) {
    match dst.symlink_metadata() {
        Ok(m) if !m.file_type().is_symlink() => {
            warn(format!(
                "{} exists and is not a link; left alone",
                dst.display()
            ));
            return;
        }
        Ok(_) => {
            let _ = fs::remove_file(dst);
        }
        Err(_) => {}
    }
    if let Err(e) = std::os::unix::fs::symlink(src, dst) {
        warn(format!("cannot link {}: {e}", dst.display()));
    }
}

#[cfg(not(unix))]
fn link(_src: &Path, dst: &Path) {
    warn(format!(
        "symlinks need a Unix-like system (use WSL); skipped {}",
        dst.display()
    ));
}

pub fn user(factory: &Path, with_guard: bool) -> i32 {
    let home = util::claude_home();
    let kit = kit(factory);
    println!(
        "Linking Product Factory @ {} into {}",
        revision(factory),
        home.display()
    );
    for d in ["skills", "agents"] {
        let _ = fs::create_dir_all(home.join(d));
    }
    link(
        &kit.join(".claude/skills/factory"),
        &home.join("skills/factory"),
    );
    for a in agent_files(factory) {
        link(&a, &home.join("agents").join(a.file_name().unwrap()));
    }
    let legacy = home.join("hooks/factory-guard.py");
    if legacy
        .symlink_metadata()
        .is_ok_and(|m| m.file_type().is_symlink())
    {
        remove_legacy(&legacy, "user link hooks/factory-guard.py");
    }
    say(format!(
        "linked /factory skill and agents (they follow 'git pull' in {})",
        factory.display()
    ));

    let claude_md = home.join("CLAUDE.md");
    let line = format!("@{}", factory.join("docs/operator/USER.md").display());
    let text = fs::read_to_string(&claude_md).unwrap_or_default();
    if !text.lines().any(|l| l == line) {
        let _ = fs::write(
            &claude_md,
            format!("{text}\n# Product Factory operator profile\n{line}\n"),
        );
        say(format!(
            "imported docs/operator/USER.md into {}",
            claude_md.display()
        ));
    }
    let settings = home.join("settings.json");
    let legacy_wired = fs::read_to_string(&settings).is_ok_and(|s| s.contains(LEGACY_HOOK_MARK));
    if with_guard || legacy_wired {
        match merge_hook(&settings, HOOK_CMD) {
            Ok(m) => say(m),
            Err(e) => warn(e),
        }
    } else {
        say("user-wide guard hook not enabled (pass --with-guard to apply it in every project)");
    }
    if !util::has_cmd("pf") {
        warn(format!(
            "pf is not on PATH; run: cargo install --path {}",
            factory.join("cli").display()
        ));
    }
    println!("Done.");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let d = std::env::temp_dir().join(format!("pf-{name}-{n}"));
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn merge_hook_is_idempotent_and_replaces_legacy() {
        let d = tmp("settings");
        let s = d.join("settings.json");
        fs::write(&s, r#"{"permissions":{"allow":["Bash(ls)"]},"hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"python3 \"$CLAUDE_PROJECT_DIR/.claude/hooks/factory-guard.py\""}]}]}}"#).unwrap();
        let first = merge_hook(&s, HOOK_CMD).unwrap();
        assert!(first.contains("removed 1 legacy"), "{first}");
        let second = merge_hook(&s, HOOK_CMD).unwrap();
        assert!(second.contains("already wired"), "{second}");
        let v: Value = serde_json::from_str(&fs::read_to_string(&s).unwrap()).unwrap();
        assert_eq!(v["hooks"]["PreToolUse"].as_array().unwrap().len(), 1);
        assert_eq!(v["permissions"]["allow"][0], "Bash(ls)");
        fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn constitution_ships_without_repo_links() {
        let t = "# C\n\nbody\n\n<!-- documentation-routing:start -->\n- [x](../README.md)\n<!-- documentation-routing:end -->\n";
        assert_eq!(shipped_constitution(t), "# C\n\nbody\n");
    }
}
