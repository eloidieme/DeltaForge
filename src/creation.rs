//! Project creation: choosing where a new project goes, checking the
//! toolchain it needs, and writing it.
//!
//! Creation is the one operation where a browser request decides a filesystem
//! path. Everywhere else the browser names an opaque registry identifier and
//! the service resolves it. [`resolve_target`] is the single place that
//! boundary is crossed, and it is deliberately narrow: a parent directory that
//! must already exist inside the learner's home, plus a leaf name restricted to
//! a conservative character set. See `docs/product/architecture.md`.

use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::config::ProjectConfig;
use crate::integrity::digest_pack_tree;
use crate::pack::{LanguageSpec, LoadedPack, StageSpec, ToolRequirement, pack_source_label};
use crate::state::ProjectState;

/// Longest project name the creation flow accepts. Long enough for a
/// descriptive name, short enough to stay well inside every platform's path
/// limits once joined to a parent directory.
const MAX_NAME_LENGTH: usize = 64;

/// Windows refuses to create files with these stems regardless of extension.
/// Rejecting them everywhere keeps a project created on one platform openable
/// on another.
const RESERVED_NAMES: [&str; 22] = [
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

/// Where new projects go when the learner does not choose somewhere else.
/// `$DELTAFORGE_WORKSPACE` overrides it, which is also how an automated
/// environment points creation at a scratch directory.
pub fn default_workspace() -> Result<PathBuf> {
    if let Some(path) = env::var_os("DELTAFORGE_WORKSPACE") {
        let path = PathBuf::from(path);
        if path.as_os_str().is_empty() {
            bail!("DELTAFORGE_WORKSPACE must not be empty");
        }
        return Ok(path);
    }
    Ok(home_directory()?.join("DeltaForge"))
}

fn home_directory() -> Result<PathBuf> {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .filter(|path| !path.as_os_str().is_empty())
        .context("could not locate the user home directory")
}

/// Validate a project name supplied by the browser. Returns the trimmed name
/// when it is acceptable.
pub fn validate_name(name: &str) -> Result<&str> {
    let name = name.trim();
    if name.is_empty() {
        bail!("choose a name for the project");
    }
    if name.chars().count() > MAX_NAME_LENGTH {
        bail!("the project name is longer than {MAX_NAME_LENGTH} characters");
    }
    if !name
        .chars()
        .next()
        .is_some_and(|first| first.is_ascii_alphanumeric())
    {
        bail!("the project name must start with a letter or a digit");
    }
    if let Some(bad) = name.chars().find(|character| {
        !character.is_ascii_alphanumeric() && *character != '-' && *character != '_'
    }) {
        bail!(
            "the project name cannot contain {bad:?}; use letters, digits, hyphens, or underscores"
        );
    }
    if RESERVED_NAMES.contains(&name.to_ascii_lowercase().as_str()) {
        bail!("{name} is a reserved name on Windows; choose another");
    }
    Ok(name)
}

/// Where DeltaForge is allowed to create a project.
///
/// Held as data rather than read from the environment at each use, so the
/// rules can be exercised against a temporary tree instead of the developer's
/// own home directory.
#[derive(Debug, Clone)]
pub struct CreationPolicy {
    /// Used when the caller supplies no parent directory.
    pub default_parent: PathBuf,
    /// A supplied parent must resolve inside one of these.
    pub permitted_roots: Vec<PathBuf>,
}

impl CreationPolicy {
    /// The real policy: the learner's home directory, widened to include an
    /// explicitly configured workspace so a scripted or containerised
    /// environment can still create projects.
    pub fn from_environment() -> Result<Self> {
        let mut permitted_roots = Vec::new();
        if let Ok(home) = home_directory() {
            permitted_roots.push(home);
        }
        if let Some(workspace) = env::var_os("DELTAFORGE_WORKSPACE") {
            let workspace = PathBuf::from(workspace);
            if !workspace.as_os_str().is_empty() {
                permitted_roots.push(workspace);
            }
        }
        if permitted_roots.is_empty() {
            bail!("could not locate a directory DeltaForge may create projects in");
        }
        Ok(Self {
            default_parent: default_workspace()?,
            permitted_roots,
        })
    }

    /// Turn a browser-supplied parent directory and leaf name into the one
    /// path a project may be created at.
    ///
    /// Every rejection below is deliberate:
    ///
    /// - the leaf is a validated single component, so no request can traverse
    ///   with `..` or an embedded separator;
    /// - the parent must already exist and canonicalize, which resolves
    ///   symlinks *before* the containment check rather than after;
    /// - the canonical parent must lie inside a permitted root, so a request
    ///   cannot reach system directories;
    /// - no path component below that root may be hidden, keeping creation out
    ///   of `~/.ssh`, `~/.config`, and every other dotted directory;
    /// - the parent must not itself be, or sit inside, a DeltaForge project, so
    ///   a project can never be nested in another project's source tree;
    /// - the leaf must not already exist, so creation never overwrites.
    pub fn resolve_target(&self, parent: Option<&Path>, name: &str) -> Result<PathBuf> {
        let name = validate_name(name)?;
        let parent = match parent {
            Some(parent) => parent.to_path_buf(),
            None => {
                fs::create_dir_all(&self.default_parent).with_context(|| {
                    format!(
                        "failed to create workspace directory {}",
                        self.default_parent.display()
                    )
                })?;
                self.default_parent.clone()
            }
        };
        if !parent.is_absolute() {
            bail!("the location must be an absolute path");
        }
        if !parent.is_dir() {
            bail!("{} is not an existing directory", parent.display());
        }
        let parent = parent
            .canonicalize()
            .with_context(|| format!("failed to resolve {}", parent.display()))?;

        let roots = self
            .permitted_roots
            .iter()
            .filter_map(|root| root.canonicalize().ok())
            .collect::<Vec<_>>();
        if !roots.iter().any(|root| parent.starts_with(root)) {
            bail!(
                "DeltaForge only creates projects inside your home directory, or the directory named by DELTAFORGE_WORKSPACE; {} is outside both",
                crate::fs_util::display_path(&parent)
            );
        }
        for root in &roots {
            if let Ok(relative) = parent.strip_prefix(root)
                && let Some(hidden) = relative.components().find_map(hidden_component)
            {
                bail!("DeltaForge does not create projects inside the hidden directory {hidden}");
            }
        }
        if let Some(existing) = enclosing_project(&parent) {
            bail!(
                "{} is inside the DeltaForge project at {}; choose a location outside it",
                parent.display(),
                existing.display()
            );
        }

        let target = parent.join(name);
        if target.exists() {
            bail!("{} already exists", target.display());
        }
        Ok(target)
    }
}

/// Resolve a creation target under the policy this machine is configured with.
pub fn resolve_target(parent: Option<&Path>, name: &str) -> Result<PathBuf> {
    CreationPolicy::from_environment()?.resolve_target(parent, name)
}

fn hidden_component(component: Component<'_>) -> Option<String> {
    let name = component.as_os_str().to_string_lossy();
    (name.starts_with('.') && name != "." && name != "..").then(|| name.to_string())
}

/// The nearest ancestor of `path` (inclusive) that is a DeltaForge project.
fn enclosing_project(path: &Path) -> Option<PathBuf> {
    path.ancestors()
        .find(|candidate| candidate.join(".deltaforge").join("state.json").is_file())
        .map(Path::to_path_buf)
}

/// One executable the creation preflight looked for.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatus {
    pub program: String,
    pub label: String,
    pub found: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_url: Option<String>,
    /// A missing optional tool degrades an experience; a missing required tool
    /// blocks creation.
    pub required: bool,
}

/// Everything checked before a project is created: the tools its language
/// needs, and whether the chosen location can hold it.
#[derive(Debug, Clone, Serialize)]
pub struct Preflight {
    pub ok: bool,
    pub tools: Vec<ToolStatus>,
    pub location: LocationStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct LocationStatus {
    pub parent: String,
    pub target: Option<String>,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem: Option<String>,
}

/// Check the toolchain one language needs, plus git, which DeltaForge itself
/// uses for stage snapshots.
pub fn language_tools(language: &LanguageSpec) -> Vec<ToolStatus> {
    let mut tools: Vec<ToolStatus> = language
        .requires
        .iter()
        .map(|requirement| tool_status(requirement, true))
        .collect();
    tools.push(tool_status(
        &ToolRequirement {
            program: "git".to_string(),
            label: Some("Git".to_string()),
            install_url: Some("https://git-scm.com/downloads".to_string()),
        },
        false,
    ));
    tools
}

fn tool_status(requirement: &ToolRequirement, required: bool) -> ToolStatus {
    let version = tool_version(&requirement.program);
    ToolStatus {
        program: requirement.program.clone(),
        label: requirement
            .label
            .clone()
            .unwrap_or_else(|| requirement.program.clone()),
        found: version.is_some(),
        version,
        install_url: requirement.install_url.clone(),
        required,
    }
}

/// First line of `<program> --version`, or `None` when the program is absent
/// or refuses the flag.
pub fn tool_version(program: &str) -> Option<String> {
    let output = Command::new(program).arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let line = text.lines().next().unwrap_or_default().trim();
    (!line.is_empty()).then(|| line.to_string())
}

/// Run the full preflight for one pack, language, and proposed location.
pub fn preflight(
    pack: &LoadedPack,
    language_id: &str,
    parent: Option<&Path>,
    name: &str,
) -> Result<Preflight> {
    let language = pack.manifest.language(language_id).with_context(|| {
        format!(
            "pack {} does not support language {language_id}",
            pack.manifest.id
        )
    })?;
    let tools = language_tools(language);
    let displayed_parent = match parent {
        Some(parent) => crate::fs_util::display_path(parent),
        None => crate::fs_util::display_path(&default_workspace()?),
    };
    let location = match resolve_target(parent, name) {
        Ok(target) => LocationStatus {
            parent: displayed_parent,
            target: Some(crate::fs_util::display_path(&target)),
            ok: true,
            problem: None,
        },
        Err(error) => LocationStatus {
            parent: displayed_parent,
            target: None,
            ok: false,
            problem: Some(format!("{error:#}")),
        },
    };
    Ok(Preflight {
        ok: location.ok && tools.iter().all(|tool| tool.found || !tool.required),
        tools,
        location,
    })
}

/// Write a new project at `target` from `pack`. The caller has already
/// resolved and validated `target`; this function only creates.
pub fn create(
    pack: &LoadedPack,
    language_id: &str,
    target: &Path,
    stage: &StageSpec,
    git: bool,
) -> Result<()> {
    let language = pack.manifest.language(language_id).with_context(|| {
        format!(
            "pack {} does not support language {language_id}",
            pack.manifest.id
        )
    })?;
    if target.exists() {
        bail!("target directory already exists: {}", target.display());
    }
    let template_root = pack.root.join(&language.template);
    copy_dir_recursive(&template_root, target).with_context(|| {
        format!(
            "failed to copy template {} to {}",
            template_root.display(),
            target.display()
        )
    })?;
    write_metadata(target, pack, language_id, stage)?;
    write_readme(target, pack, stage)?;
    write_gitignore(target)?;
    if git {
        initialize_git(target)?;
    }
    Ok(())
}

fn write_gitignore(target: &Path) -> Result<()> {
    const GENERATED: [&str; 5] = [
        "/.deltaforge/",
        "/target/",
        "/build/",
        "/node_modules/",
        "/.venv/",
    ];
    let path = target.join(".gitignore");
    let mut source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => {
            return Err(error).with_context(|| format!("failed to read {}", path.display()));
        }
    };
    if !source.is_empty() && !source.ends_with('\n') {
        source.push('\n');
    }
    let existing = source.lines().collect::<std::collections::BTreeSet<_>>();
    let additions = GENERATED
        .into_iter()
        .filter(|entry| !existing.contains(entry))
        .collect::<Vec<_>>();
    if !additions.is_empty() {
        if !source.is_empty() {
            source.push('\n');
        }
        source.push_str("# DeltaForge runtime and generated build output\n");
        source.push_str(&additions.join("\n"));
        source.push('\n');
        fs::write(&path, source).with_context(|| format!("failed to write {}", path.display()))?;
    }
    Ok(())
}

fn write_metadata(
    target: &Path,
    pack: &LoadedPack,
    language_id: &str,
    stage: &StageSpec,
) -> Result<()> {
    let metadata_dir = target.join(".deltaforge");
    fs::create_dir_all(&metadata_dir).with_context(|| {
        format!(
            "failed to create DeltaForge metadata directory {}",
            metadata_dir.display()
        )
    })?;

    let mut state = ProjectState::new(
        pack.manifest.id.clone(),
        language_id.to_string(),
        stage.id.clone(),
    )?;
    state.pack_version = pack.manifest.version.clone();
    state.pack_source = pack_source_label(&pack.root);
    state.pack_digest = digest_pack_tree(&pack.root)?;
    state.write_to(&metadata_dir.join("state.json"))?;

    ProjectConfig::default().write_to(&metadata_dir.join("config.toml"))?;
    Ok(())
}

fn write_readme(target: &Path, pack: &LoadedPack, stage: &StageSpec) -> Result<()> {
    let manifest = &pack.manifest;
    let overview = crate::commands::overview::read_pack_overview(pack);
    let roadmap = manifest
        .stages
        .iter()
        .map(|entry| {
            let marker = if entry.id == stage.id { "→" } else { "○" };
            format!("{marker} `{}` - {}", entry.id, entry.title)
        })
        .collect::<Vec<_>>()
        .join("\n");
    let readme = format!(
        "# {}\n\n{}\n\n{}\n\n## Current Stage\n\n`{}` - {}\n\n## Stage Roadmap\n\n{}\n\n## DeltaForge\n\nOpen the local workbench from this directory:\n\n```bash\ndeltaforge\n```\n\nFor terminal-only checks and diagnostics:\n\n```bash\ndeltaforge test\ndeltaforge status\ndeltaforge doctor\n```\n",
        manifest.name,
        manifest.description,
        overview.trim(),
        stage.id,
        stage.title,
        roadmap
    );
    fs::write(target.join("README.md"), readme).with_context(|| {
        format!(
            "failed to write project README {}",
            target.join("README.md").display()
        )
    })
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    if !source.is_dir() {
        bail!("template directory does not exist: {}", source.display());
    }
    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create directory {}", destination.display()))?;

    for entry in fs::read_dir(source)
        .with_context(|| format!("failed to read directory {}", source.display()))?
    {
        let entry = entry
            .with_context(|| format!("failed to read directory entry in {}", source.display()))?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to inspect {}", source_path.display()))?;

        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            fs::copy(&source_path, &destination_path).with_context(|| {
                format!(
                    "failed to copy {} to {}",
                    source_path.display(),
                    destination_path.display()
                )
            })?;
        }
    }
    Ok(())
}

fn initialize_git(target: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("init")
        .current_dir(target)
        .output()
        .with_context(|| format!("failed to run git init in {}", target.display()))?;
    if !output.status.success() {
        bail!(
            "git init failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A policy over a temporary tree, so the rules are exercised without
    /// touching the developer's home directory and without depending on
    /// process-wide environment variables.
    fn sandbox(label: &str) -> (PathBuf, CreationPolicy) {
        let root = std::env::temp_dir().join(format!(
            "deltaforge-creation-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("projects")).unwrap();
        let canonical = root.canonicalize().unwrap();
        let policy = CreationPolicy {
            default_parent: canonical.join("projects"),
            permitted_roots: vec![canonical.clone()],
        };
        (canonical, policy)
    }

    #[test]
    fn names_reject_traversal_separators_and_reserved_words() {
        assert!(validate_name("flashindex-rust").is_ok());
        assert!(validate_name("my_project2").is_ok());
        assert!(validate_name("..").is_err());
        assert!(validate_name("../escape").is_err());
        assert!(validate_name("a/b").is_err());
        assert!(validate_name("a\\b").is_err());
        assert!(validate_name(".hidden").is_err());
        assert!(validate_name("").is_err());
        assert!(validate_name("   ").is_err());
        assert!(validate_name("CON").is_err());
        assert!(validate_name("nul").is_err());
        assert!(validate_name(&"a".repeat(MAX_NAME_LENGTH + 1)).is_err());
        assert!(validate_name(&"a".repeat(MAX_NAME_LENGTH)).is_ok());
    }

    #[test]
    fn a_parent_outside_every_permitted_root_is_refused() {
        let (root, policy) = sandbox("outside");
        let outside = std::env::temp_dir().canonicalize().unwrap();
        let refusal = policy
            .resolve_target(Some(&outside), "project")
            .expect_err("a directory outside the permitted roots must be refused");
        assert!(format!("{refusal:#}").contains("is outside both"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn hidden_directories_are_refused() {
        let (root, policy) = sandbox("hidden");
        let hidden = root.join(".config");
        fs::create_dir_all(&hidden).unwrap();
        let refusal = policy
            .resolve_target(Some(&hidden), "project")
            .expect_err("a hidden directory must be refused");
        assert!(format!("{refusal:#}").contains("hidden directory .config"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn a_project_cannot_be_created_inside_another_project() {
        let (root, policy) = sandbox("nesting");
        let outer = root.join("projects").join("outer");
        fs::create_dir_all(outer.join(".deltaforge")).unwrap();
        fs::write(outer.join(".deltaforge").join("state.json"), "{}").unwrap();
        let inner = outer.join("src");
        fs::create_dir_all(&inner).unwrap();

        let refusal = policy
            .resolve_target(Some(&inner), "project")
            .expect_err("nesting must be refused");
        assert!(format!("{refusal:#}").contains("inside the DeltaForge project"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn an_existing_target_is_never_overwritten() {
        let (root, policy) = sandbox("existing");
        let parent = root.join("projects");
        fs::create_dir_all(parent.join("taken")).unwrap();

        assert!(policy.resolve_target(Some(&parent), "taken").is_err());
        assert_eq!(
            policy.resolve_target(Some(&parent), "free").unwrap(),
            parent.join("free")
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn omitting_a_parent_uses_the_default_workspace_and_creates_it() {
        let (root, mut policy) = sandbox("default");
        policy.default_parent = root.join("fresh-workspace");
        assert!(!policy.default_parent.exists());

        let target = policy.resolve_target(None, "first-project").unwrap();
        assert!(policy.default_parent.is_dir());
        assert_eq!(target.file_name().unwrap(), "first-project");
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn a_relative_parent_is_not_a_location() {
        let (root, policy) = sandbox("relative");
        let refusal = policy
            .resolve_target(Some(Path::new("relative/path")), "project")
            .expect_err("a relative parent must be refused");
        assert!(format!("{refusal:#}").contains("absolute"));
        let _ = fs::remove_dir_all(root);
    }
}
