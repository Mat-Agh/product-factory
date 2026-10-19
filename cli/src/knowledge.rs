//! `pf knowledge`: the per-stack knowledge structure every factory repo keeps:
//! current state, development history, the stack handbook (principles, tools, code
//! samples, link map, stack chapters), architecture, decisions, features and status.
//!
//! Templates live in `kit/knowledge/`: `base/` for every repo, plus
//! `stacks/<name>/` overlays. A stack's `.exclude` file lists base paths it does not
//! need. Paths under `docs/` map to the module's docs root. `init` creates only
//! missing files and never overwrites. `plan` counts an existing equivalent as
//! present: a handbook chapter with the same number, or a file of the same name
//! elsewhere in the docs.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{descriptor, util};

fn templates(factory: &Path) -> PathBuf {
    factory.join("kit/knowledge")
}

pub fn stacks(factory: &Path) -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(templates(factory).join("stacks"))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    v.sort();
    v
}

fn files_under(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    fn walk(base: &Path, d: &Path, out: &mut Vec<String>) {
        for e in fs::read_dir(d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(base, &p, out);
            } else if e.file_name() != ".exclude" {
                out.push(util::rel(base, &p));
            }
        }
    }
    walk(dir, dir, &mut out);
    out.sort();
    out
}

/// Template path (relative to the template root) → source file, for one stack.
pub fn template_set(factory: &Path, stack: &str) -> Result<BTreeMap<String, PathBuf>, String> {
    let root = templates(factory);
    let sdir = root.join("stacks").join(stack);
    if !sdir.is_dir() {
        return Err(format!(
            "unknown stack '{stack}' (known: {})",
            stacks(factory).join(", ")
        ));
    }
    let exclude: Vec<String> = fs::read_to_string(sdir.join(".exclude"))
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect();
    let mut set = BTreeMap::new();
    for f in files_under(&root.join("base")) {
        if !exclude
            .iter()
            .any(|x| f == *x || f.starts_with(&format!("{}/", x.trim_end_matches('/'))))
        {
            set.insert(f.clone(), root.join("base").join(&f));
        }
    }
    for f in files_under(&sdir) {
        set.insert(f.clone(), sdir.join(&f));
    }
    Ok(set)
}

pub struct Target {
    pub root: PathBuf,
    pub stack: String,
    pub docs_root: String,
    pub module: String,
    pub product: String,
}

/// Resolve stack and docs root: flags, then the descriptor module whose path is this
/// repo, then the single-repo descriptor's first module, then `generic` and `docs`.
pub fn target(root: &Path, stack: Option<String>, docs: Option<String>) -> Target {
    let root = util::git_root(root);
    let name = root
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let mut t = Target {
        root: root.clone(),
        stack: "generic".into(),
        docs_root: "docs".into(),
        module: name,
        product: String::new(),
    };
    if let Some(d) = descriptor::find(&root) {
        t.product = d.get("product.name").unwrap_or("").to_string();
        let rel = util::rel(&d.root, &root);
        let m = d
            .modules
            .iter()
            .find(|m| {
                m.get("path")
                    .is_some_and(|p| p.trim_end_matches('/') == rel)
            })
            .or_else(|| (!d.multi_repo()).then(|| d.modules.first()).flatten());
        if let Some(m) = m {
            if let Some(s) = m.get("stack").filter(|s| !s.is_empty()) {
                t.stack = s.clone();
            }
            if let Some(dr) = m.get("docs_root").filter(|s| !s.is_empty()) {
                t.docs_root = dr.clone();
            } else if let Some(dr) = d.get("docs.root").filter(|_| !d.multi_repo() || rel == ".") {
                t.docs_root = dr.to_string();
            }
            if let Some(a) = m.get("area") {
                t.module = a.clone();
            }
        }
    }
    if let Some(s) = stack {
        t.stack = s;
    }
    if let Some(d) = docs {
        t.docs_root = d;
    }
    t
}

fn map_path(rel: &str, docs_root: &str) -> String {
    match rel.strip_prefix("docs/") {
        Some(rest) => format!("{}/{rest}", docs_root.trim_end_matches('/')),
        None => rel.to_string(),
    }
}

enum Found {
    Same(String),
    NumberTaken(String),
}

const STOP: &[&str] = &[
    "and", "the", "for", "with", "how", "our", "via", "from", "into", "md",
];

fn topic_words(name: &str) -> Vec<String> {
    name.trim_end_matches(".md")
        .split(['-', '_'])
        .skip(1)
        .map(str::to_lowercase)
        .filter(|w| w.len() >= 3 && !STOP.contains(&w.as_str()))
        .collect()
}

fn numbered(name: &str) -> Option<&str> {
    let (num, _) = name.split_once('-')?;
    (num.len() == 2 && num.chars().all(|c| c.is_ascii_digit())).then_some(num)
}

fn md_files(dir: &Path) -> Vec<(String, PathBuf)> {
    let mut v: Vec<_> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_file())
        .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path()))
        .filter(|(n, _)| n.ends_with(".md"))
        .collect();
    v.sort();
    v
}

/// An existing file that plays the role of `wanted`, or one that already uses its chapter number.
fn equivalent(root: &Path, wanted: &str, docs_root: &str) -> Option<Found> {
    let p = root.join(wanted);
    if p.exists() {
        return Some(Found::Same(wanted.to_string()));
    }
    let name = Path::new(wanted)
        .file_name()?
        .to_string_lossy()
        .into_owned();
    let dir = Path::new(wanted).parent().unwrap_or(Path::new(""));
    // Handbook chapters may sit in the wanted folder or flat in the docs root.
    let mut dirs = vec![root.join(dir)];
    if dir != Path::new(docs_root) {
        dirs.push(root.join(docs_root));
    }
    if let Some(num) = numbered(&name) {
        let want = topic_words(&name);
        // The same number on the same topic counts; the same number on another
        // topic is a numbering conflict for the human to resolve.
        for d in &dirs {
            for (n, path) in md_files(d) {
                if numbered(&n) == Some(num) {
                    let found = util::rel(root, &path);
                    let shared = topic_words(&n).iter().any(|w| want.contains(w));
                    return Some(if shared {
                        Found::Same(found)
                    } else {
                        Found::NumberTaken(found)
                    });
                }
            }
        }
        // The same topic under another number (at least two shared topic words,
        // covering half of the wanted ones) counts as the same chapter.
        for d in &dirs {
            for (n, path) in md_files(d) {
                if numbered(&n).is_none() {
                    continue;
                }
                let have = topic_words(&n);
                let shared = want.iter().filter(|w| have.contains(w)).count();
                if shared >= 2 && shared * 2 >= want.len() {
                    return Some(Found::Same(util::rel(root, &path)));
                }
            }
        }
    }
    // A file with the same name elsewhere in the docs (READMEs excluded).
    if name != "README.md" {
        let mut stack = vec![root.join(docs_root)];
        while let Some(d) = stack.pop() {
            for e in fs::read_dir(&d).into_iter().flatten().flatten() {
                let ep = e.path();
                if ep.is_dir() {
                    if e.file_name() != "node_modules" {
                        stack.push(ep);
                    }
                } else if e.file_name().to_string_lossy().eq_ignore_ascii_case(&name) {
                    return Some(Found::Same(util::rel(root, &ep)));
                }
            }
        }
    }
    None
}

/// A template that carries real stack knowledge (not a project-state skeleton).
/// Only these are merged section by section into existing files.
const PACK_MARK: &str = "**Source:** Product Factory knowledge pack";

pub fn is_knowledge(text: &str) -> bool {
    text.contains(PACK_MARK)
}

/// `## ` sections outside code fences: (heading, full text including the heading line).
fn sections(text: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut fence = false;
    for line in text.lines() {
        let t = line.trim_start();
        if t.starts_with("```") || t.starts_with("~~~") {
            fence = !fence;
        }
        if !fence && line.starts_with("## ") {
            out.push((line[3..].trim().to_string(), String::new()));
        }
        if let Some(last) = out.last_mut() {
            last.1.push_str(line);
            last.1.push('\n');
        }
    }
    out
}

/// Every `##` and `###` heading outside code fences.
fn headings(text: &str) -> Vec<String> {
    let mut fence = false;
    let mut v = Vec::new();
    for line in text.lines() {
        let t = line.trim_start();
        if t.starts_with("```") || t.starts_with("~~~") {
            fence = !fence;
        }
        if !fence && (line.starts_with("## ") || line.starts_with("### ")) {
            v.push(line.trim_start_matches('#').trim().to_string());
        }
    }
    v
}

fn heading_words(h: &str) -> Vec<String> {
    h.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty() && !w.chars().all(|c| c.is_ascii_digit()))
        .filter(|w| w.len() >= 3 && !STOP.contains(w))
        .map(str::to_string)
        .collect()
}

/// Words match when equal or when they share a six-letter stem
/// ("classify" / "classification", "implementing" / "implementation").
fn same_word(a: &str, b: &str) -> bool {
    let stem = |w: &str| w.chars().take(6).collect::<String>();
    a == b || (a.chars().count() >= 6 && b.chars().count() >= 6 && stem(a) == stem(b))
}

/// The leading section number of a heading ("3." in "3. Clean architecture"), if any.
fn section_number(h: &str) -> Option<&str> {
    let n = h.split_whitespace().next()?;
    (n.ends_with('.')
        && n[..n.len() - 1]
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.'))
    .then_some(n)
}

/// A template section counts as covered when the file has a heading with the same
/// words, a subset or superset of them, at least half of them in common, or the same
/// section number and a shared word.
fn covered(template_heading: &str, have: &[String]) -> bool {
    let want = heading_words(template_heading);
    if want.is_empty() {
        return true;
    }
    have.iter().any(|h| {
        let got = heading_words(h);
        if got.is_empty() {
            return false;
        }
        let shared = want
            .iter()
            .filter(|w| got.iter().any(|g| same_word(w, g)))
            .count();
        let same_number = section_number(template_heading).is_some()
            && section_number(template_heading) == section_number(h);
        shared == want.len()
            || shared == got.len()
            || shared * 2 >= want.len().max(got.len())
            || (same_number && shared > 0)
    })
}

const NEVER_MERGE: &[&str] = &["related", "documentation routing"];

/// Template sections the existing file does not cover.
pub fn missing_sections(template: &str, existing: &str) -> Vec<(String, String)> {
    let have = headings(existing);
    sections(template)
        .into_iter()
        .filter(|(h, _)| !NEVER_MERGE.contains(&h.to_lowercase().as_str()))
        .filter(|(h, _)| !covered(h, &have))
        .collect()
}

pub struct Present {
    pub wanted: String,
    pub found: String,
    pub src: PathBuf,
    /// Headings of knowledge-pack sections the existing file lacks.
    pub gaps: Vec<String>,
}

pub struct Plan {
    pub present: Vec<Present>,
    pub missing: Vec<(String, PathBuf)>,
    /// (wanted chapter, existing chapter that already uses its number)
    pub conflicts: Vec<(String, String)>,
}

pub fn plan(factory: &Path, t: &Target) -> Result<Plan, String> {
    let set = template_set(factory, &t.stack)?;
    let mut p = Plan {
        present: vec![],
        missing: vec![],
        conflicts: vec![],
    };
    for (rel, src) in set {
        let dst = map_path(&rel, &t.docs_root);
        match equivalent(&t.root, &dst, &t.docs_root) {
            Some(Found::Same(found)) => {
                let tmpl = fs::read_to_string(&src).unwrap_or_default();
                let gaps = if is_knowledge(&tmpl) {
                    let have = fs::read_to_string(t.root.join(&found)).unwrap_or_default();
                    missing_sections(&tmpl, &have)
                        .into_iter()
                        .map(|(h, _)| h)
                        .collect()
                } else {
                    vec![]
                };
                p.present.push(Present {
                    wanted: dst,
                    found,
                    src,
                    gaps,
                });
            }
            Some(Found::NumberTaken(found)) => p.conflicts.push((dst, found)),
            None => p.missing.push((dst, src)),
        }
    }
    Ok(p)
}

fn fill(text: &str, t: &Target) -> String {
    let product = if t.product.is_empty() {
        &t.module
    } else {
        &t.product
    };
    text.replace("{{PRODUCT}}", product)
        .replace("{{MODULE}}", &t.module)
        .replace("{{AREA}}", &t.module)
        .replace("{{STACK}}", &t.stack)
        .replace("{{DATE}}", &util::today_utc())
}

/// Point links in imported text at files that exist in this repo: keep a link that
/// resolves, retarget a handbook chapter to the repo's chapter with the same number,
/// and turn any other dead relative link into plain text.
fn fix_links(text: &str, file: &Path) -> String {
    let re = regex::Regex::new(r"\[([^\]\n]+)\]\(([^)\s]+)\)").expect("link regex");
    let dir = file.parent().unwrap_or(Path::new("."));
    let mut fence = false;
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        let t = line.trim_start();
        if t.starts_with("```") || t.starts_with("~~~") {
            fence = !fence;
        }
        if fence {
            out.push_str(line);
            continue;
        }
        let fixed = re.replace_all(line, |c: &regex::Captures| {
            let (label, href) = (&c[1], &c[2]);
            let (path, anchor) = match href.split_once('#') {
                Some((p, a)) => (p, format!("#{a}")),
                None => (href, String::new()),
            };
            if path.is_empty() || href.contains("://") || href.starts_with("mailto:") {
                return c[0].to_string();
            }
            let target = dir.join(path);
            if target.exists() {
                return c[0].to_string();
            }
            let name = Path::new(path)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            if let (Some(num), Some(tdir)) = (numbered(&name), target.parent())
                && let Some((n, _)) = md_files(tdir)
                    .into_iter()
                    .find(|(n, _)| numbered(n) == Some(num))
            {
                let parent = Path::new(path).parent().unwrap_or(Path::new(""));
                let new = parent.join(&n).to_string_lossy().into_owned();
                return format!("[{label}]({new}{anchor})");
            }
            label.to_string()
        });
        out.push_str(&fixed);
    }
    out
}

const ROUTING_START: &str = "<!-- documentation-routing:start -->";

/// Append the missing knowledge-pack sections to an existing file, before its
/// generated routing block when it has one. Existing text is never changed.
fn merge_into(path: &Path, sections: &[(String, String)], t: &Target) -> std::io::Result<()> {
    let existing = fs::read_to_string(path)?;
    let mut block = format!(
        "<!-- pf-knowledge:start pack={} date={} -->\n> The sections below were added from the Product Factory `{}` knowledge pack because this file did not cover them. Review them against this repository: keep what applies, adapt names and commands, delete what does not.\n\n",
        t.stack,
        util::today_utc(),
        t.stack
    );
    for (_, body) in sections {
        block.push_str(body.trim_end());
        block.push_str("\n\n");
    }
    block.push_str("<!-- pf-knowledge:end -->\n");
    let block = fix_links(&fill(&block, t), path);
    let merged = match existing.find(ROUTING_START) {
        Some(i) => format!(
            "{}\n\n{block}\n{}",
            existing[..i].trim_end(),
            &existing[i..]
        ),
        None => format!("{}\n\n{block}", existing.trim_end()),
    };
    fs::write(path, merged)
}

pub struct Applied {
    pub created: Vec<String>,
    /// (file, headings appended)
    pub merged: Vec<(String, Vec<String>)>,
}

/// Import the knowledge pack: create every missing file, then append the missing
/// sections to existing knowledge files (unless `files_only`). Never overwrites.
pub fn init(factory: &Path, t: &Target, files_only: bool) -> Result<Applied, String> {
    let p = plan(factory, t)?;
    let mut a = Applied {
        created: vec![],
        merged: vec![],
    };
    for (dst, src) in &p.missing {
        let out = t.root.join(dst);
        if out.exists() {
            continue; // never overwrite
        }
        if let Some(d) = out.parent() {
            fs::create_dir_all(d).map_err(|e| e.to_string())?;
        }
        let text = fs::read_to_string(src).map_err(|e| e.to_string())?;
        fs::write(&out, fill(&text, t)).map_err(|e| e.to_string())?;
        a.created.push(dst.clone());
    }
    // Links are fixed once every new file exists, so they can point at each other.
    for dst in &a.created {
        let out = t.root.join(dst);
        let text = fs::read_to_string(&out).map_err(|e| e.to_string())?;
        let fixed = fix_links(&text, &out);
        if fixed != text {
            fs::write(&out, fixed).map_err(|e| e.to_string())?;
        }
    }
    if !files_only {
        for pr in p.present.iter().filter(|pr| !pr.gaps.is_empty()) {
            let tmpl = fs::read_to_string(&pr.src).map_err(|e| e.to_string())?;
            let path = t.root.join(&pr.found);
            let have = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let secs = missing_sections(&tmpl, &have);
            if secs.is_empty() {
                continue;
            }
            merge_into(&path, &secs, t).map_err(|e| e.to_string())?;
            a.merged
                .push((pr.found.clone(), secs.into_iter().map(|(h, _)| h).collect()));
        }
    }
    Ok(a)
}

fn roots(start: &Path, modules: bool) -> Vec<PathBuf> {
    let start = util::git_root(start);
    let mut v = vec![start.clone()];
    if modules && let Some(d) = descriptor::find(&start).filter(|d| d.multi_repo()) {
        v.extend(
            d.modules
                .iter()
                .filter_map(|m| m.get("path"))
                .map(|p| d.root.join(p)),
        );
    }
    v
}

pub fn main_plan(
    factory: &Path,
    root: Option<PathBuf>,
    stack: Option<String>,
    docs: Option<String>,
    modules: bool,
    apply: bool,
    files_only: bool,
) -> i32 {
    let start = root.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let mut code = 0;
    for r in roots(&start, modules) {
        let t = target(
            &r,
            stack.clone().filter(|_| !modules),
            docs.clone().filter(|_| !modules),
        );
        let head = format!(
            "{} (stack {}, docs {}/)",
            t.root.display(),
            t.stack,
            t.docs_root
        );
        if apply {
            match init(factory, &t, files_only) {
                Err(e) => {
                    eprintln!("pf: {}: {e}", r.display());
                    code = 1;
                }
                Ok(a) => {
                    println!(
                        "Knowledge import {head}: created {} · merged into {}",
                        a.created.len(),
                        a.merged.len()
                    );
                    for c in &a.created {
                        println!("  + {c}");
                    }
                    for (f, hs) in &a.merged {
                        println!("  ~ {f}  (+ {})", hs.join(", "));
                    }
                    println!();
                }
            }
            continue;
        }
        match plan(factory, &t) {
            Err(e) => {
                eprintln!("pf: {}: {e}", r.display());
                code = 1;
            }
            Ok(p) => {
                let gaps = p.present.iter().filter(|x| !x.gaps.is_empty()).count();
                println!("Knowledge plan {head}");
                println!(
                    "  present {} · missing {} · files missing sections {} · numbering conflicts {}",
                    p.present.len(),
                    p.missing.len(),
                    gaps,
                    p.conflicts.len()
                );
                for (want, taken) in &p.conflicts {
                    println!(
                        "  ! {want}  (number already used by {taken}; renumber one of them, or record the mapping in the handbook README)"
                    );
                }
                for x in &p.present {
                    let role = if x.wanted != x.found {
                        format!("  (plays {})", x.wanted)
                    } else {
                        String::new()
                    };
                    if !x.gaps.is_empty() {
                        println!(
                            "  ~ {}{role}  missing sections: {}",
                            x.found,
                            x.gaps.join("; ")
                        );
                    } else if !role.is_empty() {
                        println!("  = {}{role}", x.found);
                    }
                }
                for (m, _) in &p.missing {
                    println!("  + {m}");
                }
                println!();
            }
        }
    }
    if !apply {
        println!(
            "Import with: pf knowledge init. It creates the missing files (+) with the pack's content and appends the missing sections (~) to existing files, marked for review. It never overwrites or edits existing text. Then run pf docs-map --fix where the docs use the factory's routing format."
        );
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    fn factory() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
    }

    #[test]
    fn every_stack_includes_the_base_and_its_overlay() {
        let f = factory();
        assert!(stacks(&f).contains(&"spring-kotlin-service".to_string()));
        let s = template_set(&f, "spring-kotlin-service").unwrap();
        assert!(s.contains_key("CURRENT_STATE.md"));
        assert!(s.contains_key("docs/history/DEVELOPMENT_LOG.md"));
        assert!(s.contains_key("docs/development/03-clean-architecture-layers.md"));
        let p = template_set(&f, "product-docs").unwrap();
        assert!(!p.contains_key("docs/development/15-code-samples-index.md"));
        assert!(p.contains_key("docs/prd/_templates/PRD_TEMPLATE.md"));
        assert!(template_set(&f, "nope").is_err());
    }

    fn scratch(tag: &str) -> PathBuf {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let d = std::env::temp_dir().join(format!("pf-know-{tag}-{n}"));
        fs::create_dir_all(&d).unwrap();
        assert!(util::run("git", &["init", "-q"], Some(&d)));
        d
    }

    #[test]
    fn plan_counts_equivalents_and_init_never_overwrites() {
        let d = scratch("eq");
        fs::create_dir_all(d.join("doc/development")).unwrap();
        fs::write(d.join("CURRENT_STATE.md"), "# mine\n").unwrap();
        fs::write(
            d.join("doc/development/09-platform-shells.md"),
            "# 09 mine\n",
        )
        .unwrap();
        fs::write(d.join("doc/IMPLEMENTATION_STATUS.md"), "# status\n").unwrap();
        // A flat chapter in the docs root with the same number and topic.
        fs::write(
            d.join("doc/05-security.md"),
            "# Security\n\n## Threat model and controls\n",
        )
        .unwrap();
        let t = target(&d, Some("typescript-web".into()), Some("doc".into()));
        let p = plan(&factory(), &t).unwrap();
        assert!(p.conflicts.iter().any(|(w, f)| w
            == "doc/development/09-tools-scripts-and-commands.md"
            && f == "doc/development/09-platform-shells.md"));
        assert!(
            p.present
                .iter()
                .any(|x| x.wanted == "doc/features/IMPLEMENTATION_STATUS.md"
                    && x.found == "doc/IMPLEMENTATION_STATUS.md"
                    && x.gaps.is_empty())
        );
        assert!(
            p.present
                .iter()
                .any(|x| x.wanted == "doc/development/05-security.md"
                    && x.found == "doc/05-security.md"
                    && !x.gaps.is_empty())
        );
        let a = init(&factory(), &t, false).unwrap();
        assert!(
            a.created
                .contains(&"doc/history/DEVELOPMENT_LOG.md".to_string())
        );
        assert!(
            a.created
                .contains(&"doc/development/03-ui-design-system.md".to_string())
        );
        assert_eq!(
            fs::read_to_string(d.join("CURRENT_STATE.md")).unwrap(),
            "# mine\n"
        );
        let ui = fs::read_to_string(d.join("doc/development/03-ui-design-system.md")).unwrap();
        assert!(!ui.contains("{{DATE}}") && ui.contains("## Accessibility"));
        // Existing knowledge files get the missing sections, below the original text.
        let sec = fs::read_to_string(d.join("doc/05-security.md")).unwrap();
        assert!(sec.starts_with("# Security\n\n## Threat model and controls\n"));
        assert!(sec.contains("<!-- pf-knowledge:start pack=typescript-web"));
        assert!(sec.contains("## Keys and environment variables"));
        assert_eq!(sec.matches("## Threat model and controls").count(), 1);
        // A second run changes nothing.
        let again = init(&factory(), &t, false).unwrap();
        assert!(again.created.is_empty() && again.merged.is_empty());
        fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn section_matching_tolerates_numbering_and_wording() {
        let tmpl = "# T\n\n## 1. Why this architecture exists\nx\n\n## Types and time\ny\n\n```\n## not a heading\n```\n\n## Related\nz\n";
        let have = "# Mine\n\n## Why This Architecture Exists\n\n### Types, time\n";
        assert!(covered(
            "2. Classify before implementing",
            &["2. Classification before implementation".into()]
        ));
        assert!(covered(
            "3. Responsibilities by layer",
            &["3. Clean Architecture responsibilities".into()]
        ));
        assert!(!covered("Event envelope", &["Producer pattern".into()]));
        assert!(!covered("Schéma général", &["معماری سامانه".into()]));
        assert!(missing_sections(tmpl, have).is_empty());
        let gaps = missing_sections(tmpl, "# Mine\n");
        let names: Vec<_> = gaps.iter().map(|(h, _)| h.as_str()).collect();
        assert_eq!(names, ["1. Why this architecture exists", "Types and time"]);
        assert!(gaps[1].1.starts_with("## Types and time\ny"));
    }

    #[test]
    fn merge_keeps_routing_block_last_and_fixes_links() {
        let d = scratch("merge");
        let dev = d.join("docs/development");
        fs::create_dir_all(&dev).unwrap();
        fs::write(
            dev.join("03-spring-clean-architecture-layers.md"),
            "# Layers\n",
        )
        .unwrap();
        fs::write(
            dev.join("02-kotlin-language-backend.md"),
            "# Kotlin\n\n## Style\n\n<!-- documentation-routing:start -->\n## Documentation routing\n<!-- documentation-routing:end -->\n",
        )
        .unwrap();
        let t = target(&d, Some("spring-kotlin-service".into()), None);
        let p = plan(&factory(), &t).unwrap();
        // Same number and topic under a different file name.
        assert!(
            p.present
                .iter()
                .any(|x| x.wanted == "docs/development/02-kotlin-language.md"
                    && x.found == "docs/development/02-kotlin-language-backend.md")
        );
        let a = init(&factory(), &t, false).unwrap();
        assert!(
            !a.created
                .contains(&"docs/development/03-clean-architecture-layers.md".to_string())
        );
        let k = fs::read_to_string(dev.join("02-kotlin-language-backend.md")).unwrap();
        let (body, routing) = k
            .split_once("<!-- documentation-routing:start -->")
            .unwrap();
        assert!(body.contains("## Types and time") && body.contains("<!-- pf-knowledge:end -->"));
        assert!(routing.contains("## Documentation routing") && !routing.contains("pf-knowledge"));
        assert_eq!(k.matches("## Style").count(), 1);
        // A created chapter links to the repo's own chapter 03, not the pack's name.
        let p0 = fs::read_to_string(dev.join("00-engineering-principles.md")).unwrap();
        assert!(p0.contains("(03-spring-clean-architecture-layers.md)"));
        assert!(!p0.contains("(03-clean-architecture-layers.md)"));
        fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn same_topic_under_another_number_is_present() {
        let d = scratch("topic");
        let dev = d.join("docs/development");
        fs::create_dir_all(&dev).unwrap();
        fs::write(dev.join("17-client-cryptography.md"), "# Crypto\n").unwrap();
        let t = target(&d, Some("kmp-android-client".into()), None);
        let p = plan(&factory(), &t).unwrap();
        assert!(
            p.present
                .iter()
                .any(|x| x.wanted == "docs/development/12-client-cryptography.md"
                    && x.found == "docs/development/17-client-cryptography.md")
        );
        fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn every_knowledge_link_resolves_after_a_fresh_import() {
        for stack in stacks(&factory()) {
            let d = scratch(&stack);
            let t = target(&d, Some(stack.clone()), None);
            init(&factory(), &t, false).unwrap();
            let re = regex::Regex::new(r"\]\(([^)\s#]+)(#[^)]*)?\)").unwrap();
            for f in files_under(&d) {
                if !f.ends_with(".md") || f.starts_with(".git/") {
                    continue;
                }
                let path = d.join(&f);
                let text = fs::read_to_string(&path).unwrap();
                let mut fence = false;
                for line in text.lines() {
                    if line.trim_start().starts_with("```") {
                        fence = !fence;
                    }
                    if fence {
                        continue;
                    }
                    for c in re.captures_iter(line) {
                        let href = &c[1];
                        if href.contains("://") {
                            continue;
                        }
                        assert!(
                            path.parent().unwrap().join(href).exists(),
                            "{stack}: {f} links to missing {href}"
                        );
                    }
                }
            }
            fs::remove_dir_all(&d).ok();
        }
    }
}
