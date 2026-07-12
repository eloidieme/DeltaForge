use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

/// Digest pack content. Pack behavior must be self-contained and reproducible,
/// so symlinks and special files are rejected outright: a symlinked tests.yaml
/// or fixture would let the pack's effective behavior change while the
/// recorded digest stayed the same, defeating pinning.
pub fn digest_pack_tree(root: &Path) -> Result<String> {
    let entries = collect_tree(root, &[], SymlinkPolicy::Reject)?;
    hash_entries(root, entries)
}

/// Digest a learner project. Generated directories are excluded by name at any
/// depth. Symlinks to files are hashed as link path + target path + target
/// contents so the digest tracks what the toolchain actually reads; symlinks
/// to directories are rejected with an actionable error, and non-symlink
/// special files (sockets, fifos) are skipped since they are not program
/// sources.
pub fn digest_project_tree(root: &Path, excluded_names: &[&str]) -> Result<String> {
    let entries = collect_tree(root, excluded_names, SymlinkPolicy::HashFileTargets)?;
    hash_entries(root, entries)
}

/// Collect `(relative path with forward slashes, contents)` for every file in
/// a tree, rejecting symlinks and special files. Used for stage behavioral
/// digests, where fixture content must be self-contained.
pub fn strict_tree_contents(root: &Path) -> Result<Vec<(String, Vec<u8>)>> {
    let entries = collect_tree(root, &[], SymlinkPolicy::Reject)?;
    entries
        .into_iter()
        .map(|entry| {
            let contents = fs::read(root.join(&entry.relative)).with_context(|| {
                format!(
                    "failed to read {} while computing integrity digest",
                    entry.relative.display()
                )
            })?;
            let name = entry.relative.to_string_lossy().replace('\\', "/");
            Ok((name, contents))
        })
        .collect()
}

/// Digest a set of `(relative name, contents)` pairs using the same FNV-1a
/// scheme (and framing) as the tree digests. Names are sorted for determinism
/// so the same content always yields the same digest regardless of iteration
/// order.
pub fn digest_named_contents(mut entries: Vec<(String, Vec<u8>)>) -> String {
    entries.sort_by(|left, right| left.0.cmp(&right.0));

    let mut hash = FNV_OFFSET;
    for (name, contents) in entries {
        update_hash(&mut hash, name.as_bytes());
        update_hash(&mut hash, &[0]);
        update_hash(&mut hash, &contents);
        update_hash(&mut hash, &[0xff]);
    }
    format!("fnv1a64:{hash:016x}")
}

pub fn is_safe_relative_path(path: &Path) -> bool {
    let text = path.to_string_lossy();
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && !text.starts_with(['/', '\\'])
        && !text.contains(':')
        && text
            .split(['/', '\\'])
            .all(|component| !component.is_empty() && component != "." && component != "..")
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SymlinkPolicy {
    /// Fail on any symlink or special file (pack content).
    Reject,
    /// Hash file symlinks (link path + target path + target contents), reject
    /// directory symlinks, skip other special files (learner projects).
    HashFileTargets,
}

#[derive(Debug)]
struct TreeEntry {
    relative: PathBuf,
    /// `Some(target)` when the entry is a symlink to a file.
    symlink_target: Option<PathBuf>,
}

fn collect_tree(
    root: &Path,
    excluded_names: &[&str],
    policy: SymlinkPolicy,
) -> Result<Vec<TreeEntry>> {
    if !root.is_dir() {
        bail!("cannot digest missing directory {}", root.display());
    }
    let mut entries = Vec::new();
    collect_into(root, root, excluded_names, policy, &mut entries)?;
    entries.sort_by(|left, right| left.relative.cmp(&right.relative));
    Ok(entries)
}

fn collect_into(
    root: &Path,
    current: &Path,
    excluded_names: &[&str],
    policy: SymlinkPolicy,
    entries: &mut Vec<TreeEntry>,
) -> Result<()> {
    for entry in fs::read_dir(current)
        .with_context(|| format!("failed to read directory {}", current.display()))?
    {
        let entry = entry?;
        let name = entry.file_name();
        if excluded_names.iter().any(|excluded| name == *excluded) {
            continue;
        }
        let path = entry.path();
        // Does not follow symlinks, so symlinked directories cannot smuggle
        // external trees into (or out of) the digest via recursion.
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_into(root, &path, excluded_names, policy, entries)?;
        } else if file_type.is_file() {
            entries.push(TreeEntry {
                relative: path.strip_prefix(root)?.to_path_buf(),
                symlink_target: None,
            });
        } else if file_type.is_symlink() {
            collect_symlink(root, &path, policy, entries)?;
        } else if policy == SymlinkPolicy::Reject {
            bail!(
                "pack content must be self-contained: {} is a special file",
                path.display()
            );
        }
        // Learner mode ignores other special files (sockets, fifos, devices);
        // they are not program sources.
    }
    Ok(())
}

fn collect_symlink(
    root: &Path,
    path: &Path,
    policy: SymlinkPolicy,
    entries: &mut Vec<TreeEntry>,
) -> Result<()> {
    if policy == SymlinkPolicy::Reject {
        bail!(
            "pack content must be self-contained: {} is a symbolic link. Replace it with a regular copy of its target.",
            path.display()
        );
    }

    let metadata = fs::metadata(path); // follows the link
    match metadata {
        Ok(metadata) if metadata.is_file() => {
            let target = fs::read_link(path)
                .with_context(|| format!("failed to read symbolic link {}", path.display()))?;
            entries.push(TreeEntry {
                relative: path.strip_prefix(root)?.to_path_buf(),
                symlink_target: Some(target),
            });
            Ok(())
        }
        Ok(_) => bail!(
            "cannot create an integrity digest: {} is a symbolic link to a directory.\nCopy the directory into the project, remove the link, or add its name to integrity.exclude in .deltaforge/config.toml.",
            path.display()
        ),
        Err(error) => Err(error).with_context(|| {
            format!(
                "cannot create an integrity digest: {} is a symbolic link with an unreadable target.\nRemove or repair the link.",
                path.display()
            )
        }),
    }
}

fn hash_entries(root: &Path, entries: Vec<TreeEntry>) -> Result<String> {
    let mut hash = FNV_OFFSET;
    for entry in entries {
        update_hash(&mut hash, entry.relative.to_string_lossy().as_bytes());
        // Regular files keep the historical `path \0 contents \xff` framing so
        // digests of symlink-free trees are stable across versions. Symlinks
        // use a distinct separator plus the target path so relinking is
        // detected even when the new target has identical contents.
        if let Some(target) = &entry.symlink_target {
            update_hash(&mut hash, &[1]);
            update_hash(&mut hash, target.to_string_lossy().as_bytes());
        }
        update_hash(&mut hash, &[0]);
        let contents = fs::read(root.join(&entry.relative)).with_context(|| {
            format!(
                "failed to read {} while computing integrity digest",
                entry.relative.display()
            )
        })?;
        update_hash(&mut hash, &contents);
        update_hash(&mut hash, &[0xff]);
    }
    Ok(format!("fnv1a64:{hash:016x}"))
}

fn update_hash(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_relative_paths_are_portable() {
        assert!(is_safe_relative_path(Path::new("fixtures/basic/input.txt")));
        assert!(!is_safe_relative_path(Path::new("../escape")));
        assert!(!is_safe_relative_path(Path::new("..\\escape")));
        assert!(!is_safe_relative_path(Path::new("fixtures/./input.txt")));
        assert!(!is_safe_relative_path(Path::new("C:\\escape")));
        assert!(!is_safe_relative_path(Path::new("")));
    }

    fn temp_tree(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "deltaforge-integrity-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn excluded_names_apply_at_any_depth() {
        let root = temp_tree("exclusions");
        fs::create_dir_all(root.join("src/target")).unwrap();
        fs::write(root.join("src/main.rs"), "fn main() {}").unwrap();
        fs::write(root.join("src/target/cache.o"), "junk").unwrap();

        let with_junk = digest_project_tree(&root, &["target"]).unwrap();
        fs::write(root.join("src/target/cache.o"), "different junk").unwrap();
        let with_changed_junk = digest_project_tree(&root, &["target"]).unwrap();

        assert_eq!(with_junk, with_changed_junk);
        let _ = fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn pack_digest_rejects_symlinks() {
        let root = temp_tree("pack-symlink");
        fs::write(root.join("real.txt"), "content").unwrap();
        std::os::unix::fs::symlink(root.join("real.txt"), root.join("link.txt")).unwrap();

        let error = digest_pack_tree(&root).unwrap_err();
        assert!(format!("{error:#}").contains("self-contained"));
        let _ = fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn project_digest_tracks_file_symlink_targets() {
        let root = temp_tree("project-symlink");
        let external = temp_tree("project-symlink-external");
        fs::write(external.join("shared.rs"), "code A").unwrap();
        fs::write(root.join("lib.rs"), "local").unwrap();
        std::os::unix::fs::symlink(external.join("shared.rs"), root.join("main.rs")).unwrap();

        let before = digest_project_tree(&root, &[]).unwrap();
        fs::write(external.join("shared.rs"), "code B").unwrap();
        let after = digest_project_tree(&root, &[]).unwrap();
        assert_ne!(
            before, after,
            "editing a symlink target must change the project digest"
        );

        // Relinking to a different target with identical contents also changes
        // the digest because the target path is hashed.
        fs::write(external.join("other.rs"), "code B").unwrap();
        fs::remove_file(root.join("main.rs")).unwrap();
        std::os::unix::fs::symlink(external.join("other.rs"), root.join("main.rs")).unwrap();
        let relinked = digest_project_tree(&root, &[]).unwrap();
        assert_ne!(after, relinked);

        let _ = fs::remove_dir_all(root);
        let _ = fs::remove_dir_all(external);
    }

    #[cfg(unix)]
    #[test]
    fn project_digest_rejects_directory_symlinks_with_guidance() {
        let root = temp_tree("project-dir-symlink");
        let external = temp_tree("project-dir-symlink-external");
        fs::write(external.join("data.txt"), "x").unwrap();
        std::os::unix::fs::symlink(&external, root.join("vendor")).unwrap();

        let error = digest_project_tree(&root, &[]).unwrap_err();
        let message = format!("{error:#}");
        assert!(message.contains("symbolic link to a directory"));
        assert!(message.contains("integrity.exclude"));

        // Excluding the link by name makes the digest succeed.
        assert!(digest_project_tree(&root, &["vendor"]).is_ok());

        let _ = fs::remove_dir_all(root);
        let _ = fs::remove_dir_all(external);
    }
}
