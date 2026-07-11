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
        Command::List => list::run(&options),
        Command::Pack(command) => pack::run(command, &options),
        Command::Init(args) => init::run(args, &options),
        Command::ValidatePack(args) => validate_pack::run(args, &options),
        Command::Instructions(args) => instructions::run(args, &options),
        Command::Overview(args) => overview::run(args, &options),
        Command::Test(args) => test::run(args, &options),
        Command::Next => next::run(&options),
        Command::SyncPack(args) => sync_pack::run(args, &options),
        Command::Status(args) => status::run(args, &options),
        Command::Hint(args) => hint::run(args, &options),
        Command::Config(command) => config::run(command, &options),
        Command::Bench(args) => bench::run(args, &options),
        Command::Report(args) => report::run(args, &options),
        Command::Portfolio(args) => portfolio::run(args, &options),
        Command::Design(args) => design::run(args, &options),
        Command::Commit(args) => commit::run(args, &options),
        Command::Doctor(args) => doctor::run(args, &options),
        Command::ExplainFailure(args) => explain_failure::run(args, &options),
    }
}

fn default_project_directory(args: &InitArgs) -> String {
    args.name
        .clone()
        .unwrap_or_else(|| format!("{}-{}", args.project, args.lang))
}
