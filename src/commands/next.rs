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
