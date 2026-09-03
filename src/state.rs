use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::fs_util::atomic_write;
use crate::pack::{GateBound, PerformanceMetric};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectState {
    #[serde(default = "current_state_schema_version")]
    pub schema_version: u32,
    pub project: String,
    pub language: String,
    #[serde(default)]
    pub pack_version: String,
    #[serde(default)]
    pub pack_source: String,
    #[serde(default)]
    pub pack_digest: String,
    pub current_stage: String,
    #[serde(default)]
    pub completed_stages: Vec<String>,
    #[serde(default)]
    pub completed_stage_timestamps: BTreeMap<String, String>,
    #[serde(default)]
    pub completion_proofs: BTreeMap<String, CompletionProof>,
    #[serde(default)]
    pub last_test_runs: BTreeMap<String, LastTestRunSummary>,
    #[serde(default)]
    pub hint_state: BTreeMap<String, usize>,
    #[serde(default)]
    pub gate_results: BTreeMap<String, GateRecord>,
    /// Per-stage performance prediction, recorded before the stage's first
    /// benchmark run. Present and `skipped` when the learner declined.
    #[serde(default)]
    pub predictions: BTreeMap<String, LearnerNote>,
    /// Per-stage reflection, offered once measurements exist.
    #[serde(default)]
    pub reflections: BTreeMap<String, LearnerNote>,
    #[serde(default)]
    pub attempt_history: Vec<JobAttempt>,
    #[serde(default)]
    pub active_job: Option<ActiveJob>,
    #[serde(default)]
    pub observed_project_digest: String,
    #[serde(default)]
    pub source_revision: u64,
    #[serde(default)]
    pub source_event_revision: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_source_change: Option<SourceChangeRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_workbench_session: Option<WorkbenchSessionRecord>,
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GateRecord {
    pub timestamp: String,
    pub project_digest: String,
    #[serde(default)]
    pub behavioral_digest: String,
    pub results: Vec<RecordedGateResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordedGateResult {
    pub name: String,
    pub benchmark: String,
    pub metric: PerformanceMetric,
    #[serde(default)]
    pub params: BTreeMap<String, String>,
    pub bound: GateBound,
    pub measured: f64,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionProof {
    /// Whole-pack digest at pass time. Retained for context and for migrating
    /// proofs recorded before behavioral digests existed.
    pub pack_digest: String,
    /// Digest of the stage inputs that determine passing: tests, fixtures, and
    /// the language build/run commands. Empty on legacy proofs.
    #[serde(default)]
    pub behavioral_digest: String,
    pub project_digest: String,
    pub test_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LastTestRunSummary {
    pub stage_id: String,
    pub passed: usize,
    pub failed: usize,
    pub timestamp: String,
    #[serde(default)]
    pub failed_tests: Vec<LastFailedTest>,
    #[serde(default)]
    pub project_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LastFailedTest {
    pub name: String,
    pub failures: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<FailureDiagnosis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FailureDiagnosis {
    #[serde(default = "default_diagnosis_priority")]
    pub priority: u32,
    pub kind: String,
    pub headline: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual: Option<String>,
    pub contract: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixture: Option<String>,
    #[serde(default)]
    pub fixture_entries: Vec<String>,
    #[serde(default)]
    pub command: Vec<String>,
}

fn default_diagnosis_priority() -> u32 {
    1_000
}

/// What a run lease is being held for. Tests and benchmarks share the lease,
/// the journal, the cancellation path, and the attempt history, so every
/// record carries the kind that produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobKind {
    #[default]
    Tests,
    Benchmarks,
}

impl JobKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Tests => "checks",
            Self::Benchmarks => "benchmarks",
        }
    }
}

/// A learner-authored note attached to one stage. `skipped` records that the
/// learner was offered the prompt and declined, which is different from never
/// having been asked.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearnerNote {
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub skipped: bool,
    pub recorded_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveJob {
    pub id: String,
    #[serde(default)]
    pub kind: JobKind,
    pub stage_ids: Vec<String>,
    pub started_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptStatus {
    Running,
    Passed,
    Failed,
    Cancelled,
    Interrupted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobAttempt {
    pub job_id: String,
    #[serde(default)]
    pub kind: JobKind,
    pub stage_ids: Vec<String>,
    pub started_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    pub status: AttemptStatus,
    #[serde(default)]
    pub passed: usize,
    #[serde(default)]
    pub failed: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceChangeRecord {
    pub revision: u64,
    pub previous_digest: String,
    pub current_digest: String,
    pub observed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkbenchSessionRecord {
    pub id: String,
    pub started_at: String,
    pub stage_id: String,
    pub baseline_updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_session_started_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_stage_id: Option<String>,
    #[serde(default)]
    pub recovered_interrupted_job: bool,
}

impl ProjectState {
    pub fn new(project: String, language: String, current_stage: String) -> Result<Self> {
        let now = current_timestamp()?;
        Ok(Self {
            schema_version: current_state_schema_version(),
            project,
            language,
            pack_version: String::new(),
            pack_source: String::new(),
            pack_digest: String::new(),
            current_stage,
            completed_stages: Vec::new(),
            completed_stage_timestamps: BTreeMap::new(),
            completion_proofs: BTreeMap::new(),
            last_test_runs: BTreeMap::new(),
            hint_state: BTreeMap::new(),
            gate_results: BTreeMap::new(),
            predictions: BTreeMap::new(),
            reflections: BTreeMap::new(),
            attempt_history: Vec::new(),
            active_job: None,
            observed_project_digest: String::new(),
            source_revision: 0,
            source_event_revision: 0,
            last_source_change: None,
            last_workbench_session: None,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn write_to(&self, path: &Path) -> Result<()> {
        let serialized =
            serde_json::to_string_pretty(self).context("failed to serialize project state")?;
        if path.is_file() {
            let previous = std::fs::read(path)
                .with_context(|| format!("failed to preserve state file {}", path.display()))?;
            atomic_write(&previous_state_path(path), previous).with_context(|| {
                format!("failed to preserve previous state file {}", path.display())
            })?;
        }
        atomic_write(path, serialized)
            .with_context(|| format!("failed to write state file {}", path.display()))?;

        Ok(())
    }

    pub fn read_from(path: &Path) -> Result<Self> {
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read state file {}", path.display()))?;
        // Check the schema version with a two-field probe before the strict,
        // `deny_unknown_fields` deserialization below: a state file written
        // by a *newer* DeltaForge carries fields this build doesn't know
        // about, and without this probe that would fail as an "unknown
        // field" serde error rather than the actionable version mismatch it
        // actually is.
        #[derive(Deserialize)]
        struct SchemaProbe {
            #[serde(default)]
            schema_version: u32,
        }
        if let Ok(probe) = serde_json::from_str::<SchemaProbe>(&source) {
            check_schema_version(probe.schema_version, path)?;
        }
        let mut document: serde_json::Value =
            serde_json::from_str(&source).map_err(|_| damaged_state_error(path))?;
        migrate(&mut document, path).map_err(|error| {
            if format!("{error:#}").contains("newer version of DeltaForge") {
                error
            } else {
                damaged_state_error(path)
            }
        })?;
        let mut state: Self =
            serde_json::from_value(document).map_err(|_| damaged_state_error(path))?;
        state.validate(path).map_err(|error| {
            if format!("{error:#}").contains("newer version of DeltaForge") {
                error
            } else {
                damaged_state_error(path)
            }
        })?;
        if state.updated_at.is_empty() {
            state.updated_at = state.created_at.clone();
        }
        Ok(state)
    }

    pub fn validate(&self, path: &Path) -> Result<()> {
        check_schema_version(self.schema_version, path)?;
        if self.project.trim().is_empty() {
            bail!("invalid state {}: project is empty", path.display());
        }
        if self.language.trim().is_empty() {
            bail!("invalid state {}: language is empty", path.display());
        }
        if self.current_stage.trim().is_empty() {
            bail!("invalid state {}: current_stage is empty", path.display());
        }
        Ok(())
    }

    pub fn is_completed(&self, stage_id: &str) -> bool {
        self.completed_stages
            .iter()
            .any(|completed| completed == stage_id)
    }

    pub fn initialize_source_observation(&mut self, digest: String) -> bool {
        if !self.observed_project_digest.is_empty() {
            return false;
        }
        self.observed_project_digest = digest;
        true
    }

    pub fn observe_source_digest(&mut self, digest: String) -> Result<Option<SourceChangeRecord>> {
        if self.observed_project_digest.is_empty() {
            self.observed_project_digest = digest;
            return Ok(None);
        }
        if self.observed_project_digest == digest {
            return Ok(None);
        }
        let now = current_timestamp()?;
        let change = SourceChangeRecord {
            revision: self.source_revision.saturating_add(1),
            previous_digest: std::mem::replace(&mut self.observed_project_digest, digest.clone()),
            current_digest: digest,
            observed_at: now.clone(),
        };
        self.source_revision = change.revision;
        self.last_source_change = Some(change.clone());
        self.updated_at = now;
        Ok(Some(change))
    }

    pub fn acknowledge_source_event(&mut self, revision: u64) {
        self.source_event_revision = self.source_event_revision.max(revision);
    }

    pub fn mark_completed(&mut self, stage_id: &str) -> Result<()> {
        if !self.is_completed(stage_id) {
            self.completed_stages.push(stage_id.to_string());
        }
        let now = current_timestamp()?;
        self.completed_stage_timestamps
            .entry(stage_id.to_string())
            .or_insert_with(|| now.clone());
        self.updated_at = now;
        Ok(())
    }

    pub fn record_completion_proof(
        &mut self,
        stage_id: &str,
        pack_digest: String,
        behavioral_digest: String,
        project_digest: String,
        test_count: usize,
    ) -> Result<()> {
        self.mark_completed(stage_id)?;
        self.completion_proofs.insert(
            stage_id.to_string(),
            CompletionProof {
                pack_digest,
                behavioral_digest,
                project_digest,
                test_count,
            },
        );
        Ok(())
    }

    pub fn record_test_run(
        &mut self,
        stage_id: String,
        passed: usize,
        failed: usize,
        failed_tests: Vec<LastFailedTest>,
        project_digest: String,
    ) -> Result<()> {
        let now = current_timestamp()?;
        self.last_test_runs.insert(
            stage_id.clone(),
            LastTestRunSummary {
                stage_id,
                passed,
                failed,
                timestamp: now.clone(),
                failed_tests,
                project_digest,
            },
        );
        self.updated_at = now;
        Ok(())
    }

    pub fn start_job(&mut self, kind: JobKind, stage_ids: Vec<String>) -> Result<String> {
        let now = current_timestamp()?;
        let job_id = format!(
            "{}-{}",
            std::process::id(),
            OffsetDateTime::now_utc().unix_timestamp_nanos()
        );
        self.active_job = Some(ActiveJob {
            id: job_id.clone(),
            kind,
            stage_ids: stage_ids.clone(),
            started_at: now.clone(),
        });
        self.attempt_history.push(JobAttempt {
            job_id: job_id.clone(),
            kind,
            stage_ids,
            started_at: now.clone(),
            finished_at: None,
            status: AttemptStatus::Running,
            passed: 0,
            failed: 0,
            error: None,
        });
        const ATTEMPT_HISTORY_LIMIT: usize = 20;
        if self.attempt_history.len() > ATTEMPT_HISTORY_LIMIT {
            let excess = self.attempt_history.len() - ATTEMPT_HISTORY_LIMIT;
            self.attempt_history.drain(..excess);
        }
        self.updated_at = now;
        Ok(job_id)
    }

    pub fn finish_job(
        &mut self,
        job_id: &str,
        status: AttemptStatus,
        passed: usize,
        failed: usize,
        error: Option<String>,
    ) -> Result<()> {
        let now = current_timestamp()?;
        let attempt = self
            .attempt_history
            .iter_mut()
            .find(|attempt| attempt.job_id == job_id)
            .with_context(|| format!("job {job_id} is missing from attempt history"))?;
        attempt.finished_at = Some(now.clone());
        attempt.status = status;
        attempt.passed = passed;
        attempt.failed = failed;
        attempt.error = error;
        if self
            .active_job
            .as_ref()
            .is_some_and(|active| active.id == job_id)
        {
            self.active_job = None;
        }
        self.updated_at = now;
        Ok(())
    }

    pub fn clear_active_job(&mut self, job_id: &str) -> Result<()> {
        if self
            .active_job
            .as_ref()
            .is_some_and(|active| active.id == job_id)
        {
            self.active_job = None;
        }
        self.attempt_history
            .retain(|attempt| attempt.job_id != job_id || attempt.status != AttemptStatus::Running);
        self.updated_at = current_timestamp()?;
        Ok(())
    }

    pub fn recover_interrupted_job(&mut self) -> Result<bool> {
        let Some(active) = self.active_job.take() else {
            return Ok(false);
        };
        let now = current_timestamp()?;
        if let Some(attempt) = self
            .attempt_history
            .iter_mut()
            .find(|attempt| attempt.job_id == active.id)
        {
            attempt.finished_at = Some(now.clone());
            attempt.status = AttemptStatus::Interrupted;
            attempt.error = Some("DeltaForge stopped before this run finished".to_string());
        } else {
            self.attempt_history.push(JobAttempt {
                job_id: active.id,
                kind: active.kind,
                stage_ids: active.stage_ids,
                started_at: active.started_at,
                finished_at: Some(now.clone()),
                status: AttemptStatus::Interrupted,
                passed: 0,
                failed: 0,
                error: Some("DeltaForge stopped before this run finished".to_string()),
            });
        }
        self.updated_at = now;
        Ok(true)
    }

    pub fn begin_workbench_session(
        &mut self,
        session_id: String,
        recovered_interrupted_job: bool,
    ) -> Result<bool> {
        if let Some(session) = self.last_workbench_session.as_mut()
            && session.id == session_id
        {
            let changed = recovered_interrupted_job && !session.recovered_interrupted_job;
            session.recovered_interrupted_job |= recovered_interrupted_job;
            return Ok(changed);
        }
        let previous = self.last_workbench_session.take();
        self.last_workbench_session = Some(WorkbenchSessionRecord {
            id: session_id,
            started_at: current_timestamp()?,
            stage_id: self.current_stage.clone(),
            baseline_updated_at: self.updated_at.clone(),
            previous_session_started_at: previous
                .as_ref()
                .map(|session| session.started_at.clone()),
            previous_stage_id: previous.map(|session| session.stage_id),
            recovered_interrupted_job,
        });
        Ok(true)
    }

    /// Record (or replace) the learner's prediction for one stage.
    pub fn record_prediction(&mut self, stage_id: &str, text: String, skipped: bool) -> Result<()> {
        let note = LearnerNote {
            text,
            skipped,
            recorded_at: current_timestamp()?,
        };
        self.predictions.insert(stage_id.to_string(), note);
        self.updated_at = current_timestamp()?;
        Ok(())
    }

    /// Record (or replace) the learner's reflection for one stage.
    pub fn record_reflection(&mut self, stage_id: &str, text: String, skipped: bool) -> Result<()> {
        let note = LearnerNote {
            text,
            skipped,
            recorded_at: current_timestamp()?,
        };
        self.reflections.insert(stage_id.to_string(), note);
        self.updated_at = current_timestamp()?;
        Ok(())
    }

    pub fn touch(&mut self) -> Result<()> {
        self.updated_at = current_timestamp()?;
        Ok(())
    }
}

pub fn previous_state_path(path: &Path) -> std::path::PathBuf {
    path.with_file_name("state.json.prev")
}

fn damaged_state_error(path: &Path) -> anyhow::Error {
    anyhow!(
        "saved progress in {} is damaged and cannot be read; run `deltaforge doctor --repair` from this project to restore the previous saved state",
        path.display()
    )
}

fn current_state_schema_version() -> u32 {
    2
}

fn check_schema_version(found: u32, path: &Path) -> Result<()> {
    let expected = current_state_schema_version();
    if found <= expected {
        // Older is not an error; it is a migration. See `MIGRATIONS`.
        return Ok(());
    }
    bail!(
        "state schema_version {found} in {} was written by a newer version of DeltaForge (this build supports {expected}); upgrade DeltaForge to open this project",
        path.display()
    );
}

/// The chain of state-file migrations, one per schema version, in order.
///
/// Each entry takes a document written at version `n` and returns it at `n+1`.
///
/// This exists now, while there is exactly one rung to write, because the
/// alternative is what 1.0 shipped: every schema bump refused to load and told
/// the learner to run `deltaforge init` again, which destroys their progress.
/// That was defensible before there were users and is not defensible after.
/// The ladder is cheap to extend and impossible to add retroactively — by the
/// time a second bump is needed, the projects that would have needed rung one
/// are already broken.
///
/// Migrations work on `serde_json::Value`, not on `ProjectState`. That is the
/// point: `ProjectState` is the *current* shape, and a migration's whole job is
/// to handle a document that is not that shape yet. Deserializing first would
/// mean the migration could only ever run on documents that did not need it.
type Migration = fn(&mut serde_json::Map<String, serde_json::Value>);

const MIGRATIONS: &[(u32, Migration)] = &[(1, migrate_1_to_2)];

/// Bring `document` up to the current schema, in place.
///
/// A state file with no `schema_version` at all is treated as version 1: the
/// field was introduced with it, so its absence means the oldest shape.
fn migrate(document: &mut serde_json::Value, path: &Path) -> Result<()> {
    let expected = current_state_schema_version();
    let Some(object) = document.as_object_mut() else {
        bail!("invalid state {}: not a JSON object", path.display());
    };
    let mut version = object
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(1) as u32;

    while version < expected {
        let Some((_, step)) = MIGRATIONS.iter().find(|(from, _)| *from == version) else {
            bail!(
                "state schema_version {version} in {} cannot be upgraded to {expected}: no migration is defined for it",
                path.display()
            );
        };
        step(object);
        version += 1;
        object.insert(
            "schema_version".to_string(),
            serde_json::Value::from(version),
        );
    }
    Ok(())
}

/// Schema 1 to 2: jobs gained a kind, and stages gained prediction and
/// reflection notes.
///
/// Everything schema 2 added is optional to *parse* — `serde(default)` would
/// fill each one in. The migration writes them anyway, because a migration's
/// output should be a document that is genuinely at the new version rather
/// than one that happens to survive the current deserializer. Relying on the
/// defaults would make this rung silently stop working the day a field stops
/// being defaulted.
fn migrate_1_to_2(state: &mut serde_json::Map<String, serde_json::Value>) {
    // Every job recorded before schema 2 was a test run; benchmarks could not
    // be started as jobs yet.
    if let Some(active) = state
        .get_mut("active_job")
        .and_then(|job| job.as_object_mut())
    {
        active
            .entry("kind")
            .or_insert_with(|| serde_json::Value::from("tests"));
    }
    if let Some(attempts) = state
        .get_mut("attempt_history")
        .and_then(|history| history.as_array_mut())
    {
        for attempt in attempts.iter_mut().filter_map(|a| a.as_object_mut()) {
            attempt
                .entry("kind")
                .or_insert_with(|| serde_json::Value::from("tests"));
        }
    }
    for field in ["predictions", "reflections"] {
        state
            .entry(field)
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
    }
}

fn current_timestamp() -> Result<String> {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .context("failed to format current timestamp")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marking_completed_stages_is_idempotent() {
        let mut state = ProjectState::new(
            "flashindex".to_string(),
            "rust".to_string(),
            "01_scan_files".to_string(),
        )
        .unwrap();

        state.mark_completed("01_scan_files").unwrap();
        state.mark_completed("01_scan_files").unwrap();

        assert!(state.is_completed("01_scan_files"));
        assert_eq!(state.completed_stages, ["01_scan_files"]);
        assert!(
            state
                .completed_stage_timestamps
                .contains_key("01_scan_files")
        );
    }

    /// P2-6. A project written by an older DeltaForge opens, keeping every
    /// completion it had earned.
    ///
    /// 1.0 refused: `check_schema_version` rejected every older schema with
    /// "recreate the project with `deltaforge init`", so a schema bump was a
    /// promise to destroy the learner's progress. The fixture is a real
    /// schema-1 document, checked in, so this rung stays honest even after
    /// the code that wrote that shape is long gone.
    #[test]
    fn a_schema_1_project_migrates_to_the_current_schema() {
        let fixture =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/state/schema_1.json");
        let state = ProjectState::read_from(&fixture).expect("a schema 1 project must still open");

        assert_eq!(state.schema_version, current_state_schema_version());
        // Progress survives, which is the entire point.
        assert_eq!(state.completed_stages, ["01_scan_files"]);
        assert_eq!(state.current_stage, "02_filter_files");
        assert_eq!(state.hint_state.get("01_scan_files"), Some(&2));
        // And what schema 2 added is present and correct, not merely absent.
        assert_eq!(state.attempt_history.len(), 1);
        assert_eq!(state.attempt_history[0].kind, JobKind::Tests);
        assert!(state.predictions.is_empty());
        assert!(state.reflections.is_empty());
    }

    /// The ladder refuses what it cannot climb, rather than deserializing a
    /// document it has not actually migrated.
    #[test]
    fn a_schema_with_no_migration_is_refused_by_name() {
        let mut document = serde_json::json!({ "schema_version": 0 });
        let refusal =
            migrate(&mut document, Path::new("state.json")).expect_err("version 0 has no rung");
        assert!(format!("{refusal:#}").contains("no migration is defined"));
    }

    /// Forward compatibility is unchanged: a file from a newer build is a
    /// reason to upgrade, not something to guess at.
    #[test]
    fn a_newer_schema_is_still_refused_with_the_upgrade_message() {
        let newer = current_state_schema_version() + 1;
        let refusal = check_schema_version(newer, Path::new("state.json"))
            .expect_err("a newer schema must be refused");
        assert!(format!("{refusal:#}").contains("upgrade DeltaForge"));
    }

    #[test]
    fn a_damaged_state_has_a_learner_facing_recovery_message() {
        let root = crate::fs_util::create_private_scratch_dir("deltaforge-state-damaged").unwrap();
        let path = root.join("state.json");
        std::fs::write(&path, r#"{"schema_version":2,"project":"flashindex""#).unwrap();

        let message = format!("{:#}", ProjectState::read_from(&path).unwrap_err());
        assert!(message.contains("saved progress"));
        assert!(message.contains("doctor --repair"));
        assert!(!message.contains("expected"));
        assert!(!message.contains("completed_stages"));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn every_state_write_keeps_the_previous_complete_document() {
        let root = crate::fs_util::create_private_scratch_dir("deltaforge-state-backup").unwrap();
        let path = root.join("state.json");
        let mut state = ProjectState::new(
            "flashindex".to_string(),
            "rust".to_string(),
            "01_scan_files".to_string(),
        )
        .unwrap();
        state.write_to(&path).unwrap();
        let first = std::fs::read_to_string(&path).unwrap();

        state.current_stage = "02_filter_files".to_string();
        state.write_to(&path).unwrap();

        assert_eq!(
            std::fs::read_to_string(previous_state_path(&path)).unwrap(),
            first
        );
        assert_eq!(
            ProjectState::read_from(&path).unwrap().current_stage,
            "02_filter_files"
        );
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn created_at_is_rfc3339_utc() {
        let state = ProjectState::new(
            "flashindex".to_string(),
            "rust".to_string(),
            "01_scan_files".to_string(),
        )
        .unwrap();

        assert!(!state.created_at.starts_with("unix:"));
        assert!(state.created_at.ends_with('Z'));
        OffsetDateTime::parse(&state.created_at, &Rfc3339).unwrap();
    }

    #[test]
    fn source_observations_are_revisioned_and_idempotent() {
        let mut state = ProjectState::new(
            "flashindex".to_string(),
            "rust".to_string(),
            "01_scan_files".to_string(),
        )
        .unwrap();

        assert!(state.initialize_source_observation("digest-a".to_string()));
        assert!(!state.initialize_source_observation("ignored".to_string()));
        assert!(
            state
                .observe_source_digest("digest-a".to_string())
                .unwrap()
                .is_none()
        );
        let change = state
            .observe_source_digest("digest-b".to_string())
            .unwrap()
            .unwrap();
        assert_eq!(change.revision, 1);
        assert_eq!(change.previous_digest, "digest-a");
        assert_eq!(change.current_digest, "digest-b");
        assert_eq!(state.source_revision, 1);
        assert_eq!(state.source_event_revision, 0);
        assert!(
            state
                .observe_source_digest("digest-b".to_string())
                .unwrap()
                .is_none()
        );
        state.acknowledge_source_event(change.revision);
        assert_eq!(state.source_event_revision, 1);
    }
    #[test]

    fn active_job_is_recovered_as_interrupted() {
        let mut state = ProjectState::new(
            "flashindex".to_string(),
            "rust".to_string(),
            "01_scan_files".to_string(),
        )
        .unwrap();
        let job_id = state
            .start_job(JobKind::Tests, vec!["01_scan_files".to_string()])
            .unwrap();

        assert!(state.recover_interrupted_job().unwrap());
        assert!(state.active_job.is_none());
        let attempt = state
            .attempt_history
            .iter()
            .find(|attempt| attempt.job_id == job_id)
            .unwrap();
        assert_eq!(attempt.status, AttemptStatus::Interrupted);
        assert!(attempt.finished_at.is_some());
        assert!(attempt.error.is_some());
        assert!(!state.recover_interrupted_job().unwrap());
    }

    #[test]
    fn workbench_sessions_preserve_the_previous_resumption_point() {
        let mut state = ProjectState::new(
            "flashindex".to_string(),
            "rust".to_string(),
            "01_scan_files".to_string(),
        )
        .unwrap();

        assert!(
            state
                .begin_workbench_session("session-one".to_string(), false)
                .unwrap()
        );
        assert!(
            !state
                .begin_workbench_session("session-one".to_string(), false)
                .unwrap()
        );
        let first_started_at = state
            .last_workbench_session
            .as_ref()
            .unwrap()
            .started_at
            .clone();
        state.current_stage = "02_filter_files".to_string();

        assert!(
            state
                .begin_workbench_session("session-two".to_string(), true)
                .unwrap()
        );
        let resumed = state.last_workbench_session.as_ref().unwrap();
        assert_eq!(
            resumed.previous_session_started_at.as_deref(),
            Some(first_started_at.as_str())
        );
        assert_eq!(resumed.previous_stage_id.as_deref(), Some("01_scan_files"));
        assert_eq!(resumed.stage_id, "02_filter_files");
        assert!(resumed.recovered_interrupted_job);
    }

    #[test]
    fn attempt_history_keeps_only_the_latest_twenty_runs() {
        let mut state = ProjectState::new(
            "flashindex".to_string(),
            "rust".to_string(),
            "01_scan_files".to_string(),
        )
        .unwrap();

        for _ in 0..25 {
            let job_id = state
                .start_job(JobKind::Tests, vec!["01_scan_files".to_string()])
                .unwrap();
            state
                .finish_job(&job_id, AttemptStatus::Failed, 1, 1, None)
                .unwrap();
        }

        assert_eq!(state.attempt_history.len(), 20);
        assert!(state.active_job.is_none());
        assert!(
            state
                .attempt_history
                .iter()
                .all(|attempt| attempt.status == AttemptStatus::Failed)
        );
    }
}
