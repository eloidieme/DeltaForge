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
        [command, index_path, token] if command == "query" => query(Path::new(index_path), token),
        [command, root] if command == "bench" => bench(Path::new(root)),
        [command, root] if command == "summary" => summary(Path::new(root)),
        _ => Err(
            "usage: flashindex <scan|tokenize|search|index|query|bench|summary> ...".to_string(),
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
                if is_token_char(ch) {
                    token_start.get_or_insert(byte_index);
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

fn portable_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
