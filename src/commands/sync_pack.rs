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

    // Migrate legacy proofs (recorded before behavioral digests existed) only
    // when the pack is bit-identical to the one they passed against — the one
    // case where upgrading them is provably safe.
    let legacy_stages: Vec<String> = context
        .state
        .completion_proofs
        .iter()
        .filter(|(_, proof)| proof.behavioral_digest.is_empty() && proof.pack_digest == new_digest)
        .map(|(stage_id, _)| stage_id.clone())
        .collect();
    let mut migrated_proofs = 0;
    for stage_id in legacy_stages {
        let Ok(behavioral) = context.stage_behavioral_digest(&stage_id) else {
            continue;
        };
        if let Some(proof) = context.state.completion_proofs.get_mut(&stage_id) {
            proof.behavioral_digest = behavioral;
            migrated_proofs += 1;
        }
    }

    // Update only the project-level pin. Completion proofs keep the digests of
    // what actually passed; `next`/`commit` compare them against the adopted
    // pack per stage and require revalidation where behavior changed.
    context.state.pack_version = new_version.clone();
    context.state.pack_source = new_source.clone();
    context.state.pack_digest = new_digest.clone();

    let stages: Vec<StageSync> = context
        .state
        .completed_stages
        .iter()
        .map(|stage_id| StageSync {
            id: stage_id.clone(),
            status: if context.stage_needs_revalidation(stage_id).unwrap_or(true) {
                "needs_revalidation"
            } else {
                "valid"
            },
        })
        .collect();

    context.state.touch()?;
    context.save_state()?;

    let report = SyncReport {
        project: context.state.project.clone(),
        version: Change::new(old_version, new_version),
        source: Change::new(old_source, new_source),
        digest: Change::new(old_digest, new_digest),
        migrated_proofs,
        stages,
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("Re-pinned project {} to the current pack.", report.project);
    print_change("version", &report.version);
    print_change("source", &report.source);
    print_change("digest", &report.digest);
    if report.migrated_proofs > 0 {
        println!(
            "  migrated legacy completion proofs: {}",
            report.migrated_proofs
        );
    }
    if !report.stages.is_empty() {
        println!();
        println!("Completed stages:");
        for stage in &report.stages {
            if stage.status == "valid" {
                println!("  ✓ {}", stage.id);
            } else {
                println!("  ! {} (needs revalidation)", stage.id);
            }
        }
        if report.stages.iter().any(|s| s.status != "valid") {
            println!();
            println!("Stages marked ! passed against an older version of this pack.");
            println!("Run `deltaforge test --stage <id>` to revalidate them.");
        }
    }
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
    migrated_proofs: usize,
    stages: Vec<StageSync>,
}

#[derive(Debug, Serialize)]
struct StageSync {
    id: String,
    status: &'static str,
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
