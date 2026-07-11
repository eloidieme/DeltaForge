use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::fs_util::atomic_write;
use crate::integrity::is_safe_relative_path;
use crate::pack::{
    LoadedPack, PackSearchOptions, load_pack, validate_pack, validate_stage_benchmarks_source,
    validate_stage_tests_source,
};

const MAX_AUTHORED_TEXT_BYTES: usize = 1024 * 1024;
const MAX_METADATA_BYTES: usize = 4096;
const MAX_TOPICS: usize = 256;

#[derive(Debug, Clone)]
pub struct NewPackRequest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub dest: PathBuf,
    pub language: String,
    pub force: bool,
}

#[derive(Debug, Clone)]
pub struct AddStageRequest {
    pub pack_dir: PathBuf,
    pub id: String,
    pub title: String,
    pub force: bool,
}

#[derive(Debug, Clone)]
pub struct CheckReferenceRequest {
    pub project: String,
    pub language: String,
    pub reference: PathBuf,
    pub packs_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct UpdatePackMetadataRequest {
    pub project: String,
    pub packs_dir: PathBuf,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub topics: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct UpdateStageMetadataRequest {
    pub project: String,
    pub packs_dir: PathBuf,
    pub stage: String,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct WriteStageDocumentRequest {
    pub project: String,
    pub packs_dir: PathBuf,
    pub stage: String,
    pub document: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ReplaceStageTestsRequest {
    pub project: String,
    pub packs_dir: PathBuf,
    pub stage: String,
    pub tests: Value,
}

#[derive(Debug, Clone)]
pub struct WriteFixtureFileRequest {
    pub project: String,
    pub packs_dir: PathBuf,
    pub stage: String,
    pub fixture: String,
    pub path: PathBuf,
    pub content: String,
    pub overwrite: bool,
}

#[derive(Debug, Clone)]
pub struct ReplaceStageBenchmarksRequest {
    pub project: String,
    pub packs_dir: PathBuf,
    pub stage: String,
    pub benchmarks: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthoringReport {
    pub status: String,
    pub pack: Option<String>,
    pub path: Option<String>,
    pub problems: Vec<String>,
    pub next_actions: Vec<String>,
}

impl AuthoringReport {
    pub fn ok(pack: impl Into<String>, path: impl AsRef<Path>, next_actions: Vec<String>) -> Self {
        Self {
            status: "ok".to_string(),
            pack: Some(pack.into()),
            path: Some(path.as_ref().display().to_string()),
            problems: Vec::new(),
            next_actions,
        }
    }

    pub fn blocked(
        pack: Option<String>,
        path: Option<&Path>,
        problems: Vec<String>,
        next_actions: Vec<String>,
    ) -> Self {
        Self {
            status: "blocked".to_string(),
            pack,
            path: path.map(|path| path.display().to_string()),
            problems,
            next_actions,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status == "ok"
    }
}

pub fn create_pack(request: &NewPackRequest) -> Result<AuthoringReport> {
    validate_identifier(&request.id)?;
    if request.language != "rust" {
        bail!("only rust templates are currently scaffolded");
    }

    let final_root = request.dest.join(&request.id);
    if final_root.exists() && !request.force {
        bail!(
            "pack directory already exists: {}\nUse --force to replace it.",
            final_root.display()
        );
    }
    fs::create_dir_all(&request.dest)?;
    let root = unique_sibling(&final_root, "prepared")?;

    fs::create_dir_all(root.join("templates/rust/src"))?;
    fs::create_dir_all(root.join("stages/01_first_stage/fixtures/example"))?;

    atomic_write(root.join("project.yaml").as_path(), project_yaml(request))?;
    atomic_write(root.join("README.md").as_path(), readme(request))?;
    atomic_write(
        root.join("templates/rust/Cargo.toml").as_path(),
        template_cargo_toml(&request.id),
    )?;
    atomic_write(
        root.join("templates/rust/Cargo.lock").as_path(),
        "# This file is intentionally minimal for the starter template.\nversion = 4\n",
    )?;
    atomic_write(
        root.join("templates/rust/src/main.rs").as_path(),
        starter_main(&request.id),
    )?;
    atomic_write(
        root.join("stages/01_first_stage/instructions.md").as_path(),
        first_stage_instructions(&request.id),
    )?;
    atomic_write(
        root.join("stages/01_first_stage/hints.md").as_path(),
        "# Hint 1\n\nStart by making the CLI parse the command shape from the instructions.\n",
    )?;
    atomic_write(
        root.join("stages/01_first_stage/tests.yaml").as_path(),
        first_stage_tests(&request.id),
    )?;
    atomic_write(
        root.join("stages/01_first_stage/fixtures/example/input.txt")
            .as_path(),
        "hello deltaforge\n",
    )?;

    replace_directory(&root, &final_root)?;

    Ok(AuthoringReport::ok(
        request.id.clone(),
        &final_root,
        vec![
            "Edit README.md with the project context and learning outcomes.".to_string(),
            "Replace the starter stage with real behavior-specific tests.".to_string(),
            "Add a reference solution and run pack check-reference.".to_string(),
            format!(
                "Run: deltaforge --packs-dir {} validate-pack {}",
                request.dest.display(),
                request.id
            ),
        ],
    ))
}

pub fn add_stage(request: &AddStageRequest) -> Result<AuthoringReport> {
    validate_identifier(&request.id)?;
    let manifest_path = request.pack_dir.join("project.yaml");
    if !manifest_path.is_file() {
        bail!(
            "pack manifest is missing: {}\nPass --pack-dir pointing at a pack root.",
            manifest_path.display()
        );
    }

    let source = fs::read_to_string(&manifest_path)
        .with_context(|| format!("failed to read {}", manifest_path.display()))?;
    let mut manifest: serde_yaml::Value = serde_yaml::from_str(&source)
        .with_context(|| format!("failed to parse {}", manifest_path.display()))?;

    let stages = manifest
        .get_mut("stages")
        .and_then(serde_yaml::Value::as_sequence_mut)
        .context("project.yaml must contain a stages list")?;
    if stages
        .iter()
        .any(|stage| stage.get("id").and_then(serde_yaml::Value::as_str) == Some(&request.id))
    {
        bail!("stage {} already exists", request.id);
    }

    let stage_path = format!("stages/{}", request.id);
    let stage_dir = request.pack_dir.join(&stage_path);
    if stage_dir.exists() && !request.force {
        bail!(
            "stage directory already exists: {}\nUse --force to write scaffold files.",
            stage_dir.display()
        );
    }
    stages.push(serde_yaml::to_value(StageManifestEntry {
        id: request.id.clone(),
        title: request.title.clone(),
        path: stage_path.clone(),
    })?);
    let prepared_dir = unique_sibling(&stage_dir, "prepared")?;
    fs::create_dir_all(prepared_dir.join("fixtures/example"))?;
    atomic_write(
        prepared_dir.join("instructions.md").as_path(),
        stage_instructions(&request.id, &request.title),
    )?;
    atomic_write(
        prepared_dir.join("hints.md").as_path(),
        "# Hint 1\n\nIdentify the smallest CLI behavior that should pass this stage first.\n",
    )?;
    atomic_write(
        prepared_dir.join("tests.yaml").as_path(),
        stage_tests(&request.title),
    )?;
    atomic_write(
        prepared_dir.join("fixtures/example/input.txt").as_path(),
        "replace me\n",
    )?;

    let backup = if stage_dir.exists() {
        let backup = unique_sibling(&stage_dir, "backup")?;
        fs::rename(&stage_dir, &backup)?;
        Some(backup)
    } else {
        None
    };
    if let Err(error) = fs::rename(&prepared_dir, &stage_dir) {
        if let Some(backup) = &backup {
            let _ = fs::rename(backup, &stage_dir);
        }
        return Err(error).context("failed to install prepared stage scaffold");
    }
    if let Err(error) = atomic_write(&manifest_path, serde_yaml::to_string(&manifest)?) {
        let _ = fs::remove_dir_all(&stage_dir);
        if let Some(backup) = &backup {
            let _ = fs::rename(backup, &stage_dir);
        }
        return Err(error).context("failed to update manifest; restored previous stage directory");
    }
    if let Some(backup) = backup {
        fs::remove_dir_all(backup)?;
    }

    Ok(AuthoringReport::ok(
        request.id.clone(),
        stage_dir,
        vec![
            "Replace placeholder tests with behavior-specific black-box tests.".to_string(),
            "Add edge cases, examples, and non-goals to instructions.md.".to_string(),
            "Run validate-pack after editing fixtures and tests.".to_string(),
        ],
    ))
}

pub fn update_pack_metadata(request: &UpdatePackMetadataRequest) -> Result<AuthoringReport> {
    let mut pack = load_explicit_authoring_pack(&request.project, &request.packs_dir)?;
    if request.name.is_none()
        && request.description.is_none()
        && request.version.is_none()
        && request.topics.is_none()
    {
        bail!("at least one pack metadata field must be provided");
    }

    if let Some(name) = &request.name {
        validate_nonempty_text("pack name", name)?;
        pack.manifest.name = name.clone();
    }
    if let Some(description) = &request.description {
        validate_nonempty_text("pack description", description)?;
        pack.manifest.description = description.clone();
    }
    if let Some(version) = &request.version {
        validate_nonempty_text("pack version", version)?;
        pack.manifest.version = version.clone();
    }
    if let Some(topics) = &request.topics {
        if topics.is_empty() {
            bail!("pack topics must contain at least one non-empty value");
        }
        if topics.len() > MAX_TOPICS {
            bail!("pack topics exceed the {MAX_TOPICS} item authoring limit");
        }
        for topic in topics {
            validate_nonempty_text("pack topic", topic)?;
        }
        pack.manifest.topics = topics.clone();
    }

    let relative = Path::new("project.yaml");
    reject_symlink_components(&pack.root, relative)?;
    let path = pack.root.join(relative);
    atomic_write(&path, serde_yaml::to_string(&pack.manifest)?)?;
    Ok(AuthoringReport::ok(
        request.project.clone(),
        path,
        vec!["Run validate_pack before reference checking.".to_string()],
    ))
}

pub fn update_stage_metadata(request: &UpdateStageMetadataRequest) -> Result<AuthoringReport> {
    validate_nonempty_text("stage title", &request.title)?;
    let mut pack = load_explicit_authoring_pack(&request.project, &request.packs_dir)?;
    let stage = pack
        .manifest
        .stages
        .iter_mut()
        .find(|stage| stage.id == request.stage)
        .with_context(|| {
            format!(
                "pack {} does not contain stage {}",
                request.project, request.stage
            )
        })?;
    stage.title = request.title.clone();

    let relative = Path::new("project.yaml");
    reject_symlink_components(&pack.root, relative)?;
    let path = pack.root.join(relative);
    atomic_write(&path, serde_yaml::to_string(&pack.manifest)?)?;
    Ok(AuthoringReport::ok(
        request.project.clone(),
        path,
        vec!["Review the renamed stage instructions and tests for consistency.".to_string()],
    ))
}

pub fn write_stage_document(request: &WriteStageDocumentRequest) -> Result<AuthoringReport> {
    validate_authored_content("stage document", &request.content, false)?;
    let pack = load_explicit_authoring_pack(&request.project, &request.packs_dir)?;
    let stage = require_stage(&pack, &request.stage)?;
    let file_name = match request.document.as_str() {
        "instructions" => "instructions.md",
        "hints" => "hints.md",
        "design_prompt" => "design_prompt.md",
        other => bail!(
            "unsupported stage document {other}; expected instructions, hints, or design_prompt"
        ),
    };
    let relative = stage.path.join(file_name);
    reject_symlink_components(&pack.root, &relative)?;
    let path = pack.root.join(relative);
    atomic_write(&path, &request.content)?;
    Ok(AuthoringReport::ok(
        request.project.clone(),
        path,
        vec!["Run diagnose_pack to check for remaining placeholders.".to_string()],
    ))
}

pub fn replace_stage_tests(request: &ReplaceStageTestsRequest) -> Result<AuthoringReport> {
    let pack = load_explicit_authoring_pack(&request.project, &request.packs_dir)?;
    let stage = require_stage(&pack, &request.stage)?;
    let source = structured_yaml("tests", &request.tests)?;
    validate_authored_content("tests YAML", &source, false)?;
    let problems = validate_stage_tests_source(&pack, stage, &source);
    if !problems.is_empty() {
        return Ok(validation_blocked(&pack, problems));
    }

    let relative = stage.path.join("tests.yaml");
    reject_symlink_components(&pack.root, &relative)?;
    let path = pack.root.join(relative);
    atomic_write(&path, source)?;
    Ok(AuthoringReport::ok(
        request.project.clone(),
        path,
        vec!["Run check_reference after the test suite is complete.".to_string()],
    ))
}

pub fn write_fixture_file(request: &WriteFixtureFileRequest) -> Result<AuthoringReport> {
    validate_authored_content("fixture file", &request.content, true)?;
    if !is_safe_relative_path(Path::new(&request.fixture)) {
        bail!("unsafe fixture name: {}", request.fixture);
    }
    if !is_safe_relative_path(&request.path) {
        bail!("unsafe fixture file path: {}", request.path.display());
    }
    let pack = load_explicit_authoring_pack(&request.project, &request.packs_dir)?;
    let stage = require_stage(&pack, &request.stage)?;
    let relative = stage
        .path
        .join("fixtures")
        .join(&request.fixture)
        .join(&request.path);
    reject_symlink_components(&pack.root, &relative)?;
    let path = pack.root.join(relative);
    if path.exists() && !request.overwrite {
        bail!(
            "fixture file already exists: {}; set overwrite=true to replace it",
            path.display()
        );
    }
    atomic_write(&path, &request.content)?;
    Ok(AuthoringReport::ok(
        request.project.clone(),
        path,
        vec!["Reference this fixture by name from structured tests or benchmarks.".to_string()],
    ))
}

pub fn replace_stage_benchmarks(
    request: &ReplaceStageBenchmarksRequest,
) -> Result<AuthoringReport> {
    let pack = load_explicit_authoring_pack(&request.project, &request.packs_dir)?;
    let stage = require_stage(&pack, &request.stage)?;
    let source = structured_yaml("benchmarks", &request.benchmarks)?;
    validate_authored_content("benchmarks YAML", &source, false)?;
    let problems = validate_stage_benchmarks_source(&pack, stage, &source);
    if !problems.is_empty() {
        return Ok(validation_blocked(&pack, problems));
    }

    let relative = stage.path.join("benchmarks.yaml");
    reject_symlink_components(&pack.root, &relative)?;
    let path = pack.root.join(relative);
    atomic_write(&path, source)?;
    Ok(AuthoringReport::ok(
        request.project.clone(),
        path,
        vec!["Run the benchmark in a copied learner project before publishing.".to_string()],
    ))
}

pub fn diagnose_pack(pack: &LoadedPack) -> AuthoringReport {
    let mut problems = validate_pack(pack);
    let mut next_actions = Vec::new();

    if !pack.root.join("README.md").is_file() {
        problems.push("pack README.md is missing".to_string());
        next_actions
            .push("Add README.md explaining what learners are building and why.".to_string());
    }

    for stage in &pack.manifest.stages {
        let instructions = pack.instructions_path(stage);
        if let Ok(source) = fs::read_to_string(&instructions) {
            let lower = source.to_ascii_lowercase();
            if lower.contains("replace me") || lower.contains("replace-this-command") {
                problems.push(format!(
                    "stage {} instructions still contain scaffold placeholder text",
                    stage.id
                ));
            }
        }
        let tests = pack.tests_path(stage);
        if let Ok(source) = fs::read_to_string(&tests)
            && source.contains("replace-me")
        {
            problems.push(format!(
                "stage {} tests still contain scaffold placeholder text",
                stage.id
            ));
        }
    }

    if next_actions.is_empty() {
        next_actions.push("Run check-reference with a known-good implementation.".to_string());
    }

    if problems.is_empty() {
        AuthoringReport::ok(pack.manifest.id.clone(), &pack.root, next_actions)
    } else {
        AuthoringReport::blocked(
            Some(pack.manifest.id.clone()),
            Some(&pack.root),
            problems,
            next_actions,
        )
    }
}

pub fn check_reference(request: &CheckReferenceRequest) -> Result<AuthoringReport> {
    let pack = load_pack(
        &request.project,
        &PackSearchOptions {
            packs_dir: request.packs_dir.clone(),
        },
    )?;

    if !request.reference.is_file() {
        bail!(
            "reference source file is missing: {}",
            request.reference.display()
        );
    }

    let project_dir = create_temp_project_dir(&request.project)?;
    let binary = deltaforge_binary().context("failed to resolve deltaforge executable")?;
    let init_output = Command::new(&binary)
        .args(
            request
                .packs_dir
                .as_ref()
                .into_iter()
                .flat_map(|packs_dir| ["--packs-dir".to_string(), packs_dir.display().to_string()]),
        )
        .args([
            "init".to_string(),
            request.project.clone(),
            "--lang".to_string(),
            request.language.clone(),
            "--name".to_string(),
            project_dir.display().to_string(),
            "--no-git".to_string(),
        ])
        .output()
        .context("failed to run deltaforge init for reference check")?;
    if !init_output.status.success() {
        return Ok(AuthoringReport::blocked(
            Some(request.project.clone()),
            Some(&pack.root),
            vec![format!(
                "reference init failed: {}",
                String::from_utf8_lossy(&init_output.stderr).trim()
            )],
            vec!["Fix pack language/template configuration.".to_string()],
        ));
    }

    fs::copy(&request.reference, project_dir.join("src/main.rs")).with_context(|| {
        format!(
            "failed to copy reference {} into {}",
            request.reference.display(),
            project_dir.display()
        )
    })?;

    let test_output = Command::new(&binary)
        .args(["test", "--all"])
        .current_dir(&project_dir)
        .output()
        .context("failed to run reference tests")?;
    let _ = fs::remove_dir_all(&project_dir);

    if test_output.status.success() {
        Ok(AuthoringReport::ok(
            request.project.clone(),
            &pack.root,
            vec!["Reference solution passes all stages.".to_string()],
        ))
    } else {
        Ok(AuthoringReport::blocked(
            Some(request.project.clone()),
            Some(&pack.root),
            vec![format!(
                "reference solution failed:\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&test_output.stdout),
                String::from_utf8_lossy(&test_output.stderr)
            )],
            vec![
                "Inspect failing tests and update either pack tests or reference solution."
                    .to_string(),
                "Do not mark the pack complete until the reference passes all stages.".to_string(),
            ],
        ))
    }
}

#[derive(Debug, Serialize)]
struct StageManifestEntry {
    id: String,
    title: String,
    path: String,
}

fn validate_identifier(value: &str) -> Result<()> {
    if value.is_empty()
        || !value
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-')
    {
        bail!("identifier must use lowercase ascii letters, digits, '_' or '-': {value}");
    }
    Ok(())
}

fn load_explicit_authoring_pack(project: &str, packs_dir: &Path) -> Result<LoadedPack> {
    validate_identifier(project)?;
    let expected_root = packs_dir.join(project);
    let manifest = expected_root.join("project.yaml");
    if !manifest.is_file() {
        bail!(
            "authoring pack manifest is missing: {}; mutations require an explicit packs_dir",
            manifest.display()
        );
    }
    let pack = load_pack(
        project,
        &PackSearchOptions {
            packs_dir: Some(packs_dir.to_path_buf()),
        },
    )?;
    let expected = expected_root.canonicalize().with_context(|| {
        format!(
            "failed to canonicalize explicit pack root {}",
            expected_root.display()
        )
    })?;
    let actual = pack.root.canonicalize().with_context(|| {
        format!(
            "failed to canonicalize loaded pack root {}",
            pack.root.display()
        )
    })?;
    if actual != expected {
        bail!(
            "refusing to mutate pack discovered outside explicit packs_dir: {}",
            actual.display()
        );
    }
    Ok(pack)
}

fn require_stage<'a>(pack: &'a LoadedPack, stage_id: &str) -> Result<&'a crate::pack::StageSpec> {
    pack.manifest.stage(stage_id).with_context(|| {
        format!(
            "pack {} does not contain stage {stage_id}",
            pack.manifest.id
        )
    })
}

fn validate_nonempty_text(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        bail!("{label} must not be empty");
    }
    if value.len() > MAX_METADATA_BYTES {
        bail!("{label} exceeds the {MAX_METADATA_BYTES} byte authoring limit");
    }
    Ok(())
}

fn validate_authored_content(label: &str, value: &str, allow_empty: bool) -> Result<()> {
    if !allow_empty && value.trim().is_empty() {
        bail!("{label} must not be empty");
    }
    if value.len() > MAX_AUTHORED_TEXT_BYTES {
        bail!(
            "{label} exceeds the {} byte authoring limit",
            MAX_AUTHORED_TEXT_BYTES
        );
    }
    Ok(())
}

fn structured_yaml(key: &str, entries: &Value) -> Result<String> {
    if !entries.is_array() {
        bail!("{key} must be a JSON array of structured definitions");
    }
    let mut document = serde_json::Map::new();
    document.insert(key.to_string(), entries.clone());
    serde_yaml::to_string(&Value::Object(document))
        .with_context(|| format!("failed to serialize structured {key}"))
}

fn validation_blocked(pack: &LoadedPack, problems: Vec<String>) -> AuthoringReport {
    AuthoringReport::blocked(
        Some(pack.manifest.id.clone()),
        Some(&pack.root),
        problems,
        vec!["Fix the structured definitions; no file was changed.".to_string()],
    )
}

fn reject_symlink_components(root: &Path, relative: &Path) -> Result<()> {
    if !is_safe_relative_path(relative) {
        bail!("unsafe authoring path: {}", relative.display());
    }
    let mut current = root.to_path_buf();
    if fs::symlink_metadata(&current)?.file_type().is_symlink() {
        bail!("pack root must not be a symbolic link: {}", root.display());
    }
    for component in relative.components() {
        current.push(component.as_os_str());
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                bail!(
                    "authoring path crosses symbolic link: {}",
                    current.display()
                );
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("failed to inspect {}", current.display()));
            }
        }
    }
    Ok(())
}

fn create_temp_project_dir(project: &str) -> Result<PathBuf> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?
        .as_nanos();
    Ok(std::env::temp_dir().join(format!(
        "deltaforge-reference-{project}-{}-{timestamp}",
        std::process::id()
    )))
}

fn unique_sibling(path: &Path, suffix: &str) -> Result<PathBuf> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?
        .as_nanos();
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("deltaforge");
    Ok(path.with_file_name(format!(
        ".{name}.{}.{timestamp}.{suffix}",
        std::process::id()
    )))
}

fn replace_directory(prepared: &Path, destination: &Path) -> Result<()> {
    let backup = if destination.exists() {
        let backup = unique_sibling(destination, "backup")?;
        fs::rename(destination, &backup).with_context(|| {
            format!(
                "failed to preserve existing directory {}",
                destination.display()
            )
        })?;
        Some(backup)
    } else {
        None
    };
    if let Err(error) = fs::rename(prepared, destination) {
        if let Some(backup) = &backup {
            let _ = fs::rename(backup, destination);
        }
        return Err(error).with_context(|| {
            format!(
                "failed to install prepared directory {}",
                destination.display()
            )
        });
    }
    if let Some(backup) = backup {
        fs::remove_dir_all(backup)?;
    }
    Ok(())
}

fn deltaforge_binary() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("DELTAFORGE_BIN") {
        return Ok(PathBuf::from(path));
    }

    let current = std::env::current_exe()?;
    let file_name = current
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    if file_name.contains("pack-mcp")
        && let Some(parent) = current.parent()
    {
        let sibling = parent.join(format!("deltaforge{}", std::env::consts::EXE_SUFFIX));
        if sibling.is_file() {
            return Ok(sibling);
        }
    }
    Ok(current)
}

fn project_yaml(request: &NewPackRequest) -> String {
    let manifest = serde_yaml::Mapping::from_iter([
        ("schema_version".into(), 1.into()),
        ("id".into(), request.id.clone().into()),
        ("name".into(), request.name.clone().into()),
        ("version".into(), "0.1.0".into()),
        ("description".into(), request.description.clone().into()),
        ("topics".into(), vec!["systems"].into()),
        (
            "languages".into(),
            serde_yaml::from_str::<serde_yaml::Value>(
                r#"rust:
  template: templates/rust
  build:
    command: [cargo, build, --release]
  run:
    command: [cargo, run, --release, --]
"#,
            )
            .expect("static language YAML is valid"),
        ),
        ("ignored_paths".into(), vec![".git", "target"].into()),
        (
            "stages".into(),
            vec![
                serde_yaml::to_value(StageManifestEntry {
                    id: "01_first_stage".to_string(),
                    title: "First stage".to_string(),
                    path: "stages/01_first_stage".to_string(),
                })
                .expect("stage entry serializes"),
            ]
            .into(),
        ),
    ]);
    serde_yaml::to_string(&manifest).expect("pack manifest serializes")
}

fn readme(request: &NewPackRequest) -> String {
    format!(
        r#"# {name}

## What you are building

Describe the finished tool or system in concrete terms.

## Why this is useful

Explain what real engineering concepts the learner will practice.

## Big picture

1. Start with a minimal CLI behavior.
2. Add realistic edge cases.
3. Validate behavior with black-box tests.

## What good looks like

Good solutions are deterministic, locally owned, and tested at the command boundary.
"#,
        name = request.name
    )
}

fn template_cargo_toml(id: &str) -> String {
    format!(
        r#"[package]
name = "{id}"
version = "0.1.0"
edition = "2024"

[dependencies]
"#
    )
}

fn starter_main(id: &str) -> String {
    format!(
        r#"use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {{
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {{
        eprintln!("usage: {id} <command> ...");
        return ExitCode::FAILURE;
    }}

    println!("{id} starter: implement command {{:?}}", args);
    ExitCode::SUCCESS
}}
"#
    )
}

fn first_stage_instructions(id: &str) -> String {
    format!(
        r#"# First stage

Implement:

```bash
{id} echo <value>
```

Print the value followed by a newline.

Example:

```txt
hello
```

Edge cases:

- reject missing values
- preserve spaces inside one argument

Non-goals:

- persistence
- networking
- advanced parsing
"#
    )
}

fn first_stage_tests(_id: &str) -> String {
    r#"tests:
  - name: echoes one value
    command: ["echo", "hello"]
    expect:
      exit_code: 0
      stdout_exact: "hello\n"

  - name: preserves one argument with spaces
    command: ["echo", "hello world"]
    expect:
      exit_code: 0
      stdout_exact: "hello world\n"

  - name: rejects missing value
    command: ["echo"]
    expect:
      exit_code: 1
"#
    .to_string()
}

fn stage_instructions(id: &str, title: &str) -> String {
    format!(
        r#"# {title}

Describe the behavior for stage `{id}`.

Example:

```bash
replace-this-command
```

Edge cases:

- add at least two edge cases before considering this stage complete

Non-goals:

- list behavior that should not be implemented yet
"#
    )
}

fn stage_tests(title: &str) -> String {
    format!(
        r#"tests:
  - name: placeholder for {title}
    command: ["echo", "replace-me"]
    expect:
      exit_code: 0
      stdout_contains:
        - "replace-me"
"#
    )
}
