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

fn topic_words(name: &str) -> Vec<String> {
    name.trim_end_matches(".md")
        .split('-')
        .skip(1)
        .filter(|w| w.len() > 3)
        .map(str::to_lowercase)
        .collect()
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
    // Numbered handbook chapter: the same number on the same topic counts; the same
    // number on another topic is a numbering conflict for the human to resolve.
    if let Some((num, _)) = name.split_once('-')
        && num.len() == 2
        && num.chars().all(|c| c.is_ascii_digit())
    {
        let want = topic_words(&name);
        for e in fs::read_dir(root.join(dir)).into_iter().flatten().flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if n.starts_with(&format!("{num}-")) && n.ends_with(".md") {
                let found = util::rel(root, &e.path());
                let shared = topic_words(&n).iter().any(|w| want.contains(w));
                return Some(if shared {
                    Found::Same(found)
                } else {
                    Found::NumberTaken(found)
                });
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

pub struct Plan {
    pub present: Vec<(String, String)>,
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
            Some(Found::Same(found)) => p.present.push((dst, found)),
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

pub fn init(factory: &Path, t: &Target) -> Result<Vec<String>, String> {
    let p = plan(factory, t)?;
    let mut created = Vec::new();
    for (dst, src) in p.missing {
        let out = t.root.join(&dst);
        if out.exists() {
            continue; // never overwrite
        }
        if let Some(d) = out.parent() {
            fs::create_dir_all(d).map_err(|e| e.to_string())?;
        }
        let text = fs::read_to_string(&src).map_err(|e| e.to_string())?;
        fs::write(&out, fill(&text, t)).map_err(|e| e.to_string())?;
        created.push(dst);
    }
    Ok(created)
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
) -> i32 {
    let start = root.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let mut code = 0;
    for r in roots(&start, modules) {
        let t = target(
            &r,
            stack.clone().filter(|_| !modules),
            docs.clone().filter(|_| !modules),
        );
        let result = if apply {
            init(factory, &t).map(|c| (c, None))
        } else {
            plan(factory, &t).map(|p| (vec![], Some(p)))
        };
        match result {
            Err(e) => {
                eprintln!("pf: {}: {e}", r.display());
                code = 1;
            }
            Ok((created, Some(p))) => {
                let _ = created;
                println!(
                    "Knowledge plan {} (stack {}, docs {}/)",
                    t.root.display(),
                    t.stack,
                    t.docs_root
                );
                println!(
                    "  present {} · missing {} · numbering conflicts {}",
                    p.present.len(),
                    p.missing.len(),
                    p.conflicts.len()
                );
                for (want, taken) in &p.conflicts {
                    println!(
                        "  ! {want}  (number already used by {taken}; renumber one of them, or record the mapping in the handbook README)"
                    );
                }
                for (want, found) in p.present.iter().filter(|(w, f)| w != f) {
                    println!("  = {want}  (covered by {found})");
                }
                for (m, _) in &p.missing {
                    println!("  + {m}");
                }
                println!();
            }
            Ok((created, None)) => {
                println!(
                    "Knowledge init {} (stack {}, docs {}/): created {}",
                    t.root.display(),
                    t.stack,
                    t.docs_root,
                    created.len()
                );
                for c in &created {
                    println!("  + {c}");
                }
                println!();
            }
        }
    }
    if !apply {
        println!(
            "Create the missing files with: pf knowledge init (never overwrites). Then fill them, and run pf docs-map --fix where the docs use the factory's routing format."
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

    #[test]
    fn plan_counts_equivalents_and_init_never_overwrites() {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let d = std::env::temp_dir().join(format!("pf-know-{n}"));
        fs::create_dir_all(d.join("doc/development")).unwrap();
        assert!(util::run("git", &["init", "-q"], Some(&d)));
        fs::write(d.join("CURRENT_STATE.md"), "# mine\n").unwrap();
        fs::write(
            d.join("doc/development/09-platform-shells.md"),
            "# 09 mine\n",
        )
        .unwrap();
        fs::write(d.join("doc/IMPLEMENTATION_STATUS.md"), "# status\n").unwrap();
        let t = target(&d, Some("typescript-web".into()), Some("doc".into()));
        let p = plan(&factory(), &t).unwrap();
        assert!(p.conflicts.iter().any(|(w, f)| w
            == "doc/development/09-tools-scripts-and-commands.md"
            && f == "doc/development/09-platform-shells.md"));
        assert!(
            p.present
                .iter()
                .any(|(w, f)| w == "doc/features/IMPLEMENTATION_STATUS.md"
                    && f == "doc/IMPLEMENTATION_STATUS.md")
        );
        let created = init(&factory(), &t).unwrap();
        assert!(created.contains(&"doc/history/DEVELOPMENT_LOG.md".to_string()));
        assert!(created.contains(&"doc/development/03-ui-design-system.md".to_string()));
        assert_eq!(
            fs::read_to_string(d.join("CURRENT_STATE.md")).unwrap(),
            "# mine\n"
        );
        assert!(
            !fs::read_to_string(d.join("doc/history/DEVELOPMENT_LOG.md"))
                .unwrap()
                .contains("{{DATE}}")
        );
        assert!(init(&factory(), &t).unwrap().is_empty());
        fs::remove_dir_all(&d).ok();
    }
}
