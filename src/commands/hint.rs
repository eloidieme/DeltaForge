use anyhow::{Context, Result};

use crate::capability::{HelpLevel, available_help_levels, parse_help};
use crate::cli::HintArgs;
use crate::context::{GlobalOptions, ProjectContext};

pub fn run(args: HintArgs, options: &GlobalOptions) -> Result<()> {
    if args.level == Some(0) {
        anyhow::bail!("hint level must be greater than 0");
    }
    let initial = ProjectContext::load(options)?;
    let _lease = crate::application::acquire_for_learner_action(&initial.root, "update help")?;
    let mut context = ProjectContext::load(options)?;
    let stage_id = context.state.current_stage.clone();
    let stage = context
        .pack
        .manifest
        .stage(&stage_id)
        .with_context(|| format!("pack does not contain current stage {stage_id}"))?;
    let hints_path = context.pack.hints_path(stage);
    let hints_source = context.pack.read_stage_file(&hints_path)?;
    // The same parser the workbench uses. This command and that surface write
    // and read one `hint_state` counter, so parsing the ladder two different
    // ways let `hint --all` push the counter past the gated retrospective.
    let hints = parse_help(&hints_source);

    if hints.is_empty() {
        println!("No hints available for {}.", stage.id);
        return Ok(());
    }

    // The final authored hint is always the retrospective, gated until the
    // capability is acquired; every hint before it is available freely.
    let maximum = available_help_levels(hints.len(), context.state.is_completed(&stage_id));

    if args.all {
        for hint in hints.iter().take(maximum) {
            print_hint(hints.len(), hint);
        }
        context.state.hint_state.insert(stage.id.clone(), maximum);
    } else {
        let level = args.level.unwrap_or_else(|| {
            context
                .state
                .hint_state
                .get(&stage.id)
                .copied()
                .unwrap_or(0)
                + 1
        });
        if level > maximum {
            if hints.len() <= maximum {
                anyhow::bail!("all help levels are already revealed");
            }
            anyhow::bail!("the retrospective unlocks after this capability is acquired");
        }
        let capped_level = level.min(maximum);
        print_hint(hints.len(), &hints[capped_level - 1]);
        let previous = context
            .state
            .hint_state
            .get(&stage.id)
            .copied()
            .unwrap_or(0);
        context
            .state
            .hint_state
            .insert(stage.id.clone(), previous.max(capped_level));
    }

    context.state.touch()?;
    context.save_state()?;
    crate::run_journal::append(
        &context.root,
        &crate::application::RunEvent::ProjectStateChanged,
    )?;
    Ok(())
}

fn print_hint(total: usize, hint: &HelpLevel) {
    println!("Hint {}/{total}:", hint.level);
    println!("{}", hint.content.trim());
    println!();
}
