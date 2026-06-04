use anyhow::Result;
use clap::{Parser, Subcommand};

#[path = "../sparkctl/mod.rs"]
mod sparkctl;

#[derive(Parser)]
#[command(name = "agy-ct")]
#[command(about = "Antigravity-CompText SPARK CLI", long_about = None)]
struct Cli {
    #[arg(
        long,
        global = true,
        help = "Plain text output without animations/progress indicators"
    )]
    plain: bool,

    #[arg(long, global = true, help = "Structured JSON output on stdout")]
    json: bool,

    #[arg(long, global = true, help = "Output format (e.g. json)")]
    output: Option<String>,

    #[arg(
        long,
        short,
        global = true,
        help = "Verbose step-by-step diagnostic statements"
    )]
    verbose: bool,

    #[arg(
        long,
        short,
        global = true,
        help = "Quiet mode: suppress non-error output"
    )]
    quiet: bool,

    #[arg(long, global = true, help = "Disable ANSI color escapes")]
    no_color: bool,

    #[arg(
        long,
        global = true,
        help = "Disable interactive prompts and abort immediately if input required"
    )]
    non_interactive: bool,

    #[arg(long, global = true, help = "Explain a specific error code")]
    explain: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Automatically coordinate the full local step sequence")]
    Run,
    #[command(about = "Run a predefined end-to-end trace workflow")]
    Demo,
    #[command(about = "Diagnose local project readiness")]
    Doctor,
    #[command(about = "Validate current project formatting, tests, and clippy rules")]
    Validate,
    #[command(about = "Verify local repository handoff readiness")]
    Handoff,
    #[command(about = "Package commands")]
    Package {
        #[command(subcommand)]
        subcommand: PackageCommands,
    },
    #[command(about = "Context commands")]
    Context {
        #[command(subcommand)]
        subcommand: ContextCommands,
    },
    #[command(about = "Schema commands")]
    Schema {
        #[command(subcommand)]
        subcommand: SchemaCommands,
    },
    #[command(about = "Report commands")]
    Report {
        #[command(subcommand)]
        subcommand: ReportCommands,
    },
    #[command(about = "Notebook commands")]
    Notebook {
        #[command(subcommand)]
        subcommand: NotebookCommands,
    },
}

#[derive(Subcommand)]
enum PackageCommands {
    #[command(about = "Compress raw extraction files to .spkg")]
    Compress {
        #[arg(long, short)]
        input: String,
        #[arg(long, short)]
        output: String,
    },
    #[command(about = "Read sidecar properties and headers from .spkg")]
    Inspect {
        #[arg(long, short)]
        input: String,
    },
    #[command(about = "Run SHA-256 cryptographic verification of .spkg")]
    Verify {
        #[arg(long, short)]
        input: String,
    },
    #[command(about = "Deterministically reconstruct and replay the sidecar trace")]
    Replay {
        #[arg(long, short)]
        input: String,
    },
    #[command(about = "Verify robustness against tampered payload attributes")]
    Adversarial {
        #[arg(long, short)]
        input: String,
    },
}

#[derive(Subcommand)]
enum ContextCommands {
    #[command(about = "Generate structured operational context from a package")]
    Build {
        #[arg(long, short)]
        input: String,
        #[arg(long, short)]
        schema: String,
        #[arg(long, short)]
        output: String,
    },
    #[command(about = "Render operational context into token-light text")]
    Render {
        #[arg(long, short)]
        input: String,
        #[arg(long, short)]
        output: String,
    },
    #[command(about = "Run structural validation and leak checks on a context")]
    Validate {
        #[arg(long, short)]
        input: String,
        #[arg(long, short)]
        schema: Option<String>,
    },
    #[command(about = "Execute context build, render, and validate tasks in sequence")]
    All,
}

#[derive(Subcommand)]
enum SchemaCommands {
    #[command(about = "Validate raw trace files against target JSON schemas")]
    Check {
        #[arg(long, short)]
        input: String,
        #[arg(long, short)]
        schema: String,
    },
}

#[derive(Subcommand)]
enum ReportCommands {
    #[command(about = "Exporter for generated pipeline JSON reports")]
    Export,
}

#[derive(Subcommand)]
enum NotebookCommands {
    #[command(
        about = "Bundles context state and text renderings into a unified documentation payload"
    )]
    Bundle,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run => {
            run_orchestrator()?;
        }
        Commands::Demo => {
            sparkctl::spark_demo::run_spark_demo()?;
        }
        Commands::Doctor => {
            sparkctl::doctor::run_doctor()?;
        }
        Commands::Validate => {
            sparkctl::rust_validate::run_rust_validate()?;
        }
        Commands::Handoff => {
            sparkctl::handoff_check::run_handoff_check()?;
        }
        Commands::Package { subcommand } => match subcommand {
            PackageCommands::Compress { .. } => {
                println!("Placeholder: package compress");
            }
            PackageCommands::Inspect { .. } => {
                println!("Placeholder: package inspect");
            }
            PackageCommands::Verify { .. } => {
                println!("Placeholder: package verify");
            }
            PackageCommands::Replay { .. } => {
                println!("Placeholder: package replay");
            }
            PackageCommands::Adversarial { .. } => {
                println!("Placeholder: package adversarial");
            }
        },
        Commands::Context { subcommand } => match subcommand {
            ContextCommands::Build { .. } => {
                println!("Placeholder: context build");
            }
            ContextCommands::Render { .. } => {
                println!("Placeholder: context render");
            }
            ContextCommands::Validate { .. } => {
                println!("Placeholder: context validate");
            }
            ContextCommands::All => {
                sparkctl::context_all::run_context_all()?;
            }
        },
        Commands::Schema { subcommand } => match subcommand {
            SchemaCommands::Check { .. } => {
                println!("Placeholder: schema check");
            }
        },
        Commands::Report { subcommand } => match subcommand {
            ReportCommands::Export => {
                println!("Placeholder: report export");
            }
        },
        Commands::Notebook { subcommand } => match subcommand {
            NotebookCommands::Bundle => {
                println!("Placeholder: notebook bundle");
            }
        },
    }

    Ok(())
}

fn run_orchestrator() -> Result<()> {
    println!("CompText-Sparkctl run");
    println!();
    println!("plan");
    println!("  1 workspace doctor");
    println!("  2 context pipeline");
    println!("  3 spark demo");
    println!("  4 handoff check");
    println!();
    println!("run");

    // Stage 1: workspace doctor
    if let Err(e) = sparkctl::doctor::run_doctor() {
        println!("  [1/4] workspace doctor   FAIL");
        println!();
        println!("result FAIL");
        return Err(e);
    }
    println!("  [1/4] workspace doctor   PASS");

    // Stage 2: context pipeline
    if let Err(e) = sparkctl::context_all::run_context_all() {
        println!("  [2/4] context pipeline   FAIL");
        println!();
        println!("result FAIL");
        return Err(e);
    }
    println!("  [2/4] context pipeline   PASS");

    // Stage 3: spark demo
    if let Err(e) = sparkctl::spark_demo::run_spark_demo() {
        println!("  [3/4] spark demo         FAIL");
        println!();
        println!("result FAIL");
        return Err(e);
    }
    println!("  [3/4] spark demo         PASS");

    // Stage 4: handoff check
    if let Err(e) = sparkctl::handoff_check::run_handoff_check() {
        println!("  [4/4] handoff check      FAIL");
        println!();
        println!("result FAIL");
        return Err(e);
    }
    println!("  [4/4] handoff check      PASS");

    println!();
    println!("result PASS");
    Ok(())
}
