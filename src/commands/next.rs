use anyhow::Result;

use crate::context::{GlobalOptions, ProjectContext};

pub fn run(options: &GlobalOptions) -> Result<()> {
    let mut context = ProjectContext::load(options)?;
    let current_stage = context.state.current_stage.clone();

    if !context.state.is_completed(&current_stage) {
        println!("Current stage has not passed yet.");
        println!("Run: deltaforge test");
        return Ok(());
    }
    context.verify_completion_proof(&current_stage)?;

    if !context.stage_gates(&current_stage)?.is_empty() {
        if context.config.gates.enforce {
            context.verify_gate_record(&current_stage)?;
        } else {
            println!("performance gates skipped: gates.enforce = false");
        }
    }

    if let Some(next_stage) = context.pack.manifest.next_stage(&current_stage) {
        context.state.current_stage = next_stage.id.clone();
        context.state.touch()?;
        context.save_state()?;
        println!("Unlocked Stage {}: {}", next_stage.id, next_stage.title);
    } else {
        println!("All stages complete.");
    }

    Ok(())
}
