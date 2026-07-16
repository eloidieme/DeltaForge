use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::application::RunEvent;
use crate::fs_util::atomic_write;

const JOURNAL_FILE: &str = "workbench-events.json";
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
    let path = journal_path(project_root);
    let mut journal = read(&path)?;
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
}

pub fn entries_after(project_root: &Path, cursor: u64) -> Result<Vec<JournalEntry>> {
    Ok(read(&journal_path(project_root))?
        .events
        .into_iter()
        .filter(|entry| entry.id > cursor)
        .collect())
}

pub fn cursor(project_root: &Path) -> Result<u64> {
    let journal = read(&journal_path(project_root))?;
    Ok(journal.next_id.saturating_sub(1))
}

pub fn contains_source_revision(project_root: &Path, revision: u64) -> Result<bool> {
    Ok(read(&journal_path(project_root))?
        .events
        .iter()
        .any(|entry| {
            entry.event.get("type").and_then(Value::as_str) == Some("source_changed")
                && entry.event.get("revision").and_then(Value::as_u64) == Some(revision)
        }))
}

fn journal_path(project_root: &Path) -> PathBuf {
    project_root.join(".deltaforge").join(JOURNAL_FILE)
}

fn read(path: &Path) -> Result<Journal> {
    let source = match fs::read(path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Journal::default()),
        Err(error) => {
            return Err(error)
                .with_context(|| format!("failed to read event journal {}", path.display()));
        }
    };
    serde_json::from_slice(&source)
        .with_context(|| format!("failed to parse event journal {}", path.display()))
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
}
