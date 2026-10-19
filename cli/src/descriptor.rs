//! Read `.factory/factory.yaml`, the project descriptor.
//!
//! The descriptor is the factory's own small YAML dialect. Top-level sections hold
//! `key: value` scalars, and `modules:` is a list of `- key: value` items whose `vcs`
//! may be an inline `{ k: v, … }` map. This reader handles exactly that shape: it
//! is not a general YAML parser, and the template in `kit/.factory/factory.yaml`
//! stays within it.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub type Map = BTreeMap<String, String>;

#[derive(Debug, Default)]
pub struct Descriptor {
    /// `section.key` → value (top-level scalars use just `key`).
    pub values: Map,
    pub modules: Vec<Map>,
    /// Folder that holds `.factory/` (the factory root, `<repo>`).
    pub root: PathBuf,
}

fn strip_comment(line: &str) -> &str {
    let mut quote: Option<char> = None;
    let mut prev_ws = true;
    for (i, c) in line.char_indices() {
        match quote {
            Some(q) if c == q => quote = None,
            Some(_) => {}
            None if c == '"' || c == '\'' => quote = Some(c),
            None if c == '#' && prev_ws => return line[..i].trim_end(),
            None => {}
        }
        prev_ws = c.is_whitespace();
    }
    line.trim_end()
}

fn unquote(v: &str) -> String {
    let v = v.trim();
    if v.len() >= 2
        && ((v.starts_with('"') && v.ends_with('"')) || (v.starts_with('\'') && v.ends_with('\'')))
    {
        v[1..v.len() - 1].to_string()
    } else {
        v.to_string()
    }
}

fn split_kv(s: &str) -> Option<(String, String)> {
    let (k, v) = s.split_once(':')?;
    let k = k.trim();
    if k.is_empty() || k.contains(' ') {
        return None;
    }
    Some((k.to_string(), unquote(v)))
}

/// Parse `{ a: b, c: d }` into prefixed entries.
fn inline_map(prefix: &str, v: &str, into: &mut Map) -> bool {
    let v = v.trim();
    if !(v.starts_with('{') && v.ends_with('}')) {
        return false;
    }
    for part in v[1..v.len() - 1].split(',') {
        if let Some((k, val)) = split_kv(part) {
            into.insert(format!("{prefix}.{k}"), val);
        }
    }
    true
}

pub fn parse(text: &str) -> Descriptor {
    let mut d = Descriptor::default();
    let mut section = String::new();
    let mut in_modules = false;
    for raw in text.lines() {
        let line = strip_comment(raw);
        if line.trim().is_empty() {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        let body = line.trim_start();
        if indent == 0 {
            in_modules = false;
            if let Some((k, v)) = split_kv(body) {
                if v.is_empty() {
                    in_modules = k == "modules";
                    section = k;
                } else {
                    section.clear();
                    d.values.insert(k, v);
                }
            }
            continue;
        }
        if in_modules {
            let entry = if let Some(rest) = body.strip_prefix("- ") {
                d.modules.push(Map::new());
                rest
            } else {
                body
            };
            if let (Some(m), Some((k, v))) = (d.modules.last_mut(), split_kv(entry))
                && !inline_map(&k, &v, m)
            {
                m.insert(k, v);
            }
        } else if !section.is_empty()
            && let Some((k, v)) = split_kv(body)
        {
            let key = format!("{section}.{k}");
            if !inline_map(&key, &v, &mut d.values) {
                d.values.insert(key, v);
            }
        }
    }
    d
}

impl Descriptor {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values
            .get(key)
            .map(String::as_str)
            .filter(|v| !v.is_empty())
    }

    pub fn multi_repo(&self) -> bool {
        self.get("layout") == Some("multi-repo")
    }

    pub fn attribution_none(&self) -> bool {
        self.get("vcs.attribution") == Some("none")
    }

    /// Module value with fallback to the top-level `vcs` block.
    pub fn module_vcs<'a>(&'a self, m: &'a Map, key: &str) -> Option<&'a str> {
        m.get(&format!("vcs.{key}"))
            .map(String::as_str)
            .or_else(|| self.get(&format!("vcs.{key}")))
    }
}

/// Load the descriptor in `start` or the nearest parent folder.
pub fn find(start: &Path) -> Option<Descriptor> {
    let start = crate::util::absolute(start);
    for dir in start.ancestors() {
        let f = dir.join(".factory/factory.yaml");
        if let Ok(text) = fs::read_to_string(&f) {
            let mut d = parse(&text);
            d.root = dir.to_path_buf();
            return Some(d);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
factory:
  version: 1
layout: multi-repo             # comment
workspace_root: .
modules:
  - area: frontend
    path: Daria-Connect-Front-End
    gates: see AGENTS.md
    tests: yes
    vcs: { host: gitlab, base_branch: develop }
  - area: backend   # trailing
    path: "Daria-Connect-Back-End"
    tests: no
vcs:
  host: gitlab
  base_branch: develop
  attribution: none
docs:
  root: docs
"#;

    #[test]
    fn parses_the_descriptor_dialect() {
        let d = parse(SAMPLE);
        assert!(d.multi_repo());
        assert!(d.attribution_none());
        assert_eq!(d.get("factory.version"), Some("1"));
        assert_eq!(d.modules.len(), 2);
        assert_eq!(d.modules[0]["path"], "Daria-Connect-Front-End");
        assert_eq!(d.modules[0]["vcs.base_branch"], "develop");
        assert_eq!(d.modules[1]["area"], "backend");
        assert_eq!(d.modules[1]["path"], "Daria-Connect-Back-End");
        assert_eq!(d.modules[1]["tests"], "no");
        assert_eq!(d.module_vcs(&d.modules[1], "host"), Some("gitlab"));
        assert_eq!(d.get("docs.root"), Some("docs"));
    }

    #[test]
    fn parses_the_shipped_template() {
        let t = include_str!("../../kit/.factory/factory.yaml");
        let d = parse(t);
        assert_eq!(d.get("layout"), Some("single-repo"));
        assert_eq!(d.modules.len(), 1);
        assert_eq!(d.modules[0]["area"], "app");
        assert_eq!(d.get("vcs.base_branch"), Some("main"));
    }
}
