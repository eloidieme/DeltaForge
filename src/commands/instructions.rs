use anyhow::{Context, Result};

use crate::cli::InstructionsArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::learning_web::{
    InitialView, generate_learning_page, open_learning_page, should_use_browser,
};
use crate::pack::StageSpec;
use crate::terminal::Terminal;
use crate::viewer;

pub fn run(args: InstructionsArgs, options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;
    let stage_id = args
        .stage
        .as_deref()
        .unwrap_or(&context.state.current_stage);
    context
        .pack
        .manifest
        .stage(stage_id)
        .with_context(|| format!("pack does not contain stage {stage_id}"))?;

    if args.no_open {
        let overview = super::overview::read_pack_overview(&context.pack);
        let path = generate_learning_page(&context, &overview, InitialView::Stage(stage_id))?;
        println!("Generated learning page: {}", path.display());
        return Ok(());
    }
    if should_use_browser(args.terminal) {
        let overview = super::overview::read_pack_overview(&context.pack);
        let path = generate_learning_page(&context, &overview, InitialView::Stage(stage_id))?;
        let ui_dir = context.root.join(".deltaforge/ui");
        match viewer::open_live(&ui_dir, "learning.html") {
            Ok(viewer::LiveOpen::OpenedTab(_)) => {
                println!("Opened stage {stage_id} instructions in your browser.");
                return Ok(());
            }
            Ok(viewer::LiveOpen::Updated(url)) => {
                println!("Live view updated to stage {stage_id} instructions: {url}");
                return Ok(());
            }
            Err(_) => match open_learning_page(&path) {
                Ok(()) => {
                    println!("Opened stage {stage_id} instructions in your browser.");
                    return Ok(());
                }
                Err(error) => {
                    eprintln!("warning: {error:#}; showing the terminal view instead");
                }
            },
        }
    }

    render_terminal(&context, &args)
}

fn render_terminal(context: &ProjectContext, args: &InstructionsArgs) -> Result<()> {
    let mut terminal = Terminal::new();

    if args.all {
        for stage in &context.pack.manifest.stages {
            render_stage_instructions(&mut terminal, context, stage)?;
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
        render_stage_instructions(&mut terminal, context, stage)?;
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
