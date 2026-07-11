use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::cli::CommitArgs;
use crate::context::{GlobalOptions, ProjectContext};

pub fn run(args: CommitArgs, options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;
    ensure_git_repo(&context.root)?;

    let stage = context
        .pack
        .manifest
        .stage(&context.state.current_stage)
        .with_context(|| {
            format!(
                "pack does not contain current stage {}",
                context.state.current_stage
            )
        })?;

    if !args.force && !context.state.is_completed(&stage.id) {
        bail!(
            "refusing to commit because current stage {} has not passed; run `deltaforge test` or use `deltaforge commit --force`",
            stage.id
        );
    }

    run_git(&context.root, &["add", "-A"])?;
    let message = format!(
        "Complete Stage {}: {}",
        stage_number(&stage.id),
        stage.title
    );
    run_git(&context.root, &["commit", "-m", &message])?;
    let hash = git_stdout(&context.root, &["rev-parse", "HEAD"])?;

    if context.config.git.auto_tag {
        let tag = format!("deltaforge-{}", stage.id);
        let _ = run_git(&context.root, &["tag", &tag]);
    }

    println!("Created commit: {}", hash.trim());
    println!("{message}");
    Ok(())
}

fn ensure_git_repo(root: &Path) -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(root)
        .output()
        .with_context(|| format!("failed to run git in {}", root.display()))?;
    if !output.status.success() {
        bail!("not a git repository: {}", root.display());
    }
    Ok(())
}

fn run_git(root: &Path, args: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .with_context(|| format!("failed to run git {}", args.join(" ")))?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

fn git_stdout(root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .with_context(|| format!("failed to run git {}", args.join(" ")))?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn stage_number(stage_id: &str) -> String {
    stage_id
        .split_once('_')
        .map_or(stage_id, |(number, _)| number)
        .to_string()
}
