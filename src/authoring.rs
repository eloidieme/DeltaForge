use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
#[cfg(unix)]
use cap_fs_ext::MetadataExt;
use cap_fs_ext::{DirExt, FollowSymlinks, OpenOptionsFollowExt};

use cap_std::ambient_authority;
use cap_std::fs::{Dir, OpenOptions};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::fs_util::atomic_write;
use crate::integrity::is_safe_relative_path;
use crate::pack::{
    LoadedPack, PackSearchOptions, ProjectPack, StageBenchmarks, load_pack, load_pack_read_only,
    pack_search_dirs_read_only, validate_pack, validate_stage_benchmarks_source,
    validate_stage_tests_source,
};
use crate::runner::StageTests;

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
    pub performance_gates: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct ReadPackRequest {
    pub project: String,
    pub packs_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ReadStageDocumentRequest {
    pub project: String,
    pub packs_dir: Option<PathBuf>,
    pub stage: String,
    pub document: String,
}

#[derive(Debug, Clone)]
pub struct ReadStageDataRequest {
    pub project: String,
    pub packs_dir: Option<PathBuf>,
    pub stage: String,
}

#[derive(Debug, Clone)]
pub struct ListFixtureFilesRequest {
    pub project: String,
    pub packs_dir: Option<PathBuf>,
    pub stage: String,
    pub fixture: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReadFixtureFileRequest {
    pub project: String,
    pub packs_dir: Option<PathBuf>,
    pub stage: String,
    pub fixture: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DeleteFixtureFileRequest {
    pub project: String,
    pub packs_dir: PathBuf,
    pub stage: String,
    pub fixture: String,
    pub path: PathBuf,
    pub confirm: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthoringReport {
    pub status: String,
    pub pack: Option<String>,
    pub path: Option<String>,
    pub problems: Vec<String>,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ReadPackManifestReport {
    #[serde(flatten)]
    pub report: AuthoringReport,
    pub manifest: Option<ProjectPack>,
}

#[derive(Debug, Serialize)]
pub struct ReadStageDocumentReport {
    #[serde(flatten)]
    pub report: AuthoringReport,
    pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReadStageTestsReport {
    #[serde(flatten)]
    pub report: AuthoringReport,
    pub tests: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct ReadStageBenchmarksReport {
    #[serde(flatten)]
    pub report: AuthoringReport,
    pub benchmarks: Option<Value>,
    pub performance_gates: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct FixtureFileEntry {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Serialize)]
pub struct ListFixtureFilesReport {
    #[serde(flatten)]
    pub report: AuthoringReport,
    pub fixtures: Option<Vec<String>>,
    pub files: Option<Vec<FixtureFileEntry>>,
}

#[derive(Debug, Serialize)]
pub struct ReadFixtureFileReport {
    #[serde(flatten)]
    pub report: AuthoringReport,
    pub content: Option<String>,
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
    let source =
        structured_benchmarks_yaml(&request.benchmarks, request.performance_gates.as_ref())?;
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

pub fn read_pack_manifest(request: &ReadPackRequest) -> Result<ReadPackManifestReport> {
    let pack = load_read_authoring_pack(&request.project, request.packs_dir.as_deref())?;
    let relative = Path::new("project.yaml");
    reject_symlink_components(&pack.root, relative)?;
    let path = pack.root.join(relative);
    require_regular_file(&path)?;
    Ok(ReadPackManifestReport {
        report: AuthoringReport::ok(
            pack.manifest.id.clone(),
            path,
            vec![
                "Read stage documents, tests, benchmarks, and fixtures before mutating the pack."
                    .to_string(),
            ],
        ),
        manifest: Some(pack.manifest),
    })
}

pub fn read_stage_document(request: &ReadStageDocumentRequest) -> Result<ReadStageDocumentReport> {
    let pack = load_read_authoring_pack(&request.project, request.packs_dir.as_deref())?;
    let stage = require_stage(&pack, &request.stage)?;
    let (file_name, optional) = match request.document.as_str() {
        "instructions" => ("instructions.md", false),
        "hints" => ("hints.md", false),
        "design_prompt" => ("design_prompt.md", true),
        other => bail!(
            "unsupported stage document {other}; expected instructions, hints, or design_prompt"
        ),
    };
    let relative = stage.path.join(file_name);
    reject_symlink_components(&pack.root, &relative)?;
    let path = pack.root.join(&relative);
    if !path.exists() && optional {
        return Ok(ReadStageDocumentReport {
            report: AuthoringReport::ok(
                pack.manifest.id,
                path,
                vec![format!(
                    "The optional {} document is not present; write it only if the stage needs it.",
                    request.document
                )],
            ),
            content: None,
        });
    }
    let bytes = match read_authored_regular_file(&path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return Ok(ReadStageDocumentReport {
                report: read_blocked(&pack, &path, format!("{error:#}")),
                content: None,
            });
        }
    };
    let content = match String::from_utf8(bytes) {
        Ok(content) => content,
        Err(error) => {
            return Ok(ReadStageDocumentReport {
                report: read_blocked(
                    &pack,
                    &path,
                    format!("stage document is not valid UTF-8: {error}"),
                ),
                content: None,
            });
        }
    };
    Ok(ReadStageDocumentReport {
        report: AuthoringReport::ok(
            pack.manifest.id,
            path,
            vec!["Use write_stage_document to replace this exact document if needed.".to_string()],
        ),
        content: Some(content),
    })
}

pub fn read_stage_tests(request: &ReadStageDataRequest) -> Result<ReadStageTestsReport> {
    let pack = load_read_authoring_pack(&request.project, request.packs_dir.as_deref())?;
    let stage = require_stage(&pack, &request.stage)?;
    let relative = stage.path.join("tests.yaml");
    reject_symlink_components(&pack.root, &relative)?;
    let path = pack.root.join(relative);
    let source = match read_authored_yaml(&path, "stage tests") {
        Ok(source) => source,
        Err(error) => {
            return Ok(ReadStageTestsReport {
                report: read_blocked(&pack, &path, format!("{error:#}")),
                tests: None,
            });
        }
    };
    match serde_yaml::from_str::<StageTests>(&source) {
        Ok(parsed) if parsed.tests.is_empty() => {
            return Ok(ReadStageTestsReport {
                report: read_blocked(&pack, &path, "tests.yaml must contain at least one test"),
                tests: None,
            });
        }
        Ok(_) => {}
        Err(error) => {
            return Ok(ReadStageTestsReport {
                report: read_blocked(&pack, &path, format!("tests.yaml is malformed: {error}")),
                tests: None,
            });
        }
    }
    let document: Value = match serde_yaml::from_str(&source) {
        Ok(document) => document,
        Err(error) => {
            return Ok(ReadStageTestsReport {
                report: read_blocked(&pack, &path, format!("tests.yaml is malformed: {error}")),
                tests: None,
            });
        }
    };
    let tests = document.get("tests").cloned().unwrap_or_else(json_array);
    Ok(ReadStageTestsReport {
        report: AuthoringReport::ok(
            pack.manifest.id,
            path,
            vec![
                "Modify this tests array and pass it unchanged otherwise to replace_stage_tests."
                    .to_string(),
            ],
        ),
        tests: Some(tests),
    })
}

pub fn read_stage_benchmarks(request: &ReadStageDataRequest) -> Result<ReadStageBenchmarksReport> {
    let pack = load_read_authoring_pack(&request.project, request.packs_dir.as_deref())?;
    let stage = require_stage(&pack, &request.stage)?;
    let relative = stage.path.join("benchmarks.yaml");
    reject_symlink_components(&pack.root, &relative)?;
    let path = pack.root.join(relative);
    if !path.exists() {
        return Ok(ReadStageBenchmarksReport {
            report: AuthoringReport::ok(
                pack.manifest.id,
                path,
                vec!["This optional stage has no benchmarks.yaml file.".to_string()],
            ),
            benchmarks: None,
            performance_gates: None,
        });
    }
    let source = match read_authored_yaml(&path, "stage benchmarks") {
        Ok(source) => source,
        Err(error) => {
            return Ok(ReadStageBenchmarksReport {
                report: read_blocked(&pack, &path, format!("{error:#}")),
                benchmarks: None,
                performance_gates: None,
            });
        }
    };
    match serde_yaml::from_str::<StageBenchmarks>(&source) {
        Ok(parsed) if parsed.benchmarks.is_empty() => {
            return Ok(ReadStageBenchmarksReport {
                report: read_blocked(
                    &pack,
                    &path,
                    "benchmarks.yaml must contain at least one benchmark",
                ),
                benchmarks: None,
                performance_gates: None,
            });
        }
        Ok(_) => {}
        Err(error) => {
            return Ok(ReadStageBenchmarksReport {
                report: read_blocked(
                    &pack,
                    &path,
                    format!("benchmarks.yaml is malformed: {error}"),
                ),
                benchmarks: None,
                performance_gates: None,
            });
        }
    }
    let document: Value = match serde_yaml::from_str(&source) {
        Ok(document) => document,
        Err(error) => {
            return Ok(ReadStageBenchmarksReport {
                report: read_blocked(
                    &pack,
                    &path,
                    format!("benchmarks.yaml is malformed: {error}"),
                ),
                benchmarks: None,
                performance_gates: None,
            });
        }
    };
    let benchmarks = document
        .get("benchmarks")
        .cloned()
        .unwrap_or_else(json_array);
    let performance_gates = document
        .get("performance_gates")
        .cloned()
        .unwrap_or_else(json_array);
    Ok(ReadStageBenchmarksReport {
        report: AuthoringReport::ok(
            pack.manifest.id,
            path,
            vec![
                "Pass both arrays to replace_stage_benchmarks so performance gates are preserved."
                    .to_string(),
            ],
        ),
        benchmarks: Some(benchmarks),
        performance_gates: Some(performance_gates),
    })
}

pub fn list_fixture_files(request: &ListFixtureFilesRequest) -> Result<ListFixtureFilesReport> {
    let pack = load_read_authoring_pack(&request.project, request.packs_dir.as_deref())?;
    let stage = require_stage(&pack, &request.stage)?;
    let fixtures_relative = stage.path.join("fixtures");
    let fixtures_root = pack.root.join(&fixtures_relative);
    let pack_root = open_pack_root_capability(&pack.root)?;
    let fixtures_dir = match open_relative_dir_nofollow(pack_root, &fixtures_relative) {
        Ok(dir) => dir,
        Err(error) => {
            return Ok(ListFixtureFilesReport {
                report: read_blocked(&pack, &fixtures_root, format!("{error:#}")),
                fixtures: None,
                files: None,
            });
        }
    };

    if let Some(fixture) = &request.fixture {
        validate_fixture_name(fixture)?;
        let fixture_root = fixtures_root.join(fixture);
        let fixture_dir = match fixtures_dir.open_dir_nofollow(fixture) {
            Ok(dir) => dir,
            Err(error) => {
                return Ok(ListFixtureFilesReport {
                    report: read_blocked(&pack, &fixture_root, format!("{error:#}")),
                    fixtures: None,
                    files: None,
                });
            }
        };
        let mut files = Vec::new();
        if let Err(error) =
            collect_fixture_files_capability(&fixture_dir, Path::new(""), &mut files)
        {
            return Ok(ListFixtureFilesReport {
                report: read_blocked(&pack, &fixture_root, format!("{error:#}")),
                fixtures: None,
                files: None,
            });
        }
        files.sort_by(|left, right| left.path.cmp(&right.path));
        return Ok(ListFixtureFilesReport {
            report: AuthoringReport::ok(
                pack.manifest.id,
                fixture_root,
                vec!["Use read_fixture_file with one listed relative path.".to_string()],
            ),
            fixtures: None,
            files: Some(files),
        });
    }

    let mut fixtures = Vec::new();
    for entry in fixtures_dir.entries()? {
        let entry = entry?;
        let name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(_) => {
                return Ok(ListFixtureFilesReport {
                    report: read_blocked(&pack, &fixtures_root, "fixture name is not valid UTF-8"),
                    fixtures: None,
                    files: None,
                });
            }
        };
        if let Err(error) = validate_fixture_name(&name) {
            return Ok(ListFixtureFilesReport {
                report: read_blocked(&pack, &fixtures_root.join(&name), format!("{error:#}")),
                fixtures: None,
                files: None,
            });
        }
        let file_type = entry.file_type()?;
        if file_type.is_symlink()
            || !file_type.is_dir()
            || fixtures_dir.open_dir_nofollow(&name).is_err()
        {
            return Ok(ListFixtureFilesReport {
                report: read_blocked(
                    &pack,
                    &fixtures_root.join(&name),
                    "fixture entries must be real directories; symlinks and special files are forbidden",
                ),
                fixtures: None,
                files: None,
            });
        }
        fixtures.push(name);
    }
    fixtures.sort();
    Ok(ListFixtureFilesReport {
        report: AuthoringReport::ok(
            pack.manifest.id,
            fixtures_root,
            vec!["Choose a fixture and list its files before reading content.".to_string()],
        ),
        fixtures: Some(fixtures),
        files: None,
    })
}

pub fn read_fixture_file(request: &ReadFixtureFileRequest) -> Result<ReadFixtureFileReport> {
    validate_fixture_name(&request.fixture)?;
    if !is_safe_relative_path(&request.path) {
        bail!("unsafe fixture file path: {}", request.path.display());
    }
    let pack = load_read_authoring_pack(&request.project, request.packs_dir.as_deref())?;
    let stage = require_stage(&pack, &request.stage)?;
    let relative = stage
        .path
        .join("fixtures")
        .join(&request.fixture)
        .join(&request.path);
    let path = pack.root.join(&relative);
    let fixture_relative = stage.path.join("fixtures").join(&request.fixture);
    let pack_root = open_pack_root_capability(&pack.root)?;
    let fixture_dir = match open_relative_dir_nofollow(pack_root, &fixture_relative) {
        Ok(dir) => dir,
        Err(error) => {
            return Ok(ReadFixtureFileReport {
                report: read_blocked(&pack, &path, format!("{error:#}")),
                content: None,
            });
        }
    };
    let bytes = match read_capability_file(&fixture_dir, &request.path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return Ok(ReadFixtureFileReport {
                report: read_blocked(&pack, &path, format!("{error:#}")),
                content: None,
            });
        }
    };
    let content = match String::from_utf8(bytes) {
        Ok(content) => content,
        Err(_) => {
            return Ok(ReadFixtureFileReport {
                report: read_blocked(
                    &pack,
                    &path,
                    "fixture file is not valid UTF-8; binary fixtures are not supported by this tool",
                ),
                content: None,
            });
        }
    };
    Ok(ReadFixtureFileReport {
        report: AuthoringReport::ok(
            pack.manifest.id,
            path,
            vec![
                "Use write_fixture_file with overwrite=true only after grounding on this content."
                    .to_string(),
            ],
        ),
        content: Some(content),
    })
}

pub fn delete_fixture_file(request: &DeleteFixtureFileRequest) -> Result<AuthoringReport> {
    delete_fixture_file_with_hook(request, || {})
}

fn delete_fixture_file_with_hook(
    request: &DeleteFixtureFileRequest,
    before_relative_open: impl FnOnce(),
) -> Result<AuthoringReport> {
    validate_fixture_name(&request.fixture)?;
    if !is_safe_relative_path(&request.path) {
        bail!("unsafe fixture file path: {}", request.path.display());
    }
    let expected_path = request
        .packs_dir
        .join(&request.project)
        .join("stages")
        .join(&request.stage)
        .join("fixtures")
        .join(&request.fixture)
        .join(&request.path);
    if !request.confirm {
        return Ok(AuthoringReport::blocked(
            Some(request.project.clone()),
            Some(&expected_path),
            vec!["delete_fixture_file requires confirm=true".to_string()],
            vec![
                "Re-read the fixture file, then retry with confirm=true if deletion is intended."
                    .to_string(),
            ],
        ));
    }
    let pack = load_explicit_authoring_pack(&request.project, &request.packs_dir)?;
    let stage = require_stage(&pack, &request.stage)?;
    let fixture_relative = stage.path.join("fixtures").join(&request.fixture);
    let fixture_root = pack.root.join(&fixture_relative);
    let path = fixture_root.join(&request.path);
    let pack_root = open_pack_root_capability(&pack.root)?;
    let fixture_dir = match open_relative_dir_nofollow(pack_root, &fixture_relative) {
        Ok(dir) => dir,
        Err(error) => {
            return Ok(AuthoringReport::blocked(
                Some(pack.manifest.id.clone()),
                Some(&path),
                vec![format!("{error:#}")],
                vec!["Fix the fixture path and retry without symbolic links.".to_string()],
            ));
        }
    };
    before_relative_open();
    let (parent, file_name) = match open_parent_dir_nofollow(fixture_dir, &request.path) {
        Ok(opened) => opened,
        Err(error) => {
            return Ok(AuthoringReport::blocked(
                Some(pack.manifest.id.clone()),
                Some(&path),
                vec![format!("fixture file cannot be opened safely: {error:#}")],
                vec!["List the fixture again and choose an existing regular file.".to_string()],
            ));
        }
    };
    let file = match open_regular_file_nofollow(&parent, &file_name) {
        Ok(file) => file,
        Err(error) => {
            return Ok(AuthoringReport::blocked(
                Some(pack.manifest.id.clone()),
                Some(&path),
                vec![format!("delete target is not a regular file: {error:#}")],
                vec!["Choose one existing regular file beneath the named fixture.".to_string()],
            ));
        }
    };
    // Keep the verified file handle alive until the handle-relative unlink.
    // A concurrent parent rename cannot redirect this operation outside the
    // already-opened fixture-root capability.
    parent
        .remove_file(&file_name)
        .with_context(|| format!("failed to delete {}", path.display()))?;
    drop(file);
    Ok(AuthoringReport::ok(
        pack.manifest.id,
        path,
        vec![
            "Re-list the fixture and update tests or benchmarks that referenced this file."
                .to_string(),
        ],
    ))
}

fn structured_benchmarks_yaml(
    benchmarks: &Value,
    performance_gates: Option<&Value>,
) -> Result<String> {
    let mut object = serde_json::Map::new();
    object.insert("benchmarks".to_string(), benchmarks.clone());
    if let Some(gates) = performance_gates {
        object.insert("performance_gates".to_string(), gates.clone());
    }
    serde_yaml::to_string(&Value::Object(object))
        .context("failed to serialize structured benchmarks YAML")
}

fn json_array() -> Value {
    Value::Array(Vec::new())
}

fn read_blocked(pack: &LoadedPack, path: &Path, problem: impl Into<String>) -> AuthoringReport {
    AuthoringReport::blocked(
        Some(pack.manifest.id.clone()),
        Some(path),
        vec![problem.into()],
        vec!["Inspect the reported path and fix the pack before retrying.".to_string()],
    )
}

fn read_authored_yaml(path: &Path, label: &str) -> Result<String> {
    let bytes = read_authored_regular_file(path)?;
    String::from_utf8(bytes).with_context(|| format!("{label} is not valid UTF-8"))
}

fn read_authored_regular_file(path: &Path) -> Result<Vec<u8>> {
    let metadata = require_regular_file(path)?;
    if metadata.len() > MAX_AUTHORED_TEXT_BYTES as u64 {
        bail!(
            "file exceeds the {} byte authored-text limit",
            MAX_AUTHORED_TEXT_BYTES
        );
    }
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    if bytes.len() > MAX_AUTHORED_TEXT_BYTES {
        bail!(
            "file exceeds the {} byte authored-text limit",
            MAX_AUTHORED_TEXT_BYTES
        );
    }
    Ok(bytes)
}

fn require_regular_file(path: &Path) -> Result<fs::Metadata> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            bail!("refusing to follow symbolic link: {}", path.display())
        }
        Ok(metadata) if metadata.is_file() => Ok(metadata),
        Ok(_) => bail!("path is not a regular file: {}", path.display()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            bail!("required file is missing: {}", path.display())
        }
        Err(error) => Err(error).with_context(|| format!("failed to inspect {}", path.display())),
    }
}

fn validate_fixture_name(fixture: &str) -> Result<()> {
    let path = Path::new(fixture);
    if !is_safe_relative_path(path)
        || path.components().count() != 1
        || fixture.contains(['/', '\\', ':'])
    {
        bail!("unsafe fixture name: {fixture}");
    }
    Ok(())
}

fn open_pack_root_capability(root: &Path) -> Result<Dir> {
    let expected = fs::symlink_metadata(root)
        .with_context(|| format!("failed to inspect pack root {}", root.display()))?;
    if expected.file_type().is_symlink() || !expected.is_dir() {
        bail!("pack root must be a real directory: {}", root.display());
    }
    let parent = root
        .parent()
        .with_context(|| format!("pack root has no parent: {}", root.display()))?;
    let name = root
        .file_name()
        .with_context(|| format!("pack root has no directory name: {}", root.display()))?;
    let parent = Dir::open_ambient_dir(parent, ambient_authority())
        .with_context(|| format!("failed to open pack parent for {}", root.display()))?;
    let opened = parent
        .open_dir_nofollow(name)
        .with_context(|| format!("pack root must be a real directory: {}", root.display()))?;

    #[cfg(unix)]
    {
        let actual = opened.dir_metadata()?;
        if expected.dev() != actual.dev() || expected.ino() != actual.ino() {
            bail!("pack root changed while opening: {}", root.display());
        }
    }

    Ok(opened)
}

fn open_relative_dir_nofollow(mut current: Dir, relative: &Path) -> Result<Dir> {
    if !is_safe_relative_path(relative) {
        bail!("unsafe capability directory path: {}", relative.display());
    }
    for component in relative.components() {
        let std::path::Component::Normal(name) = component else {
            bail!("unsafe capability directory path: {}", relative.display());
        };
        current = current.open_dir_nofollow(name).with_context(|| {
            format!(
                "directory path crosses a symlink or non-directory component: {}",
                relative.display()
            )
        })?;
    }
    Ok(current)
}

fn open_parent_dir_nofollow(mut current: Dir, relative: &Path) -> Result<(Dir, PathBuf)> {
    if !is_safe_relative_path(relative) {
        bail!("unsafe fixture file path: {}", relative.display());
    }
    let mut components = relative.components().peekable();
    while let Some(component) = components.next() {
        let std::path::Component::Normal(name) = component else {
            bail!("unsafe fixture file path: {}", relative.display());
        };
        if components.peek().is_none() {
            return Ok((current, PathBuf::from(name)));
        }
        current = current.open_dir_nofollow(name).with_context(|| {
            format!(
                "fixture path crosses a symlink or non-directory component: {}",
                relative.display()
            )
        })?;
    }
    bail!("fixture file path must not be empty")
}

fn open_regular_file_nofollow(dir: &Dir, name: &Path) -> Result<cap_std::fs::File> {
    let metadata = dir.symlink_metadata(name)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        bail!("fixture entry is not a regular file: {}", name.display());
    }
    let mut options = OpenOptions::new();
    options.read(true).follow(FollowSymlinks::No);
    let file = dir.open_with(name, &options).with_context(|| {
        format!(
            "refusing to follow fixture-file symlink: {}",
            name.display()
        )
    })?;
    if !file.metadata()?.is_file() {
        bail!("fixture entry is not a regular file: {}", name.display());
    }
    Ok(file)
}

fn read_capability_file(fixture_dir: &Dir, relative: &Path) -> Result<Vec<u8>> {
    let (parent, name) = open_parent_dir_nofollow(fixture_dir.try_clone()?, relative)?;
    let file = open_regular_file_nofollow(&parent, &name)?;
    if file.metadata()?.len() > MAX_AUTHORED_TEXT_BYTES as u64 {
        bail!(
            "file exceeds the {} byte authored-text limit",
            MAX_AUTHORED_TEXT_BYTES
        );
    }
    let mut bytes = Vec::new();
    file.take(MAX_AUTHORED_TEXT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > MAX_AUTHORED_TEXT_BYTES {
        bail!(
            "file exceeds the {} byte authored-text limit",
            MAX_AUTHORED_TEXT_BYTES
        );
    }
    Ok(bytes)
}

fn collect_fixture_files_capability(
    current: &Dir,
    prefix: &Path,
    files: &mut Vec<FixtureFileEntry>,
) -> Result<()> {
    for entry in current.entries()? {
        let entry = entry?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("fixture path component is not valid UTF-8"))?;
        validate_fixture_name(&name)?;
        let relative = prefix.join(&name);
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            bail!(
                "fixture tree contains a symbolic link: {}",
                relative.display()
            );
        }
        if file_type.is_dir() {
            let child = current.open_dir_nofollow(&name).with_context(|| {
                format!(
                    "fixture directory changed or is a symlink: {}",
                    relative.display()
                )
            })?;
            collect_fixture_files_capability(&child, &relative, files)?;
            continue;
        }
        if !file_type.is_file() {
            bail!(
                "fixture tree contains a special file: {}",
                relative.display()
            );
        }
        let file = open_regular_file_nofollow(current, Path::new(&name))?;
        files.push(FixtureFileEntry {
            path: relative.to_string_lossy().replace('\\', "/"),
            size: file.metadata()?.len(),
        });
    }
    Ok(())
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

fn load_read_authoring_pack(project: &str, packs_dir: Option<&Path>) -> Result<LoadedPack> {
    let options = PackSearchOptions {
        packs_dir: packs_dir.map(Path::to_path_buf),
    };
    if !is_safe_relative_path(Path::new(project)) || Path::new(project).components().count() != 1 {
        bail!("unsafe project pack id: {project}");
    }
    for search_dir in pack_search_dirs_read_only(&options) {
        let root = search_dir.join(project);
        let manifest = root.join("project.yaml");
        match fs::symlink_metadata(&root) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                bail!("pack root must not be a symbolic link: {}", root.display());
            }
            Ok(metadata) if !metadata.is_dir() => continue,
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("failed to inspect pack root {}", root.display()));
            }
        }
        match fs::symlink_metadata(&manifest) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                bail!(
                    "refusing to read symlinked pack manifest: {}",
                    manifest.display()
                );
            }
            Ok(metadata) if !metadata.is_file() => {
                bail!(
                    "pack manifest is not a regular file: {}",
                    manifest.display()
                );
            }
            Ok(_) => {
                return load_pack_read_only(project, &options);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("failed to inspect {}", manifest.display()));
            }
        }
    }
    load_pack_read_only(project, &options)
}

fn load_explicit_authoring_pack(project: &str, packs_dir: &Path) -> Result<LoadedPack> {
    validate_identifier(project)?;
    let expected_root = packs_dir.join(project);
    let manifest = expected_root.join("project.yaml");
    match fs::symlink_metadata(&expected_root) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            bail!(
                "explicit authoring pack root must be a real directory: {}",
                expected_root.display()
            );
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            bail!(
                "authoring pack manifest is missing: {}; mutations require an explicit packs_dir",
                manifest.display()
            );
        }
        Err(error) => return Err(error.into()),
    }
    match fs::symlink_metadata(&manifest) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            bail!(
                "explicit authoring pack manifest must be a regular file: {}",
                manifest.display()
            );
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            bail!(
                "authoring pack manifest is missing: {}; mutations require an explicit packs_dir",
                manifest.display()
            );
        }
        Err(error) => return Err(error.into()),
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

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn capability_delete_blocks_intermediate_symlink_swap() {
        let root = std::env::temp_dir().join(format!(
            "deltaforge-cap-delete-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        create_pack(&NewPackRequest {
            id: "racepack".to_string(),
            name: "Race Pack".to_string(),
            description: "Capability deletion regression".to_string(),
            dest: root.clone(),
            language: "rust".to_string(),
            force: false,
        })
        .unwrap();

        let pack = root.join("racepack");
        let fixture = pack.join("stages/01_first_stage/fixtures/example");
        let nested = fixture.join("nested");
        let preserved = fixture.join("nested-preserved");
        let sibling = pack.join("stages/01_first_stage/fixtures/sibling");
        let external = root.join("external");
        fs::create_dir_all(&nested).unwrap();
        fs::create_dir_all(&sibling).unwrap();
        fs::create_dir_all(&external).unwrap();
        fs::write(nested.join("victim.txt"), "fixture victim").unwrap();
        fs::write(fixture.join("neighbor.txt"), "neighbor").unwrap();
        fs::write(sibling.join("sibling.txt"), "sibling").unwrap();
        fs::write(external.join("victim.txt"), "external victim").unwrap();

        let report = delete_fixture_file_with_hook(
            &DeleteFixtureFileRequest {
                project: "racepack".to_string(),
                packs_dir: root.clone(),
                stage: "01_first_stage".to_string(),
                fixture: "example".to_string(),
                path: PathBuf::from("nested/victim.txt"),
                confirm: true,
            },
            || {
                fs::rename(&nested, &preserved).unwrap();
                std::os::unix::fs::symlink(&external, &nested).unwrap();
            },
        )
        .unwrap();

        assert_eq!(report.status, "blocked");
        assert_eq!(
            fs::read_to_string(external.join("victim.txt")).unwrap(),
            "external victim"
        );
        assert_eq!(
            fs::read_to_string(preserved.join("victim.txt")).unwrap(),
            "fixture victim"
        );
        assert!(fixture.is_dir());
        assert_eq!(
            fs::read_to_string(fixture.join("neighbor.txt")).unwrap(),
            "neighbor"
        );
        assert_eq!(
            fs::read_to_string(sibling.join("sibling.txt")).unwrap(),
            "sibling"
        );
        assert!(pack.join("stages/01_first_stage/tests.yaml").is_file());
        assert!(pack.join("project.yaml").is_file());

        fs::remove_file(nested).unwrap();
        let _ = fs::remove_dir_all(root);
    }
}
