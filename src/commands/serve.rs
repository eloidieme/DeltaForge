use anyhow::Result;

use crate::cli::ServeArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::viewer;

pub fn run(args: ServeArgs, options: &GlobalOptions) -> Result<()> {
    let ui_dir = match args.ui_dir {
        Some(dir) => dir,
        None => ProjectContext::load(options)?.root.join(".deltaforge/ui"),
    };

    if args.stop {
        let stopped = viewer::stop(&ui_dir)?;
        if !args.quiet {
            if stopped {
                println!("Stopped the live viewer.");
            } else {
                println!("No live viewer is running.");
            }
        }
        return Ok(());
    }

    if args.restart {
        let stopped = viewer::stop(&ui_dir)?;
        if stopped && !args.quiet {
            println!("Stopped the previous live viewer.");
        }
    }

    viewer::serve(&ui_dir, args.auto, args.open, args.quiet)
}
