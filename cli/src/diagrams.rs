//! `pf diagrams`: Mermaid sources (`docs/assets/diagrams/*.mmd`) rendered to committed SVGs.
//!
//! Docs show SVG images, so diagrams display on GitHub, GitLab mirrors, IDEs and offline
//! bundles even when GitHub's live Mermaid renderer is unreachable. The `.mmd` is the only
//! source. Each SVG's first line records the sha256 of the source it came from.
//!
//! `check` needs nothing but this binary. `render` calls the Mermaid CLI, because Mermaid exists
//! only as JavaScript: `$MMDC`, else `mmdc` on PATH, else `npx -y @mermaid-js/mermaid-cli@11`.
//! It uses `$MERMAID_CHROME` or a system Chrome/Chromium to avoid Puppeteer's browser download.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::util;

pub const DIR: &str = "docs/assets/diagrams";

fn sources(root: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = fs::read_dir(root.join(DIR))
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "mmd"))
        .collect();
    v.sort();
    v
}

/// (number of sources, failure messages)
pub fn check(root: &Path) -> (usize, Vec<String>) {
    let srcs = sources(root);
    let mut fails = Vec::new();
    for src in &srcs {
        let svg = src.with_extension("svg");
        let shown = util::rel(root, &svg);
        let Ok(body) = fs::read_to_string(&svg) else {
            fails.push(format!("missing {shown}"));
            continue;
        };
        let sha = util::sha256_file(src).unwrap_or_default();
        if !body.contains(&format!("source-sha256: {sha}")) {
            fails.push(format!("stale {shown} (run pf diagrams render)"));
        }
    }
    (srcs.len(), fails)
}

fn mermaid_cli() -> Vec<String> {
    if let Some(m) = std::env::var_os("MMDC") {
        return vec![m.to_string_lossy().into_owned()];
    }
    if util::has_cmd("mmdc") {
        return vec!["mmdc".into()];
    }
    vec![
        "npx".into(),
        "-y".into(),
        "@mermaid-js/mermaid-cli@11".into(),
    ]
}

pub fn render(root: &Path) -> i32 {
    let cli = mermaid_cli();
    let chrome = std::env::var_os("MERMAID_CHROME")
        .map(PathBuf::from)
        .or_else(|| {
            ["google-chrome", "chromium", "chromium-browser"]
                .iter()
                .find_map(|c| util::which(c))
        });
    let pp = std::env::temp_dir().join(format!("pf-puppeteer-{}.json", std::process::id()));
    let cfg = match &chrome {
        Some(c) => serde_json::json!({"executablePath": c, "args": ["--no-sandbox"]}),
        None => serde_json::json!({"args": ["--no-sandbox"]}),
    };
    if fs::write(&pp, cfg.to_string()).is_err() {
        eprintln!("FAIL cannot write {}", pp.display());
        return 1;
    }
    let mut code = 0;
    for src in sources(root) {
        let svg = src.with_extension("svg");
        let mut cmd = Command::new(&cli[0]);
        cmd.args(&cli[1..])
            .args(["-q", "-p"])
            .arg(&pp)
            .args(["-t", "neutral", "-b", "white", "-i"])
            .arg(&src)
            .arg("-o")
            .arg(&svg);
        if chrome.is_some() {
            cmd.env("PUPPETEER_SKIP_DOWNLOAD", "1");
        }
        let ok = cmd.status().map(|s| s.success()).unwrap_or(false);
        let body = fs::read_to_string(&svg).unwrap_or_default();
        if !ok || body.is_empty() || body.to_lowercase().contains("syntax error") {
            eprintln!(
                "FAIL {}: Mermaid render or syntax error",
                util::rel(root, &src)
            );
            code = 1;
            break;
        }
        let sha = util::sha256_file(&src).unwrap_or_default();
        let name = src.file_name().unwrap().to_string_lossy();
        let header = format!(
            "<!-- source-sha256: {sha} — generated from {name} by pf diagrams render; do not edit -->\n"
        );
        let _ = fs::write(&svg, header + &body);
        println!("rendered {}", util::rel(root, &svg));
    }
    let _ = fs::remove_file(&pp);
    code
}

pub fn main(root: Option<PathBuf>, do_render: bool) -> i32 {
    let root = util::git_root(&root.unwrap_or_else(|| std::env::current_dir().unwrap_or_default()));
    if do_render {
        let c = render(&root);
        if c != 0 {
            return c;
        }
    }
    let (n, fails) = check(&root);
    for f in &fails {
        println!("FAIL {f}");
    }
    if fails.is_empty() {
        println!("diagrams OK ({n} sources)");
        0
    } else {
        1
    }
}
