use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use serde_json::Value;

struct Repo(PathBuf);
impl Drop for Repo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn ok(output: Output) -> String {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn git(repo: &Path, args: &[&str]) -> String {
    ok(Command::new("git")
        .current_dir(repo)
        .args(args)
        .output()
        .unwrap())
}

fn pf(repo: &Path, args: &[&str]) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pf"));
    cmd.current_dir(repo).args(args);
    cmd
}

fn job(repo: &Path, args: &[&str]) -> Value {
    let mut cmd = pf(repo, &["job"]);
    let text = ok(cmd.args(args).output().unwrap());
    serde_json::from_str(&text).unwrap()
}

fn reject(repo: &Path, args: &[&str], message: &str) {
    let result = pf(repo, args).output().unwrap();
    assert!(!result.status.success());
    let error = String::from_utf8_lossy(&result.stderr);
    assert!(error.contains(message), "{error}");
}

fn fixture() -> Repo {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let parent =
        Repo(std::env::temp_dir().join(format!("pf-job-fixture-{}-{unique}", std::process::id())));
    let repo = parent.0.join("app");
    fs::create_dir_all(repo.join("web")).unwrap();
    fs::create_dir_all(repo.join("service")).unwrap();
    git(&repo, &["init", "-q", "-b", "main"]);
    git(&repo, &["config", "user.email", "fixture@example.com"]);
    git(&repo, &["config", "user.name", "Factory fixture"]);
    fs::write(
        repo.join("web/package.json"),
        "{\"devDependencies\":{\"typescript\":\"*\"}}",
    )
    .unwrap();
    fs::write(
        repo.join("service/build.gradle.kts"),
        "plugins { kotlin(\"jvm\"); id(\"org.springframework.boot\") }",
    )
    .unwrap();
    fs::write(repo.join("web/main.ts"), "export const port = 1;\n").unwrap();
    fs::write(repo.join("service/main.kt"), "val port = 2\n").unwrap();
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-qm", "Fixture base"]);
    let factory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    ok(
        pf(&repo, &["--factory", factory, "install", "project", "."])
            .output()
            .unwrap(),
    );
    let path = repo.join(".factory/factory.yaml");
    let draft = fs::read_to_string(&path).unwrap();
    assert!(draft.contains("layout: single-repo"));
    assert!(
        draft.contains("stack: typescript-web") && draft.contains("stack: spring-kotlin-service")
    );
    reject(&repo, &["job", "claim", "draft"], "Review the descriptor");
    let reviewed = draft
        .lines()
        .skip(1)
        .collect::<Vec<_>>()
        .join("\n")
        .replace("Example Product", "Fixture")
        .replace("max_parallel_jobs: 3", "max_parallel_jobs: 2")
        .replace("open_pr: true", "open_pr: false");
    fs::write(path, reviewed).unwrap();
    parent
}

#[test]
fn parallel_modules_run_isolated_with_bounded_review_and_recovery() {
    let fixture = fixture();
    let repo = fixture.0.join("app");
    fs::write(
        repo.join(".factory/backlog.md"),
        "- [ ] Update web\n- [ ] Update service\n",
    )
    .unwrap();
    let start = |slug| {
        pf(&repo, &["job", "claim", slug, "--backlog"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
    };
    let first = start("job-a");
    let second = start("job-b");
    let mut claimed = [first, second]
        .map(|p| serde_json::from_str::<Value>(&ok(p.wait_with_output().unwrap())).unwrap());
    claimed.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
    let a = claimed[0]["id"].as_str().unwrap();
    let b = claimed[1]["id"].as_str().unwrap();
    assert!(a.starts_with("001-") && b.starts_with("002-"));
    assert_eq!(claimed[0]["request"], "Update web");
    assert_eq!(claimed[1]["request"], "Update service");
    let status = job(&repo, &["status"]);
    assert_eq!(status.as_array().unwrap().len(), 2);
    assert_eq!(status[0]["id"], a);
    reject(
        &repo,
        &["job", "claim", "another"],
        "All 2 active job slots are in use",
    );

    let evidence = |id: &str, path: &str, text: &str| {
        fs::write(repo.join(".factory/jobs").join(id).join(path), text).unwrap();
    };
    evidence(a, "triage.md", "# Triage: web update\n");
    evidence(b, "triage.md", "# Triage: service update\n");
    let first = job(
        &repo,
        &[
            "prepare", a, "--module", "web", "--type", "feature", "--tier", "standard",
        ],
    );
    let second = job(
        &repo,
        &[
            "prepare", b, "--module", "service", "--type", "chore", "--tier", "trivial",
        ],
    );
    let web = PathBuf::from(first["worktree"].as_str().unwrap());
    let service = PathBuf::from(second["worktree"].as_str().unwrap());
    assert_ne!(web, service);
    assert_eq!(first["module_worktree"], web.join("web").to_str().unwrap());
    assert_eq!(
        second["module_worktree"],
        service.join("service").to_str().unwrap()
    );
    assert_eq!(git(&repo, &["branch", "--show-current"]).trim(), "main");

    let advance = |id: &str, stage: &str, round: &str| {
        job(&repo, &["advance", id, "--expect", stage, "--round", round])
    };
    evidence(a, "spec.md", "STATUS: READY\n");
    assert_eq!(advance(a, "spec", "1")["stage"], "plan");
    evidence(a, "plan.md", "STATUS: READY\n");
    assert_eq!(advance(a, "plan", "1")["stage"], "build");
    fs::write(web.join("web/main.ts"), "export const port = 3;\n").unwrap();
    git(&web, &["commit", "-qam", "Change web port"]);
    evidence(a, "build.md", "STATUS: READY\n");
    assert_eq!(advance(a, "build", "1")["reviews"]["security"], "pending");
    assert_eq!(
        fs::read_to_string(service.join("web/main.ts")).unwrap(),
        "export const port = 1;\n"
    );
    assert_eq!(
        fs::read_to_string(repo.join("web/main.ts")).unwrap(),
        "export const port = 1;\n"
    );

    fs::write(service.join("web/main.ts"), "export const port = 4;\n").unwrap();
    git(&service, &["commit", "-qam", "Cross module change"]);
    evidence(b, "build.md", "STATUS: READY\n");
    reject(
        &repo,
        &["job", "advance", b, "--expect", "build", "--round", "1"],
        "Change outside the job module",
    );
    assert_eq!(
        job(&repo, &["pause", b, "--reason", "Out of scope"])["stage"],
        "needs-human"
    );
    assert_eq!(job(&repo, &["resume", b])["stage"], "build");

    evidence(
        a,
        "round-1/review-code.md",
        "VERDICT: CHANGES\nFix example.\n",
    );
    reject(
        &repo,
        &["job", "advance", a, "--expect", "review", "--round", "1"],
        "review-security.md",
    );
    assert_eq!(job(&repo, &["show", a])["stage"], "review");
    evidence(a, "round-1/review-security.md", "VERDICT: PASS\n");
    let rework = advance(a, "review", "1");
    assert_eq!(rework["stage"], "build");
    assert_eq!(rework["round"], 2);
    assert!(
        repo.join(".factory/jobs")
            .join(a)
            .join("round-1/build.md")
            .is_file()
    );
    reject(
        &repo,
        &["job", "advance", a, "--expect", "build", "--round", "2"],
        "build.md",
    );

    fs::write(web.join("web/main.ts"), "export const port = 5;\n").unwrap();
    git(&web, &["commit", "-qam", "Apply review feedback"]);
    evidence(a, "build.md", "STATUS: READY\n");
    assert_eq!(advance(a, "build", "2")["stage"], "review");
    evidence(a, "round-2/review-code.md", "VERDICT: PASS\n");
    evidence(a, "round-2/review-security.md", "VERDICT: PASS\n");
    assert_eq!(advance(a, "review", "2")["stage"], "qa");
    evidence(a, "qa.md", "VERDICT: PASS\n");
    assert_eq!(advance(a, "qa", "2")["stage"], "gate");
    evidence(a, "gate.md", "READY\n");
    evidence(a, "pr.md", "Fixture delivery evidence.\n");
    assert_eq!(advance(a, "gate", "2")["gate_verdict"], "READY");
    let local = job(&repo, &["finish", a]);
    assert_eq!(local["stage"], "needs-human");
    assert_eq!(local["reason"], "Delivery branch is ready locally");
    let feedback = job(
        &repo,
        &["rework", a, "--note", "Verify updated acceptance."],
    );
    assert_eq!(feedback["stage"], "build");
    assert_eq!(feedback["round"], 3);
    assert!(
        repo.join(".factory/jobs")
            .join(a)
            .join("round-2/gate.md")
            .is_file()
    );
}

#[test]
fn nested_git_module_uses_its_own_refs_and_issue_claims_are_unique() {
    let fixture = fixture();
    let repo = fixture.0.join("app");
    let child = repo.join("independent");
    fs::create_dir(&child).unwrap();
    git(&child, &["init", "-q", "-b", "develop"]);
    git(&child, &["config", "user.email", "fixture@example.com"]);
    git(&child, &["config", "user.name", "Factory fixture"]);
    fs::write(child.join("child.txt"), "child base\n").unwrap();
    fs::write(child.join("child.rs"), "pub fn independent_symbol() {}\n").unwrap();
    fs::write(child.join("AGENTS.md"), "# Existing module rules\n").unwrap();
    fs::write(child.join("HERMES.md"), "# Existing Hermes context\n").unwrap();
    git(&child, &["add", "."]);
    git(&child, &["commit", "-qm", "Child base"]);
    let head = git(&child, &["rev-parse", "HEAD"]);
    git(
        &repo,
        &[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("160000,{},independent", head.trim()),
        ],
    );
    fs::write(
        repo.join(".factory/factory.yaml"),
        "
product:
  name: Fixture
layout: multi-repo
modules:
  - area: backend
    path: independent
    vcs: { base_branch: develop }
  - area: web
    path: web
vcs:
  base_branch: main
  open_pr: false
",
    )
    .unwrap();
    let factory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    ok(
        pf(&repo, &["--factory", factory, "install", "project", "."])
            .output()
            .unwrap(),
    );
    assert!(child.join(".codex/agents/factory-tech-lead.toml").is_file());
    assert!(
        child
            .join(".agents/skills/product-factory/SKILL.md")
            .is_file()
    );
    let agents = fs::read_to_string(child.join("AGENTS.md")).unwrap();
    assert!(agents.starts_with("# Existing module rules\n"));
    assert!(agents.contains("$product-factory"));
    assert!(agents.contains("pf dashboard"));
    let child_graph = child.join(".factory/graphify/graph.json");
    if child_graph.is_file() {
        assert!(
            fs::read_to_string(child_graph)
                .unwrap()
                .contains("independent_symbol")
        );
        let parent_graph = fs::read_to_string(repo.join(".factory/graphify/graph.json")).unwrap();
        assert!(!parent_graph.contains("independent_symbol"));
    }
    assert!(!repo.join("web/.codex").exists());
    assert!(repo.join(".claude/rules/modules/backend.md").is_file());
    let source = "issue:https://example.invalid/issues/2";
    let result = job(
        &repo,
        &[
            "claim",
            "issue-two",
            "--source",
            source,
            "--request",
            "Update the child",
        ],
    );
    let id = result["id"].as_str().unwrap();
    reject(
        &repo,
        &["job", "claim", "duplicate", "--source", source],
        "already owns",
    );
    fs::write(
        repo.join(".factory/jobs").join(id).join("triage.md"),
        "# Triage: Child request\n",
    )
    .unwrap();
    let state = job(
        &child,
        &["prepare", id, "--type", "bug", "--tier", "standard"],
    );
    assert_eq!(state["stage"], "repro");
    assert_eq!(state["base_branch"], "develop");
    assert_eq!(state["base_commit"], head.trim());
    assert_eq!(state["worktree"], state["module_worktree"]);
    assert!(
        Path::new(state["worktree"].as_str().unwrap())
            .join("child.txt")
            .is_file()
    );
    assert_eq!(git(&child, &["branch", "--show-current"]).trim(), "develop");
    assert!(
        git(
            &repo,
            &["branch", "--list", state["branch"].as_str().unwrap()]
        )
        .trim()
        .is_empty()
    );
}

#[test]
fn graphify_tracks_new_uncommitted_source_and_removes_stale_graphs() {
    if !Command::new("graphify")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        eprintln!("Graphify CLI unavailable, skipping integration test");
        return;
    }
    let fixture = fixture();
    let repo = fixture.0.join("app");
    for file in [
        "web/main.ts",
        "web/package.json",
        "service/main.kt",
        "service/build.gradle.kts",
    ] {
        fs::remove_file(repo.join(file)).unwrap();
    }
    let graph = repo.join(".factory/graphify/graph.json");
    let refresh = || ok(pf(&repo, &["graph", "refresh"]).output().unwrap());
    assert!(refresh().contains("No supported source"));
    assert!(!graph.exists());
    fs::write(repo.join("app.rs"), "pub fn initial_symbol() {}\n").unwrap();
    let query = |symbol| ok(pf(&repo, &["graph", "query", symbol]).output().unwrap());
    assert!(query("initial_symbol").contains("initial_symbol"));
    assert!(git(&repo, &["check-ignore", ".factory/graphify/graph.json"]).contains("graph.json"));
    fs::write(repo.join("app.rs"), "pub fn replacement_symbol() {}\n").unwrap();
    assert!(query("replacement_symbol").contains("replacement_symbol"));
    assert!(
        !fs::read_to_string(&graph)
            .unwrap()
            .contains("initial_symbol")
    );
    fs::remove_file(repo.join("app.rs")).unwrap();
    assert!(refresh().contains("No supported source"));
    assert!(!graph.exists());
}

#[cfg(unix)]
#[test]
fn api_route_checks_authorization_and_dispatches_only_the_assigned_role() {
    use std::os::unix::fs::PermissionsExt;

    let fixture = fixture();
    let repo = fixture.0.join("app");
    let state = job(&repo, &["claim", "route", "--request", "Update docs"]);
    let id = state["id"].as_str().unwrap();
    let folder = repo.join(".factory/jobs").join(id);
    fs::write(folder.join("triage.md"), "# Triage: Route test\n").unwrap();
    let state = job(
        &repo,
        &[
            "prepare", id, "--module", "web", "--type", "docs", "--tier", "trivial",
        ],
    );
    assert_eq!(state["stage"], "build");
    let run = ["providers", "run", id, "engineer"];
    assert_eq!(pf(&repo, &run).output().unwrap().status.code(), Some(3));
    reject(&repo, &["providers", "run", id, "tester"], "not assigned");
    reject(
        &repo,
        &["providers", "setup", "--build", "openai-api:sk-fixture"],
        "without spaces or credentials",
    );

    let bin = fixture.0.join("bin");
    fs::create_dir(&bin).unwrap();
    let shim = bin.join("hermes");
    fs::write(
        &shim,
        r#"#!/bin/sh
set -eu
test "${OPENAI_API_KEY:-}" = fixture_token
test -z "${ANTHROPIC_API_KEY:-}"
test "$(basename "$PWD")" = web
case " $* " in
  *" --provider openai-api --model gpt-fixture --reasoning medium "*) ;;
  *) exit 12 ;;
esac
prompt=$(cat)
artifact=$(printf '%s\n' "$prompt" | sed -n '1s/.* Write \(.*\)\. Treat request.*/\1/p')
test -n "$artifact"
printf 'STATUS: READY\n' > "$artifact"
"#,
    )
    .unwrap();
    fs::set_permissions(&shim, fs::Permissions::from_mode(0o700)).unwrap();
    let path = std::env::join_paths(
        std::iter::once(bin).chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
    )
    .unwrap();
    let local = |args: &[&str]| {
        let mut cmd = pf(&repo, args);
        cmd.env("PATH", &path)
            .env("OPENAI_API_KEY", "fixture_token")
            .env("ANTHROPIC_API_KEY", "different_fixture_token");
        cmd
    };
    let setup = local(&["providers", "setup", "--build", "openai-api:gpt-fixture"])
        .output()
        .unwrap();
    assert!(ok(setup).contains("OPENAI_API_KEY present"));
    assert!(
        !fs::read_to_string(repo.join(".factory/model-routes.json"))
            .unwrap()
            .contains("fixture_token")
    );
    reject(&repo, &run, "Operator approval");
    let allowed = ["providers", "run", id, "engineer", "--allow-api"];
    let missing = local(&allowed)
        .env_remove("OPENAI_API_KEY")
        .output()
        .unwrap();
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("OPENAI_API_KEY"));
    ok(local(&allowed).output().unwrap());
    assert!(folder.join("build.md").is_file());
    assert_eq!(job(&repo, &["show", id])["stage"], "build");
    assert!(!local(&allowed).output().unwrap().status.success());
}
