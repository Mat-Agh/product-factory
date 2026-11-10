//! Durable job state for the shared coordinator. The skill selects the lane and
//! dispatches roles. The CLI serializes mutations and checks evidence on disk.

use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

use clap::Subcommand;
use serde_json::{Value, json};

use crate::{
    descriptor::{self, Descriptor, Map},
    discovery, job_knowledge, util,
};

const JOBS: &str = ".factory/jobs";
const LENSES: [&str; 4] = ["code", "security", "ux", "ui"];

#[derive(Subcommand)]
pub enum Action {
    /// Reserve a workspace job ID and a slot under limits.max_parallel_jobs
    Claim {
        slug: String,
        #[arg(long, conflicts_with = "backlog")]
        request: Option<String>,
        #[arg(long, conflicts_with = "backlog")]
        source: Option<String>,
        /// Claim the first unchecked line of .factory/backlog.md
        #[arg(long)]
        backlog: bool,
    },
    /// Prepare the job's branch and worktree and record its selected lane
    Prepare {
        id: String,
        #[arg(long)]
        module: Option<String>,
        #[arg(long = "type")]
        kind: String,
        #[arg(long)]
        tier: String,
        #[arg(long, default_value = "none")]
        surface: String,
        /// Include design in a UI feature when its design source is missing or changes
        #[arg(long)]
        design_needed: bool,
        #[arg(long, value_delimiter = ',')]
        reviews: Vec<String>,
        #[arg(long)]
        escalate: bool,
    },
    /// Validate the current stage artifacts and persist the next stage
    Advance {
        id: String,
        #[arg(long)]
        expect: String,
        #[arg(long)]
        round: u64,
    },
    /// Record a verified gate's local completion or a pull request URL
    Finish {
        id: String,
        #[arg(long)]
        pr: Option<String>,
    },
    /// Record human feedback and restart the owning lane with fresh evidence
    Rework {
        id: String,
        #[arg(long)]
        note: String,
    },
    /// List workspace jobs and their recorded stages
    Status {
        /// Include completed and closed jobs
        #[arg(long)]
        all: bool,
    },
    /// Show committed changes that may require durable project knowledge
    Knowledge { id: String },
    /// Return a job's durable state as JSON
    Show { id: String },
    /// Park a job and release its active slot while keeping its worktree
    Pause {
        id: String,
        #[arg(long)]
        reason: String,
    },
    /// Return a parked job to its recorded stage
    Resume { id: String },
}

fn fail(e: io::Error) -> String {
    e.to_string()
}

fn lock(root: &Path) -> Result<File, String> {
    let relative = Path::new(JOBS).join(".lock");
    if !discovery::local_path(root, &relative) {
        return Err("Unsafe job store path".into());
    }
    fs::create_dir_all(root.join(JOBS)).map_err(fail)?;
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(root.join(relative))
        .map_err(fail)?;
    f.lock().map_err(fail)?;
    Ok(f)
}

pub fn valid_id(id: &str) -> bool {
    id.split_once('-').is_some_and(|(n, slug)| {
        n.len() == 3 && n.bytes().all(|b| b.is_ascii_digit()) && valid_slug(slug)
    })
}

fn valid_slug(slug: &str) -> bool {
    slug.len() <= 72
        && slug.split('-').all(|word| {
            !word.is_empty()
                && word
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
}

fn folder(root: &Path, id: &str) -> Result<PathBuf, String> {
    let rel = Path::new(JOBS).join(id);
    if !valid_id(id) || !discovery::local_path(root, &rel) {
        return Err(format!("Invalid job ID: {id}"));
    }
    Ok(root.join(rel))
}

fn read(dir: &Path) -> Result<Value, String> {
    if !discovery::local_path(dir, Path::new("job.json")) {
        return Err("Unsafe job.json path".into());
    }
    let text = fs::read_to_string(dir.join("job.json")).map_err(fail)?;
    let value: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    if value["stage"].as_str().is_none() {
        return Err("Missing stage in job.json".into());
    }
    Ok(value)
}

pub(crate) fn context(root: &Path, id: &str) -> Result<(PathBuf, Value), String> {
    let dir = folder(root, id)?;
    let value = read(&dir)?;
    if value["id"] != id {
        return Err("The job ID does not match its saved state".into());
    }
    Ok((dir, value))
}

fn save(dir: &Path, value: &mut Value) -> Result<(), String> {
    value["updated"] = json!(util::now_utc_iso());
    let data = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    atomic_write(&dir.join("job.json"), &format!("{data}\n"))
}

pub(crate) fn atomic_write(path: &Path, text: &str) -> Result<(), String> {
    use std::io::Write;
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let tmp = path.with_extension(format!("tmp-{}-{nonce}", std::process::id()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp)
        .map_err(fail)?;
    let result = file
        .write_all(text.as_bytes())
        .and_then(|_| file.sync_all());
    drop(file);
    let result = result.and_then(|_| fs::rename(&tmp, path));
    if result.is_err() {
        let _ = fs::remove_file(tmp);
    }
    result.map_err(fail)
}

fn status(root: &Path, all: bool) -> Result<Value, String> {
    if !discovery::local_path(root, Path::new(JOBS)) {
        return Err("Unsafe job store path".into());
    }
    let entries = match fs::read_dir(root.join(JOBS)) {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(json!([])),
        Err(e) => return Err(fail(e)),
    };
    let mut jobs = Vec::new();
    for entry in entries {
        let entry = entry.map_err(fail)?;
        if entry.file_type().map_err(fail)?.is_dir()
            && valid_id(&entry.file_name().to_string_lossy())
        {
            let job = read(&entry.path())?;
            if all || !matches!(job["stage"].as_str(), Some("done" | "closed")) {
                jobs.push(job);
            }
        }
    }
    jobs.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
    Ok(json!(jobs))
}

fn counts(root: &Path) -> Result<(u32, u64), String> {
    let (mut highest, mut active) = (0, 0);
    for entry in fs::read_dir(root.join(JOBS)).map_err(fail)? {
        let entry = entry.map_err(fail)?;
        let id = entry.file_name();
        let id = id.to_string_lossy();
        if valid_id(&id) && entry.file_type().map_err(fail)?.is_dir() {
            highest = highest.max(id[..3].parse().unwrap_or(0));
            let job = read(&entry.path());
            if !job.is_ok_and(|j| {
                matches!(
                    j["stage"].as_str(),
                    Some("done" | "closed" | "needs-human" | "pr-open")
                )
            }) {
                active += 1; // an incomplete claim holds a slot until reconciled
            }
        }
    }
    Ok((highest, active))
}

fn limit(d: &Descriptor, key: &str, default: u64) -> Result<u64, String> {
    d.get(key)
        .map(|v| {
            v.parse()
                .map_err(|_| format!("{key} must be a non-negative integer"))
        })
        .unwrap_or(Ok(default))
}

fn check_capacity(d: &Descriptor, active: u64) -> Result<(), String> {
    let capacity = limit(d, "limits.max_parallel_jobs", 3)?;
    if active >= capacity {
        return Err(format!("All {capacity} active job slots are in use"));
    }
    Ok(())
}

fn claim(
    d: &Descriptor,
    slug: &str,
    request: Option<String>,
    source: Option<String>,
    backlog: bool,
) -> Result<Value, String> {
    if !valid_slug(slug) {
        return Err("Use a lowercase kebab-case slug (up to 72 characters)".into());
    }
    let (highest, active) = counts(&d.root)?;
    check_capacity(d, active)?;
    if highest >= 999 {
        return Err("The workspace has exhausted its three-digit job IDs".into());
    }
    let id = format!("{:03}-{slug}", highest + 1);
    let dir = folder(&d.root, &id)?;
    let backlog_path = d.root.join(".factory/backlog.md");
    let mut pending_edit = None;
    let (request, source) = if backlog {
        if !discovery::local_path(&d.root, Path::new(".factory/backlog.md")) {
            return Err("Unsafe backlog path".into());
        }
        let text = fs::read_to_string(&backlog_path).map_err(fail)?;
        let mut offset = 0;
        let mut item = None;
        for line in text.split_inclusive('\n') {
            if let Some(body) = line.trim_end_matches(['\r', '\n']).strip_prefix("- [ ] ") {
                let suffix = &line[line.trim_end_matches(['\r', '\n']).len()..];
                let updated = format!("- [x] {body} → {id}{suffix}");
                let mut new = text.clone();
                new.replace_range(offset..offset + line.len(), &updated);
                item = Some((body.to_string(), new));
                break;
            }
            offset += line.len();
        }
        let (item, new) = item.ok_or("No unchecked backlog items")?;
        pending_edit = Some(new);
        (item, "backlog".into())
    } else {
        (
            request.unwrap_or_default(),
            source.unwrap_or_else(|| "chat".into()),
        )
    };
    if backlog || source.starts_with("issue:") {
        for entry in fs::read_dir(d.root.join(JOBS)).map_err(fail)?.flatten() {
            if !entry.file_type().is_ok_and(|t| t.is_dir()) {
                continue;
            }
            let Ok(existing) = read(&entry.path()) else {
                continue;
            };
            if source.starts_with("issue:")
                && existing["source"] == source
                && !matches!(existing["stage"].as_str(), Some("done" | "closed"))
            {
                return Err(format!("{} already owns {source}", existing["id"]));
            }
            if backlog && existing["backlog_pending"] == true && existing["request"] == request {
                let marker = format!(" → {}", existing["id"].as_str().unwrap_or_default());
                let queue = fs::read_to_string(&backlog_path).map_err(fail)?;
                if !queue.lines().any(|line| line.ends_with(&marker)) {
                    return Err(format!(
                        "Reconcile the unmarked backlog claim {} before claiming again",
                        existing["id"]
                    ));
                }
            }
        }
    }
    let staging = dir.with_file_name(format!(".claim-{id}-{}", std::process::id()));
    fs::create_dir(&staging).map_err(fail)?;
    let mut job = json!({
        "id": id, "request": request, "source": source, "stage": "triage",
        "round": 1, "resumes": 0, "reviews": {}, "pr": null, "backlog_pending": backlog,
    });
    let protocol = Path::new(".claude/skills/factory/protocol.md");
    if discovery::local_path(&d.root, protocol)
        && fs::read_to_string(d.root.join(protocol))
            .is_ok_and(|s| s.lines().any(|l| l == job_knowledge::MARKER))
    {
        job["knowledge_capture"] = json!(true);
    }
    if let Err(e) = save(&staging, &mut job).and_then(|_| fs::rename(&staging, &dir).map_err(fail))
    {
        let _ = fs::remove_file(staging.join("job.json"));
        let _ = fs::remove_dir(&staging);
        return Err(e);
    }
    if let Some(new) = pending_edit {
        atomic_write(&backlog_path, &new).map_err(|e| {
            format!("Job {id} exists but backlog update failed: {e}. Reconcile the backlog before claiming again.")
        })?;
        job["backlog_pending"] = json!(false);
        save(&dir, &mut job)?;
    }
    Ok(job)
}

fn select_module<'a>(
    d: &'a Descriptor,
    cwd: &Path,
    requested: Option<&str>,
) -> Result<&'a Map, String> {
    if let Some(key) = requested {
        let mut matching = d.modules.iter().filter(|m| {
            m.get("path")
                .is_some_and(|p| p.trim_end_matches('/') == key.trim_end_matches('/'))
                || m.get("area").is_some_and(|a| a == key)
        });
        let found = matching
            .next()
            .ok_or_else(|| format!("No module named {key}"))?;
        if matching.next().is_some() {
            return Err(format!("Ambiguous module selector: {key}"));
        }
        return Ok(found);
    }
    d.modules
        .iter()
        .filter(|m| {
            m.get("path")
                .is_some_and(|p| !p.is_empty() && cwd.starts_with(d.root.join(p)))
        })
        .max_by_key(|m| m["path"].len())
        .or_else(|| (d.modules.len() == 1).then(|| &d.modules[0]))
        .ok_or_else(|| "Select the owning module with --module".into())
}

fn artifact(dir: &Path, path: &str) -> Result<String, String> {
    if !discovery::local_path(dir, Path::new(path)) {
        return Err(format!("Unsafe job artifact: {path}"));
    }
    fs::read_to_string(dir.join(path)).map_err(|e| format!("{path}: {e}"))
}

fn git_head(tree: &Path, branch: &str) -> Result<String, String> {
    if util::output("git", &["symbolic-ref", "--short", "HEAD"], Some(tree)).as_deref()
        != Some(branch)
    {
        return Err(format!(
            "{} is not on the job branch {branch}",
            tree.display()
        ));
    }
    if util::output(
        "git",
        &["status", "--porcelain", "--untracked-files=no"],
        Some(tree),
    )
    .as_deref()
        != Some("")
    {
        return Err("The job worktree has uncommitted tracked changes".into());
    }
    util::output("git", &["rev-parse", "HEAD"], Some(tree))
        .ok_or_else(|| "Cannot read job HEAD".into())
}

struct Route {
    kind: String,
    tier: String,
    surface: String,
    design_needed: bool,
    reviews: Vec<String>,
    escalate: bool,
}

fn route_lane(
    kind: &str,
    tier: &str,
    surface: &str,
    design_needed: bool,
) -> Result<&'static [&'static str], String> {
    if !["trivial", "standard", "cross-cutting"].contains(&tier)
        || !["none", "ui", "api"].contains(&surface)
        || (design_needed && (kind != "feature" || surface != "ui" || tier == "trivial"))
    {
        return Err("Invalid job tier, surface, or design flag".into());
    }
    Ok(match (kind, tier, design_needed) {
        ("feature" | "docs" | "chore", "trivial", false) => &["build", "review", "gate"],
        ("feature", _, true) => &["spec", "design", "plan", "build", "review", "qa", "gate"],
        ("feature", _, false) => &["spec", "plan", "build", "review", "qa", "gate"],
        ("bug", _, _) => &["repro", "plan", "build", "review", "qa", "gate"],
        ("design", "standard" | "cross-cutting", _) => {
            &["design", "spec", "plan", "build", "review", "qa", "gate"]
        }
        ("docs", _, _) => &["spec", "build", "review", "gate"],
        ("chore", _, _) => &["plan", "build", "review", "gate"],
        ("spike", "standard" | "cross-cutting", _) => &["plan", "gate"],
        ("idea", "standard" | "cross-cutting", _) => {
            &["discovery", "spec", "design", "plan", "gate"]
        }
        ("release", "standard" | "cross-cutting", _) => &["release", "gate"],
        _ => return Err(format!("No delivery lane for {kind}/{tier}")),
    })
}

fn prepare(
    d: &Descriptor,
    dir: &Path,
    cwd: &Path,
    requested: Option<&str>,
    route: Route,
    mut job: Value,
) -> Result<Value, String> {
    if job["stage"] != "triage" || !artifact(dir, "triage.md")?.starts_with("# Triage:") {
        return Err("Prepare requires a triage job and a # Triage: artifact".into());
    }
    let Route {
        kind,
        tier,
        surface,
        design_needed,
        reviews,
        escalate,
    } = route;
    let lane = route_lane(&kind, &tier, &surface, design_needed)?;
    let has_review = lane.contains(&"review");
    if reviews.iter().any(|s| !LENSES.contains(&s.as_str())) || (!has_review && !reviews.is_empty())
    {
        return Err("Invalid review lenses for this route".into());
    }
    let reviewed = |name: &str| {
        has_review
            && match name {
                "code" => true,
                "security" => {
                    tier == "cross-cutting" || kind == "chore" || reviews.iter().any(|s| s == name)
                }
                "ux" | "ui" => surface == "ui",
                _ => false,
            }
    };

    let module = select_module(d, cwd, requested)?;
    let path = module.get("path").ok_or("Module path missing")?;
    let scope = d.root.join(path);
    if path.is_empty() || !discovery::local_path(&d.root, Path::new(path)) || !scope.is_dir() {
        return Err("Module must be a local directory under the workspace".into());
    }
    let repo = util::git_root(&scope);
    if (repo != d.root && repo != util::normalize(&scope)) || !repo.join(".git").exists() {
        return Err("Cannot resolve the owning Git checkout".into());
    }
    let knowledge_root = module
        .get("docs_root")
        .map(String::as_str)
        .filter(|p| !p.is_empty())
        .or_else(|| d.get("docs.root"))
        .unwrap_or("docs");
    if job["knowledge_capture"] == true && !discovery::local_path(&scope, Path::new(knowledge_root))
    {
        return Err("Unsafe knowledge docs root".into());
    }
    let area = module
        .get("area")
        .filter(|s| !s.is_empty())
        .map(String::as_str)
        .unwrap_or("module");
    let workspace_name = d
        .root
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("Invalid workspace name")?;
    let id = job["id"].as_str().ok_or("Missing job id")?.to_string();
    let worktrees = d
        .root
        .parent()
        .ok_or("Workspace has no parent")?
        .join(format!("{workspace_name}-factory"));
    let relative_tree = if repo == d.root && path == "." {
        PathBuf::from(&id)
    } else {
        if !area
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
        {
            return Err("The module area must be a simple identifier".into());
        }
        Path::new(area).join(&id)
    };
    if !discovery::local_path(
        worktrees.parent().unwrap(),
        Path::new(worktrees.file_name().unwrap()),
    ) || !discovery::local_path(&worktrees, &relative_tree)
    {
        return Err("Unsafe job worktree path".into());
    }
    let tree = worktrees.join(relative_tree);
    let base = d.module_vcs(module, "base_branch").unwrap_or("main");
    let base_ref = format!("refs/heads/{base}");
    let template = d.get("vcs.branch_template").unwrap_or("factory/{job-id}");
    let branch_kind = d.get(&format!("vcs.branch_kinds.{kind}")).unwrap_or(&kind);
    let branch = template
        .replace("{job-id}", &id)
        .replace("{kind}", branch_kind);
    if !template.contains("{job-id}")
        || branch == base
        || branch.starts_with('-')
        || util::output(
            "git",
            &["check-ref-format", &format!("refs/heads/{branch}")],
            Some(&repo),
        )
        .is_none()
        || util::output("git", &["show-ref", "--verify", &base_ref], Some(&repo)).is_none()
    {
        return Err(format!(
            "Invalid job branch {branch} or missing local base {base_ref}"
        ));
    }
    let base_commit = job["preparing"]["base_commit"]
        .as_str()
        .map(str::to_string)
        .or_else(|| util::output("git", &["rev-parse", &base_ref], Some(&repo)))
        .ok_or("Cannot resolve job base commit")?;
    let prefix = if repo == d.root { path.as_str() } else { "." };
    // Record the selected path before creating the worktree so an interrupted
    // command can recognize its own Git state on retry.
    let creating =
        json!({"branch": branch, "worktree": tree, "module": path, "base_commit": base_commit});
    if !job["preparing"].is_null() && job["preparing"] != creating {
        return Err("Preparation already started with a different branch or worktree".into());
    }
    let ours = job["preparing"] == creating;
    if !ours && tree.exists() {
        return Err(format!("Worktree path already exists: {}", tree.display()));
    }
    job["preparing"] = creating;
    save(dir, &mut job)?;
    if tree.exists() {
        let common = |cwd: &Path| {
            util::output(
                "git",
                &["rev-parse", "--path-format=absolute", "--git-common-dir"],
                Some(cwd),
            )
        };
        if !ours
            || common(&tree).is_none()
            || common(&tree) != common(&repo)
            || util::git_root(&tree) != tree
            || git_head(&tree, &branch).is_err()
        {
            return Err(format!(
                "Existing worktree does not belong to job {id}: {}",
                tree.display()
            ));
        }
    } else {
        let tree_arg = tree.to_str().ok_or("Invalid worktree path")?;
        let output = std::process::Command::new("git")
            .args([
                "worktree",
                "add",
                "-b",
                &branch,
                "--",
                tree_arg,
                &base_commit,
            ])
            .current_dir(&repo)
            .output()
            .map_err(fail)?;
        if !output.status.success() {
            return Err(format!(
                "Could not create worktree: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
    }

    let lenses: serde_json::Map<String, Value> = LENSES
        .into_iter()
        .map(|name| {
            (
                name.into(),
                json!(if reviewed(name) { "pending" } else { "skipped" }),
            )
        })
        .collect();
    job.as_object_mut().unwrap().remove("preparing");
    job["module"] = json!(path);
    job["area"] = json!(area);
    job["type"] = json!(kind);
    job["tier"] = json!(tier);
    job["surface"] = json!(surface);
    job["stage"] = json!(lane[0]);
    job["lane"] = json!(
        std::iter::once("triage")
            .chain(lane.iter().copied())
            .collect::<Vec<_>>()
    );
    job["worktree"] = json!(tree);
    job["kit_checkout"] = json!(repo);
    if job["knowledge_capture"] == true {
        job["knowledge_docs_root"] = json!(knowledge_root);
    }
    job["module_worktree"] = json!(util::normalize(&tree.join(prefix)));
    if repo == d.root {
        let scope = util::normalize(&scope);
        job["excluded_scopes"] = json!(
            d.modules
                .iter()
                .filter_map(|m| m.get("path"))
                .map(Path::new)
                .filter(|p| discovery::local_path(&d.root, p))
                .filter(|p| {
                    let child = util::normalize(&d.root.join(p));
                    child != scope && child.starts_with(&scope) && util::git_root(&child) == repo
                })
                .collect::<Vec<_>>()
        );
    }
    job["branch"] = json!(branch);
    job["base_commit"] = json!(base_commit);
    job["base_branch"] = json!(base);
    job["remote"] = json!(d.module_vcs(module, "remote").unwrap_or("origin"));
    job["host"] = json!(d.module_vcs(module, "host").unwrap_or("github"));
    job["open_pr"] = json!(d.module_vcs(module, "open_pr") != Some("false"));
    job["reviews"] = json!(lenses);
    job["requires_escalation"] = json!(escalate || tier == "cross-cutting");
    save(dir, &mut job)?;
    Ok(job)
}

fn park(job: &mut Value, reason: String) {
    job["paused_stage"] = job["stage"].clone();
    job["stage"] = json!("needs-human");
    job["reason"] = json!(reason);
}

fn archive(dir: &Path, round: u64, files: &[&str]) -> Result<(), String> {
    let relative = format!("round-{round}");
    if !discovery::local_path(dir, Path::new(&relative)) {
        return Err("Unsafe round path".into());
    }
    let dest = dir.join(relative);
    fs::create_dir_all(&dest).map_err(fail)?;
    for name in files {
        if !dir.join(name).exists() {
            continue;
        }
        let dest = dest.join(name);
        if dest.exists() {
            return Err(format!("Existing archived evidence: {}", dest.display()));
        }
        if !discovery::local_path(dir, Path::new(name)) {
            return Err(format!("Unsafe artifact: {name}"));
        }
        fs::rename(dir.join(name), dest).map_err(fail)?;
    }
    Ok(())
}

fn verdict(dir: &Path, file: &str, choices: &[&str]) -> Result<String, String> {
    let text = artifact(dir, file)?;
    let first = text.lines().next().unwrap_or_default();
    if !choices.contains(&first) {
        return Err(format!("{file} must start with {}", choices.join(" or ")));
    }
    Ok(first.into())
}

fn changed_paths(tree: &Path, base: &str, head: &str) -> Result<Vec<PathBuf>, String> {
    let output = std::process::Command::new("git")
        .args([
            "diff",
            "--no-renames",
            "--name-only",
            "-z",
            base,
            head,
            "--",
        ])
        .current_dir(tree)
        .output()
        .map_err(fail)?;
    if !output.status.success() {
        return Err("Could not inspect the job diff".into());
    }
    output
        .stdout
        .split(|b| *b == 0)
        .filter(|p| !p.is_empty())
        .map(|p| {
            std::str::from_utf8(p)
                .map(PathBuf::from)
                .map_err(|_| "A changed path is not UTF-8; review the diff manually".into())
        })
        .collect()
}

fn advance(
    d: &Descriptor,
    dir: &Path,
    expect: &str,
    round: u64,
    mut job: Value,
) -> Result<Value, String> {
    if job["stage"] != expect || job["round"].as_u64().unwrap_or(1) != round {
        return Err(format!(
            "Job changed: current stage is {} in round {}",
            job["stage"], job["round"]
        ));
    }
    let lane = job["lane"].as_array().ok_or("Missing lane in job.json")?;
    let next = lane
        .iter()
        .position(|s| s == expect)
        .and_then(|i| lane.get(i + 1))
        .and_then(Value::as_str)
        .map(str::to_string);
    let tree = PathBuf::from(job["worktree"].as_str().ok_or("Missing worktree")?);
    let branch = job["branch"].as_str().ok_or("Missing branch")?;
    if !tree.is_absolute() || !tree.is_dir() || util::git_root(&tree) != tree {
        return Err("Job worktree is unavailable".into());
    }
    let head = git_head(&tree, branch)?;
    if let Some(revision) = job["reviewed_commit"].as_str()
        && matches!(expect, "review" | "qa" | "gate")
        && revision != head
    {
        return Err(format!(
            "Job HEAD changed after the evidence was requested ({revision} → {head})"
        ));
    }

    let mut changes_code = false;
    let mut capture = None;
    let base = job["base_commit"].as_str();
    if job["knowledge_capture"] == true && base.is_none() {
        return Err("Missing knowledge capture base commit".into());
    }
    if let Some(base) = base {
        let paths = changed_paths(&tree, base, &head)?;
        if expect == "gate" && paths.is_empty() {
            return Err("The delivery branch contains no changes against its recorded base".into());
        }
        if let Some(module) = job["module_worktree"].as_str() {
            let prefix = Path::new(module)
                .strip_prefix(&tree)
                .map_err(|_| "Module is outside its job worktree")?;
            let excluded: Vec<&str> = job["excluded_scopes"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            if let Some(outside) = paths.iter().find(|p| {
                !p.starts_with(prefix) || excluded.iter().any(|other| p.starts_with(other))
            }) {
                return Err(format!(
                    "Change outside the job module: {}",
                    outside.display()
                ));
            }
        }
        if job["knowledge_capture"] == true
            && (matches!(expect, "build" | "review" | "gate" | "release")
                || (expect == "plan" && next.as_deref() == Some("gate")))
        {
            capture = Some(job_knowledge::Report::inspect(
                &job, &tree, base, &head, &paths,
            )?);
        }
        changes_code = paths
            .iter()
            .any(|p| p.extension().is_none_or(|ext| ext != "md"));
    }

    let mut retry = None;
    let mut stop = None;
    match expect {
        "discovery" | "spec" | "design" | "plan" | "build" | "release" => {
            let file = format!("{expect}.md");
            if verdict(dir, &file, &["STATUS: READY", "STATUS: BLOCKED"])? == "STATUS: BLOCKED" {
                stop = Some(format!("{file} reports STATUS: BLOCKED"));
            } else if let Some(report) = &capture {
                report.validate(&file, &artifact(dir, &file)?)?;
            }
        }
        "repro" => {
            if verdict(dir, "repro.md", &["REPRODUCED", "NOT REPRODUCED"])? == "NOT REPRODUCED" {
                stop = Some("repro.md reports NOT REPRODUCED".into());
            }
        }
        "review" => {
            if let Some(report) = &capture {
                report.validate("build.md", &artifact(dir, "build.md")?)?;
            }
            let current = job["reviews"].as_object().ok_or("Missing review lenses")?;
            if !current
                .get("code")
                .is_some_and(|v| matches!(v.as_str(), Some("pending" | "PASS" | "CHANGES")))
            {
                return Err("The review round requires a code lens".into());
            }
            let mut reviews = current.clone();
            let mut changes = Vec::new();
            for (lens, value) in &mut reviews {
                if !LENSES.contains(&lens.as_str()) {
                    return Err(format!("Unknown review lens {lens}"));
                }
                if *value == "skipped" {
                    continue;
                }
                let file = format!("round-{round}/review-{lens}.md");
                let result = verdict(dir, &file, &["VERDICT: PASS", "VERDICT: CHANGES"])?;
                let result = result.trim_start_matches("VERDICT: ");
                if lens == "code" && result == "PASS" && capture.is_some() {
                    job_knowledge::validate_review(&artifact(dir, &file)?)?;
                }
                *value = json!(result);
                if result == "CHANGES" {
                    changes.push(lens.clone());
                }
            }
            job["reviews"] = json!(reviews);
            if !changes.is_empty() {
                retry = Some(format!(
                    "Round {round} requested changes from {}",
                    changes.join(", ")
                ));
            }
        }
        "qa" => {
            let archived = format!("round-{round}/qa.md");
            let file = if dir.join("qa.md").exists() {
                "qa.md"
            } else {
                &archived
            };
            if verdict(dir, file, &["VERDICT: PASS", "VERDICT: FAIL"])? == "VERDICT: FAIL" {
                retry = Some(format!("QA failed in round {round}"));
            }
        }
        "gate" => {
            let gate = verdict(dir, "gate.md", &["READY", "ESCALATE"])?;
            if let Some(report) = &capture {
                let producer = lane
                    .iter()
                    .find_map(|stage| match stage.as_str()? {
                        "build" => Some("build.md"),
                        "release" => Some("release.md"),
                        "plan" if !lane.iter().any(|s| s == "build") => Some("plan.md"),
                        _ => None,
                    })
                    .ok_or("No knowledge evidence producer in this lane")?;
                report.validate(producer, &artifact(dir, producer)?)?;
                if lane.iter().any(|s| s == "review") {
                    job_knowledge::validate_review(&artifact(
                        dir,
                        &format!("round-{round}/review-code.md"),
                    )?)?;
                }
            }
            if gate == "READY"
                && (job["requires_escalation"] == true || job["tier"] == "cross-cutting")
            {
                return Err("The selected job requires an ESCALATE gate".into());
            }
            if artifact(dir, "pr.md")?.trim().is_empty() {
                return Err("pr.md is empty".into());
            }
            job["gate_verdict"] = json!(gate);
            job["gate_commit"] = json!(head);
            save(dir, &mut job)?;
            return Ok(job);
        }
        _ => return Err(format!("No evidence transition for {expect}")),
    }
    if let Some(reason) = retry {
        let resumes = job["resumes"].as_u64().unwrap_or(0);
        if resumes >= limit(d, "limits.max_resumes", 2)?.min(2) {
            atomic_write(
                &dir.join("gate.md"),
                &format!("ESCALATE\n\n{reason}. Rework limit reached.\n"),
            )?;
            park(&mut job, reason);
        } else {
            archive(dir, round, &["build.md", "qa.md"])?;
            job["resumes"] = json!(resumes + 1);
            job["round"] = json!(round + 1);
            job["stage"] = json!("build");
            job["reason"] = json!(reason);
            job["reviewed_commit"] = Value::Null;
            job["reviews"]
                .as_object_mut()
                .unwrap()
                .values_mut()
                .filter(|v| **v != "skipped")
                .for_each(|v| *v = json!("pending"));
        }
    } else if let Some(reason) = stop {
        park(&mut job, reason);
    } else {
        let next = next.ok_or("No next stage in the selected lane")?;
        if next == "review" || next == "gate" {
            job["reviewed_commit"] = json!(head);
        }
        if next == "review" {
            if changes_code {
                job["reviews"]["security"] = json!("pending");
            }
            let relative = format!("round-{round}");
            if !discovery::local_path(dir, Path::new(&relative)) {
                return Err("Unsafe review directory".into());
            }
            fs::create_dir_all(dir.join(relative)).map_err(fail)?;
        }
        job["stage"] = json!(next);
        job["reason"] = Value::Null;
    }
    save(dir, &mut job)?;
    Ok(job)
}

fn finish(dir: &Path, mut job: Value, pr: Option<String>) -> Result<Value, String> {
    if job["stage"] != "gate" || !matches!(job["gate_verdict"].as_str(), Some("READY" | "ESCALATE"))
    {
        return Err("Advance the gate successfully before finishing".into());
    }
    let tree = Path::new(job["worktree"].as_str().ok_or("Missing worktree")?);
    let branch = job["branch"].as_str().ok_or("Missing branch")?;
    if job["gate_commit"] != git_head(tree, branch)? {
        return Err("Job HEAD changed since the gate passed".into());
    }
    if let Some(pr) = pr {
        if !pr.starts_with("https://")
            || pr.chars().any(char::is_whitespace)
            || job["open_pr"] == false
        {
            return Err("A pull request requires an HTTPS URL and vcs.open_pr enabled".into());
        }
        job["pr"] = json!(pr);
        job["stage"] = json!("pr-open");
    } else {
        let reason = if job["gate_verdict"] == "ESCALATE" {
            "Delivery branch is local and requires a human decision"
        } else {
            "Delivery branch is ready locally"
        };
        park(&mut job, reason.into());
    }
    save(dir, &mut job)?;
    Ok(job)
}

fn rework(d: &Descriptor, dir: &Path, mut job: Value, note: &str) -> Result<Value, String> {
    if !matches!(job["stage"].as_str(), Some("needs-human" | "pr-open")) || note.trim().is_empty() {
        return Err("Rework requires a parked job or open PR and a feedback note".into());
    }
    check_capacity(d, counts(&d.root)?.1)?;
    let lane = job["lane"]
        .as_array()
        .ok_or("No prepared lane; resume triage instead")?;
    let paused = job["paused_stage"].as_str();
    let stage = match paused {
        Some(stage @ ("discovery" | "spec" | "design" | "plan" | "repro" | "release")) => stage,
        _ if lane.iter().any(|s| s == "build") => "build",
        _ if lane.iter().any(|s| s == "release") => "release",
        _ if lane.iter().any(|s| s == "plan") => "plan",
        _ => return Err("No stage can apply rework in this lane".into()),
    }
    .to_string();
    let round = job["round"].as_u64().unwrap_or(1);
    let file = format!("rework-{}.md", round + 1);
    let note_path = dir.join(&file);
    if note_path.exists() {
        if artifact(dir, &file)? != note {
            return Err(format!(
                "An interrupted rework already recorded different feedback in {file}"
            ));
        }
    } else {
        atomic_write(&note_path, note)?;
    }
    let stage_artifact = format!("{stage}.md");
    archive(dir, round, &[&stage_artifact, "qa.md", "gate.md", "pr.md"])?;
    job["round"] = json!(round + 1);
    job["resumes"] = json!(0);
    job["stage"] = json!(stage);
    job["feedback"] = json!(file);
    job["paused_stage"] = Value::Null;
    job["reason"] = Value::Null;
    job["reviewed_commit"] = Value::Null;
    job["gate_commit"] = Value::Null;
    job["gate_verdict"] = Value::Null;
    if let Some(lenses) = job["reviews"].as_object_mut() {
        lenses
            .values_mut()
            .filter(|v| **v != "skipped")
            .for_each(|v| *v = json!("pending"));
    }
    save(dir, &mut job)?;
    Ok(job)
}

pub fn main(root: Option<PathBuf>, action: Action) -> i32 {
    let refresh_graph = match &action {
        Action::Prepare { .. } => true,
        Action::Advance { expect, .. } => matches!(expect.as_str(), "build" | "plan" | "release"),
        _ => false,
    };
    let result = (|| {
        let cwd =
            util::absolute(&root.unwrap_or_else(|| std::env::current_dir().unwrap_or_default()));
        let d = descriptor::find(&cwd).ok_or("No factory descriptor at or above this directory")?;
        if let Action::Status { all } = action {
            return status(&d.root, all);
        }
        if let Action::Show { id } = action {
            return read(&folder(&d.root, &id)?);
        }
        if let Action::Knowledge { id } = action {
            let (_, state) = context(&d.root, &id)?;
            let tree = Path::new(state["worktree"].as_str().ok_or("Prepare the job first")?);
            let branch = state["branch"].as_str().ok_or("Missing branch")?;
            if !tree.is_absolute() || !tree.is_dir() || util::git_root(tree) != tree {
                return Err("Job worktree is unavailable".into());
            }
            let head = git_head(tree, branch)?;
            let base = state["base_commit"].as_str().ok_or("Missing base commit")?;
            let paths = changed_paths(tree, base, &head)?;
            return Ok(
                job_knowledge::Report::inspect(&state, tree, base, &head, &paths)?.json(&head),
            );
        }
        let _lock = lock(&d.root)?;
        if let Action::Claim {
            slug,
            request,
            source,
            backlog,
        } = action
        {
            let config = fs::read_to_string(d.root.join(".factory/factory.yaml")).map_err(fail)?;
            if config.starts_with("# pf: draft descriptor")
                || (d.get("product.name") == Some("Example Product")
                    && d.get("product.one_liner") == Some("What it is, for whom, in one sentence."))
            {
                return Err(
                    "Review the descriptor with factory init/adopt before claiming a job".into(),
                );
            }
            return claim(&d, &slug, request, source, backlog);
        }
        let id = match &action {
            Action::Prepare { id, .. }
            | Action::Advance { id, .. }
            | Action::Finish { id, .. }
            | Action::Rework { id, .. }
            | Action::Pause { id, .. }
            | Action::Resume { id } => id,
            _ => unreachable!(),
        };
        let dir = folder(&d.root, id)?;
        let mut job = read(&dir)?;
        if job["id"] != id.as_str() {
            return Err("job.json ID does not match its directory".into());
        }
        match action {
            Action::Prepare {
                module,
                kind,
                tier,
                surface,
                design_needed,
                reviews,
                escalate,
                ..
            } => prepare(
                &d,
                &dir,
                &cwd,
                module.as_deref(),
                Route {
                    kind,
                    tier,
                    surface,
                    design_needed,
                    reviews,
                    escalate,
                },
                job,
            ),
            Action::Advance { expect, round, .. } => advance(&d, &dir, &expect, round, job),
            Action::Finish { pr, .. } => finish(&dir, job, pr),
            Action::Rework { note, .. } => rework(&d, &dir, job, &note),
            Action::Pause { reason, .. } => {
                if matches!(
                    job["stage"].as_str(),
                    Some("needs-human" | "done" | "closed" | "pr-open")
                ) {
                    return Err("Only an active job can be parked".into());
                }
                park(&mut job, reason);
                save(&dir, &mut job)?;
                Ok(job)
            }
            Action::Resume { .. } => {
                if job["stage"] != "needs-human" {
                    return Err("Only a parked job can be resumed".into());
                }
                let stage = job["paused_stage"]
                    .as_str()
                    .ok_or("No recorded stage to resume")?;
                check_capacity(&d, counts(&d.root)?.1)?;
                job["stage"] = json!(stage);
                job["paused_stage"] = Value::Null;
                job["reason"] = Value::Null;
                save(&dir, &mut job)?;
                Ok(job)
            }
            _ => unreachable!(),
        }
    })();
    match result {
        Ok(value) => {
            // Index after releasing the workspace state lock. Separate jobs
            // can prepare their worktrees and build graphs concurrently.
            if refresh_graph
                && value["stage"] != "needs-human"
                && let Some(tree) = value["worktree"].as_str()
            {
                crate::graph::refresh_if_available(Path::new(tree));
            }
            println!("{value}");
            0
        }
        Err(e) => {
            util::warn(e);
            1
        }
    }
}
