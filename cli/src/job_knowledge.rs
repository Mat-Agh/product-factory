//! Inspect committed job changes and enforce the knowledge evidence contract.

use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};

use crate::{discovery, util};

pub const MARKER: &str = "<!-- pf: knowledge-capture-v1 -->";

fn dependency_file(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    matches!(
        name,
        "Cargo.toml"
            | "Cargo.lock"
            | "package-lock.json"
            | "pnpm-lock.yaml"
            | "yarn.lock"
            | "bun.lock"
            | "bun.lockb"
            | "deno.lock"
            | "uv.lock"
            | "pyproject.toml"
            | "poetry.lock"
            | "Pipfile"
            | "Pipfile.lock"
            | "go.mod"
            | "go.sum"
            | "Gemfile"
            | "Gemfile.lock"
            | "composer.json"
            | "composer.lock"
            | "build.gradle"
            | "build.gradle.kts"
            | "libs.versions.toml"
            | "gradle.lockfile"
            | "pom.xml"
            | "Package.swift"
            | "Package.resolved"
            | "pubspec.yaml"
            | "pubspec.lock"
            | "packages.config"
            | "Directory.Packages.props"
            | "packages.lock.json"
    ) || path.extension().is_some_and(|ext| ext == "csproj")
        || (name.starts_with("requirements") && name.ends_with(".txt"))
}

fn package_dependencies(tree: &Path, revision: &str, path: &Path) -> Option<Value> {
    let spec = format!("{revision}:{}", path.to_str()?);
    let output = Command::new("git")
        .args(["show", &spec])
        .current_dir(tree)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let package: Value = serde_json::from_slice(&output.stdout).ok()?;
    package.as_object()?;
    Some(json!([
        package["dependencies"],
        package["devDependencies"],
        package["peerDependencies"],
        package["optionalDependencies"],
        package["overrides"],
        package["resolutions"],
        package["pnpm"]["overrides"],
        package["bundledDependencies"],
        package["bundleDependencies"]
    ]))
}

pub struct Report {
    enabled: bool,
    feature: bool,
    spike: bool,
    dependencies: Vec<PathBuf>,
    docs: Vec<PathBuf>,
}

impl Report {
    pub fn inspect(
        state: &Value,
        tree: &Path,
        base: &str,
        head: &str,
        paths: &[PathBuf],
    ) -> Result<Self, String> {
        let enabled = state["knowledge_capture"] == true;
        let prefix = state["module_worktree"]
            .as_str()
            .map_or(Ok(Path::new("")), |module| {
                Path::new(module)
                    .strip_prefix(tree)
                    .map_err(|_| "Module is outside its job worktree")
            })?;
        let root = Path::new(state["knowledge_docs_root"].as_str().unwrap_or("docs"));
        let root = root.strip_prefix(".").unwrap_or(root);
        if enabled && !discovery::local_path(&tree.join(prefix), root) {
            return Err("Unsafe knowledge docs root".into());
        }
        let mut dependencies = Vec::new();
        let mut docs = Vec::new();
        for path in paths {
            let Ok(relative) = path.strip_prefix(prefix) else {
                continue;
            };
            if dependency_file(relative) {
                dependencies.push(path.clone());
            } else if relative
                .file_name()
                .is_some_and(|name| name == "package.json")
            {
                let before = package_dependencies(tree, base, path);
                let after = package_dependencies(tree, head, path);
                if before.is_none() || after.is_none() || before != after {
                    dependencies.push(path.clone());
                }
            }
            if relative.extension().is_some_and(|ext| ext == "md")
                && (relative.starts_with(root)
                    || ["AGENTS.md", "CLAUDE.md"]
                        .iter()
                        .any(|name| relative == Path::new(name)))
                && discovery::local_path(tree, path)
                && tree.join(path).is_file()
                && util::output(
                    "git",
                    &["cat-file", "-t", &format!("{head}:{}", path.display())],
                    Some(tree),
                )
                .as_deref()
                    == Some("blob")
            {
                docs.push(path.clone());
            }
        }
        Ok(Self {
            enabled,
            feature: state["type"] == "feature",
            spike: state["type"] == "spike",
            dependencies,
            docs,
        })
    }

    fn requires_docs(&self) -> bool {
        self.feature || self.spike || !self.dependencies.is_empty()
    }

    pub fn json(&self, head: &str) -> Value {
        json!({
            "enabled": self.enabled,
            "commit": head,
            "feature": self.feature,
            "spike": self.spike,
            "dependency_files": self.dependencies,
            "changed_docs": self.docs,
            "requires_project_docs": self.requires_docs(),
        })
    }

    pub fn validate(&self, artifact: &str, text: &str) -> Result<(), String> {
        let mut project = false;
        let mut shared = false;
        let mut none = false;
        for line in section(text, "Knowledge capture") {
            if let Some(path) = line.strip_prefix("- Project: ").filter(|s| !s.is_empty()) {
                if !self.docs.iter().any(|p| p.to_str() == Some(path)) {
                    return Err(format!(
                        "{artifact}: knowledge path {path:?} must name changed, committed module documentation"
                    ));
                }
                project = true;
            } else if line
                .strip_prefix("- Shared: ")
                .is_some_and(|s| !s.trim().is_empty())
            {
                shared = true;
            } else if line
                .strip_prefix("- None: ")
                .is_some_and(|s| !s.trim().is_empty())
                && !none
            {
                none = true;
            } else {
                return Err(format!("{artifact}: invalid entry in ## Knowledge capture"));
            }
        }
        if self.requires_docs() && !project {
            return Err(format!(
                "{artifact}: ## Knowledge capture must cite changed project docs for a feature, spike or dependency change"
            ));
        }
        if !(project && !none || none && !project && !shared) {
            return Err(format!(
                "{artifact}: ## Knowledge capture needs Project paths or one None reason. Shared proposals also need a Project path"
            ));
        }
        Ok(())
    }
}

pub fn validate_review(text: &str) -> Result<(), String> {
    if section(text, "Knowledge review").any(|line| {
        line.strip_prefix("- Checked: ")
            .is_some_and(|s| !s.trim().is_empty())
    }) {
        Ok(())
    } else {
        Err("review-code.md: PASS requires ## Knowledge review with - Checked: and the review evidence".into())
    }
}

fn section<'a>(text: &'a str, title: &str) -> impl Iterator<Item = &'a str> {
    let heading = format!("## {title}");
    text.lines()
        .map(str::trim)
        .skip_while(move |line| *line != heading)
        .skip(1)
        .take_while(|line| !line.starts_with("## "))
        .filter(|line| !line.is_empty())
}
