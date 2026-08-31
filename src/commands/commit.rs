//! `deltaforge commit`: the terminal form of a stage snapshot.
//!
//! The snapshot the browser offers at the pass moment and this command share
//! one engine ([`crate::snapshot`]), so the commit and tag are identical
//! whichever surface asked for them.

use anyhow::Result;

use crate::application;
use crate::cli::CommitArgs;
use crate::context::GlobalOptions;

pub fn run(args: CommitArgs, options: &GlobalOptions) -> Result<()> {
    run_impl(args.force, options, false)
}

pub fn run_automatic(options: &GlobalOptions, quiet: bool) -> Result<()> {
    run_impl(false, options, quiet)
}

fn run_impl(force: bool, options: &GlobalOptions, quiet: bool) -> Result<()> {
    let outcome = application::create_stage_snapshot(options, force)?;
    if !quiet {
        println!("Created commit: {}", outcome.commit);
        println!("{}", outcome.message);
        if let Some(tag) = &outcome.tag {
            println!("Tagged: {tag}");
        }
        if let Some(tag) = &outcome.existing_tag {
            println!("Tag {tag} already exists and was left unchanged.");
        }
    }
    Ok(())
}
