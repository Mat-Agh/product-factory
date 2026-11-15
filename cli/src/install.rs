//! `pf install`: put the factory into a product repo (a committed copy) or into a user's
//! Claude Code config (symlinks that follow `git pull`). Never commits, never overwrites a
//! project's settings.json (it merges), and never overwrites an existing descriptor or backlog.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::LazyLock;

use regex::Regex;
use serde_json::{Value, json};

use crate::util::{self, say, warn};
use crate::{bundle, descriptor, discovery, docs_map, graph, knowledge, rules, runtime};

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
    bundle::revision(factory)
        .or_else(|| util::output("git", &["rev-parse", "--short", "HEAD"], Some(factory)))
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
fn install_claude_kit(
    factory: &Path,
    root: &Path,
    roles: &[runtime::Role],
    attribution_none: bool,
) -> Result<(), String> {
    let kit = kit(factory);
    let skill = root.join(".claude/skills/factory");
    if skill.exists() {
        let ours =
            fs::read_to_string(skill.join("SKILL.md")).is_ok_and(|t| t.contains("Product Factory"));
        if !ours {
            // Never destroy someone else's skill that happens to share the name.
            let keep = root.join(".claude/skills/factory.pre-factory");
            fs::rename(&skill, &keep).map_err(|e| e.to_string())?;
            warn(format!(
                "kept an existing non-factory skill as {}",
                keep.display()
            ));
        }
    }
    copy_tree(&kit.join(".claude/skills/factory"), &skill).map_err(|e| e.to_string())?;
    let constitution = skill.join("CONSTITUTION.md");
    let text = fs::read_to_string(&constitution).map_err(|e| e.to_string())?;
    fs::write(&constitution, shipped_constitution(&text)).map_err(|e| e.to_string())?;
    let agents_dir = root.join(".claude/agents");
    fs::create_dir_all(&agents_dir).map_err(|e| e.to_string())?;
    for role in roles {
        let dst = agents_dir.join(&role.filename);
        let foreign = fs::read_to_string(&dst).is_ok_and(|t| !t.contains("description: Factory"));
        if foreign {
            let keep = dst.with_extension("pre-factory.md");
            fs::rename(&dst, &keep).map_err(|e| e.to_string())?;
            warn(format!(
                "kept an existing non-factory agent as {}",
                keep.display()
            ));
        }
        fs::write(&dst, &role.claude).map_err(|e| e.to_string())?;
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
        roles.len()
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
/// descriptor, backlog and job state. Existing module instructions are preserved.
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
    if root == ws
        || !discovery::local_path(&ws, Path::new(&rel))
        || !d.modules.iter().any(|m| {
            m.get("path")
                .is_some_and(|p| p.trim_end_matches('/') == rel)
        })
    {
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
    if let Err(e) = install_runtimes(factory, &root, &d, no_attribution) {
        warn(e);
        return 1;
    }
    graph::install(&root);
    println!(
        "Done. Nothing was committed. Sessions opened in {rel} now use the workspace factory."
    );
    0
}

const RULE: &str = ".claude/rules/product-factory.md";

fn install_runtimes(
    factory: &Path,
    root: &Path,
    d: &descriptor::Descriptor,
    no_attribution: bool,
) -> Result<(), String> {
    let roles = runtime::roles(factory, d)?;
    let paths = [
        ".claude/settings.json",
        ".claude/skills/factory/SKILL.md",
        ".claude/skills/factory/protocol.md",
        ".claude/skills/factory/CONSTITUTION.md",
        ".claude/skills/factory.pre-factory",
        RULE,
    ];
    for relative in paths
        .into_iter()
        .map(PathBuf::from)
        .chain(roles.iter().flat_map(|role| {
            let file = Path::new(".claude/agents").join(&role.filename);
            [file.with_extension("pre-factory.md"), file]
        }))
    {
        if !discovery::local_path(root, &relative) {
            return Err(format!(
                "Unsafe install path: {}",
                root.join(relative).display()
            ));
        }
    }
    install_entrypoints(factory, root)?;
    runtime::install_shared(factory, root, &roles)?;
    install_claude_kit(
        factory,
        root,
        &roles,
        d.attribution_none() || no_attribution,
    )
}

fn install_entrypoints(factory: &Path, root: &Path) -> Result<(), String> {
    const MARKER: &str = "<!-- product-factory-entrypoints -->";
    const OLD_DOCS: &str = "- **Documentation:** all docs live under `docs/`, one folder per area, each\n  with a `README.md`. After adding or moving a doc, run\n  `pf docs-map --fix`, which regenerates the README links and\n  the map in `docs/README.md`.";
    const NEW_DOCS: &str = "- **Documentation:** preserve this repository's existing documentation root\n  and authority map. Run `pf docs-map --docs <root> --fix` only when that\n  folder uses the Factory's README routing format; review inherited links.";
    for path in ["AGENTS.md", "CLAUDE.md"] {
        if !discovery::local_path(root, Path::new(path)) {
            return Err(format!(
                "Unsafe install path: {}",
                root.join(path).display()
            ));
        }
    }
    let agents = root.join("AGENTS.md");
    let current = match fs::read_to_string(&agents) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e.to_string()),
    };
    if !current.contains(MARKER) {
        let template = fs::read_to_string(kit(factory).join("AGENTS.factory.md"))
            .map_err(|e| e.to_string())?;
        let section = if current.lines().any(|l| l == "## Product Factory") {
            template
                .split_once(MARKER)
                .ok_or("Missing kit entrypoints")?
                .1
        } else {
            template
                .split_once('\n')
                .ok_or("Missing kit entrypoints")?
                .1
        };
        let marker = if section.contains(MARKER) { "" } else { MARKER };
        let sep = if current.is_empty() { "" } else { "\n\n" };
        fs::write(agents, format!("{current}{sep}{marker}{section}")).map_err(|e| e.to_string())?;
        say("installed Product Factory entrypoints in AGENTS.md");
    } else if current.contains(OLD_DOCS) {
        fs::write(agents, current.replace(OLD_DOCS, NEW_DOCS)).map_err(|e| e.to_string())?;
        say("updated Product Factory documentation guidance in AGENTS.md");
    }
    append_entrypoint(root, "GEMINI.md", "@./AGENTS.md")?;
    // A Hermes-specific context file takes precedence over AGENTS.md.
    for file in [".hermes.md", "HERMES.md"] {
        if root.join(file).is_file() {
            append_entrypoint(
                root,
                file,
                "Read AGENTS.md and follow its Product Factory entrypoint. Load the product-factory project skill before working on a change.",
            )?;
        }
    }
    if !root.join("CLAUDE.md").exists() {
        fs::write(root.join("CLAUDE.md"), "@AGENTS.md\n").map_err(|e| e.to_string())?;
        say("created CLAUDE.md -> @AGENTS.md");
    }
    Ok(())
}

fn append_entrypoint(root: &Path, name: &str, instructions: &str) -> Result<(), String> {
    let path = root.join(name);
    if !discovery::local_path(root, Path::new(name)) {
        return Err(format!("Unsafe install path: {}", path.display()));
    }
    let current = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e.to_string()),
    };
    const MARKER: &str = "<!-- product-factory-context -->";
    if !current.contains(MARKER) && !current.lines().any(|line| line.trim() == instructions) {
        let sep = if current.is_empty() { "" } else { "\n\n" };
        fs::write(path, format!("{current}{sep}{MARKER}\n{instructions}\n"))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Draft topology and knowledge settings. Adoption supplies VCS and process rules.
fn draft_descriptor(factory: &Path, root: &Path) -> Result<String, String> {
    let template = fs::read_to_string(kit(factory).join(".factory/factory.yaml"))
        .map_err(|e| e.to_string())?;
    let (roots, missing) = discovery::inferred_roots(root);
    if !missing.is_empty() {
        return Err(format!(
            "Check out workspace modules before installing: {missing:?}"
        ));
    }
    let workspace = roots.iter().filter(|p| p.join(".git").exists()).count() > 1;
    let (before, rest) = template
        .split_once("modules:\n")
        .ok_or("Missing modules template")?;
    let (_, after) = rest.split_once("\nvcs:").ok_or("Missing vcs template")?;
    let mut out = format!("# pf: draft descriptor (review with factory init/adopt)\n{before}");
    if workspace {
        out = out.replacen("layout: single-repo", "layout: multi-repo", 1);
    }
    out.push_str("modules:\n");
    let mut areas = std::collections::BTreeSet::new();
    for repo in roots.iter().filter(|repo| !workspace || *repo != root) {
        let path = util::rel(root, repo);
        let slug: String = path
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' {
                    c.to_ascii_lowercase()
                } else {
                    '-'
                }
            })
            .collect();
        let mut area = if path == "." {
            "app".into()
        } else {
            slug.clone()
        };
        let mut suffix = 2;
        while !areas.insert(area.clone()) {
            area = format!("{slug}-{suffix}");
            suffix += 1;
        }
        if path.contains(['\r', '\n', '"', '\\']) {
            return Err(format!(
                "Module path cannot be represented in the descriptor: {path:?}"
            ));
        }
        let stack = discovery::infer_stack(repo);
        let docs = discovery::docs_root(repo, stack);
        out.push_str(&format!(
            "  - area: \"{area}\"\n    path: \"{path}\"\n    owner_role: engineer\n    agents_md: AGENTS.md\n    stack: {stack}\n    docs_root: {docs}\n    gates: see AGENTS.md\n    tests: yes # verify\n"
        ));
    }
    out.push_str("\nvcs:");
    out.push_str(after);
    Ok(out)
}

pub fn project(factory: &Path, target: &Path, no_attribution: bool, modules: bool) -> i32 {
    let root = util::absolute(target);
    if !root.join(".git").exists() || util::git_root(&root) != root {
        warn(format!("{} is not a git repo", root.display()));
        return 1;
    }
    let kit = kit(factory);
    println!(
        "Installing Product Factory @ {} into {}",
        revision(factory),
        root.display()
    );
    let step = || -> Result<Vec<PathBuf>, String> {
        let fdir = root.join(".factory");
        for path in [
            ".factory/factory.yaml",
            ".factory/backlog.md",
            ".factory/dashboard.html",
            ".factory/installed.lock",
            ".gitignore",
            "AGENTS.md",
            "CLAUDE.md",
        ] {
            if !discovery::local_path(&root, Path::new(path)) {
                return Err(format!(
                    "Unsafe install path: {}",
                    root.join(path).display()
                ));
            }
        }
        let config = fdir.join("factory.yaml");
        let (text, draft) = match fs::read_to_string(&config) {
            Ok(text) => (text, false),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                (draft_descriptor(factory, &root)?, true)
            }
            Err(e) => return Err(e.to_string()),
        };
        let d = descriptor::parse(&text);
        let mut children = Vec::new();
        if modules && d.multi_repo() {
            if d.modules.iter().any(|m| !m.contains_key("path")) {
                return Err("Descriptor module has no path".into());
            }
            let (targets, missing) = if draft {
                discovery::inferred_roots(&root)
            } else {
                discovery::roots(&root, true)
            };
            if !missing.is_empty() {
                return Err(format!(
                    "Check out workspace modules before installing: {missing:?}"
                ));
            }
            children.extend(
                targets
                    .into_iter()
                    .filter(|p| p != &root && p.join(".git").exists() && util::git_root(p) == *p),
            );
        }
        install_runtimes(factory, &root, &d, no_attribution)?;

        fs::create_dir_all(&fdir).map_err(|e| e.to_string())?;
        if draft {
            fs::write(&config, text).map_err(|e| e.to_string())?;
            say("created a draft .factory/factory.yaml (review with factory init/adopt)");
        } else {
            say("kept existing .factory/factory.yaml");
        }
        for f in ["backlog.md"] {
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

        let source = if bundle::revision(factory).is_some() {
            bundle::SOURCE_URL.into()
        } else {
            util::output("git", &["remote", "get-url", "origin"], Some(factory))
                .unwrap_or_else(|| factory.display().to_string())
        };
        fs::write(
            fdir.join("installed.lock"),
            format!(
                "source: {source}\nrevision: {}\ninstalled: {}\n",
                revision(factory),
                util::now_utc_iso()
            ),
        )
        .map_err(|e| e.to_string())?;
        if let Some(descriptor) = descriptor::find(&root).filter(|d| d.root == root) {
            rules::sync(&descriptor).map_err(|e| e.to_string())?;
        }
        Ok(children)
    };
    let children = match step() {
        Ok(children) => children,
        Err(e) => {
            warn(e);
            return 1;
        }
    };
    graph::install(&root);
    if descriptor::find(&root).is_some_and(|d| d.multi_repo()) {
        say("multi-repo workspace: keeping stack knowledge in child repositories");
        prepare_docs_map(&root, "docs");
    } else {
        prepare_knowledge(factory, &root);
    }
    for child in children {
        if module(factory, &child, &root, no_attribution) != 0 {
            return 1;
        }
        prepare_knowledge(factory, &child);
    }
    prepare_hermes(&root);
    println!(
        "Done. Open {} in your AI host and load the Factory coordinator for init.",
        root.display()
    );
    println!(
        "Claude Code: /factory init. Codex: $product-factory init. Other hosts: product-factory skill."
    );
    println!(
        "Hermes: existing routed projects are trusted automatically; otherwise run hermes skills trust in the checkout."
    );
    println!("Monitor: pf job status or pf dashboard.");
    println!(
        "Docs map: generated during install; run pf docs-map to inspect remaining inherited links."
    );
    if !util::has_cmd("pf") {
        warn("Put pf on PATH before starting an agent session so the guard hook can run.");
    }
    0
}

fn prepare_knowledge(factory: &Path, root: &Path) {
    let t = knowledge::target(root, None, None);
    match knowledge::plan(factory, &t) {
        Ok(plan) if !plan.conflicts.is_empty() => warn(format!(
            "{} has {} handbook numbering conflict(s); review pf knowledge plan before importing",
            root.display(),
            plan.conflicts.len()
        )),
        Ok(plan) => {
            if !plan.missing.is_empty() {
                match knowledge::init(factory, &t, true) {
                    Ok(applied) => say(format!(
                        "prepared {} missing {} documentation files in {}",
                        applied.created.len(),
                        t.stack,
                        root.display()
                    )),
                    Err(e) => warn(format!("knowledge preparation for {}: {e}", root.display())),
                }
            }
        }
        Err(e) => warn(format!("knowledge plan for {}: {e}", root.display())),
    }
    prepare_docs_map(root, &t.docs_root);
}

fn prepare_docs_map(root: &Path, docs: &str) {
    let report = docs_map::run(&docs_map::Options {
        root: root.to_path_buf(),
        docs: docs.into(),
        fix: true,
        strict: false,
        allow: vec![],
    });
    if !report.errors.is_empty() {
        warn(format!(
            "{}: {} documentation issue(s) remain",
            root.display(),
            report.errors.len()
        ));
    }
}

fn prepare_hermes(root: &Path) {
    if !root.join(".factory/model-routes.json").is_file() || !util::has_cmd("hermes") {
        return;
    }
    match Command::new("hermes")
        .args(["skills", "trust"])
        .arg(root)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) if status.success() => say(format!(
            "Hermes project skills trusted at {}",
            root.display()
        )),
        Ok(status) => warn(format!(
            "Hermes skills trust exited with {status}; run hermes skills trust {}",
            root.display()
        )),
        Err(e) => warn(format!("Hermes skills trust failed: {e}")),
    }
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
