use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::fs_util::atomic_write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    #[serde(default = "current_state_schema_version")]
    pub schema_version: u32,
    pub project: String,
    pub language: String,
    pub current_stage: String,
    #[serde(default)]
    pub completed_stages: Vec<String>,
    #[serde(default)]
    pub completed_stage_timestamps: BTreeMap<String, String>,
    #[serde(default)]
    pub last_test_runs: BTreeMap<String, LastTestRunSummary>,
    #[serde(default)]
    pub hint_state: BTreeMap<String, usize>,
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastTestRunSummary {
    pub stage_id: String,
    pub passed: usize,
    pub failed: usize,
    pub timestamp: String,
    #[serde(default)]
    pub failed_tests: Vec<LastFailedTest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastFailedTest {
    pub name: String,
    pub failures: Vec<String>,
}

impl ProjectState {
    pub fn new(project: String, language: String, current_stage: String) -> Result<Self> {
        let now = current_timestamp()?;
        Ok(Self {
            schema_version: current_state_schema_version(),
            project,
            language,
            current_stage,
            completed_stages: Vec::new(),
            completed_stage_timestamps: BTreeMap::new(),
            last_test_runs: BTreeMap::new(),
            hint_state: BTreeMap::new(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn write_to(&self, path: &Path) -> Result<()> {
        let serialized =
            serde_json::to_string_pretty(self).context("failed to serialize project state")?;
        atomic_write(path, serialized)
            .with_context(|| format!("failed to write state file {}", path.display()))?;

        Ok(())
    }

    pub fn read_from(path: &Path) -> Result<Self> {
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read state file {}", path.display()))?;
        let mut state: Self = serde_json::from_str(&source)
            .with_context(|| format!("failed to parse state file {}", path.display()))?;
        state.validate(path)?;
        if state.updated_at.is_empty() {
            state.updated_at = state.created_at.clone();
        }
        Ok(state)
    }

    pub fn validate(&self, path: &Path) -> Result<()> {
        if self.schema_version != current_state_schema_version() {
            bail!(
                "unsupported state schema_version {} in {}; expected {}",
                self.schema_version,
                path.display(),
                current_state_schema_version()
            );
        }
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

    pub fn record_test_run(
        &mut self,
        stage_id: String,
        passed: usize,
        failed: usize,
        failed_tests: Vec<LastFailedTest>,
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
            },
        );
        self.updated_at = now;
        Ok(())
    }

    pub fn touch(&mut self) -> Result<()> {
        self.updated_at = current_timestamp()?;
        Ok(())
    }
}

fn current_state_schema_version() -> u32 {
    1
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
}
