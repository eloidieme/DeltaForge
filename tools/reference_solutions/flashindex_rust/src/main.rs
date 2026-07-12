use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, root] if command == "scan" => scan(Path::new(root)),
        [command, root] if command == "tokenize" => tokenize_command(Path::new(root)),
        [command, root, token] if command == "search" => search(Path::new(root), token),
        [command, root] if command == "index" => print_index(Path::new(root)),
        [command, root, flag, out] if command == "index" && flag == "--out" => {
            write_index(Path::new(root), Path::new(out))
        }
        [command, root, flag, threads] if command == "index" && flag == "--threads" => {
            parallel_index(Path::new(root), threads)
        }
        [command, index_path, token] if command == "query" => query(Path::new(index_path), token),
        [command, root, query] if command == "rank" => rank(Path::new(root), query),
        [command, root] if command == "bench" => bench(Path::new(root)),
        [command, root] if command == "summary" => summary(Path::new(root)),
        _ => Err(
            "usage: flashindex <scan|tokenize|search|index|query|rank|bench|summary> ..."
                .to_string(),
        ),
    }
}

fn scan(root: &Path) -> Result<(), String> {
    for file in source_files(root)? {
        println!("{}", portable_path(&file));
    }
    Ok(())
}

fn tokenize_command(root: &Path) -> Result<(), String> {
    for occurrence in token_occurrences(root)? {
        println!(
            "{}:{}:{} {}",
            portable_path(&occurrence.path),
            occurrence.line,
            occurrence.column,
            occurrence.token
        );
    }
    Ok(())
}

fn search(root: &Path, query: &str) -> Result<(), String> {
    for occurrence in token_occurrences(root)? {
        if occurrence.token == query {
            println!(
                "{}:{}:{} {}",
                portable_path(&occurrence.path),
                occurrence.line,
                occurrence.column,
                occurrence.token
            );
        }
    }
    Ok(())
}

fn print_index(root: &Path) -> Result<(), String> {
    for (token, paths) in build_index(root)? {
        let paths = paths
            .into_iter()
            .map(|path| portable_path(&path))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{token} {paths}");
    }
    Ok(())
}

fn write_index(root: &Path, output: &Path) -> Result<(), String> {
    let mut serialized = String::new();
    for (token, paths) in build_index(root)? {
        let paths = paths
            .into_iter()
            .map(|path| portable_path(&path))
            .collect::<Vec<_>>()
            .join("\t");
        serialized.push_str(&token);
        serialized.push('\t');
        serialized.push_str(&paths);
        serialized.push('\n');
    }
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(output, serialized).map_err(|error| error.to_string())?;
    println!("wrote {}", output.display());
    Ok(())
}

fn parallel_index(root: &Path, threads: &str) -> Result<(), String> {
    let worker_count: usize = threads
        .parse()
        .ok()
        .filter(|count| *count >= 1)
        .ok_or_else(|| format!("--threads expects a positive integer, got {threads:?}"))?;

    // Collect the deterministic file list once, then partition it across
    // worker-local indexes. Each worker owns a disjoint slice of files, so no
    // shared mutable state is touched during indexing.
    let files = source_files(root)?;
    let chunk_size = files.len().div_ceil(worker_count).max(1);
    let chunks: Vec<Vec<PathBuf>> = files
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    let root = root.to_path_buf();
    let mut handles = Vec::new();
    for chunk in chunks {
        let root = root.clone();
        handles.push(std::thread::spawn(move || index_files(&root, &chunk)));
    }

    // Merge worker-local indexes deterministically. Because the merge iterates
    // sorted maps and unions sorted sets, the printed output is byte-identical
    // to the single-threaded `index` command regardless of the worker count.
    let mut merged = BTreeMap::<String, BTreeSet<PathBuf>>::new();
    for handle in handles {
        let local = handle
            .join()
            .map_err(|_| "worker thread panicked".to_string())??;
        for (token, paths) in local {
            merged.entry(token).or_default().extend(paths);
        }
    }

    for (token, paths) in merged {
        let paths = paths
            .into_iter()
            .map(|path| portable_path(&path))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{token} {paths}");
    }
    Ok(())
}

fn index_files(
    root: &Path,
    files: &[PathBuf],
) -> Result<BTreeMap<String, BTreeSet<PathBuf>>, String> {
    let mut index = BTreeMap::<String, BTreeSet<PathBuf>>::new();
    for relative_path in files {
        let path = root.join(relative_path);
        let source = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        for occurrence in tokens_in_source(&source) {
            index
                .entry(occurrence)
                .or_default()
                .insert(relative_path.clone());
        }
    }
    Ok(index)
}

fn tokens_in_source(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for line in source.lines() {
        let mut token_start = None;
        for (byte_index, ch) in line.char_indices() {
            if token_start.is_some() && is_token_char(ch) {
                continue;
            } else if is_token_start(ch) {
                token_start = Some(byte_index);
            } else if let Some(start) = token_start.take() {
                tokens.push(line[start..byte_index].to_string());
            }
        }
        if let Some(start) = token_start {
            tokens.push(line[start..].to_string());
        }
    }
    tokens
}

fn rank(root: &Path, query: &str) -> Result<(), String> {
    let query_tokens: BTreeSet<String> = query
        .split_whitespace()
        .map(|token| token.to_string())
        .collect();
    if query_tokens.is_empty() {
        return Err("rank expects a non-empty query of one or more tokens".to_string());
    }
    let total = query_tokens.len();

    // Path-keyed BTreeMap gives a path-sorted starting order; the stable sort
    // below then only reorders by the ranking metrics, leaving path ascending
    // as the deterministic final tie-break.
    let mut per_file: BTreeMap<PathBuf, (BTreeSet<String>, usize)> = BTreeMap::new();
    for occurrence in token_occurrences(root)? {
        if query_tokens.contains(&occurrence.token) {
            let entry = per_file.entry(occurrence.path).or_default();
            entry.0.insert(occurrence.token);
            entry.1 += 1;
        }
    }

    let mut ranked: Vec<(PathBuf, usize, usize)> = per_file
        .into_iter()
        .map(|(path, (matched, occurrences))| (path, matched.len(), occurrences))
        .collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)).then(a.0.cmp(&b.0)));

    for (rank, (path, matched, occurrences)) in ranked.iter().take(10).enumerate() {
        println!(
            "{}. {} (matched {}/{} tokens, {} occurrences)",
            rank + 1,
            portable_path(path),
            matched,
            total,
            occurrences
        );
    }
    Ok(())
}

fn query(index_path: &Path, query: &str) -> Result<(), String> {
    let source = fs::read_to_string(index_path).map_err(|error| error.to_string())?;
    for line in source.lines() {
        let mut parts = line.split('\t');
        if parts.next() == Some(query) {
            for path in parts {
                println!("{path}");
            }
        }
    }
    Ok(())
}

fn bench(root: &Path) -> Result<(), String> {
    let start = Instant::now();
    let files = source_files(root)?.len();
    let runtime_ms = start.elapsed().as_millis();
    println!("{{\"files\":{files},\"runtime_ms\":{runtime_ms}}}");
    Ok(())
}

fn summary(root: &Path) -> Result<(), String> {
    let files = source_files(root)?;
    let occurrences = token_occurrences(root)?;
    let unique = occurrences
        .iter()
        .map(|occurrence| occurrence.token.clone())
        .collect::<BTreeSet<_>>();
    println!("files: {}", files.len());
    println!("tokens: {}", occurrences.len());
    println!("unique_tokens: {}", unique.len());
    Ok(())
}

fn build_index(root: &Path) -> Result<BTreeMap<String, BTreeSet<PathBuf>>, String> {
    let mut index = BTreeMap::<String, BTreeSet<PathBuf>>::new();
    for occurrence in token_occurrences(root)? {
        index
            .entry(occurrence.token)
            .or_default()
            .insert(occurrence.path);
    }
    Ok(index)
}

#[derive(Debug)]
struct Occurrence {
    path: PathBuf,
    line: usize,
    column: usize,
    token: String,
}

fn token_occurrences(root: &Path) -> Result<Vec<Occurrence>, String> {
    let mut occurrences = Vec::new();
    for relative_path in source_files(root)? {
        let path = root.join(&relative_path);
        let source = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        for (line_index, line) in source.lines().enumerate() {
            let mut token_start = None;
            for (byte_index, ch) in line.char_indices() {
                if token_start.is_some() && is_token_char(ch) {
                    continue;
                } else if is_token_start(ch) {
                    token_start = Some(byte_index);
                } else if let Some(start) = token_start.take() {
                    occurrences.push(Occurrence {
                        path: relative_path.clone(),
                        line: line_index + 1,
                        column: start + 1,
                        token: line[start..byte_index].to_string(),
                    });
                }
            }
            if let Some(start) = token_start {
                occurrences.push(Occurrence {
                    path: relative_path.clone(),
                    line: line_index + 1,
                    column: start + 1,
                    token: line[start..].to_string(),
                });
            }
        }
    }
    Ok(occurrences)
}

fn source_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_files(root, root, &mut files)?;
    files.sort();
    Ok(files
        .into_iter()
        .filter(|path| is_source_like(path))
        .collect())
}

fn collect_files(root: &Path, current: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(current).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            if is_ignored_dir(&name) {
                continue;
            }
            collect_files(root, &path, files)?;
        } else if path.is_file() {
            files.push(path.strip_prefix(root).unwrap_or(&path).to_path_buf());
        }
    }
    Ok(())
}

fn is_ignored_dir(name: &str) -> bool {
    matches!(name, ".git" | "target" | "build" | "node_modules")
}

fn is_source_like(path: &Path) -> bool {
    if path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension,
                "bin" | "dat" | "png" | "jpg" | "jpeg" | "gif" | "o"
            )
        })
    {
        return false;
    }

    matches!(
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default(),
        "README.md" | "CMakeLists.txt"
    ) || path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension,
                "rs" | "cpp" | "c" | "h" | "hpp" | "py" | "txt" | "md"
            )
        })
}

fn is_token_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

fn is_token_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn portable_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
