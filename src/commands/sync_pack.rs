use anyhow::Result;
use serde::Serialize;

use crate::cli::SyncPackArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::pack::pack_source_label;

pub fn run(args: SyncPackArgs, options: &GlobalOptions) -> Result<()> {
    let mut context = ProjectContext::load_unpinned(options)?;

    let old_version = context.state.pack_version.clone();
    let old_source = context.state.pack_source.clone();
    let old_digest = context.state.pack_digest.clone();

    let new_version = context.pack.manifest.version.clone();
    let new_source = pack_source_label(&context.pack.root);
    let new_digest = context.pack_digest()?;

    context.state.pack_version = new_version.clone();
    context.state.pack_source = new_source.clone();
    context.state.pack_digest = new_digest.clone();

    // Re-pin the pack digest recorded in each existing completion proof. The
    // learner's own project_digest stays untouched so completion evidence for
    // the learner's work is preserved.
    let mut updated_proofs = 0;
    for proof in context.state.completion_proofs.values_mut() {
        if proof.pack_digest != new_digest {
            proof.pack_digest = new_digest.clone();
            updated_proofs += 1;
        }
    }

    context.state.touch()?;
    context.save_state()?;

    let report = SyncReport {
        project: context.state.project.clone(),
        version: Change::new(old_version, new_version),
        source: Change::new(old_source, new_source),
        digest: Change::new(old_digest, new_digest),
        updated_proofs,
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("Re-pinned project {} to the current pack.", report.project);
    print_change("version", &report.version);
    print_change("source", &report.source);
    print_change("digest", &report.digest);
    println!("  completion proofs updated: {}", report.updated_proofs);
    Ok(())
}

fn print_change(label: &str, change: &Change) {
    if change.old == change.new {
        println!("  {label}: {} (unchanged)", display(&change.new));
    } else {
        println!(
            "  {label}: {} → {}",
            display(&change.old),
            display(&change.new)
        );
    }
}

fn display(value: &str) -> &str {
    if value.is_empty() { "(unset)" } else { value }
}

#[derive(Debug, Serialize)]
struct SyncReport {
    project: String,
    version: Change,
    source: Change,
    digest: Change,
    updated_proofs: usize,
}

#[derive(Debug, Serialize)]
struct Change {
    old: String,
    new: String,
}

impl Change {
    fn new(old: String, new: String) -> Self {
        Self { old, new }
    }
}
