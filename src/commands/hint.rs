use anyhow::{Context, Result};

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
    let hints = parse_hints(&hints_source);

    if hints.is_empty() {
        println!("No hints available for {}.", stage.id);
        return Ok(());
    }

    // The final authored hint is always the retrospective, gated until the
    // capability is acquired; every hint before it is available freely. A
    // stage with only one hint has no separate retrospective to gate, so the
    // floor keeps that single hint visible pre-completion too.
    let maximum = if context.state.is_completed(&stage_id) {
        hints.len()
    } else {
        hints.len().saturating_sub(1).max(hints.len().min(1))
    };

    if args.all {
        for (index, hint) in hints.iter().take(maximum).enumerate() {
            print_hint(index + 1, hints.len(), hint);
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
        print_hint(capped_level, hints.len(), &hints[capped_level - 1]);
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

fn parse_hints(source: &str) -> Vec<String> {
    let mut hints = Vec::new();
    let mut current = Vec::new();

    for line in source.lines() {
        if line.starts_with("# Hint ") {
            if !current.is_empty() {
                hints.push(current.join("\n").trim().to_string());
                current.clear();
            }
            continue;
        }
        current.push(line);
    }

    if !current.is_empty() {
        hints.push(current.join("\n").trim().to_string());
    }

    hints.into_iter().filter(|hint| !hint.is_empty()).collect()
}

fn print_hint(level: usize, total: usize, hint: &str) {
    println!("Hint {level}/{total}:");
    println!("{}", hint.trim());
    println!();
}
