//! Stage snapshots: the commit and tag offered when a step is acquired.
//!
//! A snapshot is never taken without the learner asking for it. The engine
//! here reports what would be committed first, so the offer at the pass moment
//! can show the change before it is recorded rather than after.

use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::Serialize;

/// One file the snapshot would record, with a Git status code the browser can
/// render without knowing Git's porcelain format.
#[derive(Debug, Clone, Serialize)]
pub struct ChangedFile {
    pub path: String,
    /// `added`, `modified`, `deleted`, `renamed`, or `untracked`.
    pub change: &'static str,
}

/// What a snapshot would do, computed before doing it.
#[derive(Debug, Clone, Serialize)]
pub struct SnapshotPreview {
    pub available: bool,
    /// Why a snapshot cannot be taken right now, when `available` is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub changed_files: Vec<ChangedFile>,
    /// True when the tag this snapshot would create already exists, which is
    /// what a second snapshot of the same step looks like.
    pub tag_exists: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotOutcome {
    pub commit: String,
    pub message: String,
    /// The tag this snapshot created. `None` when tagging is off, or when the
    /// step's tag already existed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Set when a tag for this step was already present, so a second snapshot
    /// of the same step reports honestly instead of failing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_tag: Option<String>,
    pub changed_files: usize,
}

pub fn is_git_repository(root: &Path) -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(root)
        .output()
        .is_ok_and(|output| output.status.success())
}

pub fn ensure_git_repository(root: &Path) -> Result<()> {
    if !is_git_repository(root) {
        bail!("not a git repository: {}", root.display());
    }
    Ok(())
}

/// Commit message for one stage: `Complete Stage 07: Persist the index`.
pub fn snapshot_message(stage_id: &str, stage_title: &str) -> String {
    format!("Complete Stage {}: {stage_title}", stage_number(stage_id))
}

pub fn snapshot_tag(stage_id: &str) -> String {
    format!("deltaforge-{stage_id}")
}

/// Everything Git would include in the next `add -A` commit: tracked changes
/// and untracked files alike, with paths relative to the project root.
pub fn changed_files(root: &Path) -> Result<Vec<ChangedFile>> {
    let output = git_output(
        root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )?;
    let mut files = Vec::new();
    let mut fields = output.split('\0').filter(|entry| !entry.is_empty());
    while let Some(entry) = fields.next() {
        // Porcelain v1 records are `XY <path>`, and a rename adds a second
        // NUL-separated field holding the original path.
        let Some((status, path)) = entry.split_at_checked(3) else {
            continue;
        };
        let code = status.as_bytes();
        let (index, worktree) = (code[0], code[1]);
        if index == b'R' {
            let _ = fields.next();
        }
        files.push(ChangedFile {
            path: path.to_string(),
            change: classify(index, worktree),
        });
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

fn classify(index: u8, worktree: u8) -> &'static str {
    match (index, worktree) {
        (b'?', _) => "untracked",
        (b'A', _) => "added",
        (b'R', _) => "renamed",
        (b'D', _) | (_, b'D') => "deleted",
        _ => "modified",
    }
}

pub fn tag_exists(root: &Path, tag: &str) -> bool {
    Command::new("git")
        .args([
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/tags/{tag}"),
        ])
        .current_dir(root)
        .output()
        .is_ok_and(|output| output.status.success())
}

/// Record the snapshot. `tag` is `None` when the project has tagging turned
/// off, or when the tag already exists.
pub fn take(root: &Path, message: &str, tag: Option<&str>) -> Result<SnapshotOutcome> {
    ensure_git_repository(root)?;
    let changed = changed_files(root)?.len();
    run_git(root, &["add", "-A"])?;
    run_git(root, &["commit", "-m", message])?;
    let commit = git_output(root, &["rev-parse", "HEAD"])?.trim().to_string();
    let (tag, existing_tag) = match tag {
        Some(tag) if tag_exists(root, tag) => (None, Some(tag.to_string())),
        Some(tag) => {
            run_git(root, &["tag", tag])?;
            (Some(tag.to_string()), None)
        }
        None => (None, None),
    };
    Ok(SnapshotOutcome {
        commit,
        message: message.to_string(),
        tag,
        existing_tag,
        changed_files: changed,
    })
}

fn run_git(root: &Path, args: &[&str]) -> Result<()> {
    git_output(root, args).map(|_| ())
}

fn git_output(root: &Path, args: &[&str]) -> Result<String> {
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

/// The numeric prefix of a stage id, used in the commit subject.
fn stage_number(stage_id: &str) -> &str {
    stage_id
        .split_once('_')
        .map_or(stage_id, |(number, _)| number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_and_tags_follow_the_stage_id() {
        assert_eq!(
            snapshot_message("07_persist_index", "Persist the index"),
            "Complete Stage 07: Persist the index"
        );
        assert_eq!(
            snapshot_tag("07_persist_index"),
            "deltaforge-07_persist_index"
        );
        assert_eq!(stage_number("14_stable_ranking"), "14");
        assert_eq!(stage_number("nounderscore"), "nounderscore");
    }

    #[test]
    fn porcelain_codes_map_to_learner_readable_changes() {
        assert_eq!(classify(b'?', b'?'), "untracked");
        assert_eq!(classify(b'A', b' '), "added");
        assert_eq!(classify(b' ', b'M'), "modified");
        assert_eq!(classify(b' ', b'D'), "deleted");
        assert_eq!(classify(b'R', b' '), "renamed");
    }
}
