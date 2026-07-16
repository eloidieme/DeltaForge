use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Serialize;

use crate::context::{GlobalOptions, ProjectContext};
use crate::pack::pack_source_label;
use crate::runner::{self, RunnerOptions, TestResult, TestRunSummary};
use crate::state::{
    ActiveJob, AttemptStatus, FailureDiagnosis, LastFailedTest, LastTestRunSummary,
    SourceChangeRecord, TestAttempt,
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
    pub capture_details: bool,
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
    pub latest_attempt: Option<TestAttempt>,
    pub latest_run: Option<LastTestRunSummary>,
    pub primary_failure: Option<LastFailedTest>,
    pub source_revision: u64,
    pub last_source_change: Option<SourceChangeRecord>,
    pub event_cursor: u64,
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

pub fn load_project_health(options: &GlobalOptions) -> Result<ProjectHealth> {
    let root = crate::context::locate_project_root(options)?;
    match ProjectContext::load(options) {
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
    let _lease = crate::run_lease::RunLease::acquire(&root)
        .context("could not recover the pack while checks are running")?;
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
    workbench_state(&context, recovered_interrupted_job, None)
}

pub fn load_workbench_state_for_session(
    options: &GlobalOptions,
    session_id: &str,
) -> Result<WorkbenchState> {
    let initial = load_workbench_state(options)?;
    if initial.active_job.is_some() {
        return Ok(initial);
    }
    let current = ProjectContext::load(options)?;
    if current
        .state
        .last_workbench_session
        .as_ref()
        .is_some_and(|session| session.id == session_id)
    {
        return workbench_state(
            &current,
            initial.recovered_interrupted_job,
            Some(session_id),
        );
    }
    let _ = observe_source_changes(options)?;

    let root = crate::context::locate_project_root(options)?;
    let _session_lease = match crate::run_lease::RunLease::acquire(&root) {
        Ok(lease) => lease,
        Err(_) => return Ok(initial),
    };
    let mut context = ProjectContext::load(options)?;
    let changed = context
        .state
        .begin_workbench_session(session_id.to_string(), initial.recovered_interrupted_job)?;
    if changed {
        context.save_state()?;
    }
    workbench_state(
        &context,
        initial.recovered_interrupted_job,
        Some(session_id),
    )
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
        .context("there is no active check run to cancel")?;
    if !crate::run_lease::active(&context.root) {
        bail!("the active check run has already stopped");
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
    let _lease = crate::run_lease::RunLease::acquire(&context.root)
        .context("could not update help while checks are running")?;
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
    let maximum = if context.state.is_completed(&stage_id) {
        help.len()
    } else {
        help.len().min(4)
    };
    if maximum == 0 {
        bail!("this capability has no help levels");
    }
    if current >= maximum {
        if context.state.is_completed(&stage_id) {
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
    let _lease = crate::run_lease::RunLease::acquire(&context.root)
        .context("could not advance while checks are running")?;
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
    let _lease = crate::run_lease::RunLease::acquire(&context.root)?;
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
    let job_id = context.state.start_test_job(stage_ids.clone())?;
    context.save_state()?;
    let project_root = context.root.clone();
    let mut sink = JournalSink {
        project_root: &project_root,
        downstream: sink,
    };
    sink.emit(RunEvent::JobStarted {
        job_id: job_id.clone(),
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
        // even when the initiating CLI surface does not render those details.
        capture_details: request.capture_details || !request.list_tests,
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
        context.state.finish_test_job(
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
    let freshness = match context.state.last_test_runs.get(&current.id) {
        None => ResultFreshness::NeverRun,
        Some(run) if run.project_digest == context.project_digest()? => ResultFreshness::Fresh,
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
        primary_failure = Some(LastFailedTest {
            name: "Build project".to_string(),
            failures: vec![error.clone()],
            diagnosis: Some(FailureDiagnosis {
                priority: 0,
                kind: "build".to_string(),
                headline: "The project did not build".to_string(),
                summary: "Checks could not start because the configured build command failed."
                    .to_string(),
                expected: Some("A successful project build".to_string()),
                actual: Some(error.clone()),
                contract: "The project must build before behavioral checks can run.".to_string(),
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
                    "DeltaForge preserved the last completed evidence. Run checks again when you are ready."
                        .to_string(),
                )
            } else if let Some(change) = &stage_change {
                (
                    ResumptionKind::CapabilityChanged,
                    "Current capability changed".to_string(),
                    format!(
                        "Your previous session was on {}. Continue with {}.",
                        change.from_title, change.to_title
                    ),
                )
            } else if freshness == ResultFreshness::Stale {
                (
                    ResumptionKind::SourceChanged,
                    "Source changed since the last result".to_string(),
                    "Your previous evidence is preserved, but the current source needs a new check run."
                        .to_string(),
                )
            } else if context.state.is_completed(&current.id)
                && freshness == ResultFreshness::Fresh
            {
                (
                    ResumptionKind::CapabilityAcquired,
                    "Capability evidence is still current".to_string(),
                    "The completed capability and its passing evidence were restored without rerunning checks."
                        .to_string(),
                )
            } else if primary_failure.is_some() && freshness == ResultFreshness::Fresh {
                (
                    ResumptionKind::ChecksFailed,
                    "Your last contradiction is ready".to_string(),
                    "The latest run and primary remaining failure were restored without rerunning checks."
                        .to_string(),
                )
            } else {
                (
                    ResumptionKind::Ready,
                    "Ready to continue".to_string(),
                    "Your project and current capability were restored without rerunning checks."
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
                label: "Begin next capability".to_string(),
                enabled: true,
            }
        } else {
            PrimaryAction {
                kind: PrimaryActionKind::JourneyComplete,
                label: "Journey complete".to_string(),
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
        latest_run,
        primary_failure,
        source_revision: context.state.source_revision,
        last_source_change: context.state.last_source_change.clone(),
        event_cursor,
    })
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
    let native = project_root.to_string_lossy();
    let escaped = native.replace('\\', "\\\\");
    let value = value.replace(&escaped, "{project_root}");
    let value = value.replace(native.as_ref(), "{project_root}");
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
                headline: "Required project files are missing".to_string(),
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
