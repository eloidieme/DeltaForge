use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use include_dir::{Dir, DirEntry, include_dir};
use serde::{Deserialize, Serialize};

use crate::integrity::{digest_named_contents, is_safe_relative_path, strict_tree_contents};
use crate::runner::{Expectations, StageTests};

static EMBEDDED_PACKS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/packs");

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectPack {
    #[serde(default = "current_pack_schema_version")]
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default)]
    pub languages: BTreeMap<String, LanguageSpec>,
    #[serde(default)]
    pub ignored_paths: Vec<String>,
    #[serde(default)]
    pub stages: Vec<StageSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LanguageSpec {
    pub template: PathBuf,
    pub build: Option<CommandSpec>,
    pub run: CommandSpec,
    /// Command used by `deltaforge bench` after the build step. Falls back to
    /// `run` when absent so existing packs stay valid at schema_version 1.
    #[serde(default)]
    pub bench_run: Option<CommandSpec>,
}

impl LanguageSpec {
    /// The command benchmarks should time: `bench_run` if set, else `run`.
    pub fn bench_command(&self) -> &CommandSpec {
        self.bench_run.as_ref().unwrap_or(&self.run)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandSpec {
    pub command: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageSpec {
    pub id: String,
    pub title: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct LoadedPack {
    pub root: PathBuf,
    pub manifest: ProjectPack,
}

#[derive(Debug, Clone, Default)]
pub struct PackSearchOptions {
    pub packs_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageBenchmarks {
    #[serde(default)]
    pub benchmarks: Vec<BenchmarkSpec>,
    #[serde(default)]
    pub performance_gates: Vec<PerformanceGate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkSpec {
    pub name: String,
    pub fixture: String,
    #[serde(default)]
    pub command: Vec<String>,
    #[serde(default)]
    pub matrix: BTreeMap<String, Vec<serde_yaml::Value>>,
    pub iterations: Option<u64>,
    pub warmup: Option<u64>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum PerformanceMetric {
    RuntimeMedianMs,
    RuntimeP95Ms,
    ThroughputMbS,
    PeakMemoryMb,
    Speedup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PerformanceGate {
    pub name: String,
    pub benchmark: String,
    pub metric: PerformanceMetric,
    pub min: Option<f64>,
    pub max: Option<f64>,
    #[serde(default)]
    pub params: BTreeMap<String, String>,
    #[serde(default)]
    pub advice: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "comparison", content = "value", rename_all = "snake_case")]
pub enum GateBound {
    Min(f64),
    Max(f64),
}

impl PerformanceGate {
    pub fn bound(&self) -> Option<GateBound> {
        match (self.min, self.max) {
            (Some(value), None) if value.is_finite() => Some(GateBound::Min(value)),
            (None, Some(value)) if value.is_finite() => Some(GateBound::Max(value)),
            _ => None,
        }
    }
}

/// Placeholders always available in benchmark command args, independent of the
/// benchmark's `matrix` declaration.
pub const BENCHMARK_BUILTIN_PLACEHOLDERS: [&str; 2] = ["fixture_path", "temp_dir"];

/// Check a benchmark `matrix` declaration; an empty return means it is valid.
pub fn benchmark_matrix_problems(matrix: &BTreeMap<String, Vec<serde_yaml::Value>>) -> Vec<String> {
    let mut problems = Vec::new();
    for (name, values) in matrix {
        if !is_identifier(name) {
            problems.push(format!(
                "matrix parameter {name} is not a valid identifier (expected [A-Za-z_][A-Za-z0-9_]*)"
            ));
        } else if BENCHMARK_BUILTIN_PLACEHOLDERS.contains(&name.as_str()) {
            problems.push(format!(
                "matrix parameter {name} shadows a built-in placeholder"
            ));
        }
        if values.is_empty() {
            problems.push(format!("matrix parameter {name} has no values"));
        }
        for value in values {
            if benchmark_scalar_to_string(value).is_none() {
                problems.push(format!(
                    "matrix parameter {name} has a non-scalar value; only strings, numbers, and booleans are allowed"
                ));
            }
        }
    }
    problems
}

/// Render a matrix value as the string substituted into command args.
pub fn benchmark_scalar_to_string(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::String(text) => Some(text.clone()),
        serde_yaml::Value::Number(number) => Some(number.to_string()),
        serde_yaml::Value::Bool(flag) => Some(flag.to_string()),
        _ => None,
    }
}

fn is_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some(first) if first.is_ascii_alphabetic() || first == '_')
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

/// Identifier-shaped `{name}` placeholders in a command arg. Braces around
/// anything else (e.g. JSON literals) are not treated as placeholders.
fn placeholder_identifiers(arg: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = arg;
    while let Some(start) = rest.find('{') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('}') else {
            break;
        };
        let candidate = &after[..end];
        if is_identifier(candidate) {
            found.push(candidate.to_string());
        }
        rest = &after[end + 1..];
    }
    found
}

impl ProjectPack {
    pub fn language(&self, language: &str) -> Option<&LanguageSpec> {
        self.languages.get(language)
    }

    pub fn first_stage(&self) -> Option<&StageSpec> {
        self.stages.first()
    }

    pub fn stage(&self, stage_id: &str) -> Option<&StageSpec> {
        self.stages.iter().find(|stage| stage.id == stage_id)
    }

    pub fn stage_index(&self, stage_id: &str) -> Option<usize> {
        self.stages.iter().position(|stage| stage.id == stage_id)
    }

    pub fn next_stage(&self, stage_id: &str) -> Option<&StageSpec> {
        let next_index = self.stage_index(stage_id)? + 1;
        self.stages.get(next_index)
    }
}

impl LoadedPack {
    pub fn stage_dir(&self, stage: &StageSpec) -> PathBuf {
        self.root.join(&stage.path)
    }

    pub fn instructions_path(&self, stage: &StageSpec) -> PathBuf {
        self.stage_dir(stage).join("instructions.md")
    }

    pub fn hints_path(&self, stage: &StageSpec) -> PathBuf {
        self.stage_dir(stage).join("hints.md")
    }

    pub fn tests_path(&self, stage: &StageSpec) -> PathBuf {
        self.stage_dir(stage).join("tests.yaml")
    }

    pub fn benchmarks_path(&self, stage: &StageSpec) -> PathBuf {
        self.stage_dir(stage).join("benchmarks.yaml")
    }

    pub fn design_prompt_path(&self, stage: &StageSpec) -> PathBuf {
        self.stage_dir(stage).join("design_prompt.md")
    }

    pub fn fixture_path(&self, stage: &StageSpec, fixture: &str) -> PathBuf {
        self.stage_dir(stage).join("fixtures").join(fixture)
    }

    pub fn read_stage_file(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path)
            .with_context(|| format!("failed to read stage file {}", path.display()))
    }

    /// Digest of everything that determines whether `stage` passes for the
    /// given language: the stage's tests, its fixtures, and the language's
    /// build/run commands. Documentation (instructions, hints, design prompts,
    /// README) is deliberately excluded so doc-only pack updates do not
    /// invalidate completion proofs.
    ///
    /// Tests and fixtures are hashed as raw bytes because those bytes are the
    /// runner's semantics. Performance gates have an interpretation layer, so
    /// their progression-affecting meaning is hashed as parsed canonical data;
    /// display advice and measurement methodology intentionally stay outside.
    pub fn stage_behavioral_digest(
        &self,
        stage: &StageSpec,
        language: &LanguageSpec,
    ) -> Result<String> {
        let mut entries = Vec::new();

        let tests_path = self.tests_path(stage);
        let tests = fs::read(&tests_path).with_context(|| {
            format!(
                "failed to read stage tests {} while computing behavioral digest",
                tests_path.display()
            )
        })?;
        entries.push(("tests.yaml".to_string(), tests));

        let fixtures_dir = self.stage_dir(stage).join("fixtures");
        if fixtures_dir.is_dir() {
            for (name, contents) in strict_tree_contents(&fixtures_dir).with_context(|| {
                format!("failed to digest stage fixtures {}", fixtures_dir.display())
            })? {
                entries.push((format!("fixtures/{name}"), contents));
            }
        }

        entries.push((
            "command:build".to_string(),
            serde_json::to_vec(&language.build).context("failed to serialize build command")?,
        ));
        entries.push((
            "command:run".to_string(),
            serde_json::to_vec(&language.run).context("failed to serialize run command")?,
        ));

        let benchmarks_path = self.benchmarks_path(stage);
        if benchmarks_path.is_file() {
            let source = fs::read_to_string(&benchmarks_path).with_context(|| {
                format!(
                    "failed to read benchmarks file {}",
                    benchmarks_path.display()
                )
            })?;
            let parsed: StageBenchmarks = serde_yaml::from_str(&source).with_context(|| {
                format!(
                    "failed to parse benchmarks file {}",
                    benchmarks_path.display()
                )
            })?;
            if !parsed.performance_gates.is_empty() {
                let canonical = canonical_stage_gates(&parsed, language)?;
                entries.push((
                    "performance:gates:v1".to_string(),
                    serde_json::to_vec(&canonical)
                        .context("failed to serialize performance gates")?,
                ));
            }
        }

        Ok(digest_named_contents(entries))
    }
}

#[derive(Serialize)]
struct CanonicalStageGates {
    gates: Vec<CanonicalGate>,
    bench_run: Option<CommandSpec>,
}

#[derive(Serialize)]
struct CanonicalGate {
    benchmark: String,
    metric: PerformanceMetric,
    bound: GateBound,
    params: BTreeMap<String, String>,
    execution: CanonicalBenchmarkExecution,
}

#[derive(Serialize)]
struct CanonicalBenchmarkExecution {
    command: Vec<String>,
    fixture: String,
    matrix: BTreeMap<String, Vec<String>>,
}

fn canonical_stage_gates(
    parsed: &StageBenchmarks,
    language: &LanguageSpec,
) -> Result<CanonicalStageGates> {
    let mut gates = Vec::new();
    for gate in &parsed.performance_gates {
        let benchmark = parsed
            .benchmarks
            .iter()
            .find(|item| item.name == gate.benchmark)
            .with_context(|| {
                format!(
                    "gate {} references missing benchmark {}",
                    gate.name, gate.benchmark
                )
            })?;
        let matrix = benchmark
            .matrix
            .iter()
            .map(|(key, values)| {
                let values = values
                    .iter()
                    .map(|value| {
                        benchmark_scalar_to_string(value).with_context(|| {
                            format!("benchmark {} has non-scalar matrix value", benchmark.name)
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                Ok((key.clone(), values))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        gates.push(CanonicalGate {
            benchmark: gate.benchmark.clone(),
            metric: gate.metric,
            bound: gate
                .bound()
                .with_context(|| format!("gate {} has invalid bound", gate.name))?,
            params: gate.params.clone(),
            execution: CanonicalBenchmarkExecution {
                command: benchmark.command.clone(),
                fixture: benchmark.fixture.clone(),
                matrix,
            },
        });
    }
    gates.sort_by_key(|gate| serde_json::to_vec(gate).unwrap_or_default());
    Ok(CanonicalStageGates {
        gates,
        bench_run: language.bench_run.clone(),
    })
}

pub fn builtin_packs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("packs")
}

/// Per-user cache root for extracted embedded packs.
///
/// Uses `$XDG_CACHE_HOME` / `~/.cache/deltaforge` on Unix and
/// `%LOCALAPPDATA%\deltaforge` on Windows, falling back to the system temp
/// directory when neither is available. A per-user location avoids the
/// world-writable shared temp dir, where another user could pre-create and
/// poison packs that DeltaForge then executes build/run commands from.
pub fn embedded_cache_root() -> PathBuf {
    #[cfg(windows)]
    {
        if let Some(local) = env::var_os("LOCALAPPDATA").filter(|value| !value.is_empty()) {
            return PathBuf::from(local).join("deltaforge");
        }
    }
    #[cfg(not(windows))]
    {
        if let Some(xdg) = env::var_os("XDG_CACHE_HOME").filter(|value| !value.is_empty()) {
            return PathBuf::from(xdg).join("deltaforge");
        }
        if let Some(home) = env::var_os("HOME").filter(|value| !value.is_empty()) {
            return PathBuf::from(home).join(".cache").join("deltaforge");
        }
    }
    std::env::temp_dir().join("deltaforge")
}

fn collect_embedded_entries(dir: &Dir<'_>, entries: &mut Vec<(String, Vec<u8>)>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(child) => collect_embedded_entries(child, entries),
            DirEntry::File(file) => {
                let name = file.path().to_string_lossy().replace('\\', "/");
                entries.push((name, file.contents().to_vec()));
            }
        }
    }
}

/// Content digest of the compiled-in pack tree. Same content yields the same
/// digest (and therefore the same cache directory) regardless of crate version;
/// changed content yields a new digest and a fresh cache directory.
fn embedded_packs_digest() -> String {
    let mut entries = Vec::new();
    collect_embedded_entries(&EMBEDDED_PACKS, &mut entries);
    digest_named_contents(entries)
}

fn embedded_staging_dir(parent: &Path) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or(0);
    parent.join(format!(".packs-staging-{}-{nanos}", std::process::id()))
}

pub fn embedded_packs_dir() -> Result<PathBuf> {
    let digest = embedded_packs_digest();
    let hash = digest.strip_prefix("fnv1a64:").unwrap_or(&digest);
    let target = embedded_cache_root().join(format!("packs-{hash}"));

    if target.join("flashindex").join("project.yaml").is_file() {
        return Ok(target);
    }

    let parent = target
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    fs::create_dir_all(&parent).with_context(|| {
        format!(
            "failed to create embedded pack cache directory {}",
            parent.display()
        )
    })?;

    // Extract into a unique sibling directory, then atomically rename it into
    // place. A partially extracted staging directory never becomes the target,
    // so an interrupted extract is repaired on the next run rather than
    // silently reused.
    let staging = embedded_staging_dir(&parent);
    let _ = fs::remove_dir_all(&staging);
    EMBEDDED_PACKS.extract(&staging).with_context(|| {
        format!(
            "failed to extract bundled packs to staging directory {}",
            staging.display()
        )
    })?;

    match fs::rename(&staging, &target) {
        Ok(()) => Ok(target),
        Err(error) => {
            // Lost the race to another process (or the target already exists):
            // discard our staging copy and use the installed directory.
            let _ = fs::remove_dir_all(&staging);
            if target.join("flashindex").join("project.yaml").is_file() {
                Ok(target)
            } else {
                Err(error).with_context(|| {
                    format!("failed to install bundled packs into {}", target.display())
                })
            }
        }
    }
}

/// Whether a pinned `pack_source` string refers to a bundled/embedded pack.
///
/// Recognizes the logical `"bundled"` marker, the current per-user cache
/// location, the builtin dev-tree packs directory, and the legacy shared-temp
/// cache path (`deltaforge-embedded-packs-*`) written by older versions.
pub fn is_bundled_source(source: &str) -> bool {
    if source == "bundled" {
        return true;
    }
    is_bundled_pack_root(Path::new(source))
}

/// The logical `pack_source` label to pin for a discovered pack: `"bundled"`
/// for embedded/builtin packs, or the canonicalized absolute path for external
/// packs supplied via `--packs-dir` / `DELTAFORGE_PACKS_DIR`.
pub fn pack_source_label(pack_root: &Path) -> String {
    if is_bundled_pack_root(pack_root) {
        "bundled".to_string()
    } else {
        pack_root
            .canonicalize()
            .unwrap_or_else(|_| pack_root.to_path_buf())
            .display()
            .to_string()
    }
}

fn is_bundled_pack_root(path: &Path) -> bool {
    if path.components().any(|component| {
        matches!(component, Component::Normal(name)
            if name.to_string_lossy().starts_with("deltaforge-embedded-packs"))
    }) {
        return true;
    }
    path_is_under(path, &embedded_cache_root()) || path_is_under(path, &builtin_packs_dir())
}

fn path_is_under(path: &Path, base: &Path) -> bool {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let base = base.canonicalize().unwrap_or_else(|_| base.to_path_buf());
    path.starts_with(&base)
}

pub fn pack_search_dirs(options: &PackSearchOptions) -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(dir) = &options.packs_dir {
        dirs.push(dir.clone());
    }

    if let Some(dir) = env::var_os("DELTAFORGE_PACKS_DIR") {
        dirs.push(PathBuf::from(dir));
    }

    dirs.push(builtin_packs_dir());
    if let Ok(dir) = embedded_packs_dir() {
        dirs.push(dir);
    }
    dirs
}

#[cfg(test)]
pub fn load_builtin_pack(project_id: &str) -> Result<LoadedPack> {
    load_pack(project_id, &PackSearchOptions::default())
}

pub fn load_pack(project_id: &str, options: &PackSearchOptions) -> Result<LoadedPack> {
    if !is_safe_relative_path(Path::new(project_id))
        || Path::new(project_id).components().count() != 1
    {
        bail!("unsafe project pack id: {project_id}");
    }
    let mut searched_manifests = Vec::new();

    for packs_dir in pack_search_dirs(options) {
        let root = packs_dir.join(project_id);
        let manifest_path = root.join("project.yaml");
        searched_manifests.push(manifest_path.clone());

        if !manifest_path.is_file() {
            continue;
        }

        return load_pack_from_manifest(project_id, root, &manifest_path);
    }

    let searched = searched_manifests
        .iter()
        .map(|path| format!("  {}", path.display()))
        .collect::<Vec<_>>()
        .join("\n");

    bail!("could not find project pack {project_id}. Searched:\n{searched}");
}

/// A pack directory that could not be loaded during discovery.
#[derive(Debug, Clone)]
pub struct PackProblem {
    pub path: PathBuf,
    pub error: String,
}

/// The outcome of scanning every search directory for packs: successfully
/// loaded packs plus a list of directories that failed to parse or validate.
/// A single malformed pack no longer aborts discovery for every command.
#[derive(Debug, Clone, Default)]
pub struct PackDiscovery {
    pub packs: Vec<LoadedPack>,
    pub problems: Vec<PackProblem>,
}

pub fn discover_packs_with_options(options: &PackSearchOptions) -> Result<PackDiscovery> {
    let mut discovery = PackDiscovery::default();
    let mut seen_ids = BTreeSet::new();

    for packs_dir in pack_search_dirs(options) {
        if !packs_dir.is_dir() {
            continue;
        }

        for entry in fs::read_dir(&packs_dir)
            .with_context(|| format!("failed to read packs directory {}", packs_dir.display()))?
        {
            let entry = entry
                .with_context(|| format!("failed to read entry in {}", packs_dir.display()))?;
            let root = entry.path();
            if !root.is_dir() {
                continue;
            }

            let manifest_path = root.join("project.yaml");
            if !manifest_path.is_file() {
                continue;
            }

            match load_discovered_pack(&manifest_path, root.clone()) {
                Ok(pack) => {
                    if seen_ids.insert(pack.manifest.id.clone()) {
                        discovery.packs.push(pack);
                    }
                }
                Err(error) => discovery.problems.push(PackProblem {
                    path: manifest_path,
                    error: format!("{error:#}"),
                }),
            }
        }
    }

    discovery
        .packs
        .sort_by(|left, right| left.manifest.id.cmp(&right.manifest.id));
    discovery
        .problems
        .sort_by(|left, right| left.path.cmp(&right.path));
    Ok(discovery)
}

fn load_discovered_pack(manifest_path: &Path, root: PathBuf) -> Result<LoadedPack> {
    let source = fs::read_to_string(manifest_path)
        .with_context(|| format!("failed to read pack manifest {}", manifest_path.display()))?;
    let manifest: ProjectPack = serde_yaml::from_str(&source)
        .with_context(|| format!("failed to parse pack manifest {}", manifest_path.display()))?;
    validate_pack_schema(&manifest, manifest_path)?;
    Ok(LoadedPack { root, manifest })
}

fn validate_pack_schema(manifest: &ProjectPack, path: &Path) -> Result<()> {
    if manifest.schema_version != current_pack_schema_version() {
        bail!(
            "unsupported pack schema_version {} in {}; expected {}",
            manifest.schema_version,
            path.display(),
            current_pack_schema_version()
        );
    }
    Ok(())
}

pub fn validate_pack(pack: &LoadedPack) -> Vec<String> {
    let mut problems = Vec::new();
    let manifest = &pack.manifest;

    if manifest.schema_version != current_pack_schema_version() {
        problems.push(format!(
            "manifest schema_version must be {}",
            current_pack_schema_version()
        ));
    }
    if manifest.id.trim().is_empty() {
        problems.push("manifest id is empty".to_string());
    }
    if manifest.name.trim().is_empty() {
        problems.push("manifest name is empty".to_string());
    }
    if manifest.version.trim().is_empty() {
        problems.push("manifest version is empty".to_string());
    }
    if manifest.description.trim().is_empty() {
        problems.push("manifest description is empty".to_string());
    }

    validate_languages(pack, &mut problems);
    validate_stages(pack, &mut problems);
    validate_pack_tree_is_self_contained(pack, &mut problems);

    problems
}

/// Walk the whole pack tree and report every symlink or special file. This is
/// base (not `--strict`) validation because such a pack is guaranteed to fail
/// later: `digest_pack_tree` rejects non-regular entries during init,
/// sync-pack, and behavioral proof computation. Authors should learn that at
/// validation time, with every offender listed in one pass. Metadata-only —
/// no file contents are read.
fn validate_pack_tree_is_self_contained(pack: &LoadedPack, problems: &mut Vec<String>) {
    collect_non_regular_entries(&pack.root, problems);
}

fn collect_non_regular_entries(dir: &Path, problems: &mut Vec<String>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) => {
            problems.push(format!(
                "pack directory is unreadable: {}: {error}",
                dir.display()
            ));
            return;
        }
    };
    for entry in entries {
        let Ok(entry) = entry else {
            problems.push(format!(
                "pack directory entry is unreadable in {}",
                dir.display()
            ));
            continue;
        };
        let path = entry.path();
        // Does not follow symlinks, so a symlinked directory is reported
        // rather than traversed.
        let Ok(file_type) = entry.file_type() else {
            problems.push(format!("pack entry is uninspectable: {}", path.display()));
            continue;
        };
        if file_type.is_dir() {
            collect_non_regular_entries(&path, problems);
        } else if file_type.is_symlink() {
            problems.push(format!(
                "pack content must be self-contained: {} is a symbolic link; replace it with a regular copy of its target",
                path.display()
            ));
        } else if !file_type.is_file() {
            problems.push(format!(
                "pack content must be self-contained: {} is a special file",
                path.display()
            ));
        }
    }
}

fn current_pack_schema_version() -> u32 {
    1
}

fn validate_languages(pack: &LoadedPack, problems: &mut Vec<String>) {
    if pack.manifest.languages.is_empty() {
        problems.push("manifest defines no languages".to_string());
        return;
    }

    for (language, spec) in &pack.manifest.languages {
        if !is_safe_relative_path(&spec.template) {
            problems.push(format!(
                "language {language} template path is unsafe: {}",
                spec.template.display()
            ));
            continue;
        }
        let template = pack.root.join(&spec.template);
        if !template.is_dir() {
            problems.push(format!(
                "language {language} template directory is missing: {}",
                template.display()
            ));
        }

        if spec.run.command.is_empty() {
            problems.push(format!("language {language} run command is empty"));
        }

        if let Some(build) = &spec.build
            && build.command.is_empty()
        {
            problems.push(format!("language {language} build command is empty"));
        }

        if let Some(bench_run) = &spec.bench_run
            && bench_run.command.is_empty()
        {
            problems.push(format!("language {language} bench_run command is empty"));
        }
    }
}

fn validate_stages(pack: &LoadedPack, problems: &mut Vec<String>) {
    if pack.manifest.stages.is_empty() {
        problems.push("manifest defines no stages".to_string());
        return;
    }

    let mut stage_ids = BTreeSet::new();
    for stage in &pack.manifest.stages {
        if stage.id.trim().is_empty() {
            problems.push("stage id is empty".to_string());
        }
        if !stage_ids.insert(stage.id.clone()) {
            problems.push(format!("duplicate stage id {}", stage.id));
        }
        if stage.title.trim().is_empty() {
            problems.push(format!("stage {} title is empty", stage.id));
        }

        if !is_safe_relative_path(&stage.path) {
            problems.push(format!(
                "stage {} path is unsafe: {}",
                stage.id,
                stage.path.display()
            ));
            continue;
        }

        let stage_dir = pack.stage_dir(stage);
        if !stage_dir.is_dir() {
            problems.push(format!(
                "stage {} directory is missing: {}",
                stage.id,
                stage_dir.display()
            ));
            continue;
        }

        for required_file in ["instructions.md", "hints.md", "tests.yaml"] {
            let path = stage_dir.join(required_file);
            if !path.is_file() {
                problems.push(format!(
                    "stage {} missing required file {}",
                    stage.id,
                    path.display()
                ));
            }
        }

        validate_stage_tests(pack, stage, &stage_dir, problems);
        validate_stage_benchmarks(pack, stage, &stage_dir, problems);
    }
}

fn validate_stage_tests(
    pack: &LoadedPack,
    stage: &StageSpec,
    stage_dir: &Path,
    problems: &mut Vec<String>,
) {
    let tests_path = stage_dir.join("tests.yaml");
    if !tests_path.is_file() {
        return;
    }

    let source = match fs::read_to_string(&tests_path) {
        Ok(source) => source,
        Err(error) => {
            problems.push(format!(
                "stage {} tests file is unreadable {}: {error}",
                stage.id,
                tests_path.display()
            ));
            return;
        }
    };

    problems.extend(validate_stage_tests_source(pack, stage, &source));
}

pub fn validate_stage_tests_source(
    pack: &LoadedPack,
    stage: &StageSpec,
    source: &str,
) -> Vec<String> {
    let mut problems = Vec::new();
    let tests: StageTests = match serde_yaml::from_str(source) {
        Ok(tests) => tests,
        Err(error) => {
            problems.push(format!(
                "stage {} tests file is invalid YAML: {error}",
                stage.id
            ));
            return problems;
        }
    };

    if tests.tests.is_empty() {
        problems.push(format!("stage {} defines no tests", stage.id));
    }

    for test in tests.tests {
        if test.name.trim().is_empty() {
            problems.push(format!(
                "stage {} contains a test with an empty name",
                stage.id
            ));
        }
        if test.command.is_empty() {
            problems.push(format!(
                "stage {} test {} command is empty",
                stage.id, test.name
            ));
        }
        validate_expectations(stage, &test.name, &test.expect, &mut problems);
        if let Some(fixture) = test.fixture {
            if !is_safe_relative_path(Path::new(&fixture)) {
                problems.push(format!(
                    "stage {} test {} fixture path is unsafe: {}",
                    stage.id, test.name, fixture
                ));
                continue;
            }
            let fixture_path = pack.fixture_path(stage, &fixture);
            if !fixture_path.is_dir() {
                problems.push(format!(
                    "stage {} references missing fixture {}",
                    stage.id,
                    fixture_path.display()
                ));
            }
        }
    }
    problems
}

fn validate_expectations(
    stage: &StageSpec,
    test_name: &str,
    expect: &Expectations,
    problems: &mut Vec<String>,
) {
    let has_assertion = expect.exit_code.is_some()
        || expect.stdout_exact.is_some()
        || !expect.stdout_contains.is_empty()
        || !expect.stdout_not_contains.is_empty()
        || !expect.stderr_contains.is_empty()
        || !expect.file_exists.is_empty()
        || !expect.file_not_exists.is_empty()
        || !expect.file_contains.is_empty()
        || !expect.regex_match.is_empty()
        || expect.json_equals.is_some();
    if !has_assertion {
        problems.push(format!(
            "stage {} test {test_name} defines no assertions",
            stage.id
        ));
    }
    if expect.timeout_ms == Some(0) {
        problems.push(format!(
            "stage {} test {test_name} timeout_ms must be greater than 0",
            stage.id
        ));
    }
    for value in expect
        .file_exists
        .iter()
        .chain(&expect.file_not_exists)
        .chain(expect.file_contains.iter().map(|item| &item.path))
    {
        if !safe_expectation_path(value) {
            problems.push(format!(
                "stage {} test {test_name} expectation path is unsafe: {value}",
                stage.id
            ));
        }
    }
}

pub fn safe_expectation_path(value: &str) -> bool {
    let stripped = value.strip_prefix("{temp_dir}/").unwrap_or(value);
    !value.starts_with('/')
        && !value.starts_with('\\')
        && !value.contains(':')
        && is_safe_relative_path(Path::new(stripped))
}

fn validate_stage_benchmarks(
    pack: &LoadedPack,
    stage: &StageSpec,
    stage_dir: &Path,
    problems: &mut Vec<String>,
) {
    let benchmarks_path = stage_dir.join("benchmarks.yaml");
    if !benchmarks_path.is_file() {
        return;
    }

    let source = match fs::read_to_string(&benchmarks_path) {
        Ok(source) => source,
        Err(error) => {
            problems.push(format!(
                "stage {} benchmarks file is unreadable {}: {error}",
                stage.id,
                benchmarks_path.display()
            ));
            return;
        }
    };

    problems.extend(validate_stage_benchmarks_source(pack, stage, &source));
}

pub fn validate_stage_benchmarks_source(
    pack: &LoadedPack,
    stage: &StageSpec,
    source: &str,
) -> Vec<String> {
    let mut problems = Vec::new();
    let benchmarks: StageBenchmarks = match serde_yaml::from_str(source) {
        Ok(benchmarks) => benchmarks,
        Err(error) => {
            problems.push(format!(
                "stage {} benchmarks file is invalid YAML: {error}",
                stage.id
            ));
            return problems;
        }
    };

    if benchmarks.benchmarks.is_empty() {
        problems.push(format!("stage {} defines no benchmarks", stage.id));
    }

    let mut names = BTreeSet::new();
    for benchmark in &benchmarks.benchmarks {
        let name = benchmark.name.clone();
        if name.trim().is_empty() {
            problems.push(format!("stage {} benchmark name is empty", stage.id));
        }
        if !names.insert(name.clone()) {
            problems.push(format!(
                "stage {} benchmark name is duplicated: {name}",
                stage.id
            ));
        }
        if benchmark.command.is_empty() {
            problems.push(format!(
                "stage {} benchmark {} command is empty",
                stage.id, name
            ));
        }
        if matches!(benchmark.iterations, Some(0)) {
            problems.push(format!(
                "stage {} benchmark {} iterations must be greater than 0",
                stage.id, name
            ));
        }
        if matches!(benchmark.timeout_ms, Some(0)) {
            problems.push(format!(
                "stage {} benchmark {} timeout_ms must be greater than 0",
                stage.id, name
            ));
        }
        problems.extend(
            benchmark_matrix_problems(&benchmark.matrix)
                .into_iter()
                .map(|problem| format!("stage {} benchmark {} {problem}", stage.id, name)),
        );
        for arg in &benchmark.command {
            for placeholder in placeholder_identifiers(arg) {
                if !benchmark.matrix.contains_key(&placeholder)
                    && !BENCHMARK_BUILTIN_PLACEHOLDERS.contains(&placeholder.as_str())
                {
                    problems.push(format!(
                        "stage {} benchmark {} command references undeclared parameter {{{placeholder}}}",
                        stage.id, name
                    ));
                }
            }
        }
        if !is_safe_relative_path(Path::new(&benchmark.fixture)) {
            problems.push(format!(
                "stage {} benchmark {} fixture path is unsafe: {}",
                stage.id, name, benchmark.fixture
            ));
            continue;
        }
        let fixture_path = pack.fixture_path(stage, &benchmark.fixture);
        if !fixture_path.is_dir() {
            problems.push(format!(
                "stage {} benchmark {} references missing fixture {}",
                stage.id,
                name,
                fixture_path.display()
            ));
        }
    }
    let mut gate_names = BTreeSet::new();
    for gate in &benchmarks.performance_gates {
        if gate.name.trim().is_empty() {
            problems.push(format!("stage {} performance gate name is empty", stage.id));
        }
        if !gate_names.insert(gate.name.clone()) {
            problems.push(format!(
                "stage {} performance gate name is duplicated: {}",
                stage.id, gate.name
            ));
        }
        let matching = benchmarks
            .benchmarks
            .iter()
            .filter(|benchmark| benchmark.name == gate.benchmark)
            .collect::<Vec<_>>();
        let [benchmark] = matching.as_slice() else {
            problems.push(format!(
                "stage {} gate {} references missing or ambiguous benchmark {}",
                stage.id, gate.name, gate.benchmark
            ));
            continue;
        };
        if gate.bound().is_none() {
            problems.push(format!(
                "stage {} gate {} must define exactly one finite min or max bound",
                stage.id, gate.name
            ));
        }
        if gate.metric == PerformanceMetric::Speedup {
            if !gate.params.is_empty() {
                problems.push(format!(
                    "stage {} gate {} speedup gates require empty params",
                    stage.id, gate.name
                ));
            }
            let Some(values) = benchmark.matrix.get("threads") else {
                problems.push(format!(
                    "stage {} gate {} speedup benchmark requires a threads matrix",
                    stage.id, gate.name
                ));
                continue;
            };
            if benchmark.matrix.len() != 1 || values.len() < 2 {
                problems.push(format!(
                    "stage {} gate {} speedup benchmark requires at least two threads-only values",
                    stage.id, gate.name
                ));
            } else {
                let parsed_values = values
                    .iter()
                    .map(|value| {
                        benchmark_scalar_to_string(value)
                            .and_then(|value| value.parse::<u64>().ok())
                    })
                    .collect::<Vec<_>>();
                if parsed_values.iter().any(Option::is_none) {
                    problems.push(format!(
                        "stage {} gate {} speedup threads must all be unsigned integers",
                        stage.id, gate.name
                    ));
                } else {
                    let numeric = parsed_values.into_iter().flatten().collect::<BTreeSet<_>>();
                    if numeric.len() != values.len() {
                        problems.push(format!(
                            "stage {} gate {} speedup threads must not contain duplicate values",
                            stage.id, gate.name
                        ));
                    }
                }
            }
        } else if benchmark.matrix.is_empty() {
            if !gate.params.is_empty() {
                problems.push(format!(
                    "stage {} gate {} params are only allowed for matrix benchmarks",
                    stage.id, gate.name
                ));
            }
        } else {
            if gate.params.len() != benchmark.matrix.len()
                || !benchmark
                    .matrix
                    .keys()
                    .all(|key| gate.params.contains_key(key))
            {
                problems.push(format!(
                    "stage {} gate {} params must select every matrix key exactly once",
                    stage.id, gate.name
                ));
            }
            for (key, value) in &gate.params {
                match benchmark.matrix.get(key) {
                    Some(values)
                        if values
                            .iter()
                            .filter_map(benchmark_scalar_to_string)
                            .any(|candidate| candidate == *value) => {}
                    Some(_) => problems.push(format!(
                        "stage {} gate {} params.{key} is not a matrix value",
                        stage.id, gate.name
                    )),
                    None => problems.push(format!(
                        "stage {} gate {} params contains unknown matrix key {key}",
                        stage.id, gate.name
                    )),
                }
            }
        }
    }
    problems
}

fn load_pack_from_manifest(
    project_id: &str,
    root: PathBuf,
    manifest_path: &Path,
) -> Result<LoadedPack> {
    let manifest_source = fs::read_to_string(manifest_path)
        .with_context(|| format!("failed to read pack manifest {}", manifest_path.display()))?;
    let manifest: ProjectPack = serde_yaml::from_str(&manifest_source)
        .with_context(|| format!("failed to parse pack manifest {}", manifest_path.display()))?;
    validate_pack_schema(&manifest, manifest_path)?;

    if manifest.id != project_id {
        bail!(
            "pack manifest id mismatch: requested {project_id}, found {}",
            manifest.id
        );
    }

    let pack = LoadedPack { root, manifest };
    let unsafe_problems = validate_pack(&pack)
        .into_iter()
        .filter(|problem| problem.contains("unsafe"))
        .collect::<Vec<_>>();
    if !unsafe_problems.is_empty() {
        bail!(
            "pack contains unsafe paths:\n{}",
            unsafe_problems.join("\n")
        );
    }
    Ok(pack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_flashindex_pack() {
        let pack = load_builtin_pack("flashindex").unwrap();

        assert_eq!(pack.manifest.id, "flashindex");
        assert!(pack.manifest.language("rust").is_some());
        assert_eq!(pack.manifest.first_stage().unwrap().id, "01_scan_files");
        assert_eq!(
            pack.manifest.next_stage("01_scan_files").unwrap().id,
            "02_filter_files"
        );
    }

    #[test]
    fn flashindex_pack_validates() {
        let pack = load_builtin_pack("flashindex").unwrap();
        assert_eq!(validate_pack(&pack), Vec::<String>::new());
    }

    #[test]
    fn embedded_flashindex_pack_is_available() {
        let dir = embedded_packs_dir().unwrap();
        assert!(dir.join("flashindex/project.yaml").is_file());
    }

    #[test]
    fn valid_benchmark_matrix_passes_validation() {
        let pack = load_builtin_pack("flashindex").unwrap();
        let stage = pack.manifest.first_stage().unwrap().clone();
        let source = r#"
benchmarks:
  - name: scan_threads
    fixture: basic_project
    command: ["scan", "{fixture_path}", "--threads", "{threads}", "--mode", "{mode}"]
    matrix:
      threads: [1, 2, 4]
      mode: ["fast", "safe"]
"#;
        assert_eq!(
            validate_stage_benchmarks_source(&pack, &stage, source),
            Vec::<String>::new()
        );
    }

    #[test]
    fn invalid_benchmark_matrix_reports_each_problem() {
        let pack = load_builtin_pack("flashindex").unwrap();
        let stage = pack.manifest.first_stage().unwrap().clone();
        let source = r#"
benchmarks:
  - name: matrix_bad
    fixture: basic_project
    command: ["scan", "{fixture_path}", "--threads", "{threads}", "{undeclared}"]
    matrix:
      threads: []
      bad-name: [1]
      temp_dir: [2]
      nested: [[1, 2]]
"#;
        let problems = validate_stage_benchmarks_source(&pack, &stage, source);
        let all = problems.join("\n");
        assert!(all.contains("threads has no values"), "{all}");
        assert!(all.contains("bad-name is not a valid identifier"), "{all}");
        assert!(all.contains("temp_dir shadows a built-in"), "{all}");
        assert!(all.contains("nested has a non-scalar value"), "{all}");
        assert!(all.contains("undeclared parameter {undeclared}"), "{all}");
        assert!(
            !all.contains("{threads}"),
            "declared placeholder flagged: {all}"
        );
        assert!(
            !all.contains("{fixture_path}"),
            "built-in placeholder flagged: {all}"
        );
    }

    #[test]
    fn speedup_validation_rejects_duplicate_and_non_numeric_threads() {
        let pack = load_builtin_pack("flashindex").unwrap();
        let stage = pack.manifest.first_stage().unwrap().clone();
        for (matrix, expected) in [
            ("[1, 1, 8]", "must not contain duplicate values"),
            ("[1, 8, invalid]", "must all be unsigned integers"),
        ] {
            let source = format!(
                r#"
benchmarks:
  - name: scan_threads
    fixture: basic_project
    command: ["scan", "{{fixture_path}}", "--threads", "{{threads}}"]
    matrix:
      threads: {matrix}
performance_gates:
  - name: scaling
    benchmark: scan_threads
    metric: speedup
    min: 2
"#
            );
            let problems = validate_stage_benchmarks_source(&pack, &stage, &source).join("\n");
            assert!(problems.contains(expected), "{problems}");
        }
    }

    #[test]
    fn speedup_validation_accepts_distinct_numeric_threads() {
        let pack = load_builtin_pack("flashindex").unwrap();
        let stage = pack.manifest.first_stage().unwrap().clone();
        let source = r#"
benchmarks:
  - name: scan_threads
    fixture: basic_project
    command: ["scan", "{fixture_path}", "--threads", "{threads}"]
    matrix:
      threads: [1, 2, 8]
performance_gates:
  - name: scaling
    benchmark: scan_threads
    metric: speedup
    min: 2
"#;
        assert_eq!(
            validate_stage_benchmarks_source(&pack, &stage, source),
            Vec::<String>::new()
        );
    }
}
