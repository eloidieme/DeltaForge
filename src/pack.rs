use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};

use crate::integrity::is_safe_relative_path;
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

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ValidationBenchmarks {
    #[serde(default)]
    benchmarks: Vec<ValidationBenchmark>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ValidationBenchmark {
    name: Option<String>,
    fixture: Option<String>,
    #[serde(default)]
    command: Vec<String>,
    iterations: Option<u64>,
    warmup: Option<u64>,
    timeout_ms: Option<u64>,
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
}

pub fn builtin_packs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("packs")
}

pub fn embedded_packs_dir() -> Result<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "deltaforge-embedded-packs-{}",
        env!("CARGO_PKG_VERSION")
    ));
    if !path.join("flashindex").join("project.yaml").is_file() {
        EMBEDDED_PACKS.extract(&path).with_context(|| {
            format!(
                "failed to extract bundled packs to cache directory {}",
                path.display()
            )
        })?;
    }
    Ok(path)
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

pub fn discover_packs_with_options(options: &PackSearchOptions) -> Result<Vec<LoadedPack>> {
    let mut packs = Vec::new();
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

            let source = fs::read_to_string(&manifest_path).with_context(|| {
                format!("failed to read pack manifest {}", manifest_path.display())
            })?;
            let manifest: ProjectPack = serde_yaml::from_str(&source).with_context(|| {
                format!("failed to parse pack manifest {}", manifest_path.display())
            })?;
            validate_pack_schema(&manifest, &manifest_path)?;

            if seen_ids.insert(manifest.id.clone()) {
                packs.push(LoadedPack { root, manifest });
            }
        }
    }

    packs.sort_by(|left, right| left.manifest.id.cmp(&right.manifest.id));
    Ok(packs)
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

    problems
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
    let benchmarks: ValidationBenchmarks = match serde_yaml::from_str(source) {
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

    for benchmark in benchmarks.benchmarks {
        let name = benchmark.name.unwrap_or_else(|| "<unnamed>".to_string());
        if name.trim().is_empty() {
            problems.push(format!("stage {} benchmark name is empty", stage.id));
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
        if let Some(fixture) = benchmark.fixture {
            if !is_safe_relative_path(Path::new(&fixture)) {
                problems.push(format!(
                    "stage {} benchmark {} fixture path is unsafe: {}",
                    stage.id, name, fixture
                ));
                continue;
            }
            let fixture_path = pack.fixture_path(stage, &fixture);
            if !fixture_path.is_dir() {
                problems.push(format!(
                    "stage {} benchmark {} references missing fixture {}",
                    stage.id,
                    name,
                    fixture_path.display()
                ));
            }
        } else {
            problems.push(format!(
                "stage {} benchmark {} fixture is missing",
                stage.id, name
            ));
        }
        let _ = benchmark.warmup;
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
}
