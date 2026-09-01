//! `deltaforge portfolio`: a short narrative built from the same recorded
//! evidence as the full report.

use anyhow::Result;

use crate::cli::PortfolioArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::fs_util::atomic_write;
use crate::reporting;

pub fn run(args: PortfolioArgs, options: &GlobalOptions) -> Result<()> {
    let mut context = ProjectContext::load(options)?;
    let report = reporting::build(&context)?;
    atomic_write(&args.output, reporting::render_summary(&report))?;
    context.exclude_generated_root_file(&args.output)?;
    println!("Wrote portfolio summary: {}", args.output.display());
    Ok(())
}
