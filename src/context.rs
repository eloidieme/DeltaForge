use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

use anyhow::{Context, Result, anyhow, bail};

use crate::config::ProjectConfig;
use crate::integrity::{digest_pack_tree, digest_project_tree};
use crate::pack::{
    LoadedPack, PackSearchOptions, PerformanceGate, is_bundled_source, load_pack, pack_source_label,
};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GateStatus {
    Passed,
    NotYet,
    NotMeasured,
}

impl GateStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::NotYet => "not_yet",
            Self::NotMeasured => "not_measured",
        }
    }
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
        let root = locate_project_root(options)?;
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

    /// Register a file this process just wrote at the project root as
    /// generated evidence, so writing it never stales the completion proof it
    /// describes. Only a plain file directly inside the project root is
    /// covered (like the built-in `deltaforge-report.*` exclusions and the
    /// existing `integrity.exclude` mechanism, exclusion matching is
    /// root-only); a nested `--output` path is left alone. Safe to call
    /// repeatedly: the name is added to `integrity.exclude` at most once.
    pub fn exclude_generated_root_file(&mut self, output: &Path) -> Result<()> {
        let resolved = if output.is_absolute() {
            output.to_path_buf()
        } else {
            self.root.join(output)
        };
        let Some(parent) = resolved.parent() else {
            return Ok(());
        };
        let root = self
            .root
            .canonicalize()
            .unwrap_or_else(|_| self.root.clone());
        let parent = parent
            .canonicalize()
            .unwrap_or_else(|_| parent.to_path_buf());
        if parent != root {
            return Ok(());
        }
        let Some(name) = resolved.file_name().and_then(|name| name.to_str()) else {
            return Ok(());
        };
        if self
            .config
            .integrity
            .exclude
            .iter()
            .any(|entry| entry == name)
        {
            return Ok(());
        }
        self.config.integrity.exclude.push(name.to_string());
        self.config.write_to(&self.config_path)
    }

    pub fn pack_digest(&self) -> Result<String> {
        digest_pack_tree(&self.pack.root)
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
            // Exports DeltaForge itself writes into the project. They are
            // evidence about the project, not part of it, so producing one must
            // not invalidate the completion proof it describes.
            "deltaforge-report.md",
            "deltaforge-report.html",
            "deltaforge-report.json",
        ];
        for ignored in &self.pack.manifest.ignored_paths {
            if !excluded.contains(&ignored.as_str()) {
                excluded.push(ignored.as_str());
            }
        }
        for ignored in &self.config.integrity.exclude {
            if !excluded.contains(&ignored.as_str()) {
                excluded.push(ignored.as_str());
            }
        }
        digest_project_tree(&self.root, &excluded)
    }

    /// Behavioral digest of one stage for this project's language: the inputs
    /// that determine whether the stage passes (tests, fixtures, build/run
    /// commands).
    pub fn stage_behavioral_digest(&self, stage_id: &str) -> Result<String> {
        let stage = self
            .pack
            .manifest
            .stage(stage_id)
            .with_context(|| format!("pack does not contain stage {stage_id}"))?;
        let language = self
            .pack
            .manifest
            .language(&self.state.language)
            .with_context(|| {
                format!(
                    "pack {} does not support language {}",
                    self.state.project, self.state.language
                )
            })?;
        self.pack.stage_behavioral_digest(stage, language)
    }

    /// Whether a stage's completion proof is stale relative to the current
    /// pack: its tests, fixtures, or commands changed since the stage passed.
    /// Learner-side edits are not considered here; they are checked separately
    /// at `next`/`commit` time.
    pub fn stage_needs_revalidation(&self, stage_id: &str) -> Result<bool> {
        let Some(proof) = self.state.completion_proofs.get(stage_id) else {
            return Ok(true);
        };
        if proof.behavioral_digest.is_empty() {
            // Legacy proof recorded before behavioral digests existed. It is
            // only trustworthy if the pack is bit-identical to the one that
            // passed.
            return Ok(proof.pack_digest != self.pack_digest()?);
        }
        Ok(proof.behavioral_digest != self.stage_behavioral_digest(stage_id)?)
    }

    pub fn verify_completion_proof(&self, stage_id: &str) -> Result<()> {
        let proof = self
            .state
            .completion_proofs
            .get(stage_id)
            .with_context(|| {
                format!("stage {stage_id} has no current completion record; its checks need to pass again")
            })?;
        if self.stage_needs_revalidation(stage_id)? {
            bail!(
                "stage {stage_id} passed against an older version of this pack and must be revalidated by passing its checks again"
            );
        }
        let project_digest = self.project_digest()?;
        if proof.project_digest != project_digest {
            bail!(
                "learner project changed since stage {stage_id} passed; its checks need to pass again"
            );
        }
        Ok(())
    }

    pub fn stage_gates(&self, stage_id: &str) -> Result<Vec<PerformanceGate>> {
        let stage = self
            .pack
            .manifest
            .stage(stage_id)
            .with_context(|| format!("pack does not contain stage {stage_id}"))?;
        let path = self.pack.benchmarks_path(stage);
        if !path.is_file() {
            return Ok(Vec::new());
        }
        let source = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read benchmarks file {}", path.display()))?;
        let parsed: crate::pack::StageBenchmarks = serde_yaml::from_str(&source)
            .with_context(|| format!("failed to parse benchmarks file {}", path.display()))?;
        Ok(parsed.performance_gates)
    }

    pub fn gate_status(&self, stage_id: &str) -> Result<Option<GateStatus>> {
        let gates = self.stage_gates(stage_id)?;
        if gates.is_empty() {
            return Ok(None);
        }
        let Some(record) = self.state.gate_results.get(stage_id) else {
            // No measurement to compare against, so the learner's tree does
            // not need walking at all.
            return Ok(Some(GateStatus::NotMeasured));
        };
        let project_digest = self.project_digest()?;
        Ok(Some(self.gate_record_status(
            stage_id,
            &gates,
            record,
            &project_digest,
        )?))
    }

    /// [`Self::gate_status`] against a project digest the caller already holds.
    ///
    /// The performance view asks about every stage in one pass. Letting each
    /// question compute its own digest walked the learner's tree once per
    /// stage instead of once per pass, on a view the workbench re-renders
    /// twice a second.
    pub fn gate_status_with_digest(
        &self,
        stage_id: &str,
        project_digest: &str,
    ) -> Result<Option<GateStatus>> {
        let gates = self.stage_gates(stage_id)?;
        if gates.is_empty() {
            return Ok(None);
        }
        let Some(record) = self.state.gate_results.get(stage_id) else {
            return Ok(Some(GateStatus::NotMeasured));
        };
        Ok(Some(self.gate_record_status(
            stage_id,
            &gates,
            record,
            project_digest,
        )?))
    }

    fn gate_record_status(
        &self,
        stage_id: &str,
        gates: &[PerformanceGate],
        record: &crate::state::GateRecord,
        project_digest: &str,
    ) -> Result<GateStatus> {
        if record.behavioral_digest.is_empty()
            || record.project_digest != project_digest
            || record.behavioral_digest != self.stage_behavioral_digest(stage_id)?
            || !gate_record_matches(record, gates)
        {
            return Ok(GateStatus::NotMeasured);
        }
        Ok(if record.results.iter().all(recorded_gate_result_passes) {
            GateStatus::Passed
        } else {
            GateStatus::NotYet
        })
    }

    pub fn verify_gate_record(&self, stage_id: &str) -> Result<()> {
        match self.gate_status(stage_id)? {
            Some(GateStatus::Passed) => Ok(()),
            Some(GateStatus::NotYet) => bail!(
                "the performance target for stage {stage_id} is not met yet; measure again after changing the implementation"
            ),
            Some(GateStatus::NotMeasured) => bail!(
                "the performance target for stage {stage_id} has not been measured against the current source; run the benchmarks"
            ),
            None => Ok(()),
        }
    }
}

fn gate_record_matches(record: &crate::state::GateRecord, gates: &[PerformanceGate]) -> bool {
    if record.results.len() != gates.len() {
        return false;
    }

    // Display names and advice are deliberately absent from the behavioral
    // digest, so match the same progression-affecting identity here. Consume
    // each result once so duplicate semantic gates remain deterministic.
    let mut used = vec![false; record.results.len()];
    for gate in gates {
        let Some(bound) = gate.bound() else {
            return false;
        };
        let Some(index) = record
            .results
            .iter()
            .enumerate()
            .position(|(index, result)| {
                !used[index]
                    && result.benchmark == gate.benchmark
                    && result.metric == gate.metric
                    && result.params == gate.params
                    && result.bound == bound
                    && result.measured.is_finite()
            })
        else {
            return false;
        };
        used[index] = true;
    }
    true
}

fn recorded_gate_result_passes(result: &crate::state::RecordedGateResult) -> bool {
    result.measured.is_finite()
        && match result.bound {
            crate::pack::GateBound::Min(min) => min.is_finite() && result.measured >= min,
            crate::pack::GateBound::Max(max) => max.is_finite() && result.measured <= max,
        }
}

fn verify_pack_pin(state: &ProjectState, pack: &LoadedPack) -> Result<()> {
    if !state.pack_version.is_empty() && state.pack_version != pack.manifest.version {
        bail!(
            "project is pinned to pack {} version {}, but discovery selected version {} from {}. Adopt the currently discovered pack definition to continue.",
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
                "project is pinned to pack source {}, but discovery selected {}. Restore the original pack source or adopt the currently discovered pack definition.",
                state.pack_source,
                actual_label
            );
        }
    }
    if !state.pack_digest.is_empty() {
        // Pack content is immutable for normal learners and changes only when
        // an author edits an installed pack. Loading workbench state every
        // half-second used to walk all 541 bundled-pack files merely to prove
        // the same digest again. The pack directory mtime is the cheap gate:
        // when it and the expected digest agree with a successful check, the
        // expensive walk cannot add useful information.
        let directory_mtime = std::fs::metadata(&pack.root)
            .and_then(|metadata| metadata.modified())
            .ok();
        if pack_pin_cache_hit(&pack.root, directory_mtime, &state.pack_digest) {
            return Ok(());
        }
        let actual = digest_pack_tree(&pack.root)?;
        if state.pack_digest != actual {
            bail!(
                "pack contents changed since project initialization. Adopt the currently discovered pack definition to continue."
            );
        }
        remember_pack_pin(&pack.root, directory_mtime, &state.pack_digest);
    }
    Ok(())
}

type PackPinCache = HashMap<PathBuf, (Option<SystemTime>, String)>;

fn pack_pin_cache() -> &'static Mutex<PackPinCache> {
    static CACHE: OnceLock<Mutex<PackPinCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn pack_pin_cache_hit(root: &Path, mtime: Option<SystemTime>, expected: &str) -> bool {
    crate::sync::lock(pack_pin_cache())
        .get(root)
        .is_some_and(|cached| cached.0 == mtime && cached.1 == expected)
}

fn remember_pack_pin(root: &Path, mtime: Option<SystemTime>, expected: &str) {
    crate::sync::lock(pack_pin_cache()).insert(root.to_path_buf(), (mtime, expected.to_string()));
}

pub fn locate_project_root(options: &GlobalOptions) -> Result<PathBuf> {
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

    // `find_project_root`'s own error carries no information beyond what is
    // folded into this sentence, so it is replaced rather than chained:
    // chaining it via `with_context` would tack a stray
    // `: could not find .deltaforge/state.json` onto the end of the
    // actionable "Run `deltaforge init`" sentence instead of leaving that
    // sentence last, where a reader expects the advice to land.
    find_project_root(&start).map_err(|_| {
        anyhow!(
            "not inside a DeltaForge project: searched upward from {} without finding .deltaforge/state.json\nRun `deltaforge init <project> --lang <language>` to create one.",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pack::{GateBound, PerformanceMetric};
    use crate::state::RecordedGateResult;

    #[test]
    fn recorded_gate_truth_is_recomputed_from_measurement_and_bound() {
        let mut result = RecordedGateResult {
            name: "gate".to_string(),
            benchmark: "bench".to_string(),
            metric: PerformanceMetric::RuntimeMedianMs,
            params: Default::default(),
            bound: GateBound::Min(2.0),
            measured: 1.0,
            passed: true,
        };
        assert!(!recorded_gate_result_passes(&result));

        result.measured = 3.0;
        result.passed = false;
        assert!(recorded_gate_result_passes(&result));
    }
}
