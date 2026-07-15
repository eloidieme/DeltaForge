use anyhow::{Context, Result};
use serde::Serialize;

use crate::context::{GlobalOptions, ProjectContext};
use crate::runner::{self, RunnerOptions, TestResult, TestRunSummary};
use crate::state::{AttemptStatus, LastFailedTest};

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
    pub freshness: ResultFreshness,
    pub revealed_hint_level: usize,
    pub last_activity_at: String,
    pub recovered_interrupted_job: bool,
}

pub fn load_workbench_state(options: &GlobalOptions) -> Result<WorkbenchState> {
    let mut context = ProjectContext::load(options)?;
    let recovered_interrupted_job = context.state.recover_interrupted_job()?;
    if recovered_interrupted_job {
        context.save_state()?;
    }
    workbench_state(&context, recovered_interrupted_job)
}

pub fn run_tests(
    options: &GlobalOptions,
    request: TestRunRequest,
    sink: &mut dyn EventSink,
) -> Result<TestRunOutcome> {
    let mut context = ProjectContext::load(options)?;
    context.state.recover_interrupted_job()?;
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
    sink.emit(RunEvent::JobStarted {
        job_id: job_id.clone(),
        stage_ids,
        trigger: request.trigger,
    });

    let runner_options = RunnerOptions {
        filter: request.filter,
        list_tests: request.list_tests,
        fail_fast: request.fail_fast,
        no_build: request.no_build,
        keep_temp: request.keep_temp,
        capture_details: request.capture_details,
    };
    let mut summaries = Vec::new();
    let mut newly_completed_current = false;
    let mut execution_error = None;

    for stage in &stages {
        match runner::run_stage_tests(&context, stage, &runner_options, sink) {
            Ok(summary) => {
                if !runner_options.list_tests && summary.completion_eligible {
                    let was_completed = context.state.is_completed(&stage.id);
                    context.state.record_completion_proof(
                        &stage.id,
                        context.pack_digest()?,
                        context.stage_behavioral_digest(&stage.id)?,
                        context.project_digest()?,
                        summary.total_defined,
                    )?;
                    newly_completed_current |=
                        !was_completed && stage.id == context.state.current_stage;
                }
                if !runner_options.list_tests {
                    context.state.record_test_run(
                        stage.id.clone(),
                        summary.passed,
                        summary.failed,
                        summary
                            .results
                            .iter()
                            .filter(|result| !result.passed)
                            .map(|result| LastFailedTest {
                                name: result.name.clone(),
                                failures: result.failures.clone(),
                            })
                            .collect(),
                        context.project_digest()?,
                    )?;
                }
                summaries.push(summary);
            }
            Err(error) => {
                execution_error = Some(format!("{error:#}"));
                break;
            }
        }
    }
    let passed = summaries.iter().map(|summary| summary.passed).sum();
    let failed = summaries.iter().map(|summary| summary.failed).sum();
    let successful = execution_error.is_none()
        && !summaries.is_empty()
        && summaries.iter().all(TestRunSummary::is_success);
    if !runner_options.list_tests {
        context.state.finish_test_job(
            &job_id,
            if successful {
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

    sink.emit(RunEvent::RunCompleted {
        job_id: job_id.clone(),
        passed: successful,
        passed_tests: passed,
        failed_tests: failed,
    });
    sink.emit(RunEvent::ProjectStateChanged);

    Ok(TestRunOutcome {
        job_id,
        summaries,
        newly_completed_current,
        execution_error,
    })
}

fn workbench_state(context: &ProjectContext, recovered: bool) -> Result<WorkbenchState> {
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
        freshness,
        revealed_hint_level: context
            .state
            .hint_state
            .get(&current.id)
            .copied()
            .unwrap_or_default(),
        last_activity_at: context.state.updated_at.clone(),
        recovered_interrupted_job: recovered,
    })
}
