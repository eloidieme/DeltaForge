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
use crate::fs_util::display_path;
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
    /// This decides; it does not write. Preflight needs the decision without
    /// the side effect, so the workspace directory is created by
    /// [`ResolvedTarget::prepare`] at the moment a project is actually
    /// written.
    ///
    /// Every rejection below is deliberate:
    ///
    /// - the leaf is a validated single component, so no request can traverse
    ///   with `..` or an embedded separator;
    /// - the parent canonicalizes, which resolves symlinks *before* the
    ///   containment check rather than after;
    /// - the canonical parent must lie inside a permitted root, so a request
    ///   cannot reach system directories;
    /// - no path component below that root may be hidden, keeping creation out
    ///   of `~/.ssh`, `~/.config`, and every other dotted directory;
    /// - the parent must not itself be, or sit inside, a DeltaForge project, so
    ///   a project can never be nested in another project's source tree;
    /// - the leaf must not already exist, so creation never overwrites.
    ///
    /// The parent must already exist, with one exception: the default
    /// workspace. DeltaForge owns that directory and creates it on demand, so
    /// a learner who has never made a project is not asked to `mkdir` before
    /// the product will run. A location the learner typed is held to the
    /// stricter rule — a path that does not exist is far more likely to be a
    /// typo than a request.
    pub fn resolve(&self, parent: Option<&Path>, name: &str) -> Result<ResolvedTarget> {
        let name = validate_name(name)?;
        let requested = match parent {
            Some(parent) => parent.to_path_buf(),
            None => self.default_parent.clone(),
        };
        if !requested.is_absolute() {
            bail!("the location must be an absolute path");
        }
        let parent_state = if requested.is_dir() {
            ParentState::Exists
        } else if requested.exists() {
            bail!("{} is not a directory", display_path(&requested));
        } else if self.owns_workspace(&requested) {
            ParentState::Creatable
        } else {
            bail!("{} is not an existing directory", display_path(&requested));
        };
        let parent = match parent_state {
            ParentState::Exists => requested
                .canonicalize()
                .with_context(|| format!("failed to resolve {}", display_path(&requested)))?,
            ParentState::Creatable => canonicalize_missing(&requested)?,
        };

        // A permitted root does not have to exist either: `DELTAFORGE_WORKSPACE`
        // may name the directory this very request is about to create. Dropping
        // it for not existing yet refused the creation it was configured to
        // allow.
        let roots = self
            .permitted_roots
            .iter()
            .filter_map(|root| canonicalize_missing(root).ok())
            .collect::<Vec<_>>();
        // The hidden-component rule is measured from the *most specific*
        // permitted root that contains the parent. Applying it against every
        // matching root refused an explicitly configured
        // `DELTAFORGE_WORKSPACE` that happened to sit under a dotted
        // directory in the learner's home — a location they chose on purpose.
        let Some(root) = roots
            .iter()
            .filter(|root| parent.starts_with(root))
            .max_by_key(|root| root.components().count())
        else {
            bail!(
                "DeltaForge only creates projects inside your home directory, or the directory named by DELTAFORGE_WORKSPACE; {} is outside both",
                display_path(&parent)
            );
        };
        if let Ok(relative) = parent.strip_prefix(root)
            && let Some(hidden) = relative.components().find_map(hidden_component)
        {
            bail!("DeltaForge does not create projects inside the hidden directory {hidden}");
        }
        if let Some(existing) = enclosing_project(&parent) {
            bail!(
                "{} is inside the DeltaForge project at {}; choose a location outside it",
                display_path(&parent),
                display_path(&existing)
            );
        }

        let target = parent.join(name);
        if target.exists() {
            bail!("{} already exists", display_path(&target));
        }
        Ok(ResolvedTarget {
            target,
            parent,
            parent_state,
        })
    }

    /// Resolve a target and make its parent directory real. The form creation
    /// uses; [`CreationPolicy::resolve`] is the form preflight uses.
    pub fn resolve_target(&self, parent: Option<&Path>, name: &str) -> Result<PathBuf> {
        let resolved = self.resolve(parent, name)?;
        resolved.prepare()?;
        Ok(resolved.target)
    }

    /// Is `path` the default workspace — the one directory DeltaForge creates
    /// on the learner's behalf? Compared lexically, because neither side is
    /// guaranteed to exist yet.
    fn owns_workspace(&self, path: &Path) -> bool {
        normalize_lexically(path) == normalize_lexically(&self.default_parent)
    }
}

/// A creation target, and what resolving it found out about the directory it
/// will live in.
#[derive(Debug, Clone)]
pub struct ResolvedTarget {
    /// The directory the project will be written to.
    pub target: PathBuf,
    /// The directory that will hold it.
    pub parent: PathBuf,
    /// Whether that directory is already there.
    pub parent_state: ParentState,
}

/// Whether the directory a project goes in exists yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentState {
    /// The parent directory is already on disk.
    Exists,
    /// The parent is the default workspace and is not there yet; DeltaForge
    /// will create it when the project is written.
    Creatable,
}

impl ResolvedTarget {
    /// Make the parent directory real. Called once, immediately before the
    /// project is written, so that merely looking at the creation screen does
    /// not leave a directory behind.
    pub fn prepare(&self) -> Result<()> {
        if self.parent_state == ParentState::Creatable {
            fs::create_dir_all(&self.parent).with_context(|| {
                format!(
                    "failed to create workspace directory {}",
                    display_path(&self.parent)
                )
            })?;
        }
        Ok(())
    }
}

/// Resolve a creation target under the policy this machine is configured with,
/// without touching the filesystem.
pub fn resolve(parent: Option<&Path>, name: &str) -> Result<ResolvedTarget> {
    CreationPolicy::from_environment()?.resolve(parent, name)
}

/// Resolve a creation target and make its parent directory real.
pub fn resolve_target(parent: Option<&Path>, name: &str) -> Result<PathBuf> {
    CreationPolicy::from_environment()?.resolve_target(parent, name)
}

/// Canonicalize a path that does not exist yet, by canonicalizing the deepest
/// ancestor that does and re-attaching the rest. Symlinks in the existing part
/// are resolved before the containment check sees the result, which is the
/// property the check depends on.
fn canonicalize_missing(path: &Path) -> Result<PathBuf> {
    let mut missing = Vec::new();
    let mut cursor = path;
    loop {
        if cursor.is_dir() {
            let mut resolved = cursor
                .canonicalize()
                .with_context(|| format!("failed to resolve {}", display_path(cursor)))?;
            resolved.extend(missing.iter().rev());
            return Ok(resolved);
        }
        let (Some(parent), Some(name)) = (cursor.parent(), cursor.file_name()) else {
            bail!("failed to resolve {}", display_path(path));
        };
        // `..` in the part that does not exist cannot be resolved against a
        // real directory, so it is refused rather than guessed at.
        if name == ".." {
            bail!("the location must not contain \"..\"");
        }
        missing.push(name.to_os_string());
        cursor = parent;
    }
}

/// A lexical, comparison-only normalization: drop `.` components and any
/// trailing separator. Used to recognise the default workspace when neither
/// path exists yet, so `canonicalize` is not available.
fn normalize_lexically(path: &Path) -> PathBuf {
    path.components()
        .filter(|component| !matches!(component, Component::CurDir))
        .collect()
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
    /// True when the location is fine but the directory holding it does not
    /// exist yet. The page says so, rather than letting a folder appear
    /// without explanation.
    pub creates_parent: bool,
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
        Some(parent) => display_path(parent),
        None => display_path(&default_workspace()?),
    };
    // `resolve`, not `resolve_target`: a preflight must not leave a directory
    // behind on a screen the learner may still walk away from.
    let location = match resolve(parent, name) {
        Ok(resolved) => LocationStatus {
            parent: displayed_parent,
            target: Some(display_path(&resolved.target)),
            ok: true,
            creates_parent: resolved.parent_state == ParentState::Creatable,
            problem: None,
        },
        Err(error) => LocationStatus {
            parent: displayed_parent,
            target: None,
            ok: false,
            creates_parent: false,
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
        } else {
            // Silently skipping these produced a project missing a file the
            // template listed, with nothing said about it. Pack content is
            // required to be self-contained anyway (see
            // `validate_pack_tree_is_self_contained`), so this is an authoring
            // error worth naming rather than a case to tolerate.
            bail!(
                "template entry is not a regular file or directory: {}. Pack templates must be self-contained; replace a symbolic link with a copy of its target.",
                source_path.display()
            );
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
        // The counter matters: two threads entering this function inside the
        // same nanosecond otherwise share a directory.
        static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "deltaforge-creation-{label}-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
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

    /// A workspace the learner configured explicitly is a deliberate choice,
    /// even when it sits under a dotted directory: the hidden-directory rule
    /// exists to keep creation out of `~/.ssh`, not to veto `DELTAFORGE_WORKSPACE`.
    #[test]
    fn an_explicitly_configured_workspace_wins_over_a_dotted_ancestor() {
        let (root, mut policy) = sandbox("configured-hidden");
        let workspace = root.join(".local").join("deltaforge");
        fs::create_dir_all(&workspace).unwrap();
        policy.permitted_roots = vec![root.clone(), workspace.clone()];

        assert_eq!(
            policy.resolve_target(Some(&workspace), "project").unwrap(),
            workspace.join("project")
        );
        // A dotted directory that is not itself a permitted root is still refused.
        assert!(
            policy
                .resolve_target(Some(&root.join(".local")), "project")
                .is_err()
        );
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

    /// P0-1. The browser never reaches the `None` branch: it posts the
    /// Location field, which it prefilled with the default workspace. On a
    /// machine that has never run DeltaForge that directory does not exist,
    /// and 1.0 answered every such request with a refusal — so the product's
    /// first screen was unusable on every clean machine. The default
    /// workspace is DeltaForge's own directory, and is resolvable whether the
    /// request names it or omits it.
    #[test]
    fn the_default_workspace_resolves_before_it_exists() {
        let (root, mut policy) = sandbox("missing-default");
        policy.default_parent = root.join("fresh-workspace");
        assert!(!policy.default_parent.exists());

        for parent in [None, Some(policy.default_parent.clone())] {
            let resolved = policy
                .resolve(parent.as_deref(), "first-project")
                .expect("the default workspace is DeltaForge's to create");
            assert_eq!(resolved.parent_state, ParentState::Creatable);
            assert_eq!(resolved.target.file_name().unwrap(), "first-project");
            // Resolution decides; it does not write. A learner who opens the
            // creation screen and walks away leaves nothing behind.
            assert!(!policy.default_parent.exists());
        }
        let _ = fs::remove_dir_all(root);
    }

    /// The other half of the same rule: a location the learner typed is held
    /// to the stricter standard, because a path that is not there is far more
    /// likely to be a typo than a request.
    #[test]
    fn a_typed_parent_that_does_not_exist_is_still_refused() {
        let (root, policy) = sandbox("typed-missing");
        let refusal = policy
            .resolve(Some(&root.join("nowhere")), "project")
            .expect_err("a typed location that is not there must be refused");
        assert!(format!("{refusal:#}").contains("is not an existing directory"));
        let _ = fs::remove_dir_all(root);
    }

    /// Preflight is a read. It answers whether creation would work; it must
    /// not be the thing that makes it work.
    #[test]
    fn resolving_never_writes_but_preparing_does() {
        let (root, mut policy) = sandbox("prepare");
        policy.default_parent = root.join("workspace");

        let resolved = policy.resolve(None, "project").unwrap();
        assert!(!policy.default_parent.exists());
        resolved.prepare().unwrap();
        assert!(policy.default_parent.is_dir());
        // Preparing twice is not an error; the second create_dir_all is a
        // no-op, which is what re-running a preflight-then-create would do.
        resolved.prepare().unwrap();
        let _ = fs::remove_dir_all(root);
    }

    /// A workspace that does not exist yet still cannot be a way out of the
    /// permitted roots: containment is measured against the canonical form of
    /// the deepest ancestor that does exist.
    #[test]
    fn a_missing_workspace_outside_every_root_is_still_refused() {
        let (root, mut policy) = sandbox("missing-outside");
        policy.default_parent = std::env::temp_dir().join("deltaforge-elsewhere-workspace");
        let refusal = policy
            .resolve(None, "project")
            .expect_err("a workspace outside the permitted roots must be refused");
        assert!(format!("{refusal:#}").contains("outside both"));
        let _ = fs::remove_dir_all(root);
    }

    /// The hidden-directory rule is measured on the same canonical form, so a
    /// workspace under a dotted directory is refused whether or not it is
    /// there yet.
    #[test]
    fn a_missing_workspace_inside_a_hidden_directory_is_refused() {
        let (root, mut policy) = sandbox("missing-hidden");
        policy.default_parent = root.join(".cache").join("workspace");
        let refusal = policy
            .resolve(None, "project")
            .expect_err("a hidden workspace must be refused");
        assert!(format!("{refusal:#}").contains("hidden directory .cache"));
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
