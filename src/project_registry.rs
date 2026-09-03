use std::env;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use fs2::FileExt;
use serde::{Deserialize, Serialize};

use crate::fs_util::atomic_write;

const REGISTRY_VERSION: u32 = 1;
const REGISTRY_FILE: &str = "projects.json";
const SERVICE_RECORD_FILE: &str = "workbench.json";
const START_LOCK_FILE: &str = "workbench-start.lock";
const REGISTRY_LOCK_FILE: &str = "projects.lock";

struct RegistryLease(File);

impl RegistryLease {
    fn acquire() -> Result<Self> {
        let home = ensure_private_application_home()?;
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(home.join(REGISTRY_LOCK_FILE))
            .context("failed to open the project-registry lock")?;
        file.lock_exclusive()
            .context("failed to lock the project registry")?;
        Ok(Self(file))
    }
}

impl Drop for RegistryLease {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.0);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RegisteredProject {
    pub id: String,
    pub path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packs_dir: Option<PathBuf>,
    pub last_opened_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RegistryFile {
    #[serde(default = "registry_version")]
    version: u32,
    #[serde(default)]
    projects: Vec<RegisteredProject>,
}

impl Default for RegistryFile {
    fn default() -> Self {
        Self {
            version: REGISTRY_VERSION,
            projects: Vec::new(),
        }
    }
}

pub fn application_home() -> Result<PathBuf> {
    application_home_for(current_platform(), |name| env::var_os(name))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Platform {
    LinuxOrUnix,
    MacOs,
    Windows,
}

fn current_platform() -> Platform {
    if cfg!(windows) {
        Platform::Windows
    } else if cfg!(target_os = "macos") {
        Platform::MacOs
    } else {
        Platform::LinuxOrUnix
    }
}

fn application_home_for(
    platform: Platform,
    mut get_env: impl FnMut(&str) -> Option<OsString>,
) -> Result<PathBuf> {
    if let Some(path) = get_env("DELTAFORGE_HOME") {
        let path = PathBuf::from(path);
        if path.as_os_str().is_empty() {
            bail!("DELTAFORGE_HOME must not be empty");
        }
        return Ok(path);
    }

    let nonempty = |value: Option<OsString>| value.filter(|value| !value.is_empty());
    match platform {
        Platform::Windows => {
            // LOCALAPPDATA is stable between PowerShell, cmd.exe and Git Bash.
            // HOME is deliberately not consulted here: Git Bash may synthesize
            // a different value for the same Windows account.
            if let Some(local) = nonempty(get_env("LOCALAPPDATA")) {
                return Ok(PathBuf::from(local).join("DeltaForge"));
            }
            let profile = nonempty(get_env("USERPROFILE"))
                .map(PathBuf::from)
                .context("could not locate Windows local application data; set DELTAFORGE_HOME or LOCALAPPDATA")?;
            Ok(profile.join("AppData").join("Local").join("DeltaForge"))
        }
        Platform::MacOs => {
            let home = nonempty(get_env("HOME"))
                .map(PathBuf::from)
                .context("could not locate the user home directory; set DELTAFORGE_HOME")?;
            Ok(home
                .join("Library")
                .join("Application Support")
                .join("DeltaForge"))
        }
        Platform::LinuxOrUnix => {
            if let Some(data) = nonempty(get_env("XDG_DATA_HOME")) {
                return Ok(PathBuf::from(data).join("deltaforge"));
            }
            let home = nonempty(get_env("HOME")).map(PathBuf::from).context(
                "could not locate the user data directory; set DELTAFORGE_HOME or XDG_DATA_HOME",
            )?;
            Ok(home.join(".local").join("share").join("deltaforge"))
        }
    }
}

pub fn ensure_private_application_home() -> Result<PathBuf> {
    let home = application_home()?;
    fs::create_dir_all(&home)
        .with_context(|| format!("failed to create DeltaForge home {}", home.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&home, fs::Permissions::from_mode(0o700))
            .with_context(|| format!("failed to secure DeltaForge home {}", home.display()))?;
    }
    #[cfg(windows)]
    secure_windows_application_home(&home)?;
    Ok(home)
}

/// Replace inherited Windows permissions with a protected DACL granting full
/// control to the directory owner, SYSTEM, and Administrators. Child files and
/// directories inherit the same policy, including the capability-bearing
/// `workbench.json` record.
#[cfg(windows)]
fn secure_windows_application_home(path: &Path) -> Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::{null, null_mut};
    use windows_sys::Win32::Foundation::{ERROR_SUCCESS, LocalFree};
    use windows_sys::Win32::Security::Authorization::{
        ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1, SE_FILE_OBJECT,
        SetNamedSecurityInfoW,
    };
    use windows_sys::Win32::Security::{
        ACL, DACL_SECURITY_INFORMATION, GetSecurityDescriptorDacl,
        PROTECTED_DACL_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR,
    };

    // P = protected DACL; OICI = inherited by files and subdirectories.
    // OW is the object's owner, SY is LocalSystem, BA is Administrators.
    let sddl: Vec<u16> = "D:P(A;OICI;FA;;;OW)(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)\0"
        .encode_utf16()
        .collect();
    let mut descriptor: PSECURITY_DESCRIPTOR = null_mut();
    // SAFETY: Windows owns the returned self-relative descriptor. All pointers
    // are valid for the duration of this function, and `LocalFree` releases it.
    if unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            sddl.as_ptr(),
            SDDL_REVISION_1,
            &mut descriptor,
            null_mut(),
        )
    } == 0
    {
        return Err(std::io::Error::last_os_error())
            .context("failed to build the DeltaForge application-directory ACL");
    }
    struct Descriptor(PSECURITY_DESCRIPTOR);
    impl Drop for Descriptor {
        fn drop(&mut self) {
            // SAFETY: the descriptor was allocated by
            // ConvertStringSecurityDescriptorToSecurityDescriptorW.
            unsafe { LocalFree(self.0) };
        }
    }
    let descriptor = Descriptor(descriptor);

    let mut dacl_present = 0;
    let mut dacl_defaulted = 0;
    let mut dacl: *mut ACL = null_mut();
    // SAFETY: `descriptor` remains alive and points to a valid security
    // descriptor; the output pointers refer to local variables.
    if unsafe {
        GetSecurityDescriptorDacl(
            descriptor.0,
            &mut dacl_present,
            &mut dacl,
            &mut dacl_defaulted,
        )
    } == 0
        || dacl_present == 0
        || dacl.is_null()
    {
        return Err(std::io::Error::last_os_error())
            .context("failed to read the DeltaForge application-directory ACL");
    }

    let path_display = path.display().to_string();
    let path: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    // SAFETY: the path and DACL remain valid and NUL-terminated for the call.
    let status = unsafe {
        SetNamedSecurityInfoW(
            path.as_ptr(),
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
            null_mut(),
            null_mut(),
            dacl,
            null(),
        )
    };
    if status != ERROR_SUCCESS {
        return Err(std::io::Error::from_raw_os_error(status as i32))
            .with_context(|| format!("failed to secure DeltaForge home {path_display}"));
    }
    Ok(())
}

pub fn service_record_path() -> Result<PathBuf> {
    Ok(application_home()?.join(SERVICE_RECORD_FILE))
}

pub fn startup_lock_path() -> Result<PathBuf> {
    Ok(application_home()?.join(START_LOCK_FILE))
}

pub fn register(root: &Path, packs_dir: Option<&Path>) -> Result<RegisteredProject> {
    let root = root
        .canonicalize()
        .with_context(|| format!("failed to canonicalize project {}", root.display()))?;
    ensure_project_root(&root)?;
    let _lease = RegistryLease::acquire()?;
    let mut registry = read_registry()?;
    let now = unix_millis();
    let id = project_id(&root);
    let packs_dir = packs_dir.map(Path::to_path_buf);
    let entry = RegisteredProject {
        id: id.clone(),
        path: root,
        packs_dir,
        last_opened_at: now,
    };
    if let Some(existing) = registry
        .projects
        .iter_mut()
        .find(|project| project.id == id)
    {
        *existing = entry.clone();
    } else {
        registry.projects.push(entry.clone());
    }
    registry
        .projects
        .sort_by_key(|project| std::cmp::Reverse(project.last_opened_at));
    write_registry(&registry)?;
    Ok(entry)
}

pub fn list() -> Result<Vec<RegisteredProject>> {
    let _lease = RegistryLease::acquire()?;
    let mut registry = read_registry()?;
    let original_len = registry.projects.len();
    registry
        .projects
        .retain(|project| ensure_project_root(&project.path).is_ok());
    if registry.projects.len() != original_len {
        write_registry(&registry)?;
    }
    Ok(registry.projects)
}

pub fn resolve(id: &str) -> Result<RegisteredProject> {
    let project = list()?
        .into_iter()
        .find(|project| project.id == id)
        .with_context(|| format!("unknown registered project {id}"))?;
    ensure_project_root(&project.path)?;
    Ok(project)
}

fn ensure_project_root(root: &Path) -> Result<()> {
    let state = root.join(".deltaforge").join("state.json");
    if !state.is_file() {
        bail!("{} is not a DeltaForge project", root.display());
    }
    Ok(())
}

fn read_registry() -> Result<RegistryFile> {
    let path = application_home()?.join(REGISTRY_FILE);
    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(RegistryFile::default());
        }
        Err(error) => return Err(error.into()),
    };
    let registry: RegistryFile = serde_json::from_str(&source)
        .with_context(|| format!("failed to read project registry {}", path.display()))?;
    if registry.version != REGISTRY_VERSION {
        bail!(
            "unsupported project registry version {} in {}",
            registry.version,
            path.display()
        );
    }
    Ok(registry)
}

fn write_registry(registry: &RegistryFile) -> Result<()> {
    let home = ensure_private_application_home()?;
    atomic_write(
        &home.join(REGISTRY_FILE),
        serde_json::to_string_pretty(registry)?,
    )
}

fn project_id(root: &Path) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in root.to_string_lossy().as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let slug = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project")
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    format!(
        "{}-{hash:016x}",
        if slug.is_empty() { "project" } else { &slug }
    )
}

fn unix_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

const fn registry_version() -> u32 {
    REGISTRY_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    fn locate(platform: Platform, values: &[(&str, &str)]) -> Result<PathBuf> {
        application_home_for(platform, |name| {
            values
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| OsString::from(value))
        })
    }

    #[test]
    fn project_ids_are_stable_and_path_specific() {
        let first = project_id(Path::new("/tmp/example/flashindex"));
        assert_eq!(first, project_id(Path::new("/tmp/example/flashindex")));
        assert_ne!(first, project_id(Path::new("/tmp/other/flashindex")));
        assert!(first.starts_with("flashindex-"));
    }

    #[test]
    fn application_home_uses_each_platforms_data_directory() {
        assert_eq!(
            locate(
                Platform::LinuxOrUnix,
                &[("XDG_DATA_HOME", "/data"), ("HOME", "/home/learner")]
            )
            .unwrap(),
            Path::new("/data/deltaforge")
        );
        assert_eq!(
            locate(Platform::LinuxOrUnix, &[("HOME", "/home/learner")]).unwrap(),
            Path::new("/home/learner/.local/share/deltaforge")
        );
        assert_eq!(
            locate(Platform::MacOs, &[("HOME", "/Users/learner")]).unwrap(),
            Path::new("/Users/learner/Library/Application Support/DeltaForge")
        );
        assert_eq!(
            locate(
                Platform::Windows,
                &[("LOCALAPPDATA", r"C:\Users\learner\AppData\Local")]
            )
            .unwrap(),
            Path::new(r"C:\Users\learner\AppData\Local").join("DeltaForge")
        );
    }

    #[test]
    fn windows_never_uses_git_bash_home_for_application_data() {
        let found = locate(
            Platform::Windows,
            &[
                ("HOME", r"C:\msys64\home\learner"),
                ("USERPROFILE", r"C:\Users\learner"),
            ],
        )
        .unwrap();
        assert_eq!(
            found,
            Path::new(r"C:\Users\learner")
                .join("AppData")
                .join("Local")
                .join("DeltaForge")
        );
    }

    #[test]
    fn explicit_application_home_wins_on_every_platform() {
        for platform in [Platform::LinuxOrUnix, Platform::MacOs, Platform::Windows] {
            assert_eq!(
                locate(
                    platform,
                    &[
                        ("DELTAFORGE_HOME", "/explicit"),
                        ("HOME", "/ignored"),
                        ("LOCALAPPDATA", "/also-ignored"),
                    ],
                )
                .unwrap(),
                Path::new("/explicit")
            );
        }
    }
}
