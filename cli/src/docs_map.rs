//! `pf docs-map`: keep a repo's documentation in one connected tree under `docs/`.
//!
//! Rules (CONSTITUTION §1):
//!  1. Documentation lives under `<docs>/`. At the repo root only README.md, AGENTS.md, CLAUDE.md
//!     and tool-required files may stay. Folders whose name starts with `.` hold payload, not docs.
//!  2. Every folder under `<docs>/` that holds documents has a README.md.
//!  3. Every README carries a generated routing block: parent README, documentation home, agent
//!     guide, the documents in its folder and its subfolder READMEs. Every other document links
//!     back to its folder README.
//!  4. `<docs>/README.md` carries a generated map of the whole tree.
//!  5. Every relative link and image in every tracked Markdown file resolves, and every document
//!     is reachable from `<docs>/README.md`.
//!
//! Only git-tracked and untracked-but-not-ignored files are considered.

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use crate::util;

pub const ROUTE_START: &str = "<!-- documentation-routing:start -->";
pub const ROUTE_END: &str = "<!-- documentation-routing:end -->";
const MAP_START: &str = "<!-- documentation-map:start -->";
const MAP_END: &str = "<!-- documentation-map:end -->";
const TOOL_CMD: &str = "pf docs-map";
const ROOT_ALLOWED: &[&str] = &[
    "README.md",
    "AGENTS.md",
    "CLAUDE.md",
    "CHANGELOG.md",
    "LICENSE.md",
    "SECURITY.md",
    "CODE_OF_CONDUCT.md",
];

static LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"!?\[[^\]]*\]\(([^)\s]+)(?:\s+"[^"]*")?\)"#).unwrap());
static FENCE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)```.*?```").unwrap());
static TICKS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"`[^`\n]*`").unwrap());
static SCHEME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z][a-z0-9+.-]*:").unwrap());
static STATUS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\*\*Status:\*\*\s*(.+)").unwrap());
static BLANKS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\n{3,}").unwrap());

pub struct Options {
    pub root: PathBuf,
    pub docs: String,
    pub fix: bool,
    pub strict: bool,
    pub allow: Vec<String>,
}

#[derive(Default)]
pub struct Report {
    pub created: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub documents: usize,
}

impl Report {
    pub fn summary(&self) -> String {
        if self.errors.is_empty() {
            format!("docs map OK ({} documents)", self.documents)
        } else {
            format!("{} docs problem(s); try --fix", self.errors.len())
        }
    }
}

/// Parent of a `/`-separated relative path ("" for a top-level name), like os.path.dirname.
fn dirname(p: &str) -> &str {
    p.rfind('/').map(|i| &p[..i]).unwrap_or("")
}

fn basename(p: &str) -> &str {
    p.rsplit('/').next().unwrap_or(p)
}

fn read(p: &Path) -> String {
    fs::read_to_string(p).unwrap_or_default()
}

fn title_of(p: &Path, fallback: &str) -> String {
    read(p)
        .lines()
        .take(15)
        .find_map(|l| l.strip_prefix("# ").map(|t| t.trim().to_string()))
        .unwrap_or(fallback.into())
}

fn status_of(p: &Path) -> Option<String> {
    read(p)
        .lines()
        .take(10)
        .find_map(|l| STATUS.captures(l.trim()).map(|c| c[1].trim().to_string()))
}

fn markdown_files(root: &Path) -> Vec<String> {
    let listed = util::output(
        "git",
        &[
            "-C",
            &root.to_string_lossy(),
            "ls-files",
            "-co",
            "--exclude-standard",
            "--",
            "*.md",
        ],
        None,
    );
    let mut files: Vec<String> = match listed {
        Some(out) => out
            .lines()
            .filter(|f| !f.is_empty())
            .map(str::to_string)
            .collect(),
        None => walk(root, root),
    };
    files.retain(|f| {
        let p = root.join(f);
        p.exists() && !p.is_symlink() // links are copies, not docs
    });
    files.sort();
    files.dedup();
    files
}

fn walk(root: &Path, dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for e in fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().into_owned();
        if p.is_dir() && !name.starts_with('.') && name != "node_modules" {
            out.extend(walk(root, &p));
        } else if name.ends_with(".md") {
            out.push(util::rel(root, &p));
        }
    }
    out
}

/// Replace the text between markers, or append the block at the end.
fn replace_block(text: &str, start: &str, end: &str, block: &str) -> String {
    if let (Some(s), Some(e_rel)) = (
        text.find(start),
        text.find(start).and_then(|s| text[s..].find(end)),
    ) {
        let e = s + e_rel + end.len();
        return format!("{}{}{}", &text[..s], block, &text[e..]);
    }
    format!("{}\n\n{}\n", text.trim_end(), block)
}

struct Tree {
    root: PathBuf,
    docs: String,
    docs_files: Vec<String>,
    dirs: BTreeSet<String>,
}

impl Tree {
    fn new(root: &Path, docs: &str, files: &[String]) -> Self {
        let prefix = format!("{docs}/");
        let docs_files: Vec<String> = files
            .iter()
            .filter(|f| f.starts_with(&prefix))
            .cloned()
            .collect();
        let mut dirs = BTreeSet::new();
        for f in &docs_files {
            let mut d = dirname(f);
            while !d.is_empty() && d.starts_with(docs) {
                dirs.insert(d.to_string());
                if d == docs {
                    break;
                }
                d = dirname(d);
            }
        }
        dirs.insert(docs.to_string());
        Tree {
            root: root.to_path_buf(),
            docs: docs.to_string(),
            docs_files,
            dirs,
        }
    }

    fn children(&self, d: &str) -> (Vec<String>, Vec<String>) {
        let mut files: Vec<String> = self
            .docs_files
            .iter()
            .filter(|f| dirname(f) == d && basename(f) != "README.md")
            .cloned()
            .collect();
        files.sort();
        let subdirs = self
            .dirs
            .iter()
            .filter(|x| dirname(x) == d && *x != d)
            .cloned()
            .collect();
        (files, subdirs)
    }

    fn readme(&self, d: &str) -> PathBuf {
        self.root.join(d).join("README.md")
    }

    fn abs(&self, f: &str) -> PathBuf {
        self.root.join(f)
    }

    fn routing_block(&self, d: &str, is_readme: bool, path_abs: &Path) -> String {
        let here = path_abs.parent().unwrap_or(&self.root);
        let agents = self.root.join("AGENTS.md");
        let mut lines = vec![
            ROUTE_START.to_string(),
            "## Documentation routing".into(),
            String::new(),
        ];
        if is_readme {
            let parent = if d == self.docs {
                self.root.join("README.md")
            } else {
                self.readme(dirname(d))
            };
            if parent.exists() {
                lines.push(format!(
                    "- Parent: [{}]({})",
                    title_of(&parent, "README"),
                    util::rel(here, &parent)
                ));
            }
        } else {
            let p = self.readme(d);
            lines.push(format!(
                "- Parent: [{}]({})",
                title_of(&p, d),
                util::rel(here, &p)
            ));
        }
        let parent_is_home =
            (is_readme && dirname(d) == self.docs) || (!is_readme && d == self.docs);
        if d != self.docs && !parent_is_home {
            let home = self.readme(&self.docs);
            lines.push(format!(
                "- Documentation home: [{}]({})",
                title_of(&home, "Documentation"),
                util::rel(here, &home)
            ));
        }
        if agents.exists() {
            lines.push(format!(
                "- Agent guide: [AGENTS.md]({})",
                util::rel(here, &agents)
            ));
        }
        if is_readme {
            let (files, subdirs) = self.children(d);
            if !files.is_empty() {
                lines.push("- In this folder:".into());
                for f in &files {
                    let p = self.abs(f);
                    lines.push(format!(
                        "  - [{}]({})",
                        title_of(&p, basename(f)),
                        util::rel(here, &p)
                    ));
                }
            }
            if !subdirs.is_empty() {
                lines.push("- Subfolders:".into());
                for s in &subdirs {
                    let p = self.readme(s);
                    lines.push(format!(
                        "  - [{}]({})",
                        title_of(&p, basename(s)),
                        util::rel(here, &p)
                    ));
                }
            }
        }
        lines.push(ROUTE_END.into());
        lines.join("\n")
    }

    fn map_block(&self) -> String {
        let home = self.readme(&self.docs);
        let here = home.parent().unwrap().to_path_buf();
        let mut out = vec![
            MAP_START.to_string(),
            "## Documentation map".into(),
            String::new(),
            format!("Generated by `{TOOL_CMD} --fix`. Do not edit by hand."),
            String::new(),
        ];
        self.walk_map(&self.docs, 0, &here, &mut out);
        out.push(MAP_END.into());
        out.join("\n")
    }

    fn walk_map(&self, d: &str, depth: usize, here: &Path, out: &mut Vec<String>) {
        let entry = |p: &Path, out: &mut Vec<String>| {
            let st = if p.exists() { status_of(p) } else { None };
            let suffix = st.map(|s| format!(" — {s}")).unwrap_or_default();
            let fallback = p
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            out.push(format!(
                "{}- [{}]({}){}",
                "  ".repeat(depth),
                title_of(p, &fallback),
                util::rel(here, p),
                suffix
            ));
        };
        let (files, subdirs) = self.children(d);
        for f in &files {
            entry(&self.abs(f), out);
        }
        for s in &subdirs {
            entry(&self.readme(s), out);
            self.walk_map(s, depth + 1, here, out);
        }
    }
}

fn capitalize(s: &str) -> String {
    let lower = s.to_lowercase();
    let mut c = lower.chars();
    c.next()
        .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
        .unwrap_or_default()
}

fn fix(tree: &mut Tree) -> Vec<String> {
    let mut created = Vec::new();
    for d in tree.dirs.clone() {
        let p = tree.readme(&d);
        if !p.exists() {
            let name = capitalize(&basename(&d).replace(['-', '_'], " "));
            let _ = fs::write(
                &p,
                format!(
                    "# {name}\n\n**Status:** Draft\n**Last updated:** —\n\nDescribe what this folder holds.\n"
                ),
            );
            let relp = format!("{d}/README.md");
            created.push(relp.clone());
            tree.docs_files.push(relp);
        }
    }
    let home_rel = format!("{}/README.md", tree.docs);
    for f in tree.docs_files.clone() {
        let p = tree.abs(&f);
        let d = dirname(&f);
        let is_readme = basename(&f) == "README.md";
        let mut text = replace_block(
            &read(&p),
            ROUTE_START,
            ROUTE_END,
            &tree.routing_block(d, is_readme, &p),
        );
        if f == home_rel {
            text = replace_block(&text, MAP_START, MAP_END, &tree.map_block());
            if let (Some(m), Some(r)) = (text.find(MAP_START), text.find(ROUTE_START))
                && m > r
            {
                // keep the map above the routing block
                let e = text[m..]
                    .find(MAP_END)
                    .map(|i| m + i + MAP_END.len())
                    .unwrap();
                let block = text[m..e].to_string();
                text = format!("{}\n", text.replacen(&block, "", 1).trim_end());
                text = text.replacen(ROUTE_START, &format!("{block}\n\n{ROUTE_START}"), 1);
            }
        }
        let _ = fs::write(&p, BLANKS.replace_all(&text, "\n\n").as_ref());
    }
    created
}

fn check(tree: &Tree, files: &[String], o: &Options, r: &mut Report) {
    let prefix = format!("{}/", tree.docs);
    // 1. documentation outside docs/
    for f in files {
        if f.starts_with(&prefix) || o.allow.iter().any(|a| f.starts_with(a.as_str())) {
            continue;
        }
        let parts: Vec<&str> = f.split('/').collect();
        if parts[..parts.len() - 1].iter().any(|p| p.starts_with('.')) {
            continue; // tool folders (.claude/, .factory/, .github/) hold payload, not docs
        }
        if parts.len() == 1 && ROOT_ALLOWED.contains(&f.as_str()) {
            continue;
        }
        let msg = format!("documentation outside {}/: {f}", tree.docs);
        if o.strict {
            r.errors.push(msg)
        } else {
            r.warnings.push(msg)
        }
    }
    // 2. READMEs
    for d in &tree.dirs {
        if !tree.readme(d).exists() {
            r.errors.push(format!("missing {d}/README.md"));
        }
    }
    // 3/4. generated blocks are current
    for f in &tree.docs_files {
        let p = tree.abs(f);
        if !p.exists() {
            continue;
        }
        let text = read(&p);
        if !text.contains(&tree.routing_block(dirname(f), basename(f) == "README.md", &p)) {
            r.errors
                .push(format!("{f}: routing block missing or stale"));
        }
        if o.strict && status_of(&p).is_none() {
            r.errors.push(format!("{f}: missing **Status:** header"));
        }
    }
    let home = tree.readme(&tree.docs);
    if home.exists() && !read(&home).contains(&tree.map_block()) {
        r.errors.push(format!(
            "{}/README.md: documentation map missing or stale",
            tree.docs
        ));
    }
    // 5. links resolve; build the link graph
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for f in files {
        let p = tree.abs(f);
        let text = read(&p);
        let text = TICKS
            .replace_all(&FENCE.replace_all(&text, ""), "")
            .into_owned();
        let mut targets = Vec::new();
        for c in LINK.captures_iter(&text) {
            let t = &c[1];
            if SCHEME.is_match(t) || t.starts_with('#') || t.starts_with('<') {
                continue;
            }
            let t = t.split('#').next().unwrap_or("");
            if t.is_empty() {
                continue;
            }
            let resolved = util::normalize(&p.parent().unwrap().join(t));
            if !resolved.exists() {
                r.errors.push(format!("{f}: broken link -> {t}"));
            } else if resolved.extension().is_some_and(|e| e == "md") {
                targets.push(util::rel(&tree.root, &resolved));
            }
        }
        graph.insert(f.clone(), targets);
    }
    let start = format!("{}/README.md", tree.docs);
    let mut seen: HashSet<String> = HashSet::from([start.clone()]);
    let mut q = VecDeque::from([start.clone()]);
    while let Some(n) = q.pop_front() {
        for m in graph.get(&n).into_iter().flatten() {
            if seen.insert(m.clone()) {
                q.push_back(m.clone());
            }
        }
    }
    for f in &tree.docs_files {
        if !seen.contains(f) {
            r.errors.push(format!("{f}: not reachable from {start}"));
        }
    }
    if tree.root.join("README.md").exists()
        && !graph.get("README.md").is_some_and(|t| t.contains(&start))
    {
        r.warnings.push(format!("README.md does not link {start}"));
    }
}

/// Fix (optional) and check. Pure library call; printing is the caller's job.
pub fn run(o: &Options) -> Report {
    let mut r = Report::default();
    let docs_dir = o.root.join(&o.docs);
    if !docs_dir.is_dir() {
        if !o.fix {
            r.errors.push(format!(
                "no {}/ folder in {} (--fix creates it)",
                o.docs,
                o.root.display()
            ));
            return r;
        }
        let _ = fs::create_dir_all(&docs_dir);
    }
    let mut files = markdown_files(&o.root);
    let mut tree = Tree::new(&o.root, &o.docs, &files);
    if o.fix {
        r.created = fix(&mut tree);
        files = markdown_files(&o.root);
        tree = Tree::new(&o.root, &o.docs, &files);
    }
    check(&tree, &files, o, &mut r);
    r.documents = tree.docs_files.len();
    r
}

pub fn main(
    root: Option<PathBuf>,
    docs: String,
    fix: bool,
    strict: bool,
    allow: Vec<String>,
) -> i32 {
    let root = util::git_root(&root.unwrap_or_else(|| std::env::current_dir().unwrap_or_default()));
    let r = run(&Options {
        root,
        docs,
        fix,
        strict,
        allow,
    });
    for c in &r.created {
        println!("created {c} (stub: fill it in)");
    }
    for w in &r.warnings {
        println!("WARN {w}");
    }
    for e in &r.errors {
        println!("FAIL {e}");
    }
    println!("{}", r.summary());
    i32::from(!r.errors.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo() -> PathBuf {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let d = std::env::temp_dir().join(format!("pf-docs-{n}"));
        fs::create_dir_all(&d).unwrap();
        assert!(util::run("git", &["init", "-q"], Some(&d)));
        d
    }

    fn opts(root: &Path, fix: bool, strict: bool) -> Options {
        Options {
            root: root.to_path_buf(),
            docs: "docs".into(),
            fix,
            strict,
            allow: vec![],
        }
    }

    #[test]
    fn fix_builds_a_connected_tree() {
        let d = repo();
        fs::write(
            d.join("README.md"),
            "# Repo\n\nSee [docs](docs/README.md).\n",
        )
        .unwrap();
        fs::write(d.join("AGENTS.md"), "# Agents\n").unwrap();
        fs::create_dir_all(d.join("docs/product")).unwrap();
        fs::write(
            d.join("docs/product/PRD-001.md"),
            "# PRD-001 Login\n\n**Status:** Draft\n",
        )
        .unwrap();
        let before = run(&opts(&d, false, false));
        assert!(
            before
                .errors
                .iter()
                .any(|e| e.contains("missing docs/README.md"))
        );
        let fixed = run(&opts(&d, true, false));
        assert!(fixed.errors.is_empty(), "{:?}", fixed.errors);
        assert_eq!(
            fixed.created,
            vec!["docs/README.md", "docs/product/README.md"]
        );
        let home = read(&d.join("docs/README.md"));
        assert!(home.contains("- [Product](product/README.md) — Draft"));
        assert!(home.contains("  - [PRD-001 Login](product/PRD-001.md) — Draft"));
        assert!(
            read(&d.join("docs/product/PRD-001.md")).contains("- Parent: [Product](README.md)")
        );
        // idempotent
        let again = run(&opts(&d, true, false));
        assert!(again.created.is_empty() && again.errors.is_empty());
        fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn reports_broken_links_strays_and_staleness() {
        let d = repo();
        fs::create_dir_all(d.join("docs")).unwrap();
        fs::write(d.join("NOTES.md"), "# Notes\n").unwrap();
        fs::write(
            d.join("docs/a.md"),
            "# A\n\n**Status:** Active\n\n[x](missing.md) ![i](gone.svg)\n",
        )
        .unwrap();
        run(&opts(&d, true, false));
        let r = run(&opts(&d, false, true));
        assert!(
            r.errors
                .iter()
                .any(|e| e == "docs/a.md: broken link -> missing.md")
        );
        assert!(
            r.errors
                .iter()
                .any(|e| e == "docs/a.md: broken link -> gone.svg")
        );
        assert!(
            r.errors
                .iter()
                .any(|e| e == "documentation outside docs/: NOTES.md")
        );
        assert!(
            r.errors
                .iter()
                .any(|e| e == "docs/README.md: missing **Status:** header")
                == false
        );
        fs::write(d.join("docs/b.md"), "# B\n\n**Status:** Active\n").unwrap();
        let stale = run(&opts(&d, false, false));
        assert!(
            stale
                .errors
                .iter()
                .any(|e| e.contains("documentation map missing or stale"))
        );
        fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn capitalize_matches_python() {
        assert_eq!(capitalize("getting STARTED"), "Getting started");
    }
}
