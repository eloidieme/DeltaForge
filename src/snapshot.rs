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
///
/// `position` is the stage's 1-based position in the pack's manifest, not a
/// number parsed from its id: a pack whose stage ids are not a clean,
/// consecutive `01`-`NN` sequence (a bundled preview pack renumbered less
/// carefully than the flagship, or a third-party pack) would otherwise
/// produce two commits with the same "Complete Stage N" subject for two
/// different steps.
pub fn snapshot_message(position: usize, stage_title: &str) -> String {
    format!("Complete Stage {position:02}: {stage_title}")
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
        if path.replace('\\', "/") != ".deltaforge/run.lock" {
            files.push(ChangedFile {
                path: path.to_string(),
                change: classify(index, worktree),
            });
        }
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

fn classify(index: u8, worktree: u8) -> &'static str {
    match (index, worktree) {
        (b'?', _) => "untracked",
        // Checked before `(b'A', _)`: a file added to the index and then
        // deleted in the worktree (porcelain `AD`) is a deletion from the
        // snapshot's point of view, not an addition.
        (b'D', _) | (_, b'D') => "deleted",
        (b'A', _) => "added",
        (b'R', _) => "renamed",
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
    let changed = changed_files(root)?;
    if changed.is_empty() {
        // Nothing to add or commit — for instance the learner already
        // committed by hand. `preview_stage_snapshot` reports this same
        // condition as "nothing new to snapshot" rather than an error, so
        // taking the snapshot anyway must agree: tag the commit that is
        // already there (if it isn't tagged yet) and report honestly instead
        // of running `git commit` with nothing staged.
        let commit = git_output(root, &["rev-parse", "HEAD"])?.trim().to_string();
        let (tag, existing_tag) = match tag {
            Some(tag) if tag_exists(root, tag) => (None, Some(tag.to_string())),
            Some(tag) => {
                run_git(root, &["tag", tag])?;
                (Some(tag.to_string()), None)
            }
            None => (None, None),
        };
        return Ok(SnapshotOutcome {
            commit,
            message: message.to_string(),
            tag,
            existing_tag,
            changed_files: 0,
        });
    }

    // Capture the current index (without touching the working tree) so a
    // failed commit below can put it back rather than leaving the learner's
    // staged/unstaged split overwritten by `add -A`.
    let previous_index_tree = git_output(root, &["write-tree"])
        .ok()
        .map(|tree| tree.trim().to_string());

    let attempt: Result<()> = (|| {
        run_git(root, &["add", "-A"])?;
        // Also protect projects created before DeltaForge wrote .gitignore.
        // Remove only the live lease path from the index; source changes stay staged.
        run_git(
            root,
            &["rm", "--cached", "--ignore-unmatch", ".deltaforge/run.lock"],
        )?;
        run_git(root, &["commit", "-m", message])
    })();
    if let Err(error) = attempt {
        let restored = previous_index_tree
            .as_deref()
            .is_some_and(|tree| run_git(root, &["read-tree", tree]).is_ok());
        return Err(error.context(if restored {
            "the git index has been restored to what it was before this snapshot attempt"
        } else {
            "the git index was left staged by this failed snapshot attempt; run `git reset` to undo the staging"
        }));
    }

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
        changed_files: changed.len(),
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
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.trim();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stdout = stdout.trim();
        let reason = if !stderr.is_empty() {
            stderr.to_string()
        } else if !stdout.is_empty() {
            stdout.to_string()
        } else {
            format!(
                "git exited with status {}",
                output
                    .status
                    .code()
                    .map_or_else(|| "unknown".to_string(), |code| code.to_string())
            )
        };
        bail!("git {} failed: {reason}", args.join(" "));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_follow_manifest_position_not_the_stage_id() {
        assert_eq!(
            snapshot_message(7, "Persist the index"),
            "Complete Stage 07: Persist the index"
        );
        assert_eq!(
            snapshot_message(14, "Stable ranking"),
            "Complete Stage 14: Stable ranking"
        );
        // Two different steps that happen to share an id prefix (a pack
        // renumbered less carefully than the flagship) must not collide:
        // the position, not the id, drives the number.
        assert_ne!(
            snapshot_message(2, "Append the log"),
            snapshot_message(3, "Preserve history")
        );
    }

    #[test]
    fn tags_follow_the_stage_id() {
        assert_eq!(
            snapshot_tag("07_persist_index"),
            "deltaforge-07_persist_index"
        );
    }

    #[test]
    fn porcelain_codes_map_to_learner_readable_changes() {
        assert_eq!(classify(b'?', b'?'), "untracked");
        assert_eq!(classify(b'A', b' '), "added");
        assert_eq!(classify(b' ', b'M'), "modified");
        assert_eq!(classify(b' ', b'D'), "deleted");
        assert_eq!(classify(b'R', b' '), "renamed");
        // Added to the index, then deleted in the worktree: from the
        // snapshot's point of view that is a deletion, not an addition.
        assert_eq!(classify(b'A', b'D'), "deleted");
    }
}
