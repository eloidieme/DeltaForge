use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::application::RunEvent;
use crate::fs_util::atomic_write;

const JOURNAL_FILE: &str = "workbench-events.json";
const JOURNAL_LOCK_FILE: &str = "workbench-events.lock";
const MAX_EVENTS: usize = 256;
const MAX_BYTES: usize = 2 * 1024 * 1024;
const MAX_STRING_BYTES: usize = 16 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: u64,
    pub event: Value,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal {
    #[serde(default)]
    next_id: u64,
    #[serde(default)]
    events: Vec<JournalEntry>,
}

pub fn append(project_root: &Path, event: &RunEvent) -> Result<u64> {
    with_journal_lock(project_root, || {
        let path = journal_path(project_root);
        let mut journal = read_unlocked(&path)?;
        let id = journal.next_id.max(1);
        journal.next_id = id.saturating_add(1);
        let mut value = serde_json::to_value(event)?;
        truncate_value(&mut value);
        journal.events.push(JournalEntry { id, event: value });
        if journal.events.len() > MAX_EVENTS {
            let excess = journal.events.len() - MAX_EVENTS;
            journal.events.drain(..excess);
        }
        while journal.events.len() > 1 && serde_json::to_vec(&journal)?.len() > MAX_BYTES {
            journal.events.remove(0);
        }
        atomic_write(&path, serde_json::to_vec(&journal)?)?;
        Ok(id)
    })
}

pub fn entries_after(project_root: &Path, cursor: u64) -> Result<Vec<JournalEntry>> {
    with_journal_lock(project_root, || {
        Ok(read_unlocked(&journal_path(project_root))?
            .events
            .into_iter()
            .filter(|entry| entry.id > cursor)
            .collect())
    })
}

pub fn cursor(project_root: &Path) -> Result<u64> {
    with_journal_lock(project_root, || {
        let journal = read_unlocked(&journal_path(project_root))?;
        Ok(journal.next_id.saturating_sub(1))
    })
}

pub fn contains_source_revision(project_root: &Path, revision: u64) -> Result<bool> {
    with_journal_lock(project_root, || {
        Ok(read_unlocked(&journal_path(project_root))?
            .events
            .iter()
            .any(|entry| {
                entry.event.get("type").and_then(Value::as_str) == Some("source_changed")
                    && entry.event.get("revision").and_then(Value::as_u64) == Some(revision)
            }))
    })
}

fn journal_path(project_root: &Path) -> PathBuf {
    project_root.join(".deltaforge").join(JOURNAL_FILE)
}

fn with_journal_lock<T>(project_root: &Path, operation: impl FnOnce() -> Result<T>) -> Result<T> {
    let lock_path = project_root.join(".deltaforge").join(JOURNAL_LOCK_FILE);
    if let Some(parent) = lock_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .with_context(|| format!("failed to open event journal lock {}", lock_path.display()))?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    loop {
        match file.try_lock_exclusive() {
            Ok(()) => break,
            Err(error) if crate::fs_util::lock_unavailable(&error) => {
                if std::time::Instant::now() >= deadline {
                    anyhow::bail!("timed out waiting for the event journal lock");
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            Err(error) => return Err(error).context("failed to lock the event journal"),
        }
    }
    let result = operation();
    let _ = FileExt::unlock(&file);
    result
}

fn read_unlocked(path: &Path) -> Result<Journal> {
    let source = match fs::read(path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Journal::default()),
        Err(error) => {
            return Err(error)
                .with_context(|| format!("failed to read event journal {}", path.display()));
        }
    };
    match serde_json::from_slice(&source) {
        Ok(journal) => Ok(journal),
        Err(_) => {
            let stamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos();
            let backup = path.with_file_name(format!("workbench-events.corrupt-{stamp}.json"));
            fs::rename(path, &backup).with_context(|| {
                format!(
                    "failed to quarantine corrupt event journal {}",
                    path.display()
                )
            })?;
            Ok(Journal::default())
        }
    }
}

fn truncate_value(value: &mut Value) {
    match value {
        Value::String(text) if text.len() > MAX_STRING_BYTES => {
            let mut boundary = MAX_STRING_BYTES;
            while !text.is_char_boundary(boundary) {
                boundary -= 1;
            }
            text.truncate(boundary);
            text.push_str("\n[deltaforge: event field truncated]");
        }
        Value::Array(values) => values.iter_mut().for_each(truncate_value),
        Value::Object(values) => values.values_mut().for_each(truncate_value),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_root() -> PathBuf {
        std::env::temp_dir().join(format!(
            "deltaforge-journal-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn journal_is_bounded_and_supports_cursor_replay() {
        let root = temp_root();
        for index in 0..(MAX_EVENTS + 8) {
            append(
                &root,
                &RunEvent::BuildOutput {
                    stream: "stdout",
                    text: format!("line {index}"),
                },
            )
            .unwrap();
        }
        append(
            &root,
            &RunEvent::SourceChanged {
                revision: 7,
                previous_digest: "before".to_string(),
                current_digest: "after".to_string(),
            },
        )
        .unwrap();
        let entries = entries_after(&root, 0).unwrap();
        assert_eq!(entries.len(), MAX_EVENTS);
        assert!(contains_source_revision(&root, 7).unwrap());
        assert!(!contains_source_revision(&root, 6).unwrap());
        let cursor = entries[entries.len() - 2].id;
        assert_eq!(entries_after(&root, cursor).unwrap().len(), 1);
        assert_eq!(super::cursor(&root).unwrap(), entries.last().unwrap().id);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn corrupt_journal_is_quarantined_and_rebuilt() {
        let root = temp_root();
        fs::create_dir_all(root.join(".deltaforge")).unwrap();
        fs::write(journal_path(&root), "not json").unwrap();
        assert_eq!(cursor(&root).unwrap(), 0);
        assert_eq!(append(&root, &RunEvent::ProjectStateChanged).unwrap(), 1);
        assert_eq!(entries_after(&root, 0).unwrap().len(), 1);
        assert!(
            fs::read_dir(root.join(".deltaforge"))
                .unwrap()
                .any(|entry| {
                    entry
                        .unwrap()
                        .file_name()
                        .to_string_lossy()
                        .starts_with("workbench-events.corrupt-")
                })
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn concurrent_appends_keep_unique_ordered_ids() {
        let root = temp_root();
        let workers = (0..8)
            .map(|_| {
                let root = root.clone();
                std::thread::spawn(move || append(&root, &RunEvent::ProjectStateChanged).unwrap())
            })
            .collect::<Vec<_>>();
        let mut ids = workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>();
        ids.sort_unstable();
        assert_eq!(ids, (1..=8).collect::<Vec<_>>());
        assert_eq!(entries_after(&root, 0).unwrap().len(), 8);
        let _ = fs::remove_dir_all(root);
    }
}
