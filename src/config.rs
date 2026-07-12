use std::fs;
use std::io;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::fs_util::atomic_write;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectConfig {
    #[serde(default = "current_config_schema_version")]
    pub schema_version: u32,
    #[serde(default)]
    pub runner: RunnerConfig,
    #[serde(default)]
    pub bench: BenchConfig,
    #[serde(default)]
    pub git: GitConfig,
    #[serde(default)]
    pub integrity: IntegrityConfig,
    #[serde(default)]
    pub gates: GatesConfig,
}

/// Learner-controlled additions to the integrity digest exclusion list.
/// Entries are directory or file names matched at any depth, like the built-in
/// exclusions (`target`, `node_modules`, ...). Useful when a tool creates a
/// generated directory or directory symlink DeltaForge does not know about.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrityConfig {
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerConfig {
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
    #[serde(default = "default_build_timeout_ms")]
    pub build_timeout_ms: u64,
    #[serde(default)]
    pub keep_temp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchConfig {
    #[serde(default = "default_bench_iterations")]
    pub iterations: u64,
    #[serde(default = "default_bench_warmup")]
    pub warmup: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitConfig {
    #[serde(default)]
    pub auto_commit: bool,
    #[serde(default = "default_auto_tag")]
    pub auto_tag: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GatesConfig {
    #[serde(default = "default_gates_enforce")]
    pub enforce: bool,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            schema_version: current_config_schema_version(),
            runner: RunnerConfig::default(),
            bench: BenchConfig::default(),
            git: GitConfig::default(),
            integrity: IntegrityConfig::default(),
            gates: GatesConfig::default(),
        }
    }
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            timeout_ms: default_timeout_ms(),
            build_timeout_ms: default_build_timeout_ms(),
            keep_temp: false,
        }
    }
}

impl Default for BenchConfig {
    fn default() -> Self {
        Self {
            iterations: default_bench_iterations(),
            warmup: default_bench_warmup(),
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            auto_commit: false,
            auto_tag: default_auto_tag(),
        }
    }
}

impl Default for GatesConfig {
    fn default() -> Self {
        Self { enforce: true }
    }
}

impl ProjectConfig {
    pub fn read_from(path: &Path) -> Result<Self> {
        let source = match fs::read_to_string(path) {
            Ok(source) => source,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("failed to read config file {}", path.display()));
            }
        };

        let config: Self = toml::from_str(&source)
            .with_context(|| format!("failed to parse config file {}", path.display()))?;
        config.validate(path)?;
        Ok(config)
    }

    pub fn write_to(&self, path: &Path) -> Result<()> {
        let serialized =
            toml::to_string_pretty(self).context("failed to serialize project config")?;
        atomic_write(path, serialized)
            .with_context(|| format!("failed to write config file {}", path.display()))
    }

    pub fn validate(&self, path: &Path) -> Result<()> {
        if self.schema_version != current_config_schema_version() {
            bail!(
                "unsupported config schema_version {} in {}; expected {}",
                self.schema_version,
                path.display(),
                current_config_schema_version()
            );
        }
        if self.runner.timeout_ms == 0 {
            bail!(
                "invalid config {}: runner.timeout_ms must be greater than 0",
                path.display()
            );
        }
        if self.runner.build_timeout_ms == 0 {
            bail!(
                "invalid config {}: runner.build_timeout_ms must be greater than 0",
                path.display()
            );
        }
        if self.bench.iterations == 0 {
            bail!(
                "invalid config {}: bench.iterations must be greater than 0",
                path.display()
            );
        }
        for name in &self.integrity.exclude {
            if name.trim().is_empty() || name.contains(['/', '\\']) {
                bail!(
                    "invalid config {}: integrity.exclude entries must be plain file or directory names, got {name:?}",
                    path.display()
                );
            }
        }
        Ok(())
    }
}

fn current_config_schema_version() -> u32 {
    1
}

fn default_timeout_ms() -> u64 {
    5_000
}

fn default_build_timeout_ms() -> u64 {
    120_000
}

fn default_bench_iterations() -> u64 {
    7
}

fn default_bench_warmup() -> u64 {
    2
}

fn default_auto_tag() -> bool {
    true
}

fn default_gates_enforce() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_partial_config_with_defaults() {
        let config: ProjectConfig = toml::from_str(
            r#"
[runner]
timeout_ms = 250
"#,
        )
        .unwrap();

        assert_eq!(config.runner.timeout_ms, 250);
        assert_eq!(config.runner.build_timeout_ms, 120_000);
        assert!(!config.runner.keep_temp);
        assert_eq!(config.bench.iterations, 7);
        assert!(config.git.auto_tag);
        assert!(config.gates.enforce);
    }
}
