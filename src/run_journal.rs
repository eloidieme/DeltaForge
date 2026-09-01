use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::application::RunEvent;
use crate::fs_util::atomic_write;

const JOURNAL_FILE: &str = "workbench-events.jsonl";
const JOURNAL_META_FILE: &str = "workbench-events-meta.json";
const JOURNAL_LOCK_FILE: &str = "workbench-events.lock";
const QUARANTINE_PREFIX: &str = "workbench-events.corrupt-";
/// How many quarantined copies of a corrupt journal to keep. Enough to
/// diagnose a recurring corruption, bounded so it cannot accumulate forever.
const MAX_QUARANTINED_JOURNALS: usize = 5;
const MAX_EVENTS: usize = 256;
const MAX_BYTES: usize = 2 * 1024 * 1024;
const MAX_STRING_BYTES: usize = 16 * 1024;
/// Compact once the append-only file crosses either bound, rather than on
/// every append: the O(events) rewrite this requires is then amortized over
/// many appends (each compaction pays for roughly the appends since the last
/// one) instead of paid in full by every single one.
const COMPACTION_TRIGGER_BYTES: u64 = MAX_BYTES as u64 * 2;
const COMPACTION_EVENT_TRIGGER: u64 = MAX_EVENTS as u64 * 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: u64,
    pub event: Value,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct JournalMeta {
    #[serde(default)]
    next_id: u64,
    #[serde(default)]
    event_count: u64,
}

/// Append one event to the journal. In the common case this costs one small,
/// non-atomic write to a tiny metadata file (to allocate the id) plus one
/// `O_APPEND` write of the new line — no read, no rewrite, and no fsync of
/// prior events. The journal is compacted back down to
/// `MAX_EVENTS`/`MAX_BYTES` only occasionally (see `COMPACTION_EVENT_TRIGGER`
/// / `COMPACTION_TRIGGER_BYTES`), not on every call, so the cost of one
/// append no longer scales with how much output a run has produced so far.
///
/// Losing the last write to either file (a hard crash, not a clean process
/// exit) can drop the most recent event or two, or force a one-time rescan to
/// recover the id counter; it can never hand out an id that collides with one
/// already on disk. That is an acceptable trade for a live progress stream
/// that is not the source of truth for anything durable (that role belongs to
/// `state.json` and Git history).
pub fn append(project_root: &Path, event: &RunEvent) -> Result<u64> {
    with_journal_lock(project_root, || {
        let path = journal_path(project_root);
        let meta_path = journal_meta_path(project_root);

        let quarantined = ensure_appendable(&path)?;
        let mut meta = read_meta_unlocked(&meta_path, &path)?;
        if quarantined {
            meta.event_count = 0;
        }

        let id = meta.next_id.max(1);
        meta.next_id = id.saturating_add(1);
        meta.event_count = meta.event_count.saturating_add(1);

        let mut value = serde_json::to_value(event)?;
        truncate_value(&mut value);
        let entry = JournalEntry { id, event: value };
        let mut line = serde_json::to_vec(&entry)?;
        line.push(b'\n');
        append_line(&path, &line)?;

        let bytes = fs::metadata(&path)
            .map(|metadata| metadata.len())
            .unwrap_or(0);
        if meta.event_count > COMPACTION_EVENT_TRIGGER || bytes > COMPACTION_TRIGGER_BYTES {
            meta.event_count = compact(&path)? as u64;
        }

        write_meta_unlocked(&meta_path, &meta)?;
        Ok(id)
    })
}

pub fn entries_after(project_root: &Path, cursor: u64) -> Result<Vec<JournalEntry>> {
    with_journal_lock(project_root, || {
        Ok(read_entries_unlocked(&journal_path(project_root))?
            .into_iter()
            .filter(|entry| entry.id > cursor)
            .collect())
    })
}

pub fn cursor(project_root: &Path) -> Result<u64> {
    with_journal_lock(project_root, || {
        let path = journal_path(project_root);
        Ok(read_meta_unlocked(&journal_meta_path(project_root), &path)?
            .next_id
            .saturating_sub(1))
    })
}

pub fn contains_source_revision(project_root: &Path, revision: u64) -> Result<bool> {
    with_journal_lock(project_root, || {
        Ok(read_entries_unlocked(&journal_path(project_root))?
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

fn journal_meta_path(project_root: &Path) -> PathBuf {
    project_root.join(".deltaforge").join(JOURNAL_META_FILE)
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

/// If the journal file exists and its last byte is not a newline, an earlier
/// write was torn (or the file predates the append-only format), so appending
/// to it verbatim would corrupt the new line onto the old tail. Quarantine it
/// and report that the caller is starting from empty. Costs one `stat` plus a
/// one-byte read in the common (already-clean) case.
fn ensure_appendable(path: &Path) -> Result<bool> {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.into()),
    };
    if metadata.len() == 0 {
        return Ok(false);
    }
    let mut file = File::open(path)
        .with_context(|| format!("failed to open event journal {}", path.display()))?;
    file.seek(SeekFrom::End(-1))?;
    let mut last = [0u8; 1];
    file.read_exact(&mut last)?;
    if last[0] == b'\n' {
        return Ok(false);
    }
    quarantine(path)?;
    Ok(true)
}

/// Append one already-newline-terminated line to the journal file without
/// reading or rewriting anything already in it. Not fsynced: this is a live
/// progress stream, not durable state, and paying for a sync on every line
/// would reintroduce the per-event cost this format change removes.
fn append_line(path: &Path, line: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let is_new = !path.exists();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed to open event journal {}", path.display()))?;
    file.write_all(line)
        .with_context(|| format!("failed to append to event journal {}", path.display()))?;
    if is_new
        && let Some(parent) = path.parent()
        && let Ok(dir) = File::open(parent)
    {
        let _ = dir.sync_all();
    }
    Ok(())
}

/// Write the small metadata file directly (no temp file, no fsync): losing
/// this write recovers cleanly via the journal rescan in `read_meta_unlocked`
/// below, so paying for atomic-write durability on every append is not worth
/// it here.
fn write_meta_unlocked(path: &Path, meta: &JournalMeta) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_vec(meta)?)
        .with_context(|| format!("failed to write event journal metadata {}", path.display()))
}

fn read_meta_unlocked(meta_path: &Path, journal_path: &Path) -> Result<JournalMeta> {
    if let Ok(source) = fs::read(meta_path)
        && let Ok(meta) = serde_json::from_slice::<JournalMeta>(&source)
    {
        return Ok(meta);
    }
    // Metadata is missing or unreadable (first run, or a crash lost the last
    // write to it). Recover a safe id counter from the journal itself so a
    // reset to 0 can never hand out an id that already exists on disk.
    let entries = read_entries_unlocked(journal_path)?;
    let next_id = entries
        .iter()
        .map(|entry| entry.id)
        .max()
        .map_or(0, |max| max + 1);
    Ok(JournalMeta {
        next_id,
        event_count: entries.len() as u64,
    })
}

/// Read and parse every line of the journal file. A file that fails to parse
/// line-by-line (garbage, or a truncated write `ensure_appendable` did not
/// catch) is quarantined and treated as empty, the same recovery
/// `deltaforge` has always applied to a corrupt journal.
fn read_entries_unlocked(path: &Path) -> Result<Vec<JournalEntry>> {
    let source = match fs::read(path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(error)
                .with_context(|| format!("failed to read event journal {}", path.display()));
        }
    };
    match parse_lines(&source) {
        Some(entries) => Ok(entries),
        None => {
            quarantine(path)?;
            Ok(Vec::new())
        }
    }
}

fn quarantine(path: &Path) -> Result<()> {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let backup = path.with_file_name(format!("{QUARANTINE_PREFIX}{stamp}.json"));
    fs::rename(path, &backup).with_context(|| {
        format!(
            "failed to quarantine corrupt event journal {}",
            path.display()
        )
    })?;
    // Quarantined journals are diagnostic keepsakes, not durable state. Without
    // a cap, a repeatedly corrupted journal leaves one file per occurrence in
    // `.deltaforge/` forever.
    prune_quarantined(path);
    Ok(())
}

/// Keep only the `MAX_QUARANTINED_JOURNALS` most recent quarantine files.
/// Best-effort: failing to prune must never turn into a failure to recover the
/// journal, which is the operation the learner is actually waiting on.
fn prune_quarantined(path: &Path) {
    let Some(dir) = path.parent() else { return };
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut quarantined = entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            // Order by the parsed stamp, not the filename: string ordering
            // would put a shorter number after a longer one and drop the
            // wrong file.
            let stamp = name
                .strip_prefix(QUARANTINE_PREFIX)?
                .strip_suffix(".json")?
                .parse::<u128>()
                .ok()?;
            Some((stamp, entry.path()))
        })
        .collect::<Vec<_>>();
    if quarantined.len() <= MAX_QUARANTINED_JOURNALS {
        return;
    }
    quarantined.sort_by_key(|(stamp, _)| *stamp);
    let excess = quarantined.len() - MAX_QUARANTINED_JOURNALS;
    for (_, stale) in quarantined.into_iter().take(excess) {
        let _ = fs::remove_file(stale);
    }
}

fn parse_lines(source: &[u8]) -> Option<Vec<JournalEntry>> {
    let mut entries = Vec::new();
    for line in source.split(|byte| *byte == b'\n') {
        if line.is_empty() {
            continue;
        }
        entries.push(serde_json::from_slice::<JournalEntry>(line).ok()?);
    }
    Some(entries)
}

/// Trim the journal back down to `MAX_EVENTS`/`MAX_BYTES` and rewrite it,
/// fsync-ing the result since this is the (infrequent) point at which the
/// journal's on-disk content is condensed. Returns the number of entries kept.
fn compact(path: &Path) -> Result<usize> {
    let source = match fs::read(path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(error) => {
            return Err(error)
                .with_context(|| format!("failed to read event journal {}", path.display()));
        }
    };
    let Some(mut entries) = parse_lines(&source) else {
        // A concurrent read will quarantine this; nothing to compact.
        return Ok(0);
    };
    if entries.len() > MAX_EVENTS {
        let excess = entries.len() - MAX_EVENTS;
        entries.drain(..excess);
    }
    let mut lines: Vec<Vec<u8>> = entries
        .iter()
        .map(|entry| {
            let mut line = serde_json::to_vec(entry)?;
            line.push(b'\n');
            Ok(line)
        })
        .collect::<Result<_>>()?;
    while lines.len() > 1 && lines.iter().map(Vec::len).sum::<usize>() > MAX_BYTES {
        lines.remove(0);
    }
    let kept = lines.len();
    let mut buffer = Vec::with_capacity(lines.iter().map(Vec::len).sum());
    for line in lines {
        buffer.extend_from_slice(&line);
    }
    atomic_write(path, buffer)?;
    Ok(kept)
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
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let sequence = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        std::env::temp_dir().join(format!(
            "deltaforge-journal-{}-{}-{sequence}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn journal_compacts_and_supports_cursor_replay() {
        let root = temp_root();
        let total = COMPACTION_EVENT_TRIGGER as usize + 8;
        for index in 0..total {
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
        assert!(
            entries.len() <= COMPACTION_EVENT_TRIGGER as usize,
            "enough appends to cross the compaction trigger must trim back down, got {}",
            entries.len()
        );
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

    /// A journal that keeps getting corrupted must not leave one quarantine
    /// file per occurrence behind forever.
    #[test]
    fn quarantined_journals_are_capped_keeping_the_newest() {
        let root = temp_root();
        fs::create_dir_all(root.join(".deltaforge")).unwrap();
        let quarantine_stamps = || {
            let mut stamps = fs::read_dir(root.join(".deltaforge"))
                .unwrap()
                .flatten()
                .filter_map(|entry| {
                    entry
                        .file_name()
                        .into_string()
                        .ok()?
                        .strip_prefix(QUARANTINE_PREFIX)?
                        .strip_suffix(".json")?
                        .parse::<u128>()
                        .ok()
                })
                .collect::<Vec<_>>();
            stamps.sort_unstable();
            stamps
        };

        for _ in 0..MAX_QUARANTINED_JOURNALS + 4 {
            fs::write(journal_path(&root), "not json").unwrap();
            append(&root, &RunEvent::ProjectStateChanged).unwrap();
        }
        let stamps = quarantine_stamps();
        assert_eq!(
            stamps.len(),
            MAX_QUARANTINED_JOURNALS,
            "quarantine files must be capped, got {stamps:?}"
        );

        // The survivors must be the newest: keeping an older stamp over a newer
        // one would discard the evidence closest to the failure.
        fs::write(journal_path(&root), "not json").unwrap();
        append(&root, &RunEvent::ProjectStateChanged).unwrap();
        let after = quarantine_stamps();
        assert_eq!(after.len(), MAX_QUARANTINED_JOURNALS);
        assert!(
            !after.contains(&stamps[0]),
            "the oldest quarantine file must be the one dropped"
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn a_crash_that_loses_the_metadata_file_cannot_reuse_an_existing_id() {
        let root = temp_root();
        for _ in 0..5 {
            append(&root, &RunEvent::ProjectStateChanged).unwrap();
        }
        // Simulate losing the metadata file to a crash between writes: the
        // journal file (with ids 1..=5) survives, the counter does not.
        fs::remove_file(journal_meta_path(&root)).unwrap();
        let id = append(&root, &RunEvent::ProjectStateChanged).unwrap();
        assert_eq!(
            id, 6,
            "recovery must continue past the highest id already on disk"
        );
        let entries = entries_after(&root, 0).unwrap();
        let ids: Vec<u64> = entries.iter().map(|entry| entry.id).collect();
        let mut unique = ids.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), ids.len(), "no id may repeat: {ids:?}");
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

    #[test]
    fn appends_do_not_rewrite_prior_events() {
        // Regression guard for the O(events) rewrite this file used to do on
        // every append: within one compaction window, appends must cost
        // roughly the same regardless of how much has already been written.
        // Compared against a baseline (rather than an absolute wall-clock
        // bound) so this stays meaningful under whatever load happens to be
        // on the machine running it: the old O(events) rewrite made each
        // batch many times slower than the last, not merely a bit slower.
        let root = temp_root();
        let append_batch = |root: &Path, offset: usize| {
            let start = std::time::Instant::now();
            for index in 0..200 {
                append(
                    root,
                    &RunEvent::BuildOutput {
                        stream: "stdout",
                        text: format!("line {}", offset + index),
                    },
                )
                .unwrap();
            }
            start.elapsed()
        };
        let baseline = append_batch(&root, 0);
        for index in 0..2000 {
            append(
                &root,
                &RunEvent::BuildOutput {
                    stream: "stdout",
                    text: format!("padding line {index} {}", "x".repeat(200)),
                },
            )
            .unwrap();
        }
        let after = append_batch(&root, 10_000);
        assert!(
            after < baseline * 10 + std::time::Duration::from_millis(200),
            "200 appends cost {after:?} after a large journal vs {baseline:?} on an empty one; \
             expected the cost per append not to grow with journal size"
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn compaction_trims_to_bounds_and_keeps_ids_contiguous() {
        let root = temp_root();
        // Force several compactions by writing enough events to cross
        // COMPACTION_EVENT_TRIGGER (and COMPACTION_TRIGGER_BYTES) repeatedly.
        for index in 0..4000 {
            append(
                &root,
                &RunEvent::BuildOutput {
                    stream: "stdout",
                    text: format!("line {index} {}", "x".repeat(300)),
                },
            )
            .unwrap();
        }
        let entries = entries_after(&root, 0).unwrap();
        assert!(entries.len() <= COMPACTION_EVENT_TRIGGER as usize);
        for pair in entries.windows(2) {
            assert_eq!(
                pair[1].id,
                pair[0].id + 1,
                "surviving ids must stay contiguous"
            );
        }
        assert_eq!(super::cursor(&root).unwrap(), 4000);
        let _ = fs::remove_dir_all(root);
    }
}
