use std::env;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::config::ProjectConfig;
use crate::integrity::digest_tree;
use crate::pack::{LoadedPack, PackSearchOptions, is_bundled_source, load_pack, pack_source_label};
use crate::state::ProjectState;

#[derive(Debug, Clone, Default)]
pub struct GlobalOptions {
    pub project_dir: Option<PathBuf>,
    pub packs_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub root: PathBuf,
    pub state_path: PathBuf,
    pub config_path: PathBuf,
    pub state: ProjectState,
    pub config: ProjectConfig,
    pub pack: LoadedPack,
}

impl ProjectContext {
    pub fn load(options: &GlobalOptions) -> Result<Self> {
        Self::load_inner(options, true)
    }

    /// Load without enforcing the pack pin. Used by `deltaforge sync-pack`,
    /// which exists precisely to re-pin a project whose pack moved or changed.
    pub fn load_unpinned(options: &GlobalOptions) -> Result<Self> {
        Self::load_inner(options, false)
    }

    fn load_inner(options: &GlobalOptions, verify_pin: bool) -> Result<Self> {
        let root = resolve_project_root(options)?;
        let state_path = root.join(".deltaforge").join("state.json");
        let config_path = root.join(".deltaforge").join("config.toml");

        if !state_path.is_file() {
            bail!(
                "not inside a DeltaForge project: expected state file at {}\nRun `deltaforge init <project> --lang <language>` to create one.",
                state_path.display()
            );
        }

        let state = ProjectState::read_from(&state_path)?;
        let config = ProjectConfig::read_from(&config_path)?;
        let pack = load_pack(
            &state.project,
            &PackSearchOptions {
                packs_dir: options.packs_dir.clone(),
            },
        )?;
        if verify_pin {
            verify_pack_pin(&state, &pack)?;
        }

        Ok(Self {
            root,
            state_path,
            config_path,
            state,
            config,
            pack,
        })
    }

    pub fn save_state(&self) -> Result<()> {
        self.state.write_to(&self.state_path)
    }

    pub fn pack_digest(&self) -> Result<String> {
        digest_tree(&self.pack.root, &[])
    }

    pub fn project_digest(&self) -> Result<String> {
        let mut excluded: Vec<&str> = vec![
            ".git",
            ".deltaforge",
            "target",
            "build",
            "node_modules",
            "__pycache__",
            ".venv",
            ".DS_Store",
        ];
        for ignored in &self.pack.manifest.ignored_paths {
            if !excluded.contains(&ignored.as_str()) {
                excluded.push(ignored.as_str());
            }
        }
        digest_tree(&self.root, &excluded)
    }

    pub fn verify_completion_proof(&self, stage_id: &str) -> Result<()> {
        let proof = self
            .state
            .completion_proofs
            .get(stage_id)
            .with_context(|| {
                format!("stage {stage_id} has no integrity proof; run `deltaforge test` again")
            })?;
        let pack_digest = self.pack_digest()?;
        if proof.pack_digest != pack_digest {
            bail!(
                "pack contents changed since stage {stage_id} passed; run `deltaforge test` again"
            );
        }
        let project_digest = self.project_digest()?;
        if proof.project_digest != project_digest {
            bail!(
                "learner project changed since stage {stage_id} passed; run `deltaforge test` again"
            );
        }
        Ok(())
    }
}

fn verify_pack_pin(state: &ProjectState, pack: &LoadedPack) -> Result<()> {
    if !state.pack_version.is_empty() && state.pack_version != pack.manifest.version {
        bail!(
            "project is pinned to pack {} version {}, but discovery selected version {} from {}. Run `deltaforge sync-pack` to re-pin to the current pack.",
            state.project,
            state.pack_version,
            pack.manifest.version,
            pack.root.display()
        );
    }
    if !state.pack_source.is_empty() {
        let actual_label = pack_source_label(&pack.root);
        let matches = if is_bundled_source(&state.pack_source) {
            actual_label == "bundled"
        } else {
            let actual = pack
                .root
                .canonicalize()
                .unwrap_or_else(|_| pack.root.clone());
            Path::new(&state.pack_source) == actual
        };
        if !matches {
            bail!(
                "project is pinned to pack source {}, but discovery selected {}. Run `deltaforge sync-pack` to re-pin, or use the original --packs-dir.",
                state.pack_source,
                actual_label
            );
        }
    }
    if !state.pack_digest.is_empty() {
        let actual = digest_tree(&pack.root, &[])?;
        if state.pack_digest != actual {
            bail!(
                "pack contents changed since project initialization. Run `deltaforge sync-pack` to re-pin to the current pack."
            );
        }
    }
    Ok(())
}

fn resolve_project_root(options: &GlobalOptions) -> Result<PathBuf> {
    let start = match &options.project_dir {
        Some(path) => path.clone(),
        None => env::current_dir().context("failed to read current directory")?,
    };

    let start = normalize_existing_or_current(&start)?;
    if options.project_dir.is_some() {
        let state_path = start.join(".deltaforge").join("state.json");
        if state_path.is_file() {
            return Ok(start);
        }
        bail!(
            "not inside a DeltaForge project: expected state file at {}\nCheck --project-dir or run `deltaforge init <project> --lang <language>` to create one.",
            state_path.display()
        );
    }

    find_project_root(&start).with_context(|| {
        format!(
            "not inside a DeltaForge project: searched upward from {}\nRun `deltaforge init <project> --lang <language>` to create one.",
            start.display()
        )
    })
}

fn normalize_existing_or_current(path: &Path) -> Result<PathBuf> {
    if path.exists() {
        path.canonicalize()
            .with_context(|| format!("failed to canonicalize {}", path.display()))
    } else {
        Ok(path.to_path_buf())
    }
}

fn find_project_root(start: &Path) -> Result<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join(".deltaforge").join("state.json").is_file() {
            return Ok(current);
        }
        if !current.pop() {
            bail!("could not find .deltaforge/state.json");
        }
    }
}
