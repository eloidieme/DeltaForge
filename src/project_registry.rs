use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::fs_util::atomic_write;

const REGISTRY_VERSION: u32 = 1;
const REGISTRY_FILE: &str = "projects.json";
const SERVICE_RECORD_FILE: &str = "workbench.json";
const START_LOCK_FILE: &str = "workbench-start.lock";

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
    if let Some(path) = env::var_os("DELTAFORGE_HOME") {
        let path = PathBuf::from(path);
        if path.as_os_str().is_empty() {
            bail!("DELTAFORGE_HOME must not be empty");
        }
        return Ok(path);
    }
    let home = env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .context("could not locate the user home directory; set DELTAFORGE_HOME")?;
    Ok(home.join(".deltaforge"))
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
    Ok(home)
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

    #[test]
    fn project_ids_are_stable_and_path_specific() {
        let first = project_id(Path::new("/tmp/example/flashindex"));
        assert_eq!(first, project_id(Path::new("/tmp/example/flashindex")));
        assert_ne!(first, project_id(Path::new("/tmp/other/flashindex")));
        assert!(first.starts_with("flashindex-"));
    }
}
