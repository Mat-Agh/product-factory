//! Shared helpers: paths, git, hashing, dates, output.

use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

/// Run a command and return trimmed stdout when it succeeds.
pub fn output(cmd: &str, args: &[&str], dir: Option<&Path>) -> Option<String> {
    let mut c = Command::new(cmd);
    c.args(args).stdin(Stdio::null()).stderr(Stdio::null());
    if let Some(d) = dir {
        c.current_dir(d);
    }
    let out = c.output().ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Run a command with inherited stdio; true when it exits 0.
pub fn run(cmd: &str, args: &[&str], dir: Option<&Path>) -> bool {
    let mut c = Command::new(cmd);
    c.args(args);
    if let Some(d) = dir {
        c.current_dir(d);
    }
    c.status().map(|s| s.success()).unwrap_or(false)
}

pub fn has_cmd(name: &str) -> bool {
    env::var_os("PATH")
        .map(|p| env::split_paths(&p).any(|d| is_executable(&d.join(name))))
        .unwrap_or(false)
}

pub fn which(name: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|p| {
        env::split_paths(&p)
            .map(|d| d.join(name))
            .find(|f| is_executable(f))
    })
}

fn is_executable(p: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::metadata(p)
            .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        p.is_file()
    }
}

/// `git rev-parse --show-toplevel` of `start`, or `start` itself.
pub fn git_root(start: &Path) -> PathBuf {
    output(
        "git",
        &[
            "-C",
            &start.to_string_lossy(),
            "rev-parse",
            "--show-toplevel",
        ],
        None,
    )
    .map(PathBuf::from)
    .unwrap_or_else(|| absolute(start))
}

pub fn absolute(p: &Path) -> PathBuf {
    if p.is_absolute() {
        normalize(p)
    } else {
        normalize(&env::current_dir().unwrap_or_default().join(p))
    }
}

/// Lexical normalisation, like Python's os.path.normpath.
pub fn normalize(p: &Path) -> PathBuf {
    let mut out: Vec<Component> = Vec::new();
    for c in p.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => match out.last() {
                Some(Component::Normal(_)) => {
                    out.pop();
                }
                Some(Component::RootDir) => {}
                _ => out.push(c),
            },
            other => out.push(other),
        }
    }
    let mut pb = PathBuf::new();
    for c in out {
        pb.push(c.as_os_str());
    }
    if pb.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        pb
    }
}

/// Relative path from directory `from` to `to` (both absolute), with forward slashes.
pub fn rel(from: &Path, to: &Path) -> String {
    let (from, to) = (normalize(from), normalize(to));
    let f: Vec<_> = from.components().collect();
    let t: Vec<_> = to.components().collect();
    let common = f.iter().zip(&t).take_while(|(a, b)| a == b).count();
    let mut parts: Vec<String> = vec!["..".into(); f.len() - common];
    parts.extend(
        t[common..]
            .iter()
            .map(|c| c.as_os_str().to_string_lossy().into_owned()),
    );
    if parts.is_empty() {
        ".".into()
    } else {
        parts.join("/")
    }
}

pub fn sha256_file(p: &Path) -> std::io::Result<String> {
    let bytes = fs::read(p)?;
    Ok(Sha256::digest(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}

/// Locate the Product Factory checkout: --factory, $PRODUCT_FACTORY_HOME, an ancestor of the
/// current directory or of this binary that holds `kit/.claude`, then the build-time source tree.
pub fn factory_root(explicit: Option<&Path>) -> Result<PathBuf, String> {
    let is_factory = |p: &Path| p.join("kit/.claude/skills/factory/SKILL.md").exists();
    if let Some(p) = explicit {
        let p = absolute(p);
        return if is_factory(&p) {
            Ok(p)
        } else {
            Err(format!("{} is not a Product Factory checkout", p.display()))
        };
    }
    if let Some(p) = env::var_os("PRODUCT_FACTORY_HOME").map(PathBuf::from)
        && is_factory(&p)
    {
        return Ok(absolute(&p));
    }
    let mut starts = vec![env::current_dir().unwrap_or_default()];
    if let Ok(exe) = env::current_exe().and_then(fs::canonicalize) {
        starts.push(exe);
    }
    for s in starts {
        for a in s.ancestors() {
            if is_factory(a) {
                return Ok(a.to_path_buf());
            }
        }
    }
    let built = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    if is_factory(&built) {
        return Ok(normalize(&built));
    }
    Err("cannot find the Product Factory checkout; pass --factory <path> or set PRODUCT_FACTORY_HOME".into())
}

/// Claude Code config dir: $CLAUDE_CONFIG_DIR or ~/.claude.
pub fn claude_home() -> PathBuf {
    env::var_os("CLAUDE_CONFIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env::var_os("HOME").unwrap_or_default()).join(".claude"))
}

/// Days since 1970-01-01 → (year, month, day). Howard Hinnant's civil_from_days.
fn civil(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn today_utc() -> String {
    let (y, m, d) = civil(now_secs().div_euclid(86_400));
    format!("{y:04}-{m:02}-{d:02}")
}

pub fn now_utc_iso() -> String {
    let s = now_secs();
    let (y, m, d) = civil(s.div_euclid(86_400));
    let t = s.rem_euclid(86_400);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        t / 3600,
        t % 3600 / 60,
        t % 60
    )
}

pub fn say(msg: impl AsRef<str>) {
    println!("  {}", msg.as_ref());
}

pub fn warn(msg: impl AsRef<str>) {
    eprintln!("  ! {}", msg.as_ref());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_dates() {
        assert_eq!(civil(0), (1970, 1, 1));
        assert_eq!(civil(20_745), (2026, 10, 19));
        assert_eq!(civil(11_016), (2000, 2, 29));
    }

    #[test]
    fn relative_paths() {
        assert_eq!(
            rel(Path::new("/r/docs/guide"), Path::new("/r/docs/README.md")),
            "../README.md"
        );
        assert_eq!(
            rel(Path::new("/r/docs"), Path::new("/r/docs/a/b.md")),
            "a/b.md"
        );
        assert_eq!(
            rel(Path::new("/r/docs"), Path::new("/r/AGENTS.md")),
            "../AGENTS.md"
        );
        assert_eq!(
            normalize(Path::new("/r/docs/../x/./y.md")),
            PathBuf::from("/r/x/y.md")
        );
    }
}
