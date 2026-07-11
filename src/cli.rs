use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "deltaforge")]
#[command(about = "Local staged project learning framework", version)]
pub struct Cli {
    /// Learner project directory. Defaults to upward discovery from the current directory.
    #[arg(long, global = true)]
    pub project_dir: Option<PathBuf>,

    /// Project pack search directory for this invocation.
    #[arg(long, global = true)]
    pub packs_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List available project packs.
    List,
    /// Manage local project packs.
    #[command(subcommand)]
    Pack(PackCommand),
    /// Create a new user project from a project pack.
    Init(InitArgs),
    /// Validate one project pack or all discovered packs.
    ValidatePack(ValidatePackArgs),
    /// Show instructions for the current or selected stage.
    Instructions(InstructionsArgs),
    /// Explain the project goal, usefulness, and full stage roadmap.
    Overview(OverviewArgs),
    /// Run black-box tests for the current or selected stage.
    Test(TestArgs),
    /// Move to the next stage after the current stage passes.
    Next,
    /// Re-pin the project to the currently discovered pack after an upgrade.
    SyncPack(SyncPackArgs),
    /// Show project progress.
    Status(StatusArgs),
    /// Show progressive hints for the current stage.
    Hint(HintArgs),
    /// Inspect and validate project configuration.
    #[command(subcommand)]
    Config(ConfigCommand),
    /// Run project benchmarks.
    Bench(BenchArgs),
    /// Generate a project progress report.
    Report(ReportArgs),
    /// Generate a portfolio summary.
    Portfolio(PortfolioArgs),
    /// Show or edit design prompts and notes.
    Design(DesignArgs),
    /// Commit current project progress with a stage-aware message.
    Commit(CommitArgs),
    /// Check local tooling, pack discovery, and optional project health.
    Doctor(DoctorArgs),
    /// Explain the latest failing stage results and suggest next steps.
    ExplainFailure(ExplainFailureArgs),
}

#[derive(Debug, Args)]
pub struct ValidatePackArgs {
    /// Project pack id to validate. Validates all discovered packs when omitted.
    pub project: Option<String>,

    /// Include authoring quality checks in addition to structural validation.
    #[arg(long)]
    pub strict: bool,

    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Project pack id, such as "flashindex".
    pub project: String,

    /// Implementation language to initialize.
    #[arg(long)]
    pub lang: String,

    /// Target directory name. Defaults to "<project>-<language>".
    #[arg(long)]
    pub name: Option<String>,

    /// Do not initialize a git repository.
    #[arg(long)]
    pub no_git: bool,

    /// Start at a specific stage id.
    #[arg(long)]
    pub stage: Option<String>,
}

#[derive(Debug, Args)]
pub struct InstructionsArgs {
    /// Show instructions for a specific stage id.
    #[arg(long)]
    pub stage: Option<String>,

    /// Show instructions for all stages.
    #[arg(long)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct OverviewArgs {
    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct TestArgs {
    /// Run tests for a specific stage id.
    #[arg(long)]
    pub stage: Option<String>,

    /// Run tests for all stages.
    #[arg(long)]
    pub all: bool,

    /// Print command output and detailed diagnostics.
    #[arg(long, short)]
    pub verbose: bool,

    /// Run only tests whose name contains this pattern.
    #[arg(long)]
    pub filter: Option<String>,

    /// List selected tests without running them.
    #[arg(long)]
    pub list_tests: bool,

    /// Stop after the first failed test in each stage.
    #[arg(long)]
    pub fail_fast: bool,

    /// Skip the language build command before running tests.
    #[arg(long)]
    pub no_build: bool,

    /// Keep temporary fixture directories after test execution.
    #[arg(long)]
    pub keep_temp: bool,

    /// Print machine-readable JSON results.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct SyncPackArgs {
    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct StatusArgs {
    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct HintArgs {
    /// Show a specific hint level.
    #[arg(long)]
    pub level: Option<usize>,

    /// Show all hints for the current stage.
    #[arg(long)]
    pub all: bool,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Print the effective project configuration.
    Show(ConfigShowArgs),
    /// Validate the project configuration file.
    Validate(ConfigValidateArgs),
}

#[derive(Debug, Args)]
pub struct ConfigShowArgs {
    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct ConfigValidateArgs {
    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct BenchArgs {
    /// Run benchmarks for a specific stage id.
    #[arg(long)]
    pub stage: Option<String>,

    /// Run benchmarks for all stages with benchmark definitions.
    #[arg(long)]
    pub all: bool,

    /// Override benchmark iterations.
    #[arg(long)]
    pub iterations: Option<u64>,

    /// Override warmup iterations.
    #[arg(long)]
    pub warmup: Option<u64>,

    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,

    /// Save results to .deltaforge/benchmark_history.json.
    #[arg(long)]
    pub save: bool,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ReportFormat {
    Markdown,
    Html,
    Json,
}

#[derive(Debug, Args)]
pub struct ReportArgs {
    /// Report output format.
    #[arg(long, value_enum, default_value_t = ReportFormat::Markdown)]
    pub format: ReportFormat,

    /// Output path. Defaults to report.md.
    #[arg(long, default_value = "report.md")]
    pub output: PathBuf,
}

#[derive(Debug, Args)]
pub struct PortfolioArgs {
    /// Output path. Defaults to PORTFOLIO.md.
    #[arg(long, default_value = "PORTFOLIO.md")]
    pub output: PathBuf,
}

#[derive(Debug, Args)]
pub struct DesignArgs {
    /// Stage id. Defaults to the current stage.
    #[arg(long)]
    pub stage: Option<String>,

    /// Open the design notes file in $EDITOR.
    #[arg(long)]
    pub edit: bool,
}

#[derive(Debug, Args)]
pub struct CommitArgs {
    /// Commit even if the current stage has not passed.
    #[arg(long)]
    pub force: bool,
}

#[derive(Debug, Subcommand)]
pub enum PackCommand {
    /// List discovered packs.
    List(PackListArgs),
    /// Show one pack in detail.
    Show(PackShowArgs),
    /// Scaffold a new local pack.
    New(PackNewArgs),
    /// Add a stage scaffold to an existing pack.
    AddStage(PackAddStageArgs),
    /// Diagnose pack authoring quality gaps.
    Doctor(PackDoctorArgs),
    /// Prove a pack by running an internal reference solution.
    CheckReference(PackCheckReferenceArgs),
    /// Copy a discovered pack to a local packs directory.
    Install(PackInstallArgs),
}

#[derive(Debug, Args)]
pub struct PackListArgs {
    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct PackShowArgs {
    /// Project pack id.
    pub project: String,

    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct PackNewArgs {
    /// New pack id.
    pub id: String,

    /// Human-readable pack name.
    #[arg(long)]
    pub name: String,

    /// Short pack description.
    #[arg(long)]
    pub description: String,

    /// Destination packs directory.
    #[arg(long)]
    pub dest: PathBuf,

    /// Initial language scaffold.
    #[arg(long, default_value = "rust")]
    pub lang: String,

    /// Replace an existing generated pack directory.
    #[arg(long)]
    pub force: bool,

    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct PackAddStageArgs {
    /// Path to the pack root directory.
    #[arg(long)]
    pub pack_dir: PathBuf,

    /// New stage id, such as 02_parse_input.
    pub id: String,

    /// Human-readable stage title.
    #[arg(long)]
    pub title: String,

    /// Replace existing scaffold files for this stage.
    #[arg(long)]
    pub force: bool,

    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct PackDoctorArgs {
    /// Project pack id.
    pub project: String,

    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct PackCheckReferenceArgs {
    /// Project pack id.
    pub project: String,

    /// Implementation language to initialize.
    #[arg(long, default_value = "rust")]
    pub lang: String,

    /// Path to reference solution main.rs.
    #[arg(long)]
    pub reference: PathBuf,

    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct PackInstallArgs {
    /// Project pack id.
    pub project: String,

    /// Destination packs directory.
    #[arg(long)]
    pub dest: PathBuf,

    /// Overwrite an existing installed pack directory.
    #[arg(long)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct DoctorArgs {
    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct ExplainFailureArgs {
    /// Stage id. Defaults to the current stage.
    #[arg(long)]
    pub stage: Option<String>,

    /// Print machine-readable JSON only.
    #[arg(long)]
    pub json: bool,
}
