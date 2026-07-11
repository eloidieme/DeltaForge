use std::fs;
use std::path::{Component, Path};

use anyhow::{Context, Result, bail};

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

pub fn digest_tree(root: &Path, excluded_names: &[&str]) -> Result<String> {
    if !root.is_dir() {
        bail!("cannot digest missing directory {}", root.display());
    }
    let mut files = Vec::new();
    collect_files(root, root, excluded_names, &mut files)?;
    files.sort();

    let mut hash = FNV_OFFSET;
    for relative in files {
        update_hash(&mut hash, relative.to_string_lossy().as_bytes());
        update_hash(&mut hash, &[0]);
        let contents = fs::read(root.join(&relative)).with_context(|| {
            format!(
                "failed to read {} while computing integrity digest",
                relative.display()
            )
        })?;
        update_hash(&mut hash, &contents);
        update_hash(&mut hash, &[0xff]);
    }
    Ok(format!("fnv1a64:{hash:016x}"))
}

pub fn is_safe_relative_path(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn collect_files(
    root: &Path,
    current: &Path,
    excluded_names: &[&str],
    files: &mut Vec<std::path::PathBuf>,
) -> Result<()> {
    for entry in fs::read_dir(current)
        .with_context(|| format!("failed to read directory {}", current.display()))?
    {
        let entry = entry?;
        let name = entry.file_name();
        if current == root && excluded_names.iter().any(|excluded| name == *excluded) {
            continue;
        }
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_files(root, &path, excluded_names, files)?;
        } else if file_type.is_file() {
            files.push(path.strip_prefix(root)?.to_path_buf());
        } else {
            bail!(
                "integrity digest does not allow symlinks or special files: {}",
                path.display()
            );
        }
    }
    Ok(())
}

fn update_hash(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}
