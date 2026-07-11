use std::env;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::config::ProjectConfig;
use crate::pack::{LoadedPack, PackSearchOptions, load_pack};
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
