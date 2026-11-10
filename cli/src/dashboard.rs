//! `pf dashboard`: serve a product's `.factory/` folder on loopback so `dashboard.html` can
//! read job state. Read-only, GET only, no path escapes, directory listings for `jobs/`.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};

use crate::{descriptor, discovery, job};

fn content_type(p: &Path) -> &'static str {
    match p.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "md" | "txt" | "lock" | "yaml" => "text/plain; charset=utf-8",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%'
            && i + 2 < b.len()
            && let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16)
        {
            out.push(v);
            i += 3;
            continue;
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('"', "&quot;")
}

/// Map a request path onto the served root, refusing anything that climbs out of it.
fn resolve(root: &Path, url_path: &str) -> Option<PathBuf> {
    let decoded = percent_decode(url_path.split(['?', '#']).next().unwrap_or("/"));
    let mut p = root.to_path_buf();
    for c in Path::new(decoded.trim_start_matches('/')).components() {
        match c {
            Component::Normal(n) => p.push(n),
            Component::CurDir => {}
            _ => return None,
        }
    }
    Some(p)
}

fn visible(relative: &Path) -> bool {
    if matches!(
        relative.to_str(),
        Some("dashboard.html" | "backlog.md" | "jobs")
    ) {
        return true;
    }
    let parts: Vec<_> = relative.iter().filter_map(|p| p.to_str()).collect();
    let round = |s: &str| {
        s.strip_prefix("round-")
            .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
    };
    let markdown = |s: &str| !s.starts_with('.') && s.ends_with(".md");
    match parts.as_slice() {
        ["jobs", id] => job::valid_id(id),
        ["jobs", id, name] => {
            job::valid_id(id) && (*name == "job.json" || markdown(name) || round(name))
        }
        ["jobs", id, stage, name] => job::valid_id(id) && round(stage) && markdown(name),
        _ => false,
    }
}

fn respond(mut s: &TcpStream, status: &str, ctype: &str, body: &[u8]) {
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = s.write_all(head.as_bytes()).and_then(|_| s.write_all(body));
}

fn handle(root: &Path, stream: TcpStream) {
    let mut line = String::new();
    if BufReader::new(&stream).read_line(&mut line).is_err() {
        return;
    }
    let mut parts = line.split_whitespace();
    let (method, path) = (parts.next().unwrap_or(""), parts.next().unwrap_or("/"));
    if method != "GET" {
        return respond(&stream, "405 Method Not Allowed", "text/plain", b"GET only");
    }
    let path = if path == "/" { "/dashboard.html" } else { path };
    let Some(p) = resolve(root, path) else {
        return respond(&stream, "403 Forbidden", "text/plain", b"forbidden");
    };
    let relative = p.strip_prefix(root).unwrap();
    if !visible(relative) || !discovery::local_path(root, relative) {
        return respond(&stream, "404 Not Found", "text/plain", b"not found");
    }
    if p.is_dir() {
        if !path.ends_with('/') {
            let loc = format!("{path}/");
            let head = format!(
                "HTTP/1.1 301 Moved Permanently\r\nLocation: {loc}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            let _ = (&stream).write_all(head.as_bytes());
            return;
        }
        let mut names: Vec<String> = fs::read_dir(&p)
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| {
                let path = e.path();
                let relative = path.strip_prefix(root).unwrap();
                visible(relative) && discovery::local_path(root, relative)
            })
            .map(|e| {
                let n = e.file_name().to_string_lossy().into_owned();
                if e.path().is_dir() {
                    format!("{n}/")
                } else {
                    n
                }
            })
            .collect();
        names.sort();
        let items: String = names
            .iter()
            .map(|n| format!("<li><a href=\"{0}\">{0}</a></li>\n", escape(n)))
            .collect();
        let body = format!(
            "<!doctype html><title>{0}</title><ul>\n{items}</ul>",
            escape(path)
        );
        return respond(
            &stream,
            "200 OK",
            "text/html; charset=utf-8",
            body.as_bytes(),
        );
    }
    match fs::read(&p) {
        Ok(b) => respond(&stream, "200 OK", content_type(&p), &b),
        Err(_) => respond(&stream, "404 Not Found", "text/plain", b"not found"),
    }
}

pub fn main(dir: Option<PathBuf>, port: u16) -> i32 {
    let root = dir.unwrap_or_else(|| {
        descriptor::find(&std::env::current_dir().unwrap_or_default())
            .map(|d| d.root.join(".factory"))
            .unwrap_or_else(|| PathBuf::from(".factory"))
    });
    if !root.join("dashboard.html").exists() {
        eprintln!(
            "pf: no dashboard.html in {} (run from a product repo, or pass --dir)",
            root.display()
        );
        return 1;
    }
    let listener = match TcpListener::bind(("127.0.0.1", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("pf: cannot listen on 127.0.0.1:{port}: {e}");
            return 1;
        }
    };
    println!(
        "Dashboard: http://127.0.0.1:{port}/dashboard.html  (workspace {}; Ctrl+C to stop)",
        root.display()
    );
    for s in listener.incoming().flatten() {
        handle(&root, s);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refuses_path_escapes() {
        let r = Path::new("/srv/f");
        assert_eq!(
            resolve(r, "/jobs/001-x/job.json"),
            Some(PathBuf::from("/srv/f/jobs/001-x/job.json"))
        );
        assert_eq!(
            resolve(r, "/a%20b.md"),
            Some(PathBuf::from("/srv/f/a b.md"))
        );
        assert_eq!(resolve(r, "/../etc/passwd"), None);
        assert_eq!(resolve(r, "/jobs/%2e%2e/%2e%2e/x"), None);
    }
}
