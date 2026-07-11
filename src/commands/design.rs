use std::env;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::cli::DesignArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::fs_util::atomic_write;

pub fn run(args: DesignArgs, options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;
    let stage_id = args
        .stage
        .as_deref()
        .unwrap_or(&context.state.current_stage);
    let stage = context
        .pack
        .manifest
        .stage(stage_id)
        .with_context(|| format!("pack does not contain stage {stage_id}"))?;
    let prompt_path = context.pack.design_prompt_path(stage);
    let note_path = context
        .root
        .join(".deltaforge")
        .join("design_notes")
        .join(format!("{}.md", stage.id));

    if args.edit {
        if !note_path.exists() {
            let seed = if prompt_path.is_file() {
                format!(
                    "# Design Notes: {} - {}\n\n<!-- Prompt:\n{}\n-->\n\n",
                    stage.id,
                    stage.title,
                    std::fs::read_to_string(&prompt_path)?
                )
            } else {
                format!("# Design Notes: {} - {}\n\n", stage.id, stage.title)
            };
            atomic_write(&note_path, seed)?;
        }
        open_editor(&note_path)?;
        return Ok(());
    }

    println!("Stage {}: {}", stage.id, stage.title);
    println!("Design notes: {}", note_path.display());
    println!();
    if prompt_path.is_file() {
        println!("{}", std::fs::read_to_string(&prompt_path)?.trim_end());
    } else {
        println!("No design prompt is defined for this stage.");
    }

    Ok(())
}

fn open_editor(path: &std::path::Path) -> Result<()> {
    let editor = env::var("EDITOR").context("$EDITOR is not set; set it or run without --edit")?;
    let mut parts = editor.split_whitespace();
    let program = parts
        .next()
        .filter(|part| !part.is_empty())
        .context("$EDITOR is empty")?;
    let status = Command::new(program)
        .args(parts)
        .arg(path)
        .status()
        .with_context(|| format!("failed to run editor {program}"))?;
    if !status.success() {
        bail!("editor exited with status {status}");
    }
    Ok(())
}
