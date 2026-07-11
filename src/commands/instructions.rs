use anyhow::{Context, Result};

use crate::cli::InstructionsArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::pack::StageSpec;
use crate::terminal::Terminal;

pub fn run(args: InstructionsArgs, options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;
    let mut terminal = Terminal::new();

    if args.all {
        for stage in &context.pack.manifest.stages {
            render_stage_instructions(&mut terminal, &context, stage)?;
            terminal.blank_line();
        }
    } else {
        let stage_id = args
            .stage
            .as_deref()
            .unwrap_or(&context.state.current_stage);
        let stage = context
            .pack
            .manifest
            .stage(stage_id)
            .with_context(|| format!("pack does not contain stage {stage_id}"))?;
        render_stage_instructions(&mut terminal, &context, stage)?;
    }

    terminal.display()?;
    Ok(())
}

fn render_stage_instructions(
    terminal: &mut Terminal,
    context: &ProjectContext,
    stage: &StageSpec,
) -> Result<()> {
    let path = context.pack.instructions_path(stage);
    let instructions = context.pack.read_stage_file(&path)?;

    terminal.title(&format!("Stage {}: {}", stage.id, stage.title));
    terminal.key_value("Project", &context.pack.manifest.name);
    terminal.key_value("Pack", &context.state.project);
    terminal.key_value("Language", &context.state.language);
    terminal.key_value(
        "Status",
        if context.state.is_completed(&stage.id) {
            "complete"
        } else if stage.id == context.state.current_stage {
            "current"
        } else {
            "upcoming"
        },
    );
    terminal.section("Instructions");
    terminal.markdown(&instructions);

    Ok(())
}
