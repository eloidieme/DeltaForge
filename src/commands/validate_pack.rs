use anyhow::{Result, bail};
use serde::Serialize;

use crate::authoring::diagnose_pack;
use crate::cli::ValidatePackArgs;
use crate::context::GlobalOptions;
use crate::pack::{PackSearchOptions, discover_packs_with_options, load_pack, validate_pack};

pub fn run(args: ValidatePackArgs, options: &GlobalOptions) -> Result<()> {
    let pack_options = PackSearchOptions {
        packs_dir: options.packs_dir.clone(),
    };
    let packs = if let Some(project) = args.project {
        vec![load_pack(&project, &pack_options)?]
    } else {
        discover_packs_with_options(&pack_options)?
    };

    if packs.is_empty() {
        if args.json {
            println!("[]");
        } else {
            println!("No project packs found.");
        }
        return Ok(());
    }

    let mut failed = false;
    let mut results = Vec::new();

    for pack in packs {
        let mut problems = validate_pack(&pack);
        if args.strict {
            for problem in diagnose_pack(&pack).problems {
                if !problems.contains(&problem) {
                    problems.push(problem);
                }
            }
        }
        if problems.is_empty() {
            if !args.json {
                println!("✓ {} is valid", pack.manifest.id);
            }
        } else {
            failed = true;
            if !args.json {
                println!("✗ {} is invalid", pack.manifest.id);
                for problem in &problems {
                    println!("  - {problem}");
                }
            }
        }
        let valid = problems.is_empty();
        results.push(PackValidationResult {
            id: pack.manifest.id,
            valid,
            problems,
        });
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    }

    if failed {
        bail!("pack validation failed");
    }

    Ok(())
}

#[derive(Debug, Serialize)]
struct PackValidationResult {
    id: String,
    valid: bool,
    problems: Vec<String>,
}
