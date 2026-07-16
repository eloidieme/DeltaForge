use anyhow::Result;

use crate::cli::{Cli, Command, InitArgs};
use crate::context::GlobalOptions;

pub mod bench;
pub mod commit;
pub mod config;
pub mod design;
pub mod doctor;
pub mod explain_failure;
pub mod hint;
pub mod init;
pub mod instructions;
pub mod list;
pub mod next;
pub mod overview;
pub mod pack;
pub mod portfolio;
pub mod report;
pub mod status;
pub mod sync_pack;
pub mod test;
pub mod validate_pack;

pub fn run(cli: Cli) -> Result<()> {
    let options = GlobalOptions {
        project_dir: cli.project_dir,
        packs_dir: cli.packs_dir,
    };

    match cli.command {
        None => crate::workbench::launch(&options),
        Some(Command::List) => list::run(&options),
        Some(Command::Pack(command)) => pack::run(command, &options),
        Some(Command::Init(args)) => init::run(args, &options),
        Some(Command::ValidatePack(args)) => validate_pack::run(args, &options),
        Some(Command::Instructions(args)) => instructions::run(args, &options),
        Some(Command::Overview(args)) => overview::run(args, &options),
        Some(Command::Test(args)) => test::run(args, &options),
        Some(Command::Next) => next::run(&options),
        Some(Command::SyncPack(args)) => sync_pack::run(args, &options),
        Some(Command::Status(args)) => status::run(args, &options),
        Some(Command::Hint(args)) => hint::run(args, &options),
        Some(Command::Config(command)) => config::run(command, &options),
        Some(Command::Bench(args)) => bench::run(args, &options),
        Some(Command::Report(args)) => report::run(args, &options),
        Some(Command::Portfolio(args)) => portfolio::run(args, &options),
        Some(Command::Design(args)) => design::run(args, &options),
        Some(Command::Commit(args)) => commit::run(args, &options),
        Some(Command::Doctor(args)) => doctor::run(args, &options),
        Some(Command::ExplainFailure(args)) => explain_failure::run(args, &options),
        Some(Command::Workbench(args)) => crate::workbench::serve(
            &options,
            args.token,
            args.idle_timeout_ms.map(std::time::Duration::from_millis),
        ),
    }
}

fn default_project_directory(args: &InitArgs) -> String {
    args.name
        .clone()
        .unwrap_or_else(|| format!("{}-{}", args.project, args.lang))
}
