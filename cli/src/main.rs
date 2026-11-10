//! `pf`: the Product Factory command-line tool.
//!
//! Install once per machine: `cargo install --path cli` (from a factory checkout), or
//! `cargo install --git ssh://git@github.com/Mat-Agh/product-factory.git pf`.

mod audit;
mod backup;
mod bundle;
mod dashboard;
mod descriptor;
mod diagrams;
mod discovery;
mod docs_map;
mod doctor;
mod graph;
mod guard;
mod install;
mod job;
mod job_knowledge;
mod knowledge;
mod memory;
mod providers;
mod rules;
mod runtime;
mod util;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "pf",
    version,
    about = "Product Factory: guard hook, docs map, diagrams, install, doctor, backups"
)]
struct Cli {
    /// Product Factory checkout (default: $PRODUCT_FACTORY_HOME, an ancestor of the current
    /// directory, or the build checkout). Project commands can use the embedded kit.
    #[arg(long, global = true)]
    factory: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Claude Code PreToolUse hook: read the hook JSON on stdin; exit 2 blocks the Bash call
    Guard {
        /// Run the built-in policy cases instead of reading stdin
        #[arg(long)]
        self_test: bool,
    },
    /// Check (or --fix) the docs/ tree: README per folder, generated routing links and map
    DocsMap {
        /// Repository root (default: git top level of the current directory)
        #[arg(long)]
        root: Option<PathBuf>,
        #[arg(long, default_value = "docs")]
        docs: String,
        /// Create missing READMEs and regenerate routing blocks and the map, then check
        #[arg(long)]
        fix: bool,
        /// Also require a **Status:** header in every doc, and no docs outside docs/
        #[arg(long)]
        strict: bool,
        /// Markdown outside docs/ that is not documentation (repeatable), e.g. kit/
        #[arg(long)]
        allow: Vec<String>,
    },
    /// Inline Mermaid diagrams in Markdown: lint them, or parse them with the Mermaid CLI
    Diagrams {
        #[command(subcommand)]
        action: DiagramCmd,
        /// Repository root (default: git top level of the current directory)
        #[arg(long, global = true)]
        root: Option<PathBuf>,
    },
    /// Install the factory into a product repo (copy) or your Claude Code config (links)
    Install {
        #[command(subcommand)]
        target: InstallCmd,
    },
    /// Read-only health check; exit code = number of failures
    Doctor {
        /// Also check this product repo's installation
        project: Option<PathBuf>,
    },
    /// Backup remotes and offline bundles of the factory repo
    Mirror {
        #[command(subcommand)]
        action: MirrorCmd,
    },
    /// Copy Claude Code memory into <factory>/.claude-export/<date>/ for review
    Export,
    /// Inventory the existing agentic setup (instructions, hooks, CI, conventions, secrets) before adopting a project
    Audit {
        /// Repository to audit (default: current directory)
        path: Option<PathBuf>,
        /// Machine-readable output
        #[arg(long)]
        json: bool,
        /// Also audit configured modules or discovered submodules and stack roots
        #[arg(long)]
        modules: bool,
    },
    /// The per-stack knowledge structure (state, history, handbook, architecture, decisions, features)
    Knowledge {
        #[command(subcommand)]
        action: KnowledgeCmd,
    },
    /// Show this repo's Claude Code auto memory (for /factory memory)
    Memory {
        /// Repository to inspect (default: current directory)
        #[arg(long)]
        root: Option<PathBuf>,
        /// Machine-readable output
        #[arg(long)]
        json: bool,
    },
    /// Configure or run project model routes without storing credentials
    Providers {
        #[arg(long, global = true)]
        root: Option<PathBuf>,
        #[command(subcommand)]
        action: providers::Action,
    },
    /// Build or query a worktree-local Graphify code graph
    Graph {
        #[arg(long, global = true)]
        root: Option<PathBuf>,
        #[command(subcommand)]
        action: graph::Action,
    },
    /// Path-scoped Claude Code rules generated from the descriptor
    Rules {
        #[command(subcommand)]
        action: RulesCmd,
        /// Folder at or below the descriptor (default: current directory)
        #[arg(long, global = true)]
        root: Option<PathBuf>,
    },
    /// Claim, prepare, and advance a job using its on-disk evidence
    Job {
        #[arg(long, global = true)]
        root: Option<PathBuf>,
        #[command(subcommand)]
        action: job::Action,
    },
    /// Serve a product's .factory/ folder so dashboard.html can show live jobs (loopback only)
    Dashboard {
        /// Folder to serve (default: .factory)
        #[arg(long)]
        dir: Option<PathBuf>,
        #[arg(long, default_value_t = 8000)]
        port: u16,
    },
}

#[derive(Subcommand)]
enum DiagramCmd {
    /// Lint every ```mermaid block for constructs that break renderers (no tools needed)
    Check,
    /// Lint, then parse every block with the Mermaid CLI (needs Node.js)
    Validate,
}

#[derive(Subcommand)]
enum InstallCmd {
    /// Copy the kit into a product repo (or a multi-repo workspace root) for you to commit there
    Project {
        path: PathBuf,
        /// Provision the pinned Graphify CLI with uv if it is missing
        #[arg(long)]
        install_graphify: bool,
        /// Hide AI attribution in commits and PRs (same as vcs.attribution: none)
        #[arg(long)]
        no_ai_attribution: bool,
        /// Also install independent workspace modules (the default)
        #[arg(long, conflicts_with = "root_only")]
        modules: bool,
        /// Install only the workspace root
        #[arg(long)]
        root_only: bool,
        /// Ask for project model routes after installing the project
        #[arg(long)]
        configure_models: bool,
    },
    /// Install into one module repo of a multi-repo workspace, so sessions opened there use the factory
    Module {
        path: PathBuf,
        /// Provision the pinned Graphify CLI with uv if it is missing
        #[arg(long)]
        install_graphify: bool,
        /// The workspace folder that holds .factory/factory.yaml
        #[arg(long)]
        workspace: PathBuf,
        #[arg(long)]
        no_ai_attribution: bool,
    },
    /// Link the kit into ~/.claude (or $CLAUDE_CONFIG_DIR) so it follows git pull
    User {
        /// Also wire the guard hook into the user settings (every project on this machine)
        #[arg(long)]
        with_guard: bool,
    },
    /// Both
    All {
        path: PathBuf,
        #[arg(long)]
        with_guard: bool,
    },
}

#[derive(Subcommand)]
enum KnowledgeCmd {
    /// List the available stack templates
    Stacks,
    /// Compare a repo with its stack's knowledge pack: missing files, files missing sections, numbering conflicts
    Plan {
        #[arg(long)]
        root: Option<PathBuf>,
        /// Override the stack (default: descriptor module `stack`, else manifest detection)
        #[arg(long)]
        stack: Option<String>,
        /// Override the docs root (default: module `docs_root`, else existing layout)
        #[arg(long)]
        docs: Option<String>,
        /// Also configured modules or discovered submodules and stack roots
        #[arg(long)]
        modules: bool,
    },
    /// Import the knowledge pack: create missing files and append missing sections to existing ones (never overwrites)
    Init {
        #[arg(long)]
        root: Option<PathBuf>,
        #[arg(long)]
        stack: Option<String>,
        #[arg(long)]
        docs: Option<String>,
        #[arg(long)]
        modules: bool,
        /// Only create missing files; do not append sections to existing files
        #[arg(long)]
        files_only: bool,
    },
}

#[derive(Subcommand)]
enum RulesCmd {
    /// Write .claude/rules/modules/<area>.md for every module in a multi-repo descriptor
    Sync,
}

#[derive(Subcommand)]
enum MirrorCmd {
    /// Add a mirror remote and push all branches and tags to it
    Add { name: String, url: String },
    /// Push all branches and tags to every remote
    Push,
    /// Write and verify an offline git bundle (default ~/Backups/product-factory)
    Bundle { dir: Option<PathBuf> },
}

fn factory(cli_factory: &Option<PathBuf>) -> Result<PathBuf, i32> {
    util::factory_root(cli_factory.as_deref()).map_err(|e| {
        eprintln!("pf: {e}");
        2
    })
}

fn kit_source(cli_factory: &Option<PathBuf>) -> Result<bundle::Source, i32> {
    bundle::Source::resolve(cli_factory.as_deref()).map_err(|e| {
        eprintln!("pf: {e}");
        2
    })
}

fn provision_graphify() -> Result<(), i32> {
    graph::provision().map_err(|e| {
        util::warn(e);
        1
    })
}

fn run(cli: Cli) -> Result<i32, i32> {
    Ok(match cli.cmd {
        Cmd::Guard { self_test: false } => guard::main(),
        Cmd::Guard { self_test: true } => {
            let (passed, total, fails) = guard::self_test();
            for f in &fails {
                println!("FAIL {f}");
            }
            println!("{passed}/{total} passed");
            i32::from(!fails.is_empty())
        }
        Cmd::DocsMap {
            root,
            docs,
            fix,
            strict,
            allow,
        } => docs_map::main(root, docs, fix, strict, allow),
        Cmd::Diagrams { action, root } => {
            diagrams::main(root, matches!(action, DiagramCmd::Validate))
        }
        Cmd::Install { target } => {
            let source;
            let checkout;
            let f = if matches!(target, InstallCmd::User { .. } | InstallCmd::All { .. }) {
                checkout = factory(&cli.factory)?;
                checkout.as_path()
            } else {
                source = kit_source(&cli.factory)?;
                source.path()
            };
            match target {
                InstallCmd::Project {
                    path,
                    no_ai_attribution,
                    modules: _,
                    root_only,
                    install_graphify,
                    configure_models,
                } => {
                    if install_graphify {
                        provision_graphify()?;
                    }
                    let result = install::project(f, &path, no_ai_attribution, !root_only);
                    if result == 0
                        && (configure_models || providers::first_interactive_setup(&path))
                    {
                        providers::main(
                            Some(path),
                            providers::Action::Setup {
                                build: None,
                                deep: None,
                                economy: None,
                            },
                        )
                    } else {
                        if result == 0 {
                            println!(
                                "Optional API routing: pf providers setup (native sign-in stays in your AI host)."
                            );
                        }
                        result
                    }
                }
                InstallCmd::Module {
                    path,
                    workspace,
                    no_ai_attribution,
                    install_graphify,
                } => {
                    if install_graphify {
                        provision_graphify()?;
                    }
                    install::module(f, &path, &workspace, no_ai_attribution)
                }
                InstallCmd::User { with_guard } => install::user(f, with_guard),
                InstallCmd::All { path, with_guard } => {
                    install::user(f, with_guard).max(install::project(f, &path, false, true))
                }
            }
        }
        Cmd::Doctor { project } => {
            doctor::main(kit_source(&cli.factory)?.path(), project.as_deref())
        }
        Cmd::Mirror { action } => {
            let f = factory(&cli.factory)?;
            match action {
                MirrorCmd::Add { name, url } => backup::mirror_add(&f, &name, &url),
                MirrorCmd::Push => backup::mirror_push(&f),
                MirrorCmd::Bundle { dir } => backup::mirror_bundle(&f, dir),
            }
        }
        Cmd::Providers { root, action } => providers::main(root, action),
        Cmd::Graph { root, action } => graph::main(root, action),
        Cmd::Job { root, action } => job::main(root, action),
        Cmd::Export => backup::export(&factory(&cli.factory)?),
        Cmd::Dashboard { dir, port } => dashboard::main(dir, port),
        Cmd::Audit {
            path,
            json,
            modules,
        } => audit::main(kit_source(&cli.factory)?.path(), path, json, modules),
        Cmd::Knowledge { action } => {
            let source = kit_source(&cli.factory)?;
            let f = source.path();
            match action {
                KnowledgeCmd::Stacks => {
                    for s in knowledge::stacks(f) {
                        println!("{s}");
                    }
                    0
                }
                KnowledgeCmd::Plan {
                    root,
                    stack,
                    docs,
                    modules,
                } => knowledge::main_plan(f, root, stack, docs, modules, false, false),
                KnowledgeCmd::Init {
                    root,
                    stack,
                    docs,
                    modules,
                    files_only,
                } => knowledge::main_plan(f, root, stack, docs, modules, true, files_only),
            }
        }
        Cmd::Memory { root, json } => memory::main(root, json),
        Cmd::Rules {
            action: RulesCmd::Sync,
            root,
        } => rules::main(root),
    })
}

fn main() -> ExitCode {
    // Exit quietly when stdout is closed early (`pf … | head`) instead of panicking.
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    let code = run(Cli::parse()).unwrap_or_else(|c| c);
    ExitCode::from(code.clamp(0, 255) as u8)
}
