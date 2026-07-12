use std::path::{Path, PathBuf};

pub fn portable_relative_paths(root: &Path, candidates: Vec<PathBuf>) -> Vec<String> {
    let mut paths = candidates
        .into_iter()
        .filter_map(|path| path.strip_prefix(root).ok().map(Path::to_path_buf))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();
    paths
}
