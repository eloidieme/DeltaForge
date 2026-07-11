use anyhow::Result;

use crate::context::{GlobalOptions, ProjectContext};

pub fn run(options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;

    println!("Project: {}", context.pack.manifest.name);
    println!("Language: {}", context.state.language);
    println!("Current stage: {}", context.state.current_stage);
    println!();

    println!("Stages:");
    for stage in &context.pack.manifest.stages {
        let marker = if context.state.is_completed(&stage.id) {
            "✓"
        } else if stage.id == context.state.current_stage {
            "→"
        } else {
            "○"
        };
        println!("  {marker} {} - {}", stage.id, stage.title);
    }

    Ok(())
}
