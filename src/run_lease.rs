use std::fs::{self, File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use fs2::FileExt;
use serde::{Deserialize, Serialize};

const LEASE_FILE: &str = "run.lock";

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LeaseRecord {
    pid: u32,
}

pub struct RunLease {
    file: File,
}

impl RunLease {
    pub fn acquire(project_root: &Path) -> Result<Self> {
        Self::try_acquire(project_root)?
            .ok_or_else(|| anyhow::anyhow!("another DeltaForge check run is already active"))
    }

    pub fn acquire_with_timeout(project_root: &Path, timeout: std::time::Duration) -> Result<Self> {
        let deadline = std::time::Instant::now() + timeout;
        loop {
            if let Some(lease) = Self::try_acquire(project_root)? {
                return Ok(lease);
            }
            if std::time::Instant::now() >= deadline {
                bail!("another DeltaForge check run is already active");
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn try_acquire(project_root: &Path) -> Result<Option<Self>> {
        let path = lease_path(project_root);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .with_context(|| format!("failed to open run lease {}", path.display()))?;
        match file.try_lock_exclusive() {
            Ok(()) => {
                file.set_len(0)?;
                file.seek(SeekFrom::Start(0))?;
                let record = serde_json::to_vec(&LeaseRecord {
                    pid: std::process::id(),
                })?;
                file.write_all(&record)?;
                file.sync_all()?;
                Ok(Some(Self { file }))
            }
            Err(error) if crate::fs_util::lock_unavailable(&error) => Ok(None),
            Err(error) => {
                Err(error).with_context(|| format!("failed to lock run lease {}", path.display()))
            }
        }
    }
}

impl Drop for RunLease {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.file);
    }
}

pub fn active(project_root: &Path) -> bool {
    let path = lease_path(project_root);
    let Ok(file) = OpenOptions::new().read(true).write(true).open(path) else {
        return false;
    };
    match file.try_lock_exclusive() {
        Ok(()) => {
            let _ = FileExt::unlock(&file);
            false
        }
        Err(error) if crate::fs_util::lock_unavailable(&error) => true,
        Err(_) => true,
    }
}

fn lease_path(project_root: &Path) -> PathBuf {
    project_root.join(".deltaforge").join(LEASE_FILE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_root(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "deltaforge-lease-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn only_one_live_lease_is_allowed() {
        let root = temp_root("exclusive");
        let lease = RunLease::acquire(&root).unwrap();
        assert!(active(&root));
        assert!(RunLease::acquire(&root).is_err());
        drop(lease);
        assert!(!active(&root));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn bounded_acquisition_waits_for_a_short_lived_lease() {
        let root = temp_root("bounded-wait");
        let lease = RunLease::acquire(&root).unwrap();
        let release = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(50));
            drop(lease);
        });

        let next = RunLease::acquire_with_timeout(&root, std::time::Duration::from_secs(1))
            .expect("the released lease should be acquired before the deadline");
        release.join().unwrap();
        drop(next);
        let _ = fs::remove_dir_all(root);
    }
}
