use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Serialize;

use crate::context::{GlobalOptions, ProjectContext};
use crate::pack::pack_source_label;
use crate::runner::{self, RunnerOptions, TestResult, TestRunSummary};
use crate::state::{
    ActiveJob, AttemptStatus, FailureDiagnosis, JobAttempt, JobKind, LastFailedTest,
    LastTestRunSummary, SourceChangeRecord,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunTrigger {
    Cli,
    Workbench,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RunEvent {
    JobStarted {
        job_id: String,
        kind: JobKind,
        stage_ids: Vec<String>,
        trigger: RunTrigger,
    },
    BuildStarted {
        command: Vec<String>,
    },
    BuildOutput {
        stream: &'static str,
        text: String,
    },
    BuildCompleted {
        passed: bool,
    },
    TestStarted {
        stage_id: String,
        name: String,
        index: usize,
        total: usize,
    },
    TestPassed {
        stage_id: String,
        result: TestResult,
    },
    TestFailed {
        stage_id: String,
        result: TestResult,
    },
    RunCompleted {
        job_id: String,
        passed: bool,
        passed_tests: usize,
        failed_tests: usize,
    },
    BenchmarkStarted {
        stage_id: String,
        name: String,
        index: usize,
        total: usize,
    },
    BenchmarkPointStarted {
        stage_id: String,
        name: String,
        params_label: String,
        index: usize,
        total: usize,
    },
    BenchmarkSampleRecorded {
        stage_id: String,
        name: String,
        params_label: String,
        iteration: u64,
        iterations: u64,
        duration_ms: f64,
    },
    BenchmarkPointCompleted {
        stage_id: String,
        name: String,
        params_label: String,
        success: bool,
        runtime_median_ms: Option<f64>,
        throughput_mb_s: Option<f64>,
        peak_memory_mb: Option<f64>,
        error: Option<String>,
    },
    BenchmarkCompleted {
        stage_id: String,
        name: String,
        success: bool,
    },
    GateEvaluated {
        stage_id: String,
        status: crate::context::GateStatus,
        passed: usize,
        total: usize,
    },
    BenchmarkRunCompleted {
        job_id: String,
        passed: bool,
        benchmarks: usize,
        failed_points: usize,
    },
    SourceChanged {
        revision: u64,
        previous_digest: String,
        current_digest: String,
    },
    ProjectStateChanged,
    JobInterrupted {
        job_id: String,
        reason: String,
    },
}

pub trait EventSink {
    fn emit(&mut self, event: RunEvent);
}

impl<F> EventSink for F
where
    F: FnMut(RunEvent),
{
    fn emit(&mut self, event: RunEvent) {
        self(event);
    }
}

pub struct NullEventSink;

impl EventSink for NullEventSink {
    fn emit(&mut self, _event: RunEvent) {}
}

#[derive(Debug, Clone)]
pub struct TestRunRequest {
    pub stage: Option<String>,
    pub all: bool,
    pub filter: Option<String>,
    pub list_tests: bool,
    pub fail_fast: bool,
    pub no_build: bool,
    pub keep_temp: bool,
    pub trigger: RunTrigger,
}

#[derive(Debug, Clone, Serialize)]
pub struct TestRunOutcome {
    pub job_id: String,
    pub summaries: Vec<TestRunSummary>,
    pub newly_completed_current: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_error: Option<String>,
}

impl TestRunOutcome {
    pub fn is_success(&self) -> bool {
        self.execution_error.is_none()
            && !self.summaries.is_empty()
            && self.summaries.iter().all(TestRunSummary::is_success)
    }

    pub fn passed(&self) -> usize {
        self.summaries.iter().map(|summary| summary.passed).sum()
    }

    pub fn failed(&self) -> usize {
        self.summaries.iter().map(|summary| summary.failed).sum()
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkRunRequest {
    pub stage: Option<String>,
    pub all: bool,
    pub iterations: Option<u64>,
    pub warmup: Option<u64>,
    pub save: bool,
    pub compare: bool,
    pub trigger: RunTrigger,
}

#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkRunOutcome {
    pub job_id: String,
    pub records: Vec<crate::benchmarks::BenchmarkRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparisons: Option<Vec<crate::benchmarks::BenchmarkComparison>>,
    pub gates: Vec<StageGateReport>,
    pub saved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_error: Option<String>,
}

impl BenchmarkRunOutcome {
    /// A benchmark run succeeds when it completed and every measured point
    /// produced a result. Failing a *gate* is a learner outcome, not a run
    /// failure, and never reported here.
    pub fn is_success(&self) -> bool {
        self.execution_error.is_none()
            && self
                .records
                .iter()
                .all(|record| record.points.iter().all(|point| point.success))
    }

    pub fn failed_points(&self) -> usize {
        self.records
            .iter()
            .flat_map(|record| &record.points)
            .filter(|point| !point.success)
            .count()
    }
}

/// One stage's gate outcome after a benchmark run, in the shape both the CLI
/// and the browser consume.
#[derive(Debug, Clone, Serialize)]
pub struct StageGateReport {
    pub stage_id: String,
    pub status: crate::context::GateStatus,
    pub gates: Vec<crate::benchmarks::EvaluatedGate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultFreshness {
    NeverRun,
    Fresh,
    Stale,
}

#[derive(Debug, Clone, Serialize)]
pub struct CapabilityState {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub next_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkbenchState {
    pub project: String,
    pub language: String,
    pub capability: CapabilityState,
    pub primary_action: PrimaryAction,
    pub freshness: ResultFreshness,
    pub revealed_hint_level: usize,
    pub last_activity_at: String,
    pub recovered_interrupted_job: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumption: Option<ResumptionSummary>,
    pub active_job: Option<ActiveJob>,
    pub latest_attempt: Option<JobAttempt>,
    pub attempt_history: Vec<JobAttempt>,
    pub latest_run: Option<LastTestRunSummary>,
    pub primary_failure: Option<LastFailedTest>,
    pub source_revision: u64,
    pub last_source_change: Option<SourceChangeRecord>,
    pub event_cursor: u64,
    /// The performance picture for the current step: what it measures, what it
    /// last measured, and whether its gates are met. Present on every step, so
    /// the browser can show a gate before the learner reaches it.
    pub performance: PerformanceState,
}

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceState {
    /// Whether the current step declares any benchmarks at all.
    pub has_benchmarks: bool,
    /// Gate status for the current step. `None` when the step declares no
    /// gates, which is the common case.
    pub gate_status: Option<crate::context::GateStatus>,
    /// Whether a failing or unmeasured gate currently blocks progression.
    /// False while `gates.enforce` is off, even when the gate is unmet.
    pub gate_blocks_progress: bool,
    pub gates: Vec<GateView>,
    /// The most recent saved measurement for each of the step's benchmarks.
    pub latest: Vec<BenchmarkView>,
    /// The step's prediction prompt, when it declares one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<crate::state::LearnerNote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflection: Option<crate::state::LearnerNote>,
    /// Gate status for every step of the journey, so the roadmap can mark
    /// which steps carry a measurement before the learner arrives.
    pub roadmap: Vec<StageGateMarker>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GateView {
    pub name: String,
    pub benchmark: String,
    pub metric: &'static str,
    pub comparison: &'static str,
    pub bound: f64,
    pub params_label: String,
    pub measured: Option<f64>,
    pub passed: bool,
    pub advice: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkView {
    pub name: String,
    pub timestamp: String,
    pub points: Vec<BenchmarkPointView>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkPointView {
    pub params_label: String,
    pub success: bool,
    pub runtime_median_ms: Option<f64>,
    pub runtime_p95_ms: Option<f64>,
    pub throughput_mb_s: Option<f64>,
    pub peak_memory_mb: Option<f64>,
    /// Percent change in median runtime against the previous saved run of the
    /// same benchmark and point on this machine. Negative is faster.
    pub median_percent_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StageGateMarker {
    pub stage_id: String,
    pub has_benchmarks: bool,
    pub status: Option<crate::context::GateStatus>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrimaryAction {
    pub kind: PrimaryActionKind,
    pub label: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrimaryActionKind {
    RunChecks,
    ResumeChecks,
    CancelRun,
    BeginNextCapability,
    JourneyComplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResumptionKind {
    Interrupted,
    CapabilityChanged,
    SourceChanged,
    ChecksFailed,
    CapabilityAcquired,
    Ready,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResumptionSummary {
    pub kind: ResumptionKind,
    pub title: String,
    pub detail: String,
    pub previous_session_started_at: Option<String>,
    pub stage_change: Option<StageChangeSummary>,
    pub action_pending: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct StageChangeSummary {
    pub from_id: String,
    pub from_title: String,
    pub to_id: String,
    pub to_title: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectHealthStatus {
    Healthy,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectHealth {
    pub status: ProjectHealthStatus,
    pub project: Option<String>,
    pub issue: Option<ProjectHealthIssue>,
    pub actions: Vec<ProjectHealthAction>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectHealthIssue {
    pub code: String,
    pub title: String,
    pub detail: String,
    pub guidance: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectHealthAction {
    pub kind: ProjectHealthActionKind,
    pub label: String,
    pub primary: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectHealthActionKind {
    Recheck,
    RepinPack,
    OpenEditor,
    OpenFolder,
}

/// One pack as the catalog presents it: enough to choose between projects
/// without opening any of them.
#[derive(Debug, Clone, Serialize)]
pub struct CatalogEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub topics: Vec<String>,
    pub tier: crate::pack::PackTier,
    pub tier_label: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_hours: Option<crate::pack::HourEstimate>,
    pub step_count: usize,
    pub languages: Vec<CatalogLanguage>,
    /// The pack's own overview prose, so the catalog can show what the
    /// finished project does before the learner commits to it.
    pub overview: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CatalogLanguage {
    pub id: String,
    pub name: String,
    /// Whether every tool this language declares was found on PATH.
    pub available: bool,
    pub tools: Vec<crate::creation::ToolStatus>,
}

/// Every discovered pack, flagship first, with each language's toolchain
/// already checked so the catalog can say what is installable right now.
pub fn load_catalog(options: &GlobalOptions) -> Result<Vec<CatalogEntry>> {
    let discovery = crate::pack::discover_packs_with_options(&crate::pack::PackSearchOptions {
        packs_dir: options.packs_dir.clone(),
    })?;
    let mut entries = discovery
        .packs
        .iter()
        .map(|pack| {
            let languages = pack
                .manifest
                .languages
                .iter()
                .map(|(id, language)| {
                    let tools = crate::creation::language_tools(language);
                    CatalogLanguage {
                        id: id.clone(),
                        name: language.display_name.clone().unwrap_or_else(|| id.clone()),
                        available: tools.iter().all(|tool| tool.found || !tool.required),
                        tools,
                    }
                })
                .collect();
            CatalogEntry {
                id: pack.manifest.id.clone(),
                name: pack.manifest.name.clone(),
                description: pack.manifest.description.clone(),
                topics: pack.manifest.topics.clone(),
                tier: pack.manifest.tier,
                tier_label: pack.manifest.tier.label(),
                difficulty: pack
                    .manifest
                    .difficulty
                    .map(crate::pack::PackDifficulty::label),
                estimated_hours: pack.manifest.estimated_hours,
                step_count: pack.manifest.stages.len(),
                languages,
                overview: crate::commands::overview::read_pack_overview(pack),
            }
        })
        .collect::<Vec<_>>();
    // Flagship packs first, then alphabetically, so the catalog's first card is
    // always the one held to the full content bar.
    entries.sort_by(|left, right| {
        let rank = |entry: &CatalogEntry| u8::from(entry.tier != crate::pack::PackTier::Flagship);
        rank(left)
            .cmp(&rank(right))
            .then_with(|| left.name.cmp(&right.name))
    });
    Ok(entries)
}

#[derive(Debug, Clone)]
pub struct CreateProjectRequest {
    pub pack: String,
    pub language: String,
    /// Directory the project is created inside. `None` uses the default
    /// workspace. Validated by [`crate::creation::resolve_target`].
    pub parent_directory: Option<PathBuf>,
    pub name: String,
    pub git: bool,
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatedProject {
    /// Registry identifier; every later browser request names this, never the
    /// path.
    pub project_id: String,
    pub path: String,
    pub pack: String,
    pub language: String,
    pub stage_id: String,
    pub stage_title: String,
}

/// Check the environment for one pack, language, and proposed location
/// without creating anything.
pub fn preflight_project(
    options: &GlobalOptions,
    pack_id: &str,
    language: &str,
    parent: Option<&Path>,
    name: &str,
) -> Result<crate::creation::Preflight> {
    let pack = crate::pack::load_pack(
        pack_id,
        &crate::pack::PackSearchOptions {
            packs_dir: options.packs_dir.clone(),
        },
    )?;
    crate::creation::preflight(&pack, language, parent, name)
}

/// Create a project and register it, so the caller can open it immediately by
/// identifier. The preflight runs again here: the browser's earlier check is a
/// convenience, not the authority.
pub fn create_project(
    options: &GlobalOptions,
    request: CreateProjectRequest,
) -> Result<CreatedProject> {
    let pack = crate::pack::load_pack(
        &request.pack,
        &crate::pack::PackSearchOptions {
            packs_dir: options.packs_dir.clone(),
        },
    )?;
    let language = pack.manifest.language(&request.language).with_context(|| {
        format!(
            "pack {} does not support language {}",
            request.pack, request.language
        )
    })?;
    for tool in crate::creation::language_tools(language) {
        if tool.required && !tool.found {
            bail!(
                "{} is required to build this project and was not found",
                tool.label
            );
        }
    }
    let stage = match request.stage.as_deref() {
        Some(stage_id) => pack
            .manifest
            .stage(stage_id)
            .with_context(|| format!("pack {} does not contain stage {stage_id}", request.pack))?,
        None => pack
            .manifest
            .first_stage()
            .with_context(|| format!("pack {} does not define any stages", request.pack))?,
    }
    .clone();

    let target =
        crate::creation::resolve_target(request.parent_directory.as_deref(), &request.name)?;
    crate::creation::create(&pack, &request.language, &target, &stage, request.git)?;
    let registered = crate::project_registry::register(&target, options.packs_dir.as_deref())?;

    Ok(CreatedProject {
        project_id: registered.id,
        path: crate::fs_util::display_path(&target),
        pack: request.pack,
        language: request.language,
        stage_id: stage.id.clone(),
        stage_title: stage.title,
    })
}

/// How long a learner action waits for the run lease before giving up.
///
/// The source watcher takes the lease briefly every poll interval to observe
/// project changes, so an action that failed the instant the lease was busy
/// could be refused for a reason that had nothing to do with the learner —
/// pressing *Begin next step* at the wrong moment would report that checks
/// were running when none were. Waiting is bounded so a genuinely active run
/// still produces a prompt, honest refusal.
const LEARNER_ACTION_LEASE_WAIT: std::time::Duration = std::time::Duration::from_millis(750);

pub(crate) fn acquire_for_learner_action(
    root: &Path,
    doing: &'static str,
) -> Result<crate::run_lease::RunLease> {
    crate::run_lease::RunLease::acquire_with_timeout(root, LEARNER_ACTION_LEASE_WAIT)
        .with_context(|| format!("could not {doing} while checks are running"))
}

pub fn load_project_health(options: &GlobalOptions) -> Result<ProjectHealth> {
    let root = crate::context::locate_project_root(options)?;
    // Loading the project is necessary but not sufficient. Health used to stop
    // there, so a project whose digest could not be computed (a directory
    // symlink) or whose stage content could not be rendered (a pack missing a
    // required section) was reported healthy, and the page was then sent to
    // endpoints that could not answer.
    let loaded = ProjectContext::load(options)
        .and_then(|context| workbench_readiness(&context).map(|()| context));
    match loaded {
        Ok(context) => Ok(ProjectHealth {
            status: ProjectHealthStatus::Healthy,
            project: Some(context.state.project),
            issue: None,
            actions: vec![
                health_action(ProjectHealthActionKind::OpenEditor, "Open editor", false),
                health_action(ProjectHealthActionKind::OpenFolder, "Open folder", false),
            ],
        }),
        Err(error) => {
            let detail = sanitize_project_text(&format!("{error:#}"), &root);
            let (code, title, guidance, repinnable) = classify_project_health_error(&detail);
            let mut actions = vec![health_action(
                ProjectHealthActionKind::Recheck,
                "Check again",
                true,
            )];
            if repinnable {
                actions.push(health_action(
                    ProjectHealthActionKind::RepinPack,
                    "Adopt current pack",
                    false,
                ));
            }
            actions.push(health_action(
                ProjectHealthActionKind::OpenEditor,
                "Open editor",
                false,
            ));
            actions.push(health_action(
                ProjectHealthActionKind::OpenFolder,
                "Open folder",
                false,
            ));
            Ok(ProjectHealth {
                status: ProjectHealthStatus::Unhealthy,
                project: None,
                issue: Some(ProjectHealthIssue {
                    code: code.to_string(),
                    title: title.to_string(),
                    detail: bounded_text(&detail, 8 * 1024),
                    guidance: guidance.to_string(),
                }),
                actions,
            })
        }
    }
}

/// The work the workbench's own project endpoints will do, run up front so a
/// failure is reported as a health issue with guidance instead of surfacing as
/// a broken request on a page that has already been told everything is fine.
fn workbench_readiness(context: &ProjectContext) -> Result<()> {
    context.project_digest()?;
    crate::capability::load_current(context)?;
    Ok(())
}

pub fn project_open_target(options: &GlobalOptions) -> Result<PathBuf> {
    crate::context::locate_project_root(options)
}

pub fn repin_current_pack(options: &GlobalOptions) -> Result<ProjectHealth> {
    let health = load_project_health(options)?;
    if health
        .issue
        .as_ref()
        .is_none_or(|issue| issue.code != "pack_changed")
    {
        bail!("the project does not require pack recovery");
    }
    let root = crate::context::locate_project_root(options)?;
    let _lease = acquire_for_learner_action(&root, "recover the pack")?;
    let mut context = ProjectContext::load_unpinned(options)?;
    context.state.pack_version = context.pack.manifest.version.clone();
    context.state.pack_source = pack_source_label(&context.pack.root);
    context.state.pack_digest = context.pack_digest()?;
    context.state.touch()?;
    context.save_state()?;
    crate::run_journal::append(&root, &RunEvent::ProjectStateChanged)?;
    load_project_health(options)
}

pub fn load_workbench_state(options: &GlobalOptions) -> Result<WorkbenchState> {
    let (context, recovered) = load_and_recover(options)?;
    workbench_state(&context, recovered, None)
}

/// Load the project and, if a previous run was abandoned mid-flight, record it
/// as interrupted. Split out from [`load_workbench_state`] so a caller that
/// needs the context and the recovery flag does not have to assemble a whole
/// `WorkbenchState` to get at them.
fn load_and_recover(options: &GlobalOptions) -> Result<(ProjectContext, bool)> {
    let mut context = ProjectContext::load(options)?;
    let mut recovered_interrupted_job = false;
    if context.state.active_job.is_some()
        && let Ok(_recovery_lease) = crate::run_lease::RunLease::acquire(&context.root)
    {
        // The run may have finished between the first state read and lease
        // acquisition. Reload under the lease before deciding it was abandoned.
        context = ProjectContext::load(options)?;
        let interrupted_job_id = context.state.active_job.as_ref().map(|job| job.id.clone());
        if interrupted_job_id.is_some() {
            recovered_interrupted_job = context.state.recover_interrupted_job()?;
        }
        if recovered_interrupted_job {
            context.save_state()?;
            let _ = crate::run_journal::append(
                &context.root,
                &RunEvent::JobInterrupted {
                    job_id: interrupted_job_id.unwrap_or_default(),
                    reason: "DeltaForge stopped before this run finished".to_string(),
                },
            );
            let _ = crate::run_journal::append(&context.root, &RunEvent::ProjectStateChanged);
        }
    }
    Ok((context, recovered_interrupted_job))
}

/// The state the workbench renders, for one browser session.
///
/// One project load and one state assembly. This used to build the entire
/// state twice — once through `load_workbench_state`, whose result was then
/// discarded except for two booleans, and once again below — and load the
/// project three times, each load re-verifying the pack pin by walking every
/// file in the pack. The event stream calls this every 500 ms for every open
/// tab, so that doubling was the bulk of what an idle workbench cost.
pub fn load_workbench_state_for_session(
    options: &GlobalOptions,
    session_id: &str,
) -> Result<WorkbenchState> {
    let (context, recovered) = load_and_recover(options)?;
    if context.state.active_job.is_some() {
        return workbench_state(&context, recovered, None);
    }
    if context
        .state
        .last_workbench_session
        .as_ref()
        .is_some_and(|session| session.id == session_id)
    {
        return workbench_state(&context, recovered, Some(session_id));
    }
    // Below here the session is new, which happens once per tab rather than
    // once per tick, so the extra load under the lease is not on the hot path.
    let _ = observe_source_changes(options)?;

    let Ok(_session_lease) = crate::run_lease::RunLease::acquire(&context.root) else {
        return workbench_state(&context, recovered, None);
    };
    let mut context = ProjectContext::load(options)?;
    let changed = context
        .state
        .begin_workbench_session(session_id.to_string(), recovered)?;
    if changed {
        context.save_state()?;
    }
    workbench_state(&context, recovered, Some(session_id))
}

pub fn run_is_active(options: &GlobalOptions) -> Result<bool> {
    let context = ProjectContext::load(options)?;
    Ok(context.state.active_job.is_some() && crate::run_lease::active(&context.root))
}

pub fn cancel_active_run(options: &GlobalOptions) -> Result<String> {
    let _ = load_workbench_state(options)?;
    let context = ProjectContext::load(options)?;
    let active = context
        .state
        .active_job
        .as_ref()
        .context("there is no active run to cancel")?;
    if !crate::run_lease::active(&context.root) {
        bail!("the active run has already stopped");
    }
    let path = cancellation_path(&context.root, &active.id)?;
    crate::fs_util::atomic_write(&path, b"cancel")?;
    Ok(active.id.clone())
}

pub fn publish_event(options: &GlobalOptions, event: &RunEvent) -> Result<u64> {
    let context = ProjectContext::load(options)?;
    crate::run_journal::append(&context.root, event)
}

pub fn observe_source_changes(options: &GlobalOptions) -> Result<Option<SourceChangeRecord>> {
    let initial = ProjectContext::load(options)?;
    let current_digest = initial.project_digest()?;
    let needs_baseline = initial.state.observed_project_digest.is_empty();
    let needs_change = !needs_baseline && initial.state.observed_project_digest != current_digest;
    let has_pending_event = initial.state.source_event_revision < initial.state.source_revision;
    if !needs_baseline && !needs_change && !has_pending_event {
        return Ok(None);
    }
    if initial.state.active_job.is_some() && crate::run_lease::active(&initial.root) {
        return Ok(None);
    }
    let _lease = match crate::run_lease::RunLease::acquire(&initial.root) {
        Ok(lease) => lease,
        Err(_) if crate::run_lease::active(&initial.root) => return Ok(None),
        Err(error) => return Err(error),
    };
    let mut context = ProjectContext::load(options)?;
    if context.state.active_job.is_some() {
        return Ok(None);
    }
    let current_digest = context.project_digest()?;
    let change = observe_source_in_context(&mut context, current_digest)?;
    let pending = flush_pending_source_change(&mut context, true)?;
    Ok(change.or(pending))
}

pub fn load_capability_content(
    options: &GlobalOptions,
) -> Result<crate::capability::CapabilityContent> {
    let context = ProjectContext::load(options)?;
    crate::capability::load_current(&context)
}

pub fn reveal_next_hint(options: &GlobalOptions) -> Result<crate::capability::CapabilityContent> {
    let mut context = ProjectContext::load(options)?;
    let _lease = acquire_for_learner_action(&context.root, "update help")?;
    // Serialize every project-state mutation with test runs, then reload so a
    // queued help request cannot overwrite state saved by the run it followed.
    context = ProjectContext::load(options)?;
    let stage_id = context.state.current_stage.clone();
    let help = crate::capability::load_help(&context)?;
    let current = context
        .state
        .hint_state
        .get(&stage_id)
        .copied()
        .unwrap_or_default();
    // The final authored help level is always the retrospective, gated until
    // the capability is acquired; every level before it is available freely.
    let maximum = crate::capability::available_help_levels(
        help.len(),
        context.state.is_completed(&stage_id),
    );
    if maximum == 0 {
        bail!("this capability has no help levels");
    }
    if current >= maximum {
        if context.state.is_completed(&stage_id) || help.len() <= maximum {
            bail!("all help levels are already revealed");
        }
        bail!("the retrospective unlocks after this capability is acquired");
    }
    context.state.hint_state.insert(stage_id, current + 1);
    context.state.touch()?;
    context.save_state()?;
    let _ = crate::run_journal::append(&context.root, &RunEvent::ProjectStateChanged);
    crate::capability::load_current(&context)
}

pub fn begin_next_capability(options: &GlobalOptions) -> Result<WorkbenchState> {
    let context = ProjectContext::load(options)?;
    let _lease = acquire_for_learner_action(&context.root, "advance")?;
    let mut context = ProjectContext::load(options)?;
    let current_stage = context.state.current_stage.clone();
    if !context.state.is_completed(&current_stage) {
        bail!("the current capability has not been acquired yet");
    }
    context.verify_completion_proof(&current_stage)?;
    if !context.stage_gates(&current_stage)?.is_empty() && context.config.gates.enforce {
        context.verify_gate_record(&current_stage)?;
    }
    let next = context
        .pack
        .manifest
        .next_stage(&current_stage)
        .cloned()
        .context("the project has no later capability")?;
    context.state.current_stage = next.id;
    context.state.touch()?;
    context.save_state()?;
    let _ = crate::run_journal::append(&context.root, &RunEvent::ProjectStateChanged);
    workbench_state(&context, false, None)
}

pub fn run_tests(
    options: &GlobalOptions,
    request: TestRunRequest,
    sink: &mut dyn EventSink,
) -> Result<TestRunOutcome> {
    let mut context = ProjectContext::load(options)?;
    // A background source-health observation holds this lease only briefly.
    // Give foreground checks a bounded chance to follow it instead of exposing
    // a spurious "another run" error to the CLI or workbench.
    let _lease = crate::run_lease::RunLease::acquire_with_timeout(
        &context.root,
        std::time::Duration::from_millis(500),
    )?;
    // A previous run may have completed after the first load but before this
    // lease was acquired. Never mutate from that stale in-memory snapshot.
    context = ProjectContext::load(options)?;
    if context.state.active_job.is_some() {
        context.state.recover_interrupted_job()?;
        context.save_state()?;
    }
    let run_project_digest = context.project_digest()?;
    let observed_change = observe_source_in_context(&mut context, run_project_digest.clone())?;
    let pending_change = flush_pending_source_change(&mut context, true)?;
    if let Some(change) = observed_change.or(pending_change) {
        sink.emit(RunEvent::SourceChanged {
            revision: change.revision,
            previous_digest: change.previous_digest,
            current_digest: change.current_digest,
        });
    }
    let stages = if request.all {
        context.pack.manifest.stages.clone()
    } else {
        let stage_id = request
            .stage
            .as_deref()
            .unwrap_or(&context.state.current_stage);
        vec![
            context
                .pack
                .manifest
                .stage(stage_id)
                .with_context(|| format!("pack does not contain stage {stage_id}"))?
                .clone(),
        ]
    };
    let stage_ids = stages
        .iter()
        .map(|stage| stage.id.clone())
        .collect::<Vec<_>>();
    let job_id = context.state.start_job(JobKind::Tests, stage_ids.clone())?;
    context.save_state()?;
    let project_root = context.root.clone();
    let mut sink = JournalSink {
        project_root: &project_root,
        downstream: sink,
    };
    sink.emit(RunEvent::JobStarted {
        job_id: job_id.clone(),
        kind: JobKind::Tests,
        stage_ids,
        trigger: request.trigger,
    });

    let cancellation_path = cancellation_path(&context.root, &job_id)?;
    let _ = fs::remove_file(&cancellation_path);
    let runner_options = RunnerOptions {
        filter: request.filter,
        list_tests: request.list_tests,
        fail_fast: request.fail_fast,
        no_build: request.no_build,
        keep_temp: request.keep_temp,
        // Durable workbench diagnosis needs the sanitized command and fixture
        // whatever surface started the run, so this is not a caller's choice:
        // callers used to pass a flag that this line then overrode anyway.
        // Listing tests runs nothing, so there is nothing to capture.
        capture_details: !request.list_tests,
        cancellation_path: Some(cancellation_path.clone()),
    };
    let mut summaries = Vec::new();
    let mut newly_completed_current = false;
    let mut execution_error = None;

    for stage in &stages {
        match runner::run_stage_tests(&context, stage, &runner_options, &mut sink) {
            Ok(summary) => {
                if !runner_options.list_tests && summary.completion_eligible {
                    let was_completed = context.state.is_completed(&stage.id);
                    context.state.record_completion_proof(
                        &stage.id,
                        context.pack_digest()?,
                        context.stage_behavioral_digest(&stage.id)?,
                        run_project_digest.clone(),
                        summary.total_defined,
                    )?;
                    newly_completed_current |=
                        !was_completed && stage.id == context.state.current_stage;
                }
                // A focused rerun is useful evidence for one contradiction, but it
                // is not a replacement for the latest complete stage result.
                if !runner_options.list_tests && runner_options.filter.is_none() {
                    let mut failed_tests = summary
                        .results
                        .iter()
                        .filter(|result| !result.passed)
                        .map(record_failed_test)
                        .collect::<Vec<_>>();
                    failed_tests.sort_by_key(|failure| {
                        failure
                            .diagnosis
                            .as_ref()
                            .map_or(1_000, |diagnosis| diagnosis.priority)
                    });
                    context.state.record_test_run(
                        stage.id.clone(),
                        summary.passed,
                        summary.failed,
                        failed_tests,
                        run_project_digest.clone(),
                    )?;
                }
                summaries.push(summary);
            }
            Err(error) => {
                execution_error = Some(bounded_text(
                    &sanitize_project_text(&format!("{error:#}"), &context.root),
                    16 * 1024,
                ));
                break;
            }
        }
    }
    let passed = summaries.iter().map(|summary| summary.passed).sum();
    let failed = summaries.iter().map(|summary| summary.failed).sum();
    let ending_source_change = match context.project_digest() {
        Ok(digest) => match observe_source_in_context(&mut context, digest) {
            Ok(change) => change,
            Err(error) => {
                execution_error.get_or_insert_with(|| {
                    bounded_text(
                        &sanitize_project_text(&format!("{error:#}"), &context.root),
                        16 * 1024,
                    )
                });
                None
            }
        },
        Err(error) => {
            execution_error.get_or_insert_with(|| {
                bounded_text(
                    &sanitize_project_text(&format!("{error:#}"), &context.root),
                    16 * 1024,
                )
            });
            None
        }
    };
    let successful = execution_error.is_none()
        && !summaries.is_empty()
        && summaries.iter().all(TestRunSummary::is_success);
    let cancelled = execution_error
        .as_deref()
        .is_some_and(|error| error.contains("run cancelled"));
    if !runner_options.list_tests {
        context.state.finish_job(
            &job_id,
            if cancelled {
                AttemptStatus::Cancelled
            } else if successful {
                AttemptStatus::Passed
            } else {
                AttemptStatus::Failed
            },
            passed,
            failed,
            execution_error.clone(),
        )?;
        context.save_state()?;
    } else {
        context.state.clear_active_job(&job_id)?;
        context.save_state()?;
    }

    if let Some(change) = ending_source_change {
        sink.emit(RunEvent::SourceChanged {
            revision: change.revision,
            previous_digest: change.previous_digest,
            current_digest: change.current_digest,
        });
        let _ = flush_pending_source_change(&mut context, false)?;
    }

    if cancelled {
        sink.emit(RunEvent::JobInterrupted {
            job_id: job_id.clone(),
            reason: "Run cancelled by the learner".to_string(),
        });
    } else {
        sink.emit(RunEvent::RunCompleted {
            job_id: job_id.clone(),
            passed: successful,
            passed_tests: passed,
            failed_tests: failed,
        });
    }
    sink.emit(RunEvent::ProjectStateChanged);
    let _ = fs::remove_file(cancellation_path);

    Ok(TestRunOutcome {
        job_id,
        summaries,
        newly_completed_current,
        execution_error,
    })
}

/// Run the current or selected stage's benchmarks under the same run lease,
/// journal, cancellation path, and event stream as `run_tests`. A benchmark
/// started from the CLI and one started from the browser are indistinguishable
/// to project state and to the event stream; only `trigger` differs.
/// Export the engineering record into the project's own directory and return
/// both the path and the rendered document, so the browser can show it
/// immediately and the learner still has a file.
pub fn export_report(
    options: &GlobalOptions,
    format: crate::reporting::ReportFormat,
) -> Result<ExportedReport> {
    let context = ProjectContext::load(options)?;
    let report = crate::reporting::build(&context)?;
    let content = crate::reporting::render(&report, format)?;
    let path = context.root.join(format.export_file_name());
    crate::fs_util::atomic_write(&path, &content)?;
    Ok(ExportedReport {
        path: crate::fs_util::display_path(&path),
        markdown: crate::reporting::render_markdown(&report),
        content,
        report,
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct ExportedReport {
    pub path: String,
    /// The record as Markdown, whatever `format` was requested, so the browser
    /// always has something it can render.
    pub markdown: String,
    /// The document actually written to `path`.
    pub content: String,
    pub report: crate::reporting::ProjectReport,
}

/// What a stage snapshot would record right now: the message, the tag, and
/// the files it would include. Computed without touching the repository, so
/// the pass moment can show the change before offering to commit it.
pub fn preview_stage_snapshot(options: &GlobalOptions) -> Result<crate::snapshot::SnapshotPreview> {
    let context = ProjectContext::load(options)?;
    let stage = context
        .pack
        .manifest
        .stage(&context.state.current_stage)
        .with_context(|| {
            format!(
                "pack does not contain current stage {}",
                context.state.current_stage
            )
        })?;
    let position = context.pack.manifest.stage_index(&stage.id).unwrap_or(0) + 1;
    let message = crate::snapshot::snapshot_message(position, &stage.title);
    let tag = context
        .config
        .git
        .auto_tag
        .then(|| crate::snapshot::snapshot_tag(&stage.id));

    if !crate::snapshot::is_git_repository(&context.root) {
        return Ok(crate::snapshot::SnapshotPreview {
            available: false,
            blocked_reason: Some(
                "This project is not a Git repository, so there is nothing to snapshot into."
                    .to_string(),
            ),
            message,
            tag,
            changed_files: Vec::new(),
            tag_exists: false,
        });
    }
    let changed_files = crate::snapshot::changed_files(&context.root)?;
    let blocked_reason = if !context.state.is_completed(&stage.id) {
        Some("This step has not passed its checks yet.".to_string())
    } else if changed_files.is_empty() {
        Some("Every change is already recorded; there is nothing new to snapshot.".to_string())
    } else {
        None
    };
    let tag_exists = tag
        .as_deref()
        .is_some_and(|tag| crate::snapshot::tag_exists(&context.root, tag));
    Ok(crate::snapshot::SnapshotPreview {
        available: blocked_reason.is_none(),
        blocked_reason,
        message,
        tag,
        changed_files,
        tag_exists,
    })
}

/// Record a stage snapshot. Never automatic: a caller reaches this only
/// because the learner asked, from either surface.
pub fn create_stage_snapshot(
    options: &GlobalOptions,
    force: bool,
) -> Result<crate::snapshot::SnapshotOutcome> {
    let context = ProjectContext::load(options)?;
    crate::snapshot::ensure_git_repository(&context.root)?;
    let _lease = acquire_for_learner_action(&context.root, "snapshot")?;
    let context = ProjectContext::load(options)?;
    let stage = context
        .pack
        .manifest
        .stage(&context.state.current_stage)
        .with_context(|| {
            format!(
                "pack does not contain current stage {}",
                context.state.current_stage
            )
        })?;
    if !force {
        if !context.state.is_completed(&stage.id) {
            bail!(
                "refusing to snapshot because step {} has not passed its checks yet",
                stage.id
            );
        }
        context.verify_completion_proof(&stage.id)?;
    }
    let position = context.pack.manifest.stage_index(&stage.id).unwrap_or(0) + 1;
    let message = crate::snapshot::snapshot_message(position, &stage.title);
    let tag = context
        .config
        .git
        .auto_tag
        .then(|| crate::snapshot::snapshot_tag(&stage.id));
    let outcome = crate::snapshot::take(&context.root, &message, tag.as_deref())?;
    let _ = crate::run_journal::append(&context.root, &RunEvent::ProjectStateChanged);
    Ok(outcome)
}

pub fn run_benchmarks(
    options: &GlobalOptions,
    request: BenchmarkRunRequest,
    sink: &mut dyn EventSink,
) -> Result<BenchmarkRunOutcome> {
    let mut context = ProjectContext::load(options)?;
    let _lease = crate::run_lease::RunLease::acquire_with_timeout(
        &context.root,
        std::time::Duration::from_millis(500),
    )?;
    context = ProjectContext::load(options)?;
    if context.state.active_job.is_some() {
        context.state.recover_interrupted_job()?;
        context.save_state()?;
    }

    let stages = if request.all {
        context.pack.manifest.stages.clone()
    } else {
        let stage_id = request
            .stage
            .as_deref()
            .unwrap_or(&context.state.current_stage);
        vec![
            context
                .pack
                .manifest
                .stage(stage_id)
                .with_context(|| format!("pack does not contain stage {stage_id}"))?
                .clone(),
        ]
    };
    let stage_ids = stages
        .iter()
        .map(|stage| stage.id.clone())
        .collect::<Vec<_>>();
    let job_id = context
        .state
        .start_job(JobKind::Benchmarks, stage_ids.clone())?;
    context.save_state()?;
    let project_root = context.root.clone();
    let mut sink = JournalSink {
        project_root: &project_root,
        downstream: sink,
    };
    sink.emit(RunEvent::JobStarted {
        job_id: job_id.clone(),
        kind: JobKind::Benchmarks,
        stage_ids,
        trigger: request.trigger,
    });

    let cancellation_path = cancellation_path(&context.root, &job_id)?;
    let _ = fs::remove_file(&cancellation_path);
    let benchmark_options = crate::benchmarks::BenchmarkOptions {
        iterations: request.iterations,
        warmup: request.warmup,
        cancellation_path: Some(cancellation_path.clone()),
    };

    let mut records = Vec::new();
    let mut execution_error = None;
    for stage in &stages {
        match crate::benchmarks::run_stage_benchmarks(
            &context,
            stage,
            &benchmark_options,
            &mut sink,
        ) {
            Ok(stage_records) => records.extend(stage_records),
            Err(error) => {
                execution_error = Some(bounded_text(
                    &sanitize_project_text(&format!("{error:#}"), &context.root),
                    16 * 1024,
                ));
                break;
            }
        }
    }

    let evaluations = stages
        .iter()
        .map(|stage| crate::benchmarks::evaluate_stage_gates(&context, stage, &records))
        .collect::<Result<Vec<_>>>()?;

    // Read history before appending so a saved run is compared with a genuinely
    // prior result, never with itself.
    let comparisons = if request.compare {
        let history = crate::benchmarks::read_history(&crate::benchmarks::history_path(&context))?;
        Some(crate::benchmarks::compare_records(&records, &history))
    } else {
        None
    };

    let saved = request.save && !records.is_empty() && execution_error.is_none();
    if saved {
        crate::benchmarks::append_history(&context, &records)?;
    }

    // Gate records are written after history so a history write failure can
    // never forge a progression proof without its measurement.
    let mut changed_gate_state = false;
    for evaluation in &evaluations {
        if let Some(record) = evaluation.record(&context)? {
            context
                .state
                .gate_results
                .insert(evaluation.stage.clone(), record);
            changed_gate_state = true;
        }
    }
    if changed_gate_state {
        context.state.touch()?;
        context.save_state()?;
    }

    let gates = evaluations
        .iter()
        .filter(|evaluation| !evaluation.gates.is_empty())
        .map(|evaluation| StageGateReport {
            stage_id: evaluation.stage.clone(),
            status: evaluation.status(),
            gates: evaluation.gates.clone(),
        })
        .collect::<Vec<_>>();
    for report in &gates {
        sink.emit(RunEvent::GateEvaluated {
            stage_id: report.stage_id.clone(),
            status: report.status,
            passed: report.gates.iter().filter(|gate| gate.passed).count(),
            total: report.gates.len(),
        });
    }

    let outcome = BenchmarkRunOutcome {
        job_id: job_id.clone(),
        records,
        comparisons,
        gates,
        saved,
        execution_error,
    };
    let cancelled = outcome
        .execution_error
        .as_deref()
        .is_some_and(|error| error.contains("run cancelled"));
    let successful = outcome.is_success();
    context.state.finish_job(
        &job_id,
        if cancelled {
            AttemptStatus::Cancelled
        } else if successful {
            AttemptStatus::Passed
        } else {
            AttemptStatus::Failed
        },
        outcome.records.len() - outcome.failed_points().min(outcome.records.len()),
        outcome.failed_points(),
        outcome.execution_error.clone(),
    )?;
    context.save_state()?;

    if cancelled {
        sink.emit(RunEvent::JobInterrupted {
            job_id: job_id.clone(),
            reason: "Run cancelled by the learner".to_string(),
        });
    } else {
        sink.emit(RunEvent::BenchmarkRunCompleted {
            job_id: job_id.clone(),
            passed: successful,
            benchmarks: outcome.records.len(),
            failed_points: outcome.failed_points(),
        });
    }
    sink.emit(RunEvent::ProjectStateChanged);
    let _ = fs::remove_file(cancellation_path);

    Ok(outcome)
}

/// Record the learner's prediction for the current step. `skipped` marks that
/// the prompt was offered and declined, which is not the same as never asked.
pub fn record_prediction(
    options: &GlobalOptions,
    text: String,
    skipped: bool,
) -> Result<WorkbenchState> {
    record_learner_note(options, text, skipped, NoteKind::Prediction)
}

/// Record the learner's reflection for the current step.
pub fn record_reflection(
    options: &GlobalOptions,
    text: String,
    skipped: bool,
) -> Result<WorkbenchState> {
    record_learner_note(options, text, skipped, NoteKind::Reflection)
}

#[derive(Debug, Clone, Copy)]
enum NoteKind {
    Prediction,
    Reflection,
}

fn record_learner_note(
    options: &GlobalOptions,
    text: String,
    skipped: bool,
    kind: NoteKind,
) -> Result<WorkbenchState> {
    if text.len() > 4 * 1024 {
        bail!("the note is longer than DeltaForge stores for one step");
    }
    let text = text.trim().to_string();
    if text.is_empty() && !skipped {
        bail!("write a prediction or skip it");
    }
    let context = ProjectContext::load(options)?;
    let _lease = acquire_for_learner_action(&context.root, "save the note")?;
    let mut context = ProjectContext::load(options)?;
    let stage_id = context.state.current_stage.clone();
    match kind {
        NoteKind::Prediction => context.state.record_prediction(&stage_id, text, skipped)?,
        NoteKind::Reflection => context.state.record_reflection(&stage_id, text, skipped)?,
    }
    context.save_state()?;
    let _ = crate::run_journal::append(&context.root, &RunEvent::ProjectStateChanged);
    workbench_state(&context, false, None)
}

fn observe_source_in_context(
    context: &mut ProjectContext,
    current_digest: String,
) -> Result<Option<SourceChangeRecord>> {
    let initialized = if context.state.observed_project_digest.is_empty() {
        let baseline = context
            .state
            .last_test_runs
            .get(&context.state.current_stage)
            .map(|run| run.project_digest.as_str())
            .filter(|digest| !digest.is_empty())
            .unwrap_or(&current_digest)
            .to_string();
        context.state.initialize_source_observation(baseline)
    } else {
        false
    };
    let change = context.state.observe_source_digest(current_digest)?;
    if initialized || change.is_some() {
        context.save_state()?;
    }
    Ok(change)
}

fn flush_pending_source_change(
    context: &mut ProjectContext,
    append_project_state: bool,
) -> Result<Option<SourceChangeRecord>> {
    if context.state.source_event_revision >= context.state.source_revision {
        return Ok(None);
    }
    let change = context
        .state
        .last_source_change
        .clone()
        .context("source revision is missing its persisted transition")?;
    if !crate::run_journal::contains_source_revision(&context.root, change.revision)? {
        crate::run_journal::append(
            &context.root,
            &RunEvent::SourceChanged {
                revision: change.revision,
                previous_digest: change.previous_digest.clone(),
                current_digest: change.current_digest.clone(),
            },
        )?;
    }
    context.state.acknowledge_source_event(change.revision);
    context.save_state()?;
    if append_project_state {
        crate::run_journal::append(&context.root, &RunEvent::ProjectStateChanged)?;
    }
    Ok(Some(change))
}

fn workbench_state(
    context: &ProjectContext,
    recovered: bool,
    session_id: Option<&str>,
) -> Result<WorkbenchState> {
    // Capture the cursor before computing state. Events appended while state is
    // assembled will then be replayed by the browser instead of falling into
    // the fetch-to-stream handoff gap.
    let event_cursor = crate::run_journal::cursor(&context.root)?;
    let current = context
        .pack
        .manifest
        .stage(&context.state.current_stage)
        .with_context(|| {
            format!(
                "pack does not contain stage {}",
                context.state.current_stage
            )
        })?;
    let current_index = context
        .pack
        .manifest
        .stages
        .iter()
        .position(|stage| stage.id == current.id)
        .unwrap_or_default();
    // One walk of the learner's tree for the whole state assembly. The
    // performance view below asks about every stage, and each question used to
    // recompute this.
    let project_digest = context.project_digest()?;
    let freshness = match context.state.last_test_runs.get(&current.id) {
        None => ResultFreshness::NeverRun,
        Some(run) if run.project_digest == project_digest => ResultFreshness::Fresh,
        Some(_) => ResultFreshness::Stale,
    };
    let latest_run = context.state.last_test_runs.get(&current.id).cloned();
    let mut primary_failure = latest_run
        .as_ref()
        .and_then(|run| run.failed_tests.first().cloned());
    if let Some(attempt) = context.state.attempt_history.last()
        && attempt.status == AttemptStatus::Failed
        && attempt.stage_ids.iter().any(|stage| stage == &current.id)
        && let Some(error) = &attempt.error
    {
        // A build failure is the common way a run stops before it reports,
        // but it is not the only one: an unsafe pack path, an unreadable
        // tests file, or a language the pack does not support all land here
        // too, and calling every one of them a build failure sends the
        // learner looking in the wrong place.
        let build_failed = error.contains("build failed");
        primary_failure = Some(LastFailedTest {
            name: if build_failed { "Build project" } else { "Run checks" }.to_string(),
            failures: vec![error.clone()],
            diagnosis: Some(FailureDiagnosis {
                priority: 0,
                kind: if build_failed { "build" } else { "runner" }.to_string(),
                headline: if build_failed {
                    "The project did not build".to_string()
                } else {
                    "The checks could not run".to_string()
                },
                summary: if build_failed {
                    "Checks could not start because the configured build command failed."
                        .to_string()
                } else {
                    "The check run stopped before any behavior could be checked.".to_string()
                },
                expected: Some(
                    if build_failed {
                        "A successful project build"
                    } else {
                        "A check run that reaches its first test"
                    }
                    .to_string(),
                ),
                actual: Some(error.clone()),
                contract: if build_failed {
                    "The project must build before behavioral checks can run.".to_string()
                } else {
                    "The check run must start before any behavior can be checked.".to_string()
                },
                fixture: None,
                fixture_entries: Vec::new(),
                command: Vec::new(),
            }),
        });
    }
    let resumption = session_id
        .zip(context.state.last_workbench_session.as_ref())
        .filter(|(session_id, session)| session.id == **session_id)
        .and_then(|(_, session)| {
            let stage_change = session.previous_stage_id.as_ref().and_then(|previous_id| {
                (previous_id != &current.id).then(|| {
                    let previous_title = context
                        .pack
                        .manifest
                        .stage(previous_id)
                        .map_or_else(|| previous_id.clone(), |stage| stage.title.clone());
                    StageChangeSummary {
                        from_id: previous_id.clone(),
                        from_title: previous_title,
                        to_id: current.id.clone(),
                        to_title: current.title.clone(),
                    }
                })
            });
            if session.previous_session_started_at.is_none()
                && !session.recovered_interrupted_job
            {
                return None;
            }
            let (kind, title, detail) = if session.recovered_interrupted_job {
                (
                    ResumptionKind::Interrupted,
                    "Previous run interrupted".to_string(),
                    "DeltaForge preserved the last completed test result. Run checks again when you are ready."
                        .to_string(),
                )
            } else if let Some(change) = &stage_change {
                (
                    ResumptionKind::CapabilityChanged,
                    "Current step changed".to_string(),
                    format!(
                        "Your previous session was on {}. Continue with {}.",
                        change.from_title, change.to_title
                    ),
                )
            } else if freshness == ResultFreshness::Stale {
                (
                    ResumptionKind::SourceChanged,
                    "Source changed since the last result".to_string(),
                    "Your previous test result is preserved, but the current source needs a new check run."
                        .to_string(),
                )
            } else if context.state.is_completed(&current.id)
                && freshness == ResultFreshness::Fresh
            {
                (
                    ResumptionKind::CapabilityAcquired,
                    "Completed step is still current".to_string(),
                    "The completed step and its passing test result were restored without rerunning checks."
                        .to_string(),
                )
            } else if primary_failure.is_some() && freshness == ResultFreshness::Fresh {
                (
                    ResumptionKind::ChecksFailed,
                    "Your last failing check is ready".to_string(),
                    "The latest run and first failing check were restored without rerunning checks."
                        .to_string(),
                )
            } else {
                (
                    ResumptionKind::Ready,
                    "Ready to continue".to_string(),
                    "Your project and current step were restored without rerunning checks."
                        .to_string(),
                )
            };
            Some(ResumptionSummary {
                kind,
                title,
                detail,
                previous_session_started_at: session.previous_session_started_at.clone(),
                stage_change,
                action_pending: context.state.updated_at == session.baseline_updated_at,
            })
        });
    let active_job = context.state.active_job.clone();
    let mut primary_action = if active_job.is_some() {
        PrimaryAction {
            kind: PrimaryActionKind::CancelRun,
            label: "Cancel run".to_string(),
            enabled: true,
        }
    } else if context.state.is_completed(&current.id) && freshness == ResultFreshness::Fresh {
        if context
            .pack
            .manifest
            .stages
            .get(current_index + 1)
            .is_some()
        {
            PrimaryAction {
                kind: PrimaryActionKind::BeginNextCapability,
                label: "Begin next step".to_string(),
                enabled: true,
            }
        } else {
            PrimaryAction {
                kind: PrimaryActionKind::JourneyComplete,
                label: "Project complete".to_string(),
                enabled: false,
            }
        }
    } else {
        PrimaryAction {
            kind: PrimaryActionKind::RunChecks,
            label: if freshness == ResultFreshness::NeverRun {
                "Run initial checks".to_string()
            } else {
                "Run checks again".to_string()
            },
            enabled: true,
        }
    };
    if resumption
        .as_ref()
        .is_some_and(|summary| summary.action_pending)
        && matches!(primary_action.kind, PrimaryActionKind::RunChecks)
    {
        primary_action = PrimaryAction {
            kind: PrimaryActionKind::ResumeChecks,
            label: if resumption
                .as_ref()
                .is_some_and(|summary| summary.kind == ResumptionKind::Interrupted)
            {
                "Run checks again".to_string()
            } else {
                "Resume with checks".to_string()
            },
            enabled: true,
        };
    }
    Ok(WorkbenchState {
        project: context.state.project.clone(),
        language: context.state.language.clone(),
        capability: CapabilityState {
            id: current.id.clone(),
            title: current.title.clone(),
            completed: context.state.is_completed(&current.id),
            next_id: context
                .pack
                .manifest
                .stages
                .get(current_index + 1)
                .map(|stage| stage.id.clone()),
        },
        primary_action,
        freshness,
        revealed_hint_level: context
            .state
            .hint_state
            .get(&current.id)
            .copied()
            .unwrap_or_default(),
        last_activity_at: context.state.updated_at.clone(),
        recovered_interrupted_job: resumption.as_ref().is_some_and(|summary| {
            summary.kind == ResumptionKind::Interrupted && summary.action_pending
        }) || recovered,
        resumption,
        active_job,
        latest_attempt: context.state.attempt_history.last().cloned(),
        attempt_history: context.state.attempt_history.clone(),
        latest_run,
        primary_failure,
        source_revision: context.state.source_revision,
        last_source_change: context.state.last_source_change.clone(),
        event_cursor,
        performance: performance_state(context, &current.id, &project_digest)?,
    })
}

/// Assemble the performance picture for one step: its gates, its most recent
/// saved measurements, its prediction prompt and recorded notes, and a
/// gate marker for every step of the journey.
fn performance_state(
    context: &ProjectContext,
    stage_id: &str,
    project_digest: &str,
) -> Result<PerformanceState> {
    let stage = context
        .pack
        .manifest
        .stage(stage_id)
        .with_context(|| format!("pack does not contain stage {stage_id}"))?;
    let benchmarks = context.pack.benchmarks_path(stage).is_file();
    let declared = context.stage_gates(stage_id)?;
    let status = context.gate_status_with_digest(stage_id, project_digest)?;
    let record = context.state.gate_results.get(stage_id);
    let gates = declared
        .iter()
        .map(|gate| {
            let bound = gate.bound();
            let recorded = record.and_then(|record| {
                record.results.iter().find(|result| {
                    result.benchmark == gate.benchmark
                        && result.metric == gate.metric
                        && result.params == gate.params
                })
            });
            let measured = recorded.map(|result| result.measured);
            GateView {
                name: gate.name.clone(),
                benchmark: gate.benchmark.clone(),
                metric: crate::benchmarks::metric_name(gate.metric),
                comparison: match bound {
                    Some(crate::pack::GateBound::Min(_)) => "at least",
                    _ => "at most",
                },
                bound: match bound {
                    Some(
                        crate::pack::GateBound::Min(value) | crate::pack::GateBound::Max(value),
                    ) => value,
                    None => f64::NAN,
                },
                params_label: gate
                    .params
                    .iter()
                    .map(|(name, value)| format!("{name}={value}"))
                    .collect::<Vec<_>>()
                    .join(", "),
                measured,
                // Only a gate whose measurement is current counts as passing;
                // `gate_status` already applied the digest checks.
                passed: status == Some(crate::context::GateStatus::Passed)
                    || recorded.is_some_and(|result| {
                        result.passed && status != Some(crate::context::GateStatus::NotMeasured)
                    }),
                advice: gate.advice.clone(),
            }
        })
        .collect();

    let history = crate::benchmarks::read_history(&crate::benchmarks::history_path(context))
        .unwrap_or_default();
    let latest = latest_benchmark_views(&history, stage_id);

    let mut roadmap = Vec::new();
    for stage in &context.pack.manifest.stages {
        let has_benchmarks = context.pack.benchmarks_path(stage).is_file();
        let status = context
            .gate_status_with_digest(&stage.id, project_digest)
            .unwrap_or(None);
        if has_benchmarks || status.is_some() {
            roadmap.push(StageGateMarker {
                stage_id: stage.id.clone(),
                has_benchmarks,
                status,
            });
        }
    }

    Ok(PerformanceState {
        has_benchmarks: benchmarks,
        gate_status: status,
        gate_blocks_progress: context.config.gates.enforce
            && matches!(
                status,
                Some(crate::context::GateStatus::NotYet | crate::context::GateStatus::NotMeasured)
            ),
        gates,
        latest,
        prediction_prompt: crate::capability::read_prediction_prompt(context, stage),
        prediction: context.state.predictions.get(stage_id).cloned(),
        reflection: context.state.reflections.get(stage_id).cloned(),
        roadmap,
    })
}

/// The most recent saved run of each benchmark for one stage, with each point
/// compared against the run before it on the same machine.
fn latest_benchmark_views(
    history: &[crate::benchmarks::BenchmarkRecord],
    stage_id: &str,
) -> Vec<BenchmarkView> {
    let stage_records: Vec<&crate::benchmarks::BenchmarkRecord> = history
        .iter()
        .filter(|record| record.stage == stage_id)
        .collect();
    let mut names: Vec<&str> = stage_records
        .iter()
        .map(|record| record.benchmark.as_str())
        .collect();
    names.sort_unstable();
    names.dedup();

    names
        .into_iter()
        .filter_map(|name| {
            let mut runs = stage_records
                .iter()
                .filter(|record| record.benchmark == name)
                .rev();
            let current = runs.next()?;
            let previous = runs.next();
            Some(BenchmarkView {
                name: name.to_string(),
                timestamp: current.timestamp.clone(),
                points: current
                    .points
                    .iter()
                    .map(|point| {
                        let prior = previous.and_then(|previous| {
                            previous
                                .points
                                .iter()
                                .find(|candidate| candidate.params == point.params)
                        });
                        BenchmarkPointView {
                            params_label: point.params_label(),
                            success: point.success,
                            runtime_median_ms: point.runtime_median_ms,
                            runtime_p95_ms: point.runtime_p95_ms,
                            throughput_mb_s: point.throughput_mb_s,
                            peak_memory_mb: point.peak_memory_mb,
                            median_percent_delta: percent_delta(
                                prior.and_then(|prior| prior.runtime_median_ms),
                                point.runtime_median_ms,
                            ),
                            error: point.error.clone(),
                        }
                    })
                    .collect(),
            })
        })
        .collect()
}

fn percent_delta(previous: Option<f64>, current: Option<f64>) -> Option<f64> {
    let (previous, current) = (previous?, current?);
    (previous.is_finite() && current.is_finite() && previous != 0.0)
        .then(|| (current - previous) / previous * 100.0)
}

struct JournalSink<'a> {
    project_root: &'a Path,
    downstream: &'a mut dyn EventSink,
}

impl EventSink for JournalSink<'_> {
    fn emit(&mut self, event: RunEvent) {
        let _ = crate::run_journal::append(self.project_root, &event);
        self.downstream.emit(event);
    }
}

fn cancellation_path(project_root: &Path, job_id: &str) -> Result<PathBuf> {
    if job_id.is_empty()
        || !job_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    {
        bail!("active job identifier is invalid");
    }
    Ok(project_root
        .join(".deltaforge")
        .join(format!("cancel-{job_id}")))
}

fn record_failed_test(result: &TestResult) -> LastFailedTest {
    let diagnostic = result.diagnostics.first();
    let metadata = result.diagnosis.as_ref();
    let fixture = result
        .input
        .as_ref()
        .and_then(|input| input.fixture_name.clone());
    let fixture_entries = result
        .input
        .as_ref()
        .and_then(|input| input.fixture.as_ref())
        .map(|fixture| {
            fixture
                .entries
                .iter()
                .take(80)
                .map(|entry| entry.path.clone())
                .collect()
        })
        .unwrap_or_default();
    let command = result
        .input
        .as_ref()
        .map(|input| input.command.clone())
        .unwrap_or_default();
    let kind = diagnostic.map_or("behavior", |diagnostic| diagnostic.kind);
    let runner_failure = kind == "runner";
    let priority = if runner_failure {
        1
    } else {
        metadata.map_or_else(
            || if kind == "exit-code" { 50 } else { 1_000 },
            |diagnosis| diagnosis.priority,
        )
    };
    let diagnosis = FailureDiagnosis {
        priority,
        kind: kind.to_string(),
        headline: if runner_failure {
            diagnostic.map_or_else(
                || "The check command did not finish".to_string(),
                |diagnostic| diagnostic.title.clone(),
            )
        } else {
            metadata.map_or_else(
                || {
                    diagnostic.map_or_else(
                        || "The observed behavior contradicts the contract".to_string(),
                        |diagnostic| diagnostic.title.clone(),
                    )
                },
                |diagnosis| diagnosis.headline.clone(),
            )
        },
        summary: diagnostic.map_or_else(
            || result.failures.first().cloned().unwrap_or_default(),
            |diagnostic| diagnostic.summary.clone(),
        ),
        expected: if runner_failure {
            result.input.as_ref().map(|input| {
                format!(
                    "The command finishes successfully within {} ms",
                    input.timeout_ms
                )
            })
        } else {
            diagnostic
                .and_then(|diagnostic| diagnostic.expected.as_deref())
                .map(|value| bounded_text(value, 8 * 1024))
        },
        actual: if runner_failure {
            Some(bounded_text(
                diagnostic.map_or("The command did not finish", |diagnostic| {
                    diagnostic.summary.as_str()
                }),
                8 * 1024,
            ))
        } else {
            diagnostic
                .and_then(|diagnostic| diagnostic.actual.as_deref())
                .or_else(|| (!result.stdout.is_empty()).then_some(result.stdout.as_str()))
                .map(|value| {
                    if value.is_empty() {
                        "(no standard output)".to_string()
                    } else {
                        bounded_text(value, 8 * 1024)
                    }
                })
        },
        contract: if runner_failure {
            "The command must finish within the configured timeout so its behavior can be checked."
                .to_string()
        } else {
            metadata.map_or_else(
                || result.expectations.first().cloned().unwrap_or_default(),
                |diagnosis| diagnosis.contract.clone(),
            )
        },
        fixture,
        fixture_entries,
        command,
    };
    LastFailedTest {
        name: result.name.clone(),
        failures: result
            .failures
            .iter()
            .map(|failure| bounded_text(failure, 4 * 1024))
            .collect(),
        diagnosis: Some(diagnosis),
    }
}

fn bounded_text(value: &str, maximum_bytes: usize) -> String {
    if value.len() <= maximum_bytes {
        return value.to_string();
    }
    let mut boundary = maximum_bytes;
    while !value.is_char_boundary(boundary) {
        boundary -= 1;
    }
    format!("{}\n[deltaforge: detail truncated]", &value[..boundary])
}

fn sanitize_project_text(value: &str, project_root: &Path) -> String {
    // `locate_project_root` canonicalizes, which on Windows yields an
    // extended-length path (`\\?\C:\...`). Child processes such as cargo print
    // the ordinary form, so matching only the canonical spelling would leak the
    // learner's absolute path into persisted diagnoses and the workbench.
    // The prefixed spelling is replaced first: it contains the plain one, so the
    // reverse order would leave a stray `\\?\` behind.
    let mut value = value.to_string();
    for spelling in project_root_spellings(project_root) {
        value = replace_project_path(&value, &spelling);
    }
    value
}

fn project_root_spellings(project_root: &Path) -> Vec<String> {
    let native = project_root.to_string_lossy().to_string();
    let plain = native
        .strip_prefix(r"\\?\UNC\")
        .map(|rest| format!(r"\\{rest}"))
        .or_else(|| {
            native
                .strip_prefix(r"\\?\")
                .map(std::string::ToString::to_string)
        });
    match plain {
        Some(plain) if plain != native => vec![native, plain],
        _ => vec![native],
    }
}

fn replace_project_path(value: &str, native: &str) -> String {
    let escaped = native.replace('\\', "\\\\");
    let value = value.replace(&escaped, "{project_root}");
    let value = value.replace(native, "{project_root}");
    let portable = native.replace('\\', "/");
    if portable == native {
        value
    } else {
        value.replace(&portable, "{project_root}")
    }
}

fn health_action(kind: ProjectHealthActionKind, label: &str, primary: bool) -> ProjectHealthAction {
    ProjectHealthAction {
        kind,
        label: label.to_string(),
        primary,
    }
}

fn classify_project_health_error(detail: &str) -> (&'static str, &'static str, &'static str, bool) {
    if detail.contains("sync-pack")
        || detail.contains("pack contents changed")
        || detail.contains("project is pinned to pack")
    {
        (
            "pack_changed",
            "The project pack changed",
            "Review the change, then adopt the currently installed pack. Completed capabilities may require revalidation.",
            true,
        )
    } else if detail.contains("integrity digest") {
        (
            "integrity_blocked",
            "DeltaForge cannot fingerprint this project",
            "Follow the detail below — usually replacing a directory symlink with a copy, or naming it in integrity.exclude in .deltaforge/config.toml — then check again.",
            false,
        )
    } else if detail.contains("capability instructions are missing") {
        (
            "pack_content_invalid",
            "This step's guide cannot be rendered",
            "The pack is missing a required section for this step. Fix the pack's instructions.md, or run `deltaforge validate-pack <pack> --strict` to see every gap, then check again.",
            false,
        )
    } else if detail.contains("config.toml") {
        (
            "configuration_invalid",
            "The project configuration cannot be read",
            "Open the project and correct .deltaforge/config.toml, then check again.",
            false,
        )
    } else if detail.contains("state.json") {
        (
            "state_invalid",
            "The project state cannot be read",
            "Restore .deltaforge/state.json from version control or a backup, then check again.",
            false,
        )
    } else if detail.contains("does not support language") {
        (
            "language_unavailable",
            "The configured language is unavailable",
            "Restore the matching pack or correct the project language in .deltaforge/state.json.",
            false,
        )
    } else {
        (
            "project_unavailable",
            "DeltaForge cannot load this project",
            "Open the project, resolve the reported problem, then check again.",
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::{TestDiagnosis, TestDiagnostic, TestInput};

    #[test]
    fn a_runner_failure_precedes_pack_behavior_priorities() {
        let result = TestResult {
            name: "scans files in a basic project".to_string(),
            passed: false,
            diagnosis: Some(TestDiagnosis {
                priority: 10,
                headline: "Your scanner did not report required files".to_string(),
                contract: "Every regular file must be reported.".to_string(),
            }),
            failures: vec!["command timed out".to_string()],
            diagnostics: vec![TestDiagnostic {
                kind: "runner",
                title: "The test command did not finish".to_string(),
                summary: "command timed out".to_string(),
                expected: None,
                actual: None,
            }],
            expectations: Vec::new(),
            actual_exit_code: None,
            duration_ms: Some(100),
            stdout: String::new(),
            stderr: String::new(),
            report_stdout: None,
            report_stderr: None,
            input: Some(TestInput {
                command: vec!["{project_root}/target/debug/flashindex".to_string()],
                stdin: None,
                env: std::collections::BTreeMap::new(),
                timeout_ms: 100,
                working_directory: "{project_root}".to_string(),
                fixture_name: Some("basic_project".to_string()),
                fixture: None,
            }),
            kept_temp_dir: None,
        };

        let failure = record_failed_test(&result);
        let diagnosis = failure.diagnosis.unwrap();
        assert_eq!(diagnosis.priority, 1);
        assert_eq!(diagnosis.headline, "The test command did not finish");
        assert!(diagnosis.contract.contains("configured timeout"));
        assert_eq!(
            diagnosis.expected.as_deref(),
            Some("The command finishes successfully within 100 ms")
        );
        assert_eq!(diagnosis.actual.as_deref(), Some("command timed out"));
    }

    #[test]
    fn extended_length_roots_do_not_leak_the_ordinary_windows_path() {
        // `canonicalize` yields `\\?\C:\...` on Windows while child processes
        // print `C:\...`; both spellings must be redacted.
        let root = Path::new(r"\\?\C:\Users\learner\AppData\Local\Temp\project");

        assert_eq!(
            sanitize_project_text(
                concat!(
                    r"Compiling flashindex (C:\Users\learner\AppData\Local\Temp\project)",
                    "\n",
                    r"canonical \\?\C:\Users\learner\AppData\Local\Temp\project",
                    "\n",
                    r#"json "C:\\Users\\learner\\AppData\\Local\\Temp\\project""#,
                    "\nportable C:/Users/learner/AppData/Local/Temp/project",
                ),
                root,
            ),
            concat!(
                "Compiling flashindex ({project_root})\n",
                "canonical {project_root}\n",
                r#"json "{project_root}""#,
                "\nportable {project_root}",
            )
        );
        assert!(
            !sanitize_project_text(r"at C:\Users\learner\AppData\Local\Temp\project\src", root)
                .contains("learner")
        );
    }

    #[test]
    fn unc_and_plain_roots_are_still_redacted() {
        assert_eq!(
            sanitize_project_text(
                r"at \\server\share\project\src",
                Path::new(r"\\?\UNC\server\share\project")
            ),
            "at {project_root}\\src"
        );
        assert_eq!(
            sanitize_project_text(
                "at /home/learner/project/src",
                Path::new("/home/learner/project")
            ),
            "at {project_root}/src"
        );
    }

    #[test]
    fn project_health_errors_choose_actionable_recovery() {
        let config = classify_project_health_error(
            "failed to parse project/.deltaforge/config.toml: invalid value",
        );
        assert_eq!(config.0, "configuration_invalid");
        assert!(!config.3);

        let pack = classify_project_health_error(
            "pack contents changed since project initialization. Run `deltaforge sync-pack`",
        );
        assert_eq!(pack.0, "pack_changed");
        assert!(pack.3);
    }
}
