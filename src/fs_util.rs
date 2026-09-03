use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};

pub fn lock_unavailable(error: &std::io::Error) -> bool {
    error.kind() == std::io::ErrorKind::WouldBlock
        || cfg!(windows) && matches!(error.raw_os_error(), Some(32 | 33))
}

/// Create a fresh, unpredictable, owner-only scratch directory under the
/// system temp directory. Unlike a plain `create_dir_all`, an existing path
/// (whether a stale leftover, a symlink another local user planted there, or
/// a directory a watcher pre-created to race the caller) is refused rather
/// than silently adopted, and 128 bits of randomness in the name make the
/// path impossible to predict from the pid and the clock alone. On Unix the
/// directory is created `0700` so other local accounts on a shared host
/// cannot read fixture content copied into it or anything a learner's
/// program writes there.
pub fn create_private_scratch_dir(prefix: &str) -> Result<PathBuf> {
    let mut random = [0u8; 16];
    getrandom::fill(&mut random)
        .map_err(|error| anyhow::anyhow!("operating system randomness is unavailable: {error}"))?;
    let suffix: String = random.iter().map(|byte| format!("{byte:02x}")).collect();
    let path = std::env::temp_dir().join(format!("{prefix}-{suffix}"));

    // `mut` only matters on Unix, where `DirBuilderExt::mode` takes `&mut
    // self`. Windows has no such call, so the binding is never mutated there
    // and `-D warnings` refuses the `mut`.
    #[cfg_attr(not(unix), allow(unused_mut))]
    let mut builder = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder
        .create(&path)
        .with_context(|| format!("failed to create scratch directory {}", path.display()))?;
    Ok(path)
}

/// Render a path the way a human should see it, stripping the
/// extended-length prefixes (`\\?\C:\...`, `\\?\UNC\server\share\...`)
/// `Path::canonicalize` adds on Windows. DeltaForge needs the prefixed form
/// for its own filesystem calls, but it is not a spelling a learner ever
/// typed, and it does not paste cleanly into most tools; child processes like
/// cargo print the ordinary form for the same path.
pub fn display_path(path: &Path) -> String {
    let native = path.to_string_lossy();
    if let Some(rest) = native.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{rest}")
    } else if let Some(rest) = native.strip_prefix(r"\\?\") {
        rest.to_string()
    } else {
        native.into_owned()
    }
}

pub fn atomic_write(path: &Path, contents: impl AsRef<[u8]>) -> Result<()> {
    atomic_write_with_mode(path, contents, false)
}

/// Atomically write a credential-bearing file. On Unix both the temporary
/// file and the installed file are owner-readable/writable only.
pub fn atomic_write_private(path: &Path, contents: impl AsRef<[u8]>) -> Result<()> {
    atomic_write_with_mode(path, contents, true)
}

fn atomic_write_with_mode(path: &Path, contents: impl AsRef<[u8]>, private: bool) -> Result<()> {
    #[cfg(windows)]
    let _ = private;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }

    let temp_path = temporary_path(path)?;
    {
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        if private {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options
            .open(&temp_path)
            .with_context(|| format!("failed to create temp file {}", temp_path.display()))?;
        file.write_all(contents.as_ref())
            .with_context(|| format!("failed to write temp file {}", temp_path.display()))?;
        file.sync_all()
            .with_context(|| format!("failed to sync temp file {}", temp_path.display()))?;
    }

    replace_file(&temp_path, path).with_context(|| {
        format!(
            "failed to rename temp file {} to {}",
            temp_path.display(),
            path.display()
        )
    })?;

    #[cfg(unix)]
    if private {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .with_context(|| format!("failed to secure {}", path.display()))?;
    }

    if let Some(parent) = path.parent()
        && let Ok(dir) = File::open(parent)
    {
        let _ = dir.sync_all();
    }

    Ok(())
}

#[cfg(not(windows))]
fn replace_file(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::rename(source, destination)
}

/// Read a file that some other process may be atomically replacing.
///
/// On Windows a handle opened without `FILE_SHARE_DELETE` blocks
/// `MoveFileEx`, so an ordinary reader makes a concurrent writer's replace fail
/// with "Access is denied". Every file DeltaForge writes through
/// [`atomic_write`] is read through here instead, so reading one can never be
/// the reason writing it failed.
///
/// On Unix a rename over an open file has always been fine, and this is a plain
/// read.
pub fn read_to_string_shared(path: &Path) -> std::io::Result<String> {
    #[cfg(windows)]
    {
        use std::io::Read as _;
        use std::os::windows::fs::OpenOptionsExt as _;
        const FILE_SHARE_READ: u32 = 0x1;
        const FILE_SHARE_WRITE: u32 = 0x2;
        const FILE_SHARE_DELETE: u32 = 0x4;
        let mut file = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
            .open(path)?;
        let mut source = String::new();
        file.read_to_string(&mut source)?;
        Ok(source)
    }
    #[cfg(not(windows))]
    {
        fs::read_to_string(path)
    }
}

#[cfg(windows)]
fn replace_file(source: &Path, destination: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn MoveFileExW(existing: *const u16, replacement: *const u16, flags: u32) -> i32;
    }
    const MOVEFILE_REPLACE_EXISTING: u32 = 0x1;
    const MOVEFILE_WRITE_THROUGH: u32 = 0x8;
    let source = source
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let destination = destination
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    // A replace can be refused by a handle DeltaForge does not control — an
    // antivirus scanner or the search indexer opening the file it just saw
    // change. Those handles are held for milliseconds, so a short retry turns
    // a spurious failure into a small delay. `read_to_string_shared` removes
    // the case DeltaForge causes itself; this covers the rest.
    const ERROR_ACCESS_DENIED: i32 = 5;
    const ERROR_SHARING_VIOLATION: i32 = 32;
    let mut backoff = std::time::Duration::from_millis(2);
    for attempt in 0..6 {
        let result = unsafe {
            MoveFileExW(
                source.as_ptr(),
                destination.as_ptr(),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
            )
        };
        if result != 0 {
            return Ok(());
        }
        let error = std::io::Error::last_os_error();
        let transient = matches!(
            error.raw_os_error(),
            Some(ERROR_ACCESS_DENIED) | Some(ERROR_SHARING_VIOLATION)
        );
        if !transient || attempt == 5 {
            return Err(error);
        }
        std::thread::sleep(backoff);
        backoff *= 2;
    }
    unreachable!("the loop returns on its last attempt")
}

fn temporary_path(path: &Path) -> Result<PathBuf> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("deltaforge");
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?
        .as_nanos();
    Ok(parent.join(format!(".{file_name}.{}.{}.tmp", std::process::id(), nanos)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_path_strips_extended_length_windows_prefixes() {
        assert_eq!(
            display_path(Path::new(r"\\?\C:\Users\learner\ws\project")),
            r"C:\Users\learner\ws\project"
        );
        assert_eq!(
            display_path(Path::new(r"\\?\UNC\server\share\project")),
            r"\\server\share\project"
        );
    }

    #[test]
    fn display_path_leaves_ordinary_paths_alone() {
        assert_eq!(
            display_path(Path::new("/home/learner/ws/project")),
            "/home/learner/ws/project"
        );
        assert_eq!(
            display_path(Path::new(r"C:\Users\learner")),
            r"C:\Users\learner"
        );
    }
}
