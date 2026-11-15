//! Local Graphify code indexes, scoped to the Git worktree being inspected.

use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use clap::Subcommand;

use crate::{discovery, util};

const OUT: &str = ".factory/graphify";
const OWNER: &str = "# Product Factory Graphify cache\n*\n";
const PACKAGE: &str = "graphifyy==0.9.79";

#[derive(Subcommand)]
pub enum Action {
    /// Build or refresh the local code graph
    Refresh {
        /// Refresh each independent checkout in the workspace
        #[arg(long)]
        modules: bool,
    },
    /// Refresh the graph and search a small slice of it
    Query { question: String },
    /// Write a local interactive 3D view of the current code graph
    View,
}

pub fn available() -> bool {
    tool().is_some()
}

fn tool() -> Option<PathBuf> {
    util::which("graphify").map(|path| util::absolute(&path))
}

pub fn provision() -> Result<(), String> {
    if available() {
        return Ok(());
    }
    if !util::has_cmd("uv") {
        return Err(format!(
            "Install uv or put Graphify on PATH (uv tool install {PACKAGE})"
        ));
    }
    if !util::run("uv", &["tool", "install", PACKAGE], None) {
        return Err(format!("uv could not install {PACKAGE}"));
    }
    if available() {
        Ok(())
    } else {
        Err("Graphify was installed. Add the uv tool bin directory to PATH and rerun the Factory install.".into())
    }
}

fn command(executable: &Path, root: &Path) -> Command {
    let mut command = Command::new(executable);
    command
        .current_dir(root)
        .env("GRAPHIFY_OUT", OUT)
        .env("GRAPHIFY_BACKEND", "none")
        .env("GRAPHIFY_QUERY_LOG_DISABLE", "1");
    command
}

fn graph_file(root: &Path) -> PathBuf {
    root.join(OUT).join("graph.json")
}

fn write_view(root: &Path) -> Result<PathBuf, String> {
    let _lock = output_folder(root)?;
    let graph = fs::read_to_string(graph_file(root)).map_err(|e| e.to_string())?;
    let safe_graph = graph.replace('<', "\\u003c").replace('>', "\\u003e");
    let page = include_str!("../assets/graph-view.html").replace("__GRAPH_JSON__", &safe_graph);
    let path = root.join(OUT).join("view.html");
    fs::write(&path, page).map_err(|e| e.to_string())?;
    Ok(path)
}

fn output_folder(root: &Path) -> Result<File, String> {
    let relative = Path::new(OUT);
    let dir = root.join(relative);
    if !discovery::local_path(root, relative) {
        return Err(format!("Unsafe Graphify output path: {}", dir.display()));
    }
    fs::create_dir_all(root.join(".factory")).map_err(|e| e.to_string())?;
    let created = match fs::create_dir(&dir) {
        Ok(()) => true,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => false,
        Err(e) => return Err(e.to_string()),
    };
    let lock = relative.join(".pf-graph.lock");
    if !discovery::local_path(root, &lock) {
        return Err("Unsafe Graphify lock path".into());
    }
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(root.join(lock))
        .map_err(|e| e.to_string())?;
    lock.lock().map_err(|e| e.to_string())?;

    let ignore = relative.join(".gitignore");
    if !discovery::local_path(root, &ignore) {
        return Err("Unsafe Graphify ignore path".into());
    }
    let marker = root.join(ignore);
    if created
        || fs::read_dir(&dir)
            .map_err(|e| e.to_string())?
            .all(|entry| entry.is_ok_and(|entry| entry.file_name() == ".pf-graph.lock"))
    {
        fs::write(&marker, OWNER).map_err(|e| e.to_string())?;
    }
    if fs::read_to_string(&marker).ok().as_deref() != Some(OWNER) {
        return Err(format!(
            "{} already exists outside Factory's control",
            dir.display()
        ));
    }
    let mut dirs = vec![dir];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let kind = entry.file_type().map_err(|e| e.to_string())?;
            if kind.is_symlink() {
                return Err(format!(
                    "Unsafe symlink in Graphify cache: {}",
                    entry.path().display()
                ));
            }
            if kind.is_dir() {
                dirs.push(entry.path());
            }
        }
    }
    Ok(lock)
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Ready,
    Empty,
}

fn errors(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr)
        .lines()
        .chain(String::from_utf8_lossy(&output.stdout).lines())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .take(5)
        .map(|s| s.chars().take(240).collect::<String>())
        .collect::<Vec<_>>()
        .join(" | ")
}

fn rebuild(tool: &Path, root: &Path) -> Result<State, String> {
    // Submodules belong to separate Git histories. Exclude them even when
    // checked out and index them separately via workspace installation.
    let mut cmd = command(tool, root);
    cmd.args(["extract", ".", "--code-only"]);
    for folder in ["/.factory/", "/.claude/", "/.agents/", "/.codex/"] {
        cmd.args(["--exclude", folder]);
    }
    for relative in discovery::checkout_boundaries(root) {
        let path = relative
            .to_str()
            .ok_or("Graphify cannot exclude a non-UTF-8 submodule path")?;
        let escaped: String = path
            .chars()
            .flat_map(|c| {
                let mut result = Vec::new();
                if "*?[]!#\\".contains(c) {
                    result.push('\\');
                }
                result.push(c);
                result
            })
            .collect();
        cmd.arg("--exclude").arg(format!("/{escaped}/"));
    }
    let output = cmd.output().map_err(|e| e.to_string())?;
    let graph = graph_file(root);
    if output.status.success() && graph.is_file() {
        return Ok(State::Ready);
    }
    if String::from_utf8_lossy(&output.stderr).contains("graph is empty")
        || String::from_utf8_lossy(&output.stdout).contains("graph is empty")
    {
        for file in ["graph.json", "manifest.json"] {
            match fs::remove_file(root.join(OUT).join(file)) {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                Err(e) => return Err(e.to_string()),
            }
        }
        return Ok(State::Empty);
    }
    Err(format!(
        "Graphify could not build a graph: {}",
        errors(&output)
    ))
}

fn index(root: &Path) -> Result<State, String> {
    let executable =
        tool().ok_or_else(|| format!("Graphify is missing. Run uv tool install {PACKAGE}."))?;
    let _lock = output_folder(root)?;
    rebuild(&executable, root)
}

pub fn refresh_if_available(root: &Path) {
    if available()
        && let Err(e) = index(root)
    {
        util::warn(format!("Graphify ({}): {e}", root.display()));
    }
}

pub fn install(root: &Path) {
    match index(root) {
        Ok(State::Ready) => match write_view(root) {
            Ok(path) => util::say(format!(
                "Graphify code graph and 3D view ready at {}",
                path.display()
            )),
            Err(e) => util::warn(e),
        },
        Ok(State::Empty) => {
            util::say("Graphify will index this checkout when its first source files appear.")
        }
        Err(e) => util::warn(e),
    }
}

pub fn main(root: Option<PathBuf>, action: Action) -> i32 {
    let start = root.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let root = util::git_root(&start);
    if !root.join(".git").exists() {
        util::warn("Run pf graph from a Git checkout or pass --root");
        return 1;
    }
    match action {
        Action::View => {
            let view = || -> Result<(), String> {
                if index(&root)? == State::Empty {
                    return Err("No supported source to visualize".into());
                }
                let path = write_view(&root)?;
                println!("{}", path.display());
                Ok(())
            };
            view().map_or_else(
                |e| {
                    util::warn(e);
                    1
                },
                |()| 0,
            )
        }
        Action::Refresh { modules } => {
            let (roots, missing) = discovery::roots(&root, modules);
            for path in &missing {
                util::warn(format!("Module unavailable: {}", path.display()));
            }
            let mut result = i32::from(!missing.is_empty());
            for checkout in roots.into_iter().filter(|p| p.join(".git").exists()) {
                match index(&checkout) {
                    Ok(State::Ready) => match write_view(&checkout) {
                        Ok(path) => println!("{}", path.display()),
                        Err(e) => {
                            util::warn(e);
                            result = 1;
                        }
                    },
                    Ok(State::Empty) => {
                        println!("No supported source to index in {}", checkout.display())
                    }
                    Err(e) => {
                        util::warn(format!("{}: {e}", checkout.display()));
                        result = 1;
                    }
                }
            }
            result
        }
        Action::Query { question } => {
            let query = || -> Result<i32, String> {
                if question.trim().is_empty() || question.starts_with('-') {
                    return Err("Supply a graph query beginning with a search term".into());
                }
                let tool = tool().ok_or_else(|| {
                    format!("Graphify is missing. Run uv tool install {PACKAGE}.")
                })?;
                let _lock = output_folder(&root)?;
                if rebuild(&tool, &root)? == State::Empty {
                    println!("No supported source to index in {}", root.display());
                    return Ok(4);
                }
                command(&tool, &root)
                    .args(["query", &question, "--graph"])
                    .arg(graph_file(&root))
                    .args(["--budget", "1500"])
                    .status()
                    .map(|s| s.code().unwrap_or(1))
                    .map_err(|e| e.to_string())
            };
            query().unwrap_or_else(|e| {
                util::warn(e);
                1
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewer_embeds_graph_without_html_script_breakout() {
        let root = std::env::temp_dir().join(format!("pf-graph-view-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        drop(output_folder(&root).unwrap());
        fs::write(
            graph_file(&root),
            r#"{"nodes":[{"id":"a","label":"</script>","source_file":"a.rs"}],"links":[]}"#,
        )
        .unwrap();
        let page = fs::read_to_string(write_view(&root).unwrap()).unwrap();
        assert!(page.contains("\\u003c/script\\u003e"));
        assert!(page.contains("Rotatable three dimensional code network"));
        fs::remove_dir_all(root).unwrap();
    }
}
