use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

const LEASE_FILE: &str = "run.lock";

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LeaseRecord {
    pid: u32,
}

pub struct RunLease {
    path: PathBuf,
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
        for _ in 0..2 {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let lease = Self { path: path.clone() };
                    let record = serde_json::to_vec(&LeaseRecord {
                        pid: std::process::id(),
                    })?;
                    file.write_all(&record)?;
                    file.sync_all()?;
                    return Ok(Some(lease));
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    if active(project_root) {
                        return Ok(None);
                    }
                }
                Err(error) => {
                    return Err(error).with_context(|| {
                        format!("failed to acquire run lease {}", path.display())
                    });
                }
            }
        }
        Ok(None)
    }
}

impl Drop for RunLease {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

pub fn active(project_root: &Path) -> bool {
    let path = lease_path(project_root);
    let record = fs::read(&path)
        .ok()
        .and_then(|source| serde_json::from_slice::<LeaseRecord>(&source).ok());
    match record {
        Some(record) if process_is_alive(record.pid) => true,
        Some(_) => {
            let _ = fs::remove_file(path);
            false
        }
        None if path.exists() && recently_created(&path) => true,
        None if path.exists() => {
            let _ = fs::remove_file(path);
            false
        }
        None => false,
    }
}

fn recently_created(path: &Path) -> bool {
    path.metadata()
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|modified| modified.elapsed().ok())
        .is_some_and(|elapsed| elapsed < std::time::Duration::from_secs(5))
}

fn lease_path(project_root: &Path) -> PathBuf {
    project_root.join(".deltaforge").join(LEASE_FILE)
}

#[cfg(unix)]
pub(crate) fn process_is_alive(pid: u32) -> bool {
    unsafe extern "C" {
        fn kill(pid: i32, signal: i32) -> i32;
    }
    i32::try_from(pid)
        .ok()
        .is_some_and(|pid| unsafe { kill(pid, 0) } == 0)
}

#[cfg(windows)]
pub(crate) fn process_is_alive(pid: u32) -> bool {
    const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
    const STILL_ACTIVE: u32 = 259;
    unsafe extern "system" {
        fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void;
        fn GetExitCodeProcess(process: *mut std::ffi::c_void, code: *mut u32) -> i32;
        fn CloseHandle(handle: *mut std::ffi::c_void) -> i32;
    }
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
    if handle.is_null() {
        return false;
    }
    let mut code = 0;
    let result = unsafe { GetExitCodeProcess(handle, &mut code) };
    let _ = unsafe { CloseHandle(handle) };
    result != 0 && code == STILL_ACTIVE
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn process_is_alive(pid: u32) -> bool {
    pid == std::process::id()
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
