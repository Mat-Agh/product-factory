//! Resolve a development checkout or unpack the embedded project-install kit.

use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::util;

include!(concat!(env!("OUT_DIR"), "/bundle.rs"));

pub const SOURCE_URL: &str = "https://github.com/Mat-Agh/product-factory";
const MARKER: &str = ".pf-bundled";

pub struct Source {
    path: PathBuf,
    bundled: bool,
}

impl Source {
    pub fn resolve(explicit: Option<&Path>) -> Result<Self, String> {
        match util::factory_root(explicit) {
            Ok(path) => Ok(Self {
                path,
                bundled: false,
            }),
            Err(e) if explicit.is_some() => Err(e),
            Err(_) => Self::unpack(),
        }
    }

    fn unpack() -> Result<Self, String> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("pf-kit-{}-{nonce}", std::process::id()));
        let mut dir = DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            dir.mode(0o700);
        }
        dir.create(&path)
            .map_err(|e| format!("could not unpack Factory kit: {e}"))?;
        let source = Self {
            path,
            bundled: true,
        };
        for (relative, bytes) in ASSETS {
            let file = source.path.join(relative);
            fs::create_dir_all(file.parent().unwrap()).map_err(|e| e.to_string())?;
            fs::write(file, bytes).map_err(|e| e.to_string())?;
        }
        fs::write(
            source.path.join(MARKER),
            format!(
                "bundle-{}-{:016x}\n",
                env!("CARGO_PKG_VERSION"),
                fingerprint()
            ),
        )
        .map_err(|e| e.to_string())?;
        Ok(source)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        if self.bundled {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

pub fn revision(root: &Path) -> Option<String> {
    fs::read_to_string(root.join(MARKER))
        .ok()
        .map(|text| text.trim().into())
}

fn fingerprint() -> u64 {
    // Stable payload identifier for installed.lock. This is a revision label,
    // not a cryptographic integrity check.
    let mut hash = 0xcbf29ce484222325u64;
    for (path, bytes) in ASSETS {
        for byte in path
            .bytes()
            .chain([0])
            .chain(bytes.iter().copied())
            .chain([0])
        {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
        }
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_assets_install_without_a_checkout() {
        let source = Source::unpack().unwrap();
        let path = source.path().to_path_buf();
        let product = Source::unpack().unwrap();
        let app = product.path().join("app");
        fs::create_dir(&app).unwrap();
        assert!(util::run("git", &["init", "-q"], Some(&app)));
        assert_eq!(crate::install::project(&path, &app, false, false), 0);
        let roles = fs::read_dir(app.join(".codex/agents")).unwrap().count();
        assert_eq!(roles, 10);
        let installed = fs::read_to_string(app.join(".factory/installed.lock")).unwrap();
        assert!(installed.contains(&format!("source: {SOURCE_URL}")));
        assert!(installed.contains("revision: bundle-"));
        let target = crate::knowledge::target(&app, None, None);
        crate::knowledge::init(&path, &target, true).unwrap();
        assert!(
            app.join("docs/development/00-engineering-principles.md")
                .is_file()
        );
        assert!(!path.join("docs/operator/USER.md").exists());
        drop(source);
        assert!(!path.exists());
        assert!(app.join(".claude/skills/factory/CONSTITUTION.md").is_file());
    }
}
