use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use deltaforge::application;
use deltaforge::context::GlobalOptions;

struct Case {
    name: &'static str,
    stage: &'static str,
    source: String,
    timeout_ms: Option<u64>,
    expected_priority: u32,
    expected_kind: &'static str,
    expected_headline: &'static str,
}

fn deltaforge_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_deltaforge"))
}

fn temp_project_path(case: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "deltaforge-phase1-corpus-{case}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn scanner_source(
    print_output: bool,
    maximum_depth: &str,
    absolute_paths: bool,
    reverse_order: bool,
    apply_ignores: bool,
) -> String {
    format!(
        r#"use std::env;
use std::fs;
use std::path::{{Path, PathBuf}};
use std::process::ExitCode;

const PRINT_OUTPUT: bool = {print_output};
const MAXIMUM_DEPTH: usize = {maximum_depth};
const ABSOLUTE_PATHS: bool = {absolute_paths};
const REVERSE_ORDER: bool = {reverse_order};
const APPLY_IGNORES: bool = {apply_ignores};

fn main() -> ExitCode {{
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 || args[0] != "scan" {{
        return ExitCode::FAILURE;
    }}
    match scan(Path::new(&args[1])) {{
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {{ eprintln!("{{error}}"); ExitCode::FAILURE }}
    }}
}}

fn scan(root: &Path) -> Result<(), String> {{
    let mut files = Vec::new();
    visit(root, root, 0, &mut files)?;
    let mut output = files.into_iter().map(|path| {{
        let visible = if ABSOLUTE_PATHS {{ path }} else {{
            path.strip_prefix(root).map_err(|error| error.to_string())?.to_path_buf()
        }};
        Ok(visible.components().map(|part| part.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/"))
    }}).collect::<Result<Vec<String>, String>>()?;
    output.sort();
    if REVERSE_ORDER {{ output.reverse(); }}
    if PRINT_OUTPUT {{ for path in output {{ println!("{{path}}"); }} }}
    Ok(())
}}

fn visit(root: &Path, current: &Path, depth: usize, files: &mut Vec<PathBuf>) -> Result<(), String> {{
    for entry in fs::read_dir(current).map_err(|error| error.to_string())? {{
        let entry = entry.map_err(|error| error.to_string())?;
        let kind = entry.file_type().map_err(|error| error.to_string())?;
        let path = entry.path();
        if kind.is_dir() {{
            let name = entry.file_name();
            let ignored = matches!(name.to_string_lossy().as_ref(), ".git" | "target" | "build" | "node_modules");
            if (!APPLY_IGNORES || !ignored) && depth < MAXIMUM_DEPTH {{
                visit(root, &path, depth + 1, files)?;
            }}
        }} else if kind.is_file() {{ files.push(path); }}
    }}
    Ok(())
}}
"#
    )
}

fn timeout_source() -> String {
    r#"use std::env;
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 || args[0] != "scan" { return ExitCode::FAILURE; }
    thread::sleep(Duration::from_secs(2));
    ExitCode::SUCCESS
}
"#
    .to_string()
}

/// Every non-scaffolding case below is built by taking the reference
/// FlashIndex solution and applying one or more targeted, deliberate
/// substitutions. Each substitution must match the reference text at least
/// once; if the reference is later edited and a pattern stops matching, this
/// panics loudly instead of silently building an unmodified (correct)
/// program.
fn mutate(mutations: &[(&str, &str)]) -> String {
    let mut source =
        include_str!("../tools/reference_solutions/flashindex_rust/src/main.rs").to_string();
    for (old, new) in mutations {
        assert!(
            source.contains(old),
            "reference solution no longer contains expected snippet: {old:?}"
        );
        source = source.replace(old, new);
    }
    source
}

fn cases() -> Vec<Case> {
    vec![
        // ------------------------------------------------------------------
        // Stage 01 - Scan files (existing corpus, kept exactly as-is).
        // ------------------------------------------------------------------
        Case {
            name: "no-output",
            stage: "01_scan_files",
            source: scanner_source(false, "usize::MAX", false, false, true),
            timeout_ms: None,
            expected_priority: 10,
            expected_kind: "stdout-contains",
            expected_headline: "Your scanner did not report required files",
        },
        Case {
            name: "missing-nested-files",
            stage: "01_scan_files",
            source: scanner_source(true, "1", false, false, true),
            timeout_ms: None,
            expected_priority: 20,
            expected_kind: "stdout-contains",
            expected_headline: "Discovery stops before nested files",
        },
        Case {
            name: "absolute-paths",
            stage: "01_scan_files",
            source: scanner_source(true, "usize::MAX", true, false, true),
            timeout_ms: None,
            expected_priority: 25,
            expected_kind: "stdout-excludes",
            expected_headline: "A discovered path leaks the machine-specific root",
        },
        Case {
            name: "unstable-ordering",
            stage: "01_scan_files",
            source: scanner_source(true, "usize::MAX", false, true, true),
            timeout_ms: None,
            expected_priority: 40,
            expected_kind: "stdout-exact",
            expected_headline: "The path stream is not deterministic",
        },
        Case {
            name: "unexpected-files",
            stage: "01_scan_files",
            source: scanner_source(true, "usize::MAX", false, false, false),
            timeout_ms: None,
            expected_priority: 30,
            expected_kind: "stdout-excludes",
            expected_headline: "Generated or dependency files entered the scan",
        },
        Case {
            name: "build-failure",
            stage: "01_scan_files",
            source: "fn main( {\n".to_string(),
            timeout_ms: None,
            expected_priority: 0,
            expected_kind: "build",
            expected_headline: "The project did not build",
        },
        Case {
            name: "timeout",
            stage: "01_scan_files",
            source: timeout_source(),
            timeout_ms: Some(100),
            expected_priority: 1,
            expected_kind: "runner",
            expected_headline: "The test command did not finish",
        },
        // ------------------------------------------------------------------
        // Stage 02 - Choose searchable files
        // ------------------------------------------------------------------
        Case {
            name: "extension-match-case-insensitive",
            stage: "02_filter_files",
            source: mutate(&[(
                r#"fn is_source_like(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension,
                "rs" | "cpp" | "c" | "h" | "hpp" | "py" | "txt" | "md" | "cmake"
            )
        })
}"#,
                r#"fn is_source_like(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            let extension = extension.to_lowercase();
            matches!(
                extension.as_str(),
                "rs" | "cpp" | "c" | "h" | "hpp" | "py" | "txt" | "md" | "cmake"
            )
        })
}"#,
            )]),
            timeout_ms: None,
            expected_priority: 50,
            expected_kind: "stdout-excludes",
            expected_headline: "Extension matching ignores letter case",
        },
        Case {
            name: "extension-match-substring-not-suffix",
            stage: "02_filter_files",
            source: mutate(&[(
                r#"fn is_source_like(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension,
                "rs" | "cpp" | "c" | "h" | "hpp" | "py" | "txt" | "md" | "cmake"
            )
        })
}"#,
                r#"fn is_source_like(path: &Path) -> bool {
    let name = path.to_string_lossy();
    const EXTENSIONS: [&str; 9] =
        ["rs", "cpp", "c", "h", "hpp", "py", "txt", "md", "cmake"];
    EXTENSIONS.iter().any(|ext| name.ends_with(ext))
}"#,
            )]),
            timeout_ms: None,
            expected_priority: 45,
            expected_kind: "stdout-exact",
            expected_headline: "A filename is matched by substring instead of exact suffix",
        },
        // ------------------------------------------------------------------
        // Stage 03 - Recognize tokens
        // ------------------------------------------------------------------
        Case {
            name: "tokenize-digits-start-token",
            stage: "03_tokenize",
            source: mutate(&[(
                r#"fn is_token_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}"#,
                r#"fn is_token_start(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}"#,
            )]),
            timeout_ms: None,
            expected_priority: 42,
            expected_kind: "stdout-exact",
            expected_headline: "A token's column does not point to its first byte",
        },
        Case {
            name: "tokenize-zero-based-positions",
            stage: "03_tokenize",
            source: mutate(&[("line_index + 1", "line_index"), ("start + 1", "start")]),
            timeout_ms: None,
            expected_priority: 10,
            expected_kind: "stdout-contains",
            expected_headline: "Basic identifiers are not recognized as tokens",
        },
        Case {
            name: "tokenize-reverse-file-order",
            stage: "03_tokenize",
            source: mutate(&[(
                r#"fn source_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_files(root, root, &mut files)?;
    files.sort();"#,
                r#"fn source_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_files(root, root, &mut files)?;
    files.sort_by(|a, b| b.cmp(a));"#,
            )]),
            timeout_ms: None,
            expected_priority: 40,
            expected_kind: "stdout-exact",
            expected_headline: "Multiple files are tokenized out of sorted order",
        },
        // ------------------------------------------------------------------
        // Stage 04 - Find an exact token
        // ------------------------------------------------------------------
        Case {
            name: "search-substring-match",
            stage: "04_exact_search",
            source: mutate(&[(
                r#"fn search(root: &Path, query: &str) -> Result<(), String> {
    for occurrence in token_occurrences(root)? {
        if occurrence.token == query {"#,
                r#"fn search(root: &Path, query: &str) -> Result<(), String> {
    for occurrence in token_occurrences(root)? {
        if occurrence.token.contains(query) {"#,
            )]),
            timeout_ms: None,
            expected_priority: 10,
            expected_kind: "stdout-excludes",
            expected_headline: "An exact token query returns no matches",
        },
        Case {
            name: "search-missing-token-not-rejected",
            stage: "04_exact_search",
            source: mutate(&[(
                r#"        [command, root, token] if command == "search" => search(Path::new(root), token),"#,
                r#"        [command, root, token] if command == "search" => search(Path::new(root), token),
        [command, root] if command == "search" => search(Path::new(root), ""),"#,
            )]),
            timeout_ms: None,
            expected_priority: 60,
            expected_kind: "exit-code",
            expected_headline: "A missing query token argument does not raise an error",
        },
        // ------------------------------------------------------------------
        // Stage 05 - Group files by token
        // ------------------------------------------------------------------
        Case {
            name: "index-inverted-the-wrong-way",
            stage: "05_inverted_index",
            source: mutate(&[(
                r#"fn build_index(root: &Path) -> Result<BTreeMap<String, BTreeSet<PathBuf>>, String> {
    let mut index = BTreeMap::<String, BTreeSet<PathBuf>>::new();
    for occurrence in token_occurrences(root)? {
        index
            .entry(occurrence.token)
            .or_default()
            .insert(occurrence.path);
    }
    Ok(index)
}"#,
                r#"fn build_index(root: &Path) -> Result<BTreeMap<String, BTreeSet<PathBuf>>, String> {
    let mut index = BTreeMap::<String, BTreeSet<PathBuf>>::new();
    for occurrence in token_occurrences(root)? {
        index
            .entry(occurrence.path.to_string_lossy().to_string())
            .or_default()
            .insert(PathBuf::from(occurrence.token));
    }
    Ok(index)
}"#,
            )]),
            timeout_ms: None,
            expected_priority: 10,
            expected_kind: "stdout-contains",
            expected_headline: "Tokens are not mapped to the files containing them",
        },
        Case {
            name: "index-prints-a-heading",
            stage: "05_inverted_index",
            source: mutate(&[(
                r#"fn print_index(root: &Path) -> Result<(), String> {
    for (token, paths) in build_index(root)? {"#,
                r#"fn print_index(root: &Path) -> Result<(), String> {
    println!("index");
    for (token, paths) in build_index(root)? {"#,
            )]),
            timeout_ms: None,
            expected_priority: 42,
            expected_kind: "stdout-regex",
            expected_headline: "An index line does not follow the token-then-paths shape",
        },
        // ------------------------------------------------------------------
        // Stage 06 - Make the index canonical
        // ------------------------------------------------------------------
        Case {
            name: "index-no-dedup-repeated-occurrences",
            stage: "06_canonical_index",
            source: mutate(&[(
                r#"fn build_index(root: &Path) -> Result<BTreeMap<String, BTreeSet<PathBuf>>, String> {
    let mut index = BTreeMap::<String, BTreeSet<PathBuf>>::new();
    for occurrence in token_occurrences(root)? {
        index
            .entry(occurrence.token)
            .or_default()
            .insert(occurrence.path);
    }
    Ok(index)
}"#,
                r#"fn build_index(root: &Path) -> Result<BTreeMap<String, Vec<PathBuf>>, String> {
    let mut index = BTreeMap::<String, Vec<PathBuf>>::new();
    for occurrence in token_occurrences(root)? {
        index
            .entry(occurrence.token)
            .or_default()
            .push(occurrence.path);
    }
    Ok(index)
}"#,
            )]),
            timeout_ms: None,
            expected_priority: 30,
            expected_kind: "stdout-excludes",
            expected_headline: "A file path appears more than once in one posting",
        },
        Case {
            name: "index-case-insensitive-token-sort",
            stage: "06_canonical_index",
            source: mutate(&[(
                r#"fn print_index(root: &Path) -> Result<(), String> {
    for (token, paths) in build_index(root)? {
        let paths = paths
            .into_iter()
            .map(|path| portable_path(&path))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{token} {paths}");
    }
    Ok(())
}"#,
                r#"fn print_index(root: &Path) -> Result<(), String> {
    let mut entries: Vec<(String, BTreeSet<PathBuf>)> = build_index(root)?.into_iter().collect();
    entries.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    for (token, paths) in entries {
        let paths = paths
            .into_iter()
            .map(|path| portable_path(&path))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{token} {paths}");
    }
    Ok(())
}"#,
            )]),
            timeout_ms: None,
            expected_priority: 42,
            expected_kind: "stdout-exact",
            expected_headline: "Tokens are sorted alphabetically instead of by byte value",
        },
        // ------------------------------------------------------------------
        // Stage 07 - Write the index to disk
        // ------------------------------------------------------------------
        Case {
            name: "persist-appends-instead-of-replacing",
            stage: "07_persist_index",
            source: mutate(&[(
                r#"    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(output, serialized).map_err(|error| error.to_string())?;
    println!("wrote {}", output.display());"#,
                r#"    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    {
        use std::io::Write as _;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output)
            .map_err(|error| error.to_string())?;
        file.write_all(serialized.as_bytes())
            .map_err(|error| error.to_string())?;
    }
    println!("wrote {}", output.display());"#,
            )]),
            timeout_ms: None,
            expected_priority: 32,
            expected_kind: "file-excludes",
            expected_headline: "A rebuilt index still contains stale content",
        },
        Case {
            name: "persist-wrong-field-separator",
            stage: "07_persist_index",
            source: mutate(&[(
                r#"        serialized.push_str(&token);
        serialized.push('\t');
        serialized.push_str(&paths);
        serialized.push('\n');"#,
                r#"        serialized.push_str(&token);
        serialized.push(' ');
        serialized.push_str(&paths);
        serialized.push('\n');"#,
            )]),
            timeout_ms: None,
            expected_priority: 22,
            expected_kind: "file-contains",
            expected_headline: "A persisted record is missing its token or path field",
        },
        // ------------------------------------------------------------------
        // Stage 08 - Query a saved index
        // ------------------------------------------------------------------
        Case {
            name: "query-wrong-field-separator",
            stage: "08_query_index",
            source: mutate(&[(
                r#"    for line in source.lines() {
        let mut parts = line.split('\t');
        if parts.next() == Some(query) {"#,
                r#"    for line in source.lines() {
        let mut parts = line.split(' ');
        if parts.next() == Some(query) {"#,
            )]),
            timeout_ms: None,
            expected_priority: 10,
            expected_kind: "stdout-exact",
            expected_headline: "Querying a persisted index returns no results",
        },
        Case {
            name: "query-swallows-read-errors",
            stage: "08_query_index",
            source: mutate(&[(
                r#"fn query(index_path: &Path, query: &str) -> Result<(), String> {
    let source = fs::read_to_string(index_path).map_err(|error| error.to_string())?;"#,
                r#"fn query(index_path: &Path, query: &str) -> Result<(), String> {
    let source = fs::read_to_string(index_path).unwrap_or_default();"#,
            )]),
            timeout_ms: None,
            expected_priority: 65,
            expected_kind: "exit-code",
            expected_headline: "An unreadable index file does not raise an error",
        },
        // ------------------------------------------------------------------
        // Stage 09 - Describe a scan as data
        // ------------------------------------------------------------------
        Case {
            name: "bench-ignores-corpus-filter",
            stage: "09_benchmark_mode",
            source: mutate(&[(
                r#"fn bench(root: &Path) -> Result<(), String> {
    let start = Instant::now();
    let files = source_files(root)?.len();
    let runtime_ms = start.elapsed().as_millis();"#,
                r#"fn bench(root: &Path) -> Result<(), String> {
    let start = Instant::now();
    let mut files = Vec::new();
    collect_files(root, root, &mut files)?;
    let files = files.len();
    let runtime_ms = start.elapsed().as_millis();"#,
            )]),
            timeout_ms: None,
            expected_priority: 20,
            expected_kind: "stdout-contains",
            expected_headline: "The reported file count does not match the corpus",
        },
        // ------------------------------------------------------------------
        // Stage 10 - Summarize the corpus
        // ------------------------------------------------------------------
        Case {
            name: "summary-case-folds-unique-tokens",
            stage: "10_report_summary",
            source: mutate(&[(
                r#"    let unique = occurrences
        .iter()
        .map(|occurrence| occurrence.token.clone())
        .collect::<BTreeSet<_>>();"#,
                r#"    let unique = occurrences
        .iter()
        .map(|occurrence| occurrence.token.to_lowercase())
        .collect::<BTreeSet<_>>();"#,
            )]),
            timeout_ms: None,
            expected_priority: 50,
            expected_kind: "stdout-exact",
            expected_headline: "Unique token counting folds differing letter case together",
        },
        // ------------------------------------------------------------------
        // Stage 11 - Build the index with several workers
        // ------------------------------------------------------------------
        Case {
            name: "parallel-chunks-exact-drops-remainder",
            stage: "11_parallel_indexing",
            source: mutate(&[(
                r#"    let chunks: Vec<Vec<PathBuf>> = files
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();"#,
                r#"    let chunks: Vec<Vec<PathBuf>> = files
        .chunks_exact(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();"#,
            )]),
            timeout_ms: None,
            expected_priority: 21,
            expected_kind: "stdout-exact",
            expected_headline: "Two worker threads produce non-identical index output",
        },
        Case {
            name: "parallel-zero-threads-not-rejected",
            stage: "11_parallel_indexing",
            source: mutate(&[(
                r#"    let worker_count: usize = threads
        .parse()
        .ok()
        .filter(|count| *count >= 1)
        .ok_or_else(|| format!("--threads expects a positive integer, got {threads:?}"))?;"#,
                r#"    let worker_count: usize = threads
        .parse()
        .ok()
        .ok_or_else(|| format!("--threads expects a positive integer, got {threads:?}"))?;"#,
            )]),
            timeout_ms: None,
            expected_priority: 60,
            expected_kind: "exit-code",
            expected_headline: "A zero thread count is accepted instead of rejected",
        },
        // ------------------------------------------------------------------
        // Stage 12 - Measure parallel speedup
        // ------------------------------------------------------------------
        Case {
            name: "parallel-merge-overwrites-instead-of-unions",
            stage: "12_parallel_performance",
            source: mutate(&[(
                r#"    let mut merged = BTreeMap::<String, BTreeSet<PathBuf>>::new();
    for handle in handles {
        let local = handle
            .join()
            .map_err(|_| "worker thread panicked".to_string())??;
        for (token, paths) in local {
            merged.entry(token).or_default().extend(paths);
        }
    }"#,
                r#"    let mut merged = BTreeMap::<String, BTreeSet<PathBuf>>::new();
    for handle in handles {
        let local = handle
            .join()
            .map_err(|_| "worker thread panicked".to_string())??;
        for (token, paths) in local {
            merged.insert(token, paths);
        }
    }"#,
            )]),
            timeout_ms: None,
            expected_priority: 24,
            expected_kind: "stdout-exact",
            expected_headline: "Eight-thread runs diverge from the canonical index",
        },
        // ------------------------------------------------------------------
        // Stage 13 - Score multi-token matches
        // ------------------------------------------------------------------
        Case {
            name: "rank-duplicate-query-tokens-inflate-total",
            stage: "13_ranked_search",
            source: mutate(&[(
                r#"    let query_tokens: BTreeSet<String> = query
        .split_whitespace()
        .map(|token| token.to_string())
        .collect();"#,
                r#"    let query_tokens: Vec<String> = query
        .split_whitespace()
        .map(|token| token.to_string())
        .collect();"#,
            )]),
            timeout_ms: None,
            expected_priority: 25,
            expected_kind: "stdout-contains",
            expected_headline: "Repeated query tokens inflate the coverage denominator",
        },
        Case {
            name: "rank-whitespace-query-not-rejected",
            stage: "13_ranked_search",
            source: mutate(&[(
                r#"    let query_tokens: BTreeSet<String> = query
        .split_whitespace()
        .map(|token| token.to_string())
        .collect();
    if query_tokens.is_empty() {
        return Err("rank expects a non-empty query of one or more tokens".to_string());
    }
    let total = query_tokens.len();"#,
                r#"    if query.is_empty() {
        return Err("rank expects a non-empty query of one or more tokens".to_string());
    }
    let query_tokens: BTreeSet<String> = query
        .split_whitespace()
        .map(|token| token.to_string())
        .collect();
    let total = query_tokens.len();"#,
            )]),
            timeout_ms: None,
            expected_priority: 62,
            expected_kind: "exit-code",
            expected_headline: "A whitespace-only query is accepted instead of rejected",
        },
        // ------------------------------------------------------------------
        // Stage 14 - Make ranking stable
        // ------------------------------------------------------------------
        Case {
            name: "rank-limit-applied-before-sort",
            stage: "14_stable_ranking",
            source: mutate(&[(
                r#"    let mut ranked: Vec<(PathBuf, usize, usize)> = per_file
        .into_iter()
        .map(|(path, (matched, occurrences))| (path, matched.len(), occurrences))
        .collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)).then(a.0.cmp(&b.0)));

    for (rank, (path, matched, occurrences)) in ranked.iter().take(10).enumerate() {"#,
                r#"    let mut ranked: Vec<(PathBuf, usize, usize)> = per_file
        .into_iter()
        .map(|(path, (matched, occurrences))| (path, matched.len(), occurrences))
        .collect();
    ranked.truncate(10);
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)).then(a.0.cmp(&b.0)));

    for (rank, (path, matched, occurrences)) in ranked.iter().enumerate() {"#,
            )]),
            timeout_ms: None,
            expected_priority: 22,
            expected_kind: "stdout-exact",
            expected_headline: "The result limit is applied before the final sort",
        },
        Case {
            name: "rank-tiebreak-descending-path",
            stage: "14_stable_ranking",
            source: mutate(&[(
                "ranked.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)).then(a.0.cmp(&b.0)));",
                "ranked.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)).then(b.0.cmp(&a.0)));",
            )]),
            timeout_ms: None,
            expected_priority: 10,
            expected_kind: "stdout-exact",
            expected_headline: "Occurrence and path tie-breaks do not fully order results",
        },
    ]
}

fn prepare_project(path: &Path, case: &Case) {
    let init = Command::new(deltaforge_bin())
        .args([
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            path.to_str().unwrap(),
            "--no-git",
            "--stage",
            case.stage,
        ])
        .output()
        .unwrap();
    assert!(
        init.status.success(),
        "{}: initialization failed: {}",
        case.name,
        String::from_utf8_lossy(&init.stderr)
    );
    fs::write(path.join("src/main.rs"), &case.source).unwrap();
    if let Some(timeout_ms) = case.timeout_ms {
        let config_path = path.join(".deltaforge/config.toml");
        let config = fs::read_to_string(&config_path).unwrap();
        let config = config.replace("timeout_ms = 5000", &format!("timeout_ms = {timeout_ms}"));
        assert!(config.contains(&format!("timeout_ms = {timeout_ms}")));
        fs::write(config_path, config).unwrap();
    }
}

/// Runs one case end-to-end through the real application and asserts that
/// the exact primary diagnosis a stuck learner would be shown matches the
/// intended diagnosis recorded in the pack's `tests.yaml`. This is the
/// single shared assertion body every grouped test below calls; keeping it
/// in one place means every case is held to the same bar: correct priority,
/// kind, and headline; non-empty summary/contract/expected/actual evidence;
/// a non-empty command and fixture listing for behavioral (non-build)
/// failures; and no leaked temporary-directory paths anywhere in the
/// evidence surfaced to the learner.
fn assert_primary_diagnosis(case: &Case) {
    let project = temp_project_path(case.name);
    prepare_project(&project, case);
    let run = Command::new(deltaforge_bin())
        .args(["test", "--stage", case.stage, "--fail-fast"])
        .current_dir(&project)
        .output()
        .unwrap();
    assert!(!run.status.success(), "{} unexpectedly passed", case.name);

    let options = GlobalOptions {
        project_dir: Some(project.clone()),
        ..GlobalOptions::default()
    };
    let state = application::load_workbench_state(&options)
        .unwrap_or_else(|error| panic!("{}: state failed: {error:#}", case.name));
    let failure = state
        .primary_failure
        .unwrap_or_else(|| panic!("{}: no primary failure", case.name));
    let diagnosis = failure
        .diagnosis
        .unwrap_or_else(|| panic!("{}: failure has no diagnosis", case.name));

    assert_eq!(diagnosis.priority, case.expected_priority, "{}", case.name);
    assert_eq!(diagnosis.kind, case.expected_kind, "{}", case.name);
    assert_eq!(diagnosis.headline, case.expected_headline, "{}", case.name);
    assert!(!diagnosis.summary.trim().is_empty(), "{}", case.name);
    assert!(!diagnosis.contract.trim().is_empty(), "{}", case.name);
    assert!(
        diagnosis
            .expected
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty()),
        "{}: expected evidence is empty",
        case.name
    );
    assert!(
        diagnosis
            .actual
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty()),
        "{}: observed evidence is empty",
        case.name
    );
    if case.expected_kind != "build" {
        assert!(!diagnosis.command.is_empty(), "{}", case.name);
        assert!(!diagnosis.fixture_entries.is_empty(), "{}", case.name);
    }
    let temporary_root = std::env::temp_dir().to_string_lossy().to_string();
    for evidence in failure
        .failures
        .iter()
        .chain(diagnosis.command.iter())
        .chain(std::iter::once(&diagnosis.summary))
        .chain(diagnosis.expected.iter())
        .chain(diagnosis.actual.iter())
    {
        assert!(
            !evidence.contains(&temporary_root),
            "{} leaked a temporary path in {evidence:?}",
            case.name
        );
    }

    let _ = fs::remove_dir_all(project);
}

/// Runs the shared assertion over every case whose stage id falls within
/// `[start, end]` (inclusive), comparing on the numeric stage prefix so the
/// four grouped tests below partition the full corpus without overlap.
fn run_stage_range(start: u32, end: u32) {
    for case in cases() {
        let stage_number: u32 = case.stage[..2].parse().expect("stage id starts with NN_");
        if stage_number >= start && stage_number <= end {
            assert_primary_diagnosis(&case);
        }
    }
}

// The full corpus is split across several `#[test]` functions grouped by
// stage range so cargo can run them in parallel: each case compiles a fresh
// learner project, which dominates runtime, and serializing all ~30 cases
// in one test would risk the CI timeout.

#[test]
fn stages_01_to_04_have_the_expected_primary_diagnosis() {
    run_stage_range(1, 4);
}

#[test]
fn stages_05_to_08_have_the_expected_primary_diagnosis() {
    run_stage_range(5, 8);
}

#[test]
fn stages_09_to_11_have_the_expected_primary_diagnosis() {
    run_stage_range(9, 11);
}

#[test]
fn stages_12_to_14_have_the_expected_primary_diagnosis() {
    run_stage_range(12, 14);
}
