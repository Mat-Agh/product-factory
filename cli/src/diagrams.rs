//! `pf diagrams`: inline ` ```mermaid ` blocks in Markdown.
//!
//! Diagrams live inside the documents as Mermaid, so GitHub, GitLab and IDE previews
//! render them live and they stay editable text. `check` lints every block with no
//! tools: a known diagram type, no init directives, no HTML except `<br/>`, no `&`
//! edge chaining, balanced quotes, a size limit. Those are the constructs that break
//! older renderers such as GitHub's. `validate` also parses every block with the
//! Mermaid CLI (`$MMDC`, else `mmdc` on PATH, else `npx -y @mermaid-js/mermaid-cli@11`).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::util;

const TYPES: &[&str] = &[
    "flowchart",
    "graph",
    "sequenceDiagram",
    "stateDiagram-v2",
    "stateDiagram",
    "erDiagram",
    "classDiagram",
    "gantt",
    "journey",
    "gitGraph",
    "timeline",
    "pie",
];

/// Lines per diagram. Longer diagrams are split into several focused ones.
const MAX_LINES: usize = 60;

pub struct Block {
    pub file: String,
    /// 1-based line of the opening fence.
    pub line: usize,
    pub body: String,
}

fn markdown_files(root: &Path) -> Vec<String> {
    let out = util::output(
        "git",
        &["ls-files", "-co", "--exclude-standard", "--", "*.md"],
        Some(root),
    )
    .unwrap_or_default();
    let mut v: Vec<String> = out
        .lines()
        .filter(|l| !l.is_empty() && root.join(l).is_file())
        .map(str::to_string)
        .collect();
    v.sort();
    v
}

/// Every Mermaid block in one Markdown text; an unclosed block is returned with
/// `None` as its body end marker in `unclosed`.
fn blocks_in(file: &str, text: &str, unclosed: &mut Vec<usize>) -> Vec<Block> {
    let mut out = Vec::new();
    let mut fence: Option<(String, bool, usize)> = None; // (marker, is_mermaid, line)
    let mut body = String::new();
    for (i, line) in text.lines().enumerate() {
        let t = line.trim_start();
        match &fence {
            None => {
                if let Some(marker) = ["```", "~~~"].iter().find(|m| t.starts_with(**m)) {
                    let lang = t.trim_start_matches(['`', '~']).trim();
                    fence = Some((marker.to_string(), lang == "mermaid", i + 1));
                    body.clear();
                }
            }
            Some((marker, mermaid, start)) => {
                if t.starts_with(marker.as_str())
                    && t.trim_start_matches(['`', '~']).trim().is_empty()
                {
                    if *mermaid {
                        out.push(Block {
                            file: file.to_string(),
                            line: *start,
                            body: body.clone(),
                        });
                    }
                    fence = None;
                } else {
                    body.push_str(line);
                    body.push('\n');
                }
            }
        }
    }
    if let Some((_, true, start)) = fence {
        unclosed.push(start);
    }
    out
}

pub fn blocks(root: &Path) -> (Vec<Block>, Vec<String>) {
    let mut all = Vec::new();
    let mut fails = Vec::new();
    for f in markdown_files(root) {
        let text = fs::read_to_string(root.join(&f)).unwrap_or_default();
        let mut unclosed = Vec::new();
        all.extend(blocks_in(&f, &text, &mut unclosed));
        for l in unclosed {
            fails.push(format!("{f}:{l}: mermaid block is never closed"));
        }
    }
    (all, fails)
}

/// Problems that make a block fail on common renderers.
pub fn lint(b: &Block) -> Vec<String> {
    let mut v = Vec::new();
    let at = |offset: usize| format!("{}:{}", b.file, b.line + offset);
    let lines: Vec<&str> = b.body.lines().collect();
    let first = lines
        .iter()
        .enumerate()
        .find(|(_, l)| !l.trim().is_empty() && !l.trim_start().starts_with("%%"));
    let kind = match first {
        Some((_, l)) => l.split_whitespace().next().unwrap_or("").to_string(),
        None => {
            v.push(format!("{}: empty mermaid block", at(0)));
            return v;
        }
    };
    if !TYPES.contains(&kind.as_str()) {
        v.push(format!(
            "{}: unknown diagram type '{kind}' (use one of: {})",
            at(1),
            TYPES.join(", ")
        ));
    }
    if lines.len() > MAX_LINES {
        v.push(format!(
            "{}: {} lines; split it into focused diagrams of at most {MAX_LINES}",
            at(0),
            lines.len()
        ));
    }
    let flow = kind == "flowchart" || kind == "graph";
    for (i, l) in lines.iter().enumerate() {
        let n = i + 1;
        if l.contains("%%{") {
            v.push(format!(
                "{}: init directive; keep the renderer's default theme",
                at(n)
            ));
        }
        let mut rest = *l;
        while let Some(p) = rest.find('<') {
            let after = &rest[p + 1..];
            let tag = after.starts_with(|c: char| c.is_ascii_alphabetic() || c == '/');
            let br =
                after.starts_with("br>") || after.starts_with("br/>") || after.starts_with("br />");
            if tag && !br {
                v.push(format!(
                    "{}: HTML-like text '<{}'; write it without angle brackets",
                    at(n),
                    after.chars().take(12).collect::<String>()
                ));
                break;
            }
            rest = after;
        }
        if flow && l.contains("--") && l.contains(" & ") {
            v.push(format!(
                "{}: '&' edge chaining; write one edge per line",
                at(n)
            ));
        }
        if l.matches('"').count() % 2 == 1 {
            v.push(format!("{}: unbalanced quotes", at(n)));
        }
    }
    v
}

/// (number of blocks, failure messages). Needs no tools.
pub fn check(root: &Path) -> (usize, Vec<String>) {
    let (bs, mut fails) = blocks(root);
    for b in &bs {
        fails.extend(lint(b));
    }
    (bs.len(), fails)
}

fn mermaid_cli() -> Vec<String> {
    if let Ok(m) = std::env::var("MMDC") {
        return vec![m];
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

/// Parse every block with the Mermaid CLI. Returns failure messages.
pub fn validate(root: &Path, bs: &[Block]) -> Vec<String> {
    let cli = mermaid_cli();
    let chrome = std::env::var_os("MERMAID_CHROME")
        .map(PathBuf::from)
        .or_else(|| {
            ["google-chrome", "chromium", "chromium-browser"]
                .iter()
                .find_map(|c| util::which(c))
        });
    let dir = std::env::temp_dir().join(format!("pf-mermaid-{}", std::process::id()));
    if fs::create_dir_all(&dir).is_err() {
        return vec![format!("cannot create {}", dir.display())];
    }
    let pp = dir.join("puppeteer.json");
    let cfg = match &chrome {
        Some(c) => serde_json::json!({"executablePath": c, "args": ["--no-sandbox"]}),
        None => serde_json::json!({"args": ["--no-sandbox"]}),
    };
    let _ = fs::write(&pp, cfg.to_string());
    let mut fails = Vec::new();
    for (i, b) in bs.iter().enumerate() {
        let src = dir.join(format!("{i}.mmd"));
        let out = dir.join(format!("{i}.svg"));
        let _ = fs::write(&src, &b.body);
        let mut cmd = Command::new(&cli[0]);
        cmd.args(&cli[1..])
            .args(["-q", "-p"])
            .arg(&pp)
            .arg("-i")
            .arg(&src)
            .arg("-o")
            .arg(&out)
            .current_dir(root);
        if chrome.is_some() {
            cmd.env("PUPPETEER_SKIP_DOWNLOAD", "1");
        }
        match cmd.output() {
            Ok(o) if o.status.success() && out.is_file() => {}
            Ok(o) => {
                let err = String::from_utf8_lossy(&o.stderr);
                let msg = err
                    .lines()
                    .find(|l| l.contains("Error") || l.contains("error"))
                    .unwrap_or("Mermaid could not parse it")
                    .trim()
                    .to_string();
                fails.push(format!("{}:{}: {msg}", b.file, b.line));
            }
            Err(e) => {
                fails.push(format!("cannot run {}: {e}", cli.join(" ")));
                break;
            }
        }
    }
    let _ = fs::remove_dir_all(&dir);
    fails
}

pub fn main(root: Option<PathBuf>, full: bool) -> i32 {
    let root = util::git_root(&root.unwrap_or_else(|| std::env::current_dir().unwrap_or_default()));
    let (bs, mut fails) = blocks(&root);
    for b in &bs {
        fails.extend(lint(b));
    }
    if full && fails.is_empty() {
        fails.extend(validate(&root, &bs));
    }
    for f in &fails {
        println!("FAIL {f}");
    }
    if fails.is_empty() {
        let how = if full {
            "parsed by the Mermaid CLI"
        } else {
            "lint clean"
        };
        println!("diagrams OK ({} mermaid blocks, {how})", bs.len());
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(body: &str) -> Block {
        Block {
            file: "a.md".into(),
            line: 3,
            body: body.into(),
        }
    }

    #[test]
    fn finds_blocks_and_unclosed_fences() {
        let text = "# T\n\n```mermaid\nflowchart LR\n  A --> B\n```\n\n```bash\nls\n```\n\n```mermaid\nflowchart TD\n";
        let mut unclosed = Vec::new();
        let bs = blocks_in("a.md", text, &mut unclosed);
        assert_eq!(bs.len(), 1);
        assert_eq!(bs[0].line, 3);
        assert_eq!(bs[0].body, "flowchart LR\n  A --> B\n");
        assert_eq!(unclosed, vec![12]);
    }

    #[test]
    fn lint_flags_constructs_that_break_renderers() {
        assert!(lint(&block("flowchart LR\n  A[\"one<br/>two\"] --> B\n")).is_empty());
        assert!(lint(&block("sequenceDiagram\n  A->>B: hello\n")).is_empty());
        let bad = lint(&block(
            "%%{init: {}}%%\nflowchart LR\n  A --> B & C\n  E[\"engineer-<job>\"] --> F\n  G[\"open --> H\n",
        ));
        let all = bad.join("\n");
        assert!(all.contains("init directive"));
        assert!(all.contains("'&' edge chaining"));
        assert!(all.contains("HTML-like"));
        assert!(all.contains("unbalanced quotes"));
        assert!(lint(&block("mindmap\n  root\n"))[0].contains("unknown diagram type"));
        let long = format!("flowchart TD\n{}", "  A --> B\n".repeat(61));
        assert!(lint(&block(&long))[0].contains("split it"));
    }
}
