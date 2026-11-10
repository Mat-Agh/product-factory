//! Package the generic install assets alongside the CLI.

use std::fmt::Write;
use std::{env, fs, io, path::Path};

fn collect(root: &Path, relative: &Path, assets: &mut Vec<String>) -> io::Result<()> {
    let path = root.join(relative);
    println!("cargo:rerun-if-changed={}", path.display());
    if path.is_dir() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let name = entry.file_name();
            let kind = entry.file_type()?;
            if kind.is_symlink() {
                continue;
            }
            if (kind.is_dir() && !name.to_string_lossy().starts_with('.'))
                || matches!(name.to_str(), Some("pack.map" | ".exclude"))
                || matches!(
                    entry.path().extension().and_then(|ext| ext.to_str()),
                    Some("md" | "html")
                )
            {
                collect(root, &relative.join(name), assets)?;
            }
        }
    } else {
        assert!(path.is_file(), "missing bundled file: {}", path.display());
        assets.push(relative.to_str().unwrap().replace('\\', "/"));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let cli = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = Path::new(&cli).parent().unwrap();
    let out = env::var_os("OUT_DIR").unwrap();
    let mut assets = Vec::new();
    for item in [
        "kit/.agents/skills/product-factory",
        "kit/.claude/agents",
        "kit/.claude/rules",
        "kit/.claude/skills/factory/SKILL.md",
        "kit/.claude/skills/factory/protocol.md",
        "kit/.factory/backlog.md",
        "kit/.factory/dashboard.html",
        "kit/.factory/factory.yaml",
        "kit/AGENTS.factory.md",
        "kit/knowledge",
        "docs/knowledge",
        "docs/governance/CONSTITUTION.md",
    ] {
        collect(root, Path::new(item), &mut assets)?;
    }
    assets.sort();
    let mut generated = String::from("const ASSETS: &[(&str, &[u8])] = &[\n");
    for relative in assets {
        // Resolve the Constitution's symlink in the bundle without relying on
        // how Git on the build host checks out symlinks.
        let installed = if relative == "docs/governance/CONSTITUTION.md" {
            "kit/.claude/skills/factory/CONSTITUTION.md"
        } else {
            &relative
        };
        writeln!(
            generated,
            "    ({installed:?}, include_bytes!({:?})),",
            root.join(&relative).to_str().unwrap(),
        )
        .unwrap();
    }
    generated.push_str("];\n");
    fs::write(Path::new(&out).join("bundle.rs"), generated)
}
