use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let cli = deltaforge::cli::Cli::parse();

    match deltaforge::commands::run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
    }
}
