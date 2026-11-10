//! Infer knowledge targets from local Git and manifest evidence, without running project code.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::{descriptor, util};

fn regular_file(path: &Path) -> bool {
    path.symlink_metadata().is_ok_and(|m| m.is_file())
}

fn stack(root: &Path, web_workspace: bool) -> &'static str {
    let gradle = [
        "build.gradle.kts",
        "build.gradle",
        "gradle/libs.versions.toml",
    ]
    .into_iter()
    .map(|name| root.join(name))
    .filter(|p| regular_file(p))
    .filter_map(|p| fs::read_to_string(p).ok())
    .collect::<Vec<_>>()
    .join("\n");
    if ["org.jetbrains.kotlin.multiplatform", "kotlin.multiplatform"]
        .iter()
        .any(|id| gradle.contains(id))
    {
        "kmp-android-client"
    } else if ["org.springframework.boot", "spring.boot"]
        .iter()
        .any(|id| gradle.contains(id))
        && gradle.contains("kotlin")
    {
        "spring-kotlin-service"
    } else if regular_file(&root.join("package.json"))
        && (web_workspace
            || regular_file(&root.join("tsconfig.json"))
            || fs::read_to_string(root.join("package.json")).is_ok_and(|s| {
                serde_json::from_str::<serde_json::Value>(&s).is_ok_and(|v| {
                    ["dependencies", "devDependencies"]
                        .iter()
                        .any(|key| v[key].get("typescript").is_some())
                })
            }))
    {
        "typescript-web"
    } else if regular_file(&root.join("product/README.md"))
        && regular_file(&root.join("development/README.md"))
    {
        "product-docs"
    } else {
        "generic"
    }
}

pub fn docs_root(root: &Path, stack: &str) -> &'static str {
    if stack == "product-docs" && root.join("product").is_dir() {
        "product"
    } else if root.join("doc").is_dir() && !root.join("docs").is_dir() {
        "doc"
    } else {
        "docs"
    }
}

pub fn local_path(root: &Path, relative: &Path) -> bool {
    let mut path = root.to_path_buf();
    relative.components().all(|part| match part {
        Component::CurDir => true,
        Component::Normal(part) if part != ".git" => {
            path.push(part);
            match path.symlink_metadata() {
                Ok(m) => !m.is_symlink(),
                Err(e) => e.kind() == std::io::ErrorKind::NotFound,
            }
        }
        _ => false,
    })
}

fn git_paths(root: &Path, args: &[&str]) -> Vec<String> {
    // Use the raw output because trimming would corrupt whitespace in a path.
    let Ok(out) = std::process::Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
    else {
        return vec![];
    };
    if !out.status.success() {
        return vec![];
    }
    out.stdout
        .split(|b| *b == 0)
        .filter_map(|p| std::str::from_utf8(p).ok())
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect()
}

fn workspace_has_typescript(root: &Path, files: &[String]) -> bool {
    if !regular_file(&root.join("pnpm-workspace.yaml"))
        && !fs::read_to_string(root.join("package.json")).is_ok_and(|s| {
            serde_json::from_str::<serde_json::Value>(&s)
                .is_ok_and(|v| v.get("workspaces").is_some())
        })
    {
        return false;
    }
    files.iter().any(|f| {
        f.ends_with("/tsconfig.json")
            && local_path(root, Path::new(f))
            && regular_file(&root.join(f))
    })
}

pub fn infer_stack(root: &Path) -> &'static str {
    let detected = stack(root, false);
    if detected == "generic" && regular_file(&root.join("package.json")) {
        return stack(root, workspace_has_typescript(root, &project_files(root)));
    }
    detected
}

fn project_files(root: &Path) -> Vec<String> {
    git_paths(
        root,
        &[
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
            "--",
            ".",
        ],
    )
}

pub fn gitlinks(root: &Path) -> Vec<PathBuf> {
    git_paths(root, &["ls-files", "-z", "--stage"])
        .iter()
        .filter(|line| line.starts_with("160000 "))
        .filter_map(|line| line.split_once('\t').map(|(_, path)| PathBuf::from(path)))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Gitlinks and visible nested Git checkouts, plus checkouts declared by the workspace.
pub fn checkout_boundaries(root: &Path) -> BTreeSet<PathBuf> {
    let mut paths: BTreeSet<_> = gitlinks(root).into_iter().collect();
    let visible = project_files(root);
    let descriptor = descriptor::find(root).filter(|d| d.root == root && d.multi_repo());
    let declared = descriptor
        .iter()
        .flat_map(|d| d.modules.iter())
        .filter_map(|m| m.get("path"))
        .map(|p| p.trim_end_matches('/'));
    for path in visible
        .iter()
        .filter_map(|p| p.strip_suffix('/'))
        .chain(declared)
    {
        let path = Path::new(path);
        let child = root.join(path);
        if !path.as_os_str().is_empty()
            && path != Path::new(".")
            && local_path(root, path)
            && child.join(".git").exists()
            && util::git_root(&child) == child
        {
            paths.insert(path.to_path_buf());
        }
    }
    paths
}

fn discover(root: &Path, roots: &mut BTreeSet<PathBuf>, missing: &mut Vec<PathBuf>) {
    if !roots.insert(root.to_path_buf()) {
        return;
    }
    for relative in checkout_boundaries(root) {
        let child = root.join(&relative);
        if local_path(root, &relative)
            && child.is_dir()
            && util::git_root(&child) == child
            && child.join(".git").exists()
        {
            discover(&child, roots, missing);
        } else {
            missing.push(child);
        }
    }
    // A recognized manifest can start a stack in a shared Git repo. Packages of
    // the same stack inherit their nearest handbook unless they declare their own.
    let mut candidates = BTreeSet::new();
    for file in project_files(root) {
        let path = Path::new(&file);
        let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) else {
            continue;
        };
        if parent.components().any(|p| {
            let name = p.as_os_str().to_string_lossy();
            name.starts_with('.')
                || ["docs", "doc", "node_modules", "vendor"].contains(&name.as_ref())
        }) {
            continue;
        }
        if matches!(
            path.file_name().and_then(|name| name.to_str()),
            Some("AGENTS.md" | "package.json" | "build.gradle.kts" | "build.gradle")
        ) && local_path(root, path)
            && regular_file(&root.join(path))
        {
            candidates.insert(parent.to_path_buf());
        }
    }
    let mut owners = vec![(root.to_path_buf(), infer_stack(root))];
    for relative in candidates {
        let module = root.join(relative);
        let stack = infer_stack(&module);
        let same_as_parent = owners
            .iter()
            .rev()
            .find(|(p, _)| module.starts_with(p))
            .is_some_and(|(_, parent)| *parent == stack);
        if stack != "generic" && (!same_as_parent || regular_file(&module.join("AGENTS.md"))) {
            roots.insert(module.clone());
            owners.push((module, stack));
        }
    }
}

pub fn inferred_roots(root: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut roots = BTreeSet::new();
    let mut missing = Vec::new();
    discover(root, &mut roots, &mut missing);
    (roots.into_iter().collect(), missing)
}

/// In workspace mode the descriptor is authoritative. Before adoption, discover
/// checked-out Git submodules recursively and report missing checkouts.
pub fn roots(start: &Path, modules: bool) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let start = util::absolute(start);
    if !modules {
        return (vec![start], vec![]);
    }
    if let Some(d) = descriptor::find(&start).filter(|d| d.multi_repo() || d.modules.len() > 1) {
        let mut roots = BTreeSet::from([d.root.clone()]);
        let mut missing = Vec::new();
        let git_root = util::git_root(&d.root);
        let links = gitlinks(&d.root);
        for path in d.modules.iter().filter_map(|m| m.get("path")) {
            let relative = Path::new(path);
            let child = util::normalize(&d.root.join(relative));
            if !path.is_empty()
                && local_path(&d.root, relative)
                && child.is_dir()
                && (util::git_root(&child) == git_root
                    || (child.join(".git").exists() && util::git_root(&child) == child))
                && !links
                    .iter()
                    .any(|p| p == relative && !child.join(".git").exists())
            {
                roots.insert(child);
            } else {
                missing.push(child);
            }
        }
        (roots.into_iter().collect(), missing)
    } else {
        inferred_roots(&util::git_root(&start))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_stack_boundaries_and_reports_unavailable_gitlinks() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("pf-discover-{unique}"));
        fs::create_dir_all(root.join("apps/web")).unwrap();
        fs::create_dir_all(root.join("apps/web/packages/ui")).unwrap();
        assert!(util::run("git", &["init", "-q"], Some(&root)));
        fs::write(
            root.join("apps/web/package.json"),
            r#"{"devDependencies":{"typescript":"*"} }"#,
        )
        .unwrap();
        fs::write(root.join("apps/web/packages/ui/package.json"), "{}").unwrap();
        assert!(util::run(
            "git",
            &[
                "update-index",
                "--add",
                "--cacheinfo",
                "160000,0123456789012345678901234567890123456789,modules/absent",
            ],
            Some(&root)
        ));
        let child = root.join("modules/standalone");
        fs::create_dir_all(&child).unwrap();
        assert!(util::run("git", &["init", "-q"], Some(&child)));
        fs::write(child.join("main.rs"), "fn main() {}").unwrap();
        let (roots, missing) = inferred_roots(&root);
        assert_eq!(roots, [root.clone(), root.join("apps/web"), child]);
        assert_eq!(missing, [root.join("modules/absent")]);
        assert_eq!(infer_stack(&roots[1]), "typescript-web");
        fs::remove_dir_all(root).unwrap();
    }
}
