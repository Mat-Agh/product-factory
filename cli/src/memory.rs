//! `pf memory`: show a repository's Claude Code auto memory, so `/factory memory` can
//! promote durable knowledge into the repo.
//!
//! Location, per Claude Code's docs: `autoMemoryDirectory` from settings (local, then
//! project, then user; an absolute path or `~/…`), else
//! `<claude config dir>/projects/<project>/memory/`. `<project>` is the repository's
//! main checkout path with every non-alphanumeric character replaced by `-`, so all
//! worktrees of a repo share one directory.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::util;

fn home() -> PathBuf {
    PathBuf::from(std::env::var_os("HOME").unwrap_or_default())
}

fn expand(p: &str) -> PathBuf {
    match p.strip_prefix("~/") {
        Some(rest) => home().join(rest),
        None => PathBuf::from(p),
    }
}

/// The repo's main checkout: the parent of the common `.git` dir when it is a normal
/// repo or a linked worktree; otherwise (submodules) the working tree's top level.
pub fn main_checkout(start: &Path) -> PathBuf {
    let s = start.to_string_lossy();
    if let Some(common) = util::output(
        "git",
        &[
            "-C",
            &s,
            "rev-parse",
            "--path-format=absolute",
            "--git-common-dir",
        ],
        None,
    ) {
        let c = PathBuf::from(common);
        if c.file_name().is_some_and(|n| n == ".git")
            && let Some(p) = c.parent()
        {
            return p.to_path_buf();
        }
    }
    util::git_root(start)
}

pub fn sanitize(p: &Path) -> String {
    p.to_string_lossy()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect()
}

fn setting(file: &Path) -> Option<String> {
    let v: Value = serde_json::from_str(&fs::read_to_string(file).ok()?).ok()?;
    v.get("autoMemoryDirectory")?.as_str().map(str::to_string)
}

/// (directory, how it was chosen)
pub fn memory_dir(start: &Path) -> (PathBuf, String) {
    let root = main_checkout(start);
    for (file, scope) in [
        (
            root.join(".claude/settings.local.json"),
            "project-local settings",
        ),
        (root.join(".claude/settings.json"), "project settings"),
        (util::claude_home().join("settings.json"), "user settings"),
    ] {
        if let Some(d) = setting(&file) {
            return (expand(&d), format!("autoMemoryDirectory in {scope}"));
        }
    }
    let dir = util::claude_home()
        .join("projects")
        .join(sanitize(&root))
        .join("memory");
    (dir, format!("default for {}", root.display()))
}

fn frontmatter(text: &str) -> (Value, String) {
    let mut meta = serde_json::Map::new();
    let Some(rest) = text.strip_prefix("---\n") else {
        return (Value::Object(meta), text.to_string());
    };
    let Some(end) = rest.find("\n---") else {
        return (Value::Object(meta), text.to_string());
    };
    for line in rest[..end].lines() {
        let line = line.trim();
        if let Some((k, v)) = line.split_once(':') {
            let v = v.trim().trim_matches('"').replace("\\\"", "\"");
            if !v.is_empty() {
                meta.insert(k.trim().to_string(), Value::String(v.to_string()));
            }
        }
    }
    let body = rest[end + 4..].trim_start_matches(['-', '\n']).to_string();
    (Value::Object(meta), body)
}

pub fn collect(start: &Path) -> Value {
    let (dir, source) = memory_dir(start);
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|e| e == "md")
                && p.file_name().is_some_and(|n| n != "MEMORY.md")
        })
        .collect();
    files.sort();
    let memories: Vec<Value> = files
        .iter()
        .map(|f| {
            let text = fs::read_to_string(f).unwrap_or_default();
            let (meta, body) = frontmatter(&text);
            let get = |k: &str| {
                meta.get(k)
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            };
            json!({
                "file": f.file_name().unwrap().to_string_lossy(),
                "name": get("name"),
                "description": get("description"),
                "type": if get("type").is_empty() { get("metadata.type") } else { get("type") },
                "modified": get("modified"),
                "body": body.trim(),
            })
        })
        .collect();
    json!({
        "dir": dir,
        "source": source,
        "exists": dir.is_dir(),
        "index": fs::read_to_string(dir.join("MEMORY.md")).unwrap_or_default(),
        "memories": memories,
    })
}

pub fn main(root: Option<PathBuf>, as_json: bool) -> i32 {
    let start = root.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let v = collect(&start);
    if as_json {
        println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
        return 0;
    }
    println!(
        "Auto memory: {} ({})",
        v["dir"].as_str().unwrap_or(""),
        v["source"].as_str().unwrap_or("")
    );
    let mems = v["memories"].as_array().cloned().unwrap_or_default();
    if mems.is_empty() {
        println!("  no memories");
    }
    for m in &mems {
        println!(
            "  {:<10} {:<40} {}",
            m["type"].as_str().filter(|t| !t.is_empty()).unwrap_or("?"),
            m["file"].as_str().unwrap_or(""),
            m["description"].as_str().unwrap_or("")
        );
    }
    if !mems.is_empty() {
        println!("Promote durable knowledge into the repo with /factory memory.");
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitizes_like_claude_code() {
        assert_eq!(
            sanitize(Path::new("/home/matin-aghalarkhani/Work/Connect")),
            "-home-matin-aghalarkhani-Work-Connect"
        );
        assert_eq!(
            sanitize(Path::new("/home/a/.config/x_y")),
            "-home-a--config-x-y"
        );
    }

    #[test]
    fn reads_frontmatter() {
        let (m, body) = frontmatter(
            "---\nname: x\ndescription: \"d\"\nmetadata:\n  type: feedback\n---\n\nBody line\n",
        );
        assert_eq!(m["name"], "x");
        assert_eq!(m["description"], "d");
        assert_eq!(m["type"], "feedback");
        assert_eq!(body.trim(), "Body line");
    }
}
