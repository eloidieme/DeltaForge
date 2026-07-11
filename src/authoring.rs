use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::fs_util::atomic_write;
use crate::pack::{LoadedPack, PackSearchOptions, load_pack, validate_pack};

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

    let root = request.dest.join(&request.id);
    if root.exists() {
        if !request.force {
            bail!(
                "pack directory already exists: {}\nUse --force to replace it.",
                root.display()
            );
        }
        fs::remove_dir_all(&root)
            .with_context(|| format!("failed to remove existing pack {}", root.display()))?;
    }

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

    Ok(AuthoringReport::ok(
        request.id.clone(),
        &root,
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
    stages.push(serde_yaml::to_value(StageManifestEntry {
        id: request.id.clone(),
        title: request.title.clone(),
        path: stage_path.clone(),
    })?);
    atomic_write(&manifest_path, serde_yaml::to_string(&manifest)?)?;

    let stage_dir = request.pack_dir.join(&stage_path);
    if stage_dir.exists() && !request.force {
        bail!(
            "stage directory already exists: {}\nUse --force to write scaffold files.",
            stage_dir.display()
        );
    }
    fs::create_dir_all(stage_dir.join("fixtures/example"))?;
    atomic_write(
        stage_dir.join("instructions.md").as_path(),
        stage_instructions(&request.id, &request.title),
    )?;
    atomic_write(
        stage_dir.join("hints.md").as_path(),
        "# Hint 1\n\nIdentify the smallest CLI behavior that should pass this stage first.\n",
    )?;
    atomic_write(
        stage_dir.join("tests.yaml").as_path(),
        stage_tests(&request.title),
    )?;
    atomic_write(
        stage_dir.join("fixtures/example/input.txt").as_path(),
        "replace me\n",
    )?;

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
        .args([
            "init",
            &request.project,
            "--lang",
            &request.language,
            "--name",
            &project_dir.display().to_string(),
            "--no-git",
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
    format!(
        r#"schema_version: 1
id: {id}
name: {name}
version: 0.1.0
description: {description}
topics:
  - systems
languages:
  rust:
    template: templates/rust
    build:
      command: ["cargo", "build", "--release"]
    run:
      command: ["cargo", "run", "--release", "--"]
ignored_paths:
  - .git
  - target
stages:
  - id: 01_first_stage
    title: First stage
    path: stages/01_first_stage
"#,
        id = request.id,
        name = request.name,
        description = request.description
    )
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
