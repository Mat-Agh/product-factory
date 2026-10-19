//! `pf mirror` and `pf export`: keep the factory alive if any one account or machine disappears.

use std::fs;
use std::path::{Path, PathBuf};

use crate::util;

pub fn mirror_add(factory: &Path, name: &str, url: &str) -> i32 {
    let ok = util::run("git", &["remote", "add", name, url], Some(factory))
        && util::run("git", &["push", name, "--all"], Some(factory))
        && util::run("git", &["push", name, "--tags"], Some(factory));
    if ok {
        println!("Mirror '{name}' added and pushed.");
        0
    } else {
        eprintln!("mirror add failed");
        1
    }
}

pub fn mirror_push(factory: &Path) -> i32 {
    let remotes = util::output("git", &["remote"], Some(factory)).unwrap_or_default();
    let mut code = 0;
    for r in remotes.lines() {
        println!("-> {r}");
        if !(util::run("git", &["push", r, "--all"], Some(factory))
            && util::run("git", &["push", r, "--tags"], Some(factory)))
        {
            println!("  push to {r} failed");
            code = 1;
        }
    }
    code
}

/// Offline copy. Restore with: git clone product-factory-<date>.bundle product-factory
pub fn mirror_bundle(factory: &Path, dir: Option<PathBuf>) -> i32 {
    let dir = dir.unwrap_or_else(|| {
        PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join("Backups/product-factory")
    });
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("cannot create {}: {e}", dir.display());
        return 1;
    }
    let f = dir.join(format!(
        "product-factory-{}.bundle",
        util::today_utc().replace('-', "")
    ));
    let fs_ = f.to_string_lossy();
    if util::run("git", &["bundle", "create", &fs_, "--all"], Some(factory))
        && util::output("git", &["bundle", "verify", &fs_], Some(factory)).is_some()
    {
        println!("Wrote {}", f.display());
        0
    } else {
        eprintln!("bundle failed");
        1
    }
}

/// Copy what Claude Code learned on this machine into `<factory>/.claude-export/<date>/`
/// (gitignored) for review: the global CLAUDE.md, every project's memory/*.md, and an inventory
/// of user-level skills and agents not managed by the factory. No credentials, sessions or transcripts.
pub fn export(factory: &Path) -> i32 {
    let home = util::claude_home();
    let out = factory.join(".claude-export").join(util::today_utc());
    if let Err(e) = fs::create_dir_all(out.join("memory")) {
        eprintln!("cannot create {}: {e}", out.display());
        return 1;
    }
    let _ = fs::copy(home.join("CLAUDE.md"), out.join("CLAUDE.global.md"));
    let mut n = 0;
    for e in fs::read_dir(home.join("projects"))
        .into_iter()
        .flatten()
        .flatten()
    {
        let mem = e.path().join("memory");
        if !mem.is_dir() {
            continue;
        }
        let dst = out.join("memory").join(e.file_name());
        let _ = fs::create_dir_all(&dst);
        for m in fs::read_dir(&mem).into_iter().flatten().flatten() {
            if m.path().extension().is_some_and(|x| x == "md") {
                let _ = fs::copy(m.path(), dst.join(m.file_name()));
            }
        }
        n += 1;
    }
    let mut inv = String::from("# User-level skills and agents not managed by the factory\n");
    for sub in ["skills", "agents"] {
        let mut names: Vec<_> = fs::read_dir(home.join(sub))
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| {
                e.path()
                    .symlink_metadata()
                    .is_ok_and(|m| !m.file_type().is_symlink())
            })
            .map(|e| format!("- {sub}/{}\n", e.file_name().to_string_lossy()))
            .collect();
        names.sort();
        inv.extend(names);
    }
    let _ = fs::write(out.join("inventory.md"), inv);
    println!(
        "Exported to {} ({n} project memory dirs).",
        util::rel(factory, &out)
    );
    println!(
        "Review it, move durable facts into docs/operator/USER.md, then delete the raw export."
    );
    0
}
