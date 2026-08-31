//! `deltaforge report`: the terminal form of the engineering record export.
//!
//! The record itself is assembled by [`crate::reporting`], which the browser
//! export uses too, so both surfaces make exactly the same claims.

use anyhow::Result;

use crate::cli::ReportArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::fs_util::atomic_write;
use crate::reporting;

pub fn run(args: ReportArgs, options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;
    let report = reporting::build(&context)?;
    atomic_write(&args.output, reporting::render(&report, args.format)?)?;
    println!("Wrote report: {}", args.output.display());
    Ok(())
}
