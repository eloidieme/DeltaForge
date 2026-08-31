use std::collections::BTreeMap;
use std::fs;

use anyhow::{Context, Result};
use serde::Serialize;

use crate::context::ProjectContext;

#[derive(Debug, Clone, Serialize)]
pub struct CapabilityContent {
    pub project_overview: ProjectOverview,
    pub roadmap: Vec<RoadmapCapability>,
    pub stage_id: String,
    pub title: String,
    pub mission: String,
    pub why: String,
    pub success_conditions: Vec<String>,
    pub example: String,
    pub requirements: Vec<String>,
    pub edge_cases: Vec<String>,
    pub non_goals: Vec<String>,
    pub capability_statement: String,
    pub next: Option<CapabilityPreview>,
    pub help_levels: usize,
    pub revealed_help: Vec<HelpLevel>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectOverview {
    pub name: String,
    pub description: String,
    pub topics: Vec<String>,
    pub context: Vec<String>,
    pub why: Vec<String>,
    pub sections: Vec<OverviewSection>,
    pub capability_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverviewSection {
    pub title: String,
    pub blocks: Vec<OverviewBlock>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OverviewBlock {
    Paragraph { text: String },
    Code { language: String, content: String },
    List { items: Vec<String> },
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadmapCapability {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub position: usize,
    pub status: RoadmapStatus,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RoadmapStatus {
    Complete,
    Current,
    Upcoming,
}

#[derive(Debug, Clone, Serialize)]
pub struct CapabilityPreview {
    pub id: String,
    pub title: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HelpLevel {
    pub level: usize,
    pub label: String,
    pub content: String,
}

pub fn load_current(context: &ProjectContext) -> Result<CapabilityContent> {
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
    let source = context
        .pack
        .read_stage_file(&context.pack.instructions_path(stage))?;
    let sections = markdown_sections(&source);
    let hints = load_help(context)?;
    let revealed_level = context
        .state
        .hint_state
        .get(&stage.id)
        .copied()
        .unwrap_or_default();
    let revealed_help = hints
        .iter()
        .filter(|hint| hint.level <= revealed_level)
        .cloned()
        .collect();
    let next = context
        .pack
        .manifest
        .next_stage(&stage.id)
        .map(|next| -> Result<CapabilityPreview> {
            let source = context
                .pack
                .read_stage_file(&context.pack.instructions_path(next))?;
            let sections = markdown_sections(&source);
            Ok(CapabilityPreview {
                id: next.id.clone(),
                title: next.title.clone(),
                summary: first_paragraph(required_section(&sections, "Goal")?),
            })
        })
        .transpose()?;
    let project_overview = load_project_overview(context);
    let roadmap = load_roadmap(context)?;

    Ok(CapabilityContent {
        project_overview,
        roadmap,
        stage_id: stage.id.clone(),
        title: stage.title.clone(),
        mission: first_paragraph(required_section(&sections, "Goal")?),
        why: first_paragraph(required_section(&sections, "Background")?),
        success_conditions: bullet_items(required_section(&sections, "Success criteria")?),
        example: first_code_block(required_section(&sections, "Example")?),
        requirements: bullet_items(required_section(&sections, "Requirements")?),
        edge_cases: bullet_items(required_section(&sections, "Edge cases")?),
        non_goals: bullet_items(required_section(&sections, "Non-goals")?),
        capability_statement: sections
            .get("Capability acquired")
            .map(|section| first_paragraph(section))
            .unwrap_or_else(|| format!("Your program can now {}.", stage.title.to_lowercase())),
        next,
        help_levels: hints.len(),
        revealed_help,
    })
}

fn load_project_overview(context: &ProjectContext) -> ProjectOverview {
    let source = fs::read_to_string(context.pack.root.join("README.md")).unwrap_or_default();
    let sections = markdown_sections(&source);
    let overview_sections = parse_overview_sections(&source);
    let context_paragraphs = sections
        .get("What you are building")
        .map(|section| {
            narrative_paragraphs(section, 6)
                .into_iter()
                .filter(|paragraph| !paragraph.ends_with(':'))
                .take(5)
                .collect()
        })
        .unwrap_or_default();
    ProjectOverview {
        name: context.pack.manifest.name.clone(),
        description: context.pack.manifest.description.clone(),
        topics: context.pack.manifest.topics.clone(),
        context: context_paragraphs,
        why: sections
            .get("Why this is useful")
            .map(|section| narrative_paragraphs(section, 2))
            .unwrap_or_default(),
        sections: overview_sections,
        capability_count: context.pack.manifest.stages.len(),
    }
}

fn parse_overview_sections(source: &str) -> Vec<OverviewSection> {
    let mut sections = Vec::new();
    let mut title = None::<String>;
    let mut lines = Vec::new();
    for line in source.lines() {
        if let Some(next) = line.strip_prefix("## ") {
            if let Some(previous) = title.replace(next.trim().to_string()) {
                sections.push(OverviewSection {
                    title: previous,
                    blocks: parse_overview_blocks(&lines),
                });
                lines.clear();
            }
        } else if title.is_some() {
            lines.push(line);
        }
    }
    if let Some(title) = title {
        sections.push(OverviewSection {
            title,
            blocks: parse_overview_blocks(&lines),
        });
    }
    sections.retain(|section| !section.blocks.is_empty());
    sections
}

fn parse_overview_blocks(lines: &[&str]) -> Vec<OverviewBlock> {
    let mut blocks = Vec::new();
    let mut paragraph = Vec::new();
    let mut list = Vec::new();
    let mut code = Vec::new();
    let mut code_language = String::new();
    let mut in_code = false;

    let flush_paragraph = |paragraph: &mut Vec<&str>, blocks: &mut Vec<OverviewBlock>| {
        if paragraph.is_empty() {
            return;
        }
        let text = strip_inline_markdown(
            &paragraph
                .drain(..)
                .map(str::trim)
                .collect::<Vec<_>>()
                .join(" "),
        );
        if !text.is_empty() {
            blocks.push(OverviewBlock::Paragraph { text });
        }
    };
    let flush_list = |list: &mut Vec<String>, blocks: &mut Vec<OverviewBlock>| {
        if !list.is_empty() {
            blocks.push(OverviewBlock::List {
                items: std::mem::take(list),
            });
        }
    };

    for line in lines {
        let trimmed = line.trim();
        if let Some(fence) = trimmed.strip_prefix("```") {
            if in_code {
                blocks.push(OverviewBlock::Code {
                    language: std::mem::take(&mut code_language),
                    content: code.join("\n"),
                });
                code.clear();
                in_code = false;
            } else {
                flush_paragraph(&mut paragraph, &mut blocks);
                flush_list(&mut list, &mut blocks);
                code_language = fence.trim().to_string();
                in_code = true;
            }
        } else if in_code {
            code.push(*line);
        } else if let Some(item) = markdown_list_item(trimmed) {
            flush_paragraph(&mut paragraph, &mut blocks);
            list.push(strip_inline_markdown(item));
        } else if trimmed.is_empty() {
            flush_paragraph(&mut paragraph, &mut blocks);
            flush_list(&mut list, &mut blocks);
        } else {
            flush_list(&mut list, &mut blocks);
            paragraph.push(line);
        }
    }
    flush_paragraph(&mut paragraph, &mut blocks);
    flush_list(&mut list, &mut blocks);
    blocks
}

fn markdown_list_item(line: &str) -> Option<&str> {
    line.strip_prefix("- ").or_else(|| {
        let (number, item) = line.split_once(". ")?;
        (!number.is_empty() && number.chars().all(|character| character.is_ascii_digit()))
            .then_some(item)
    })
}

fn load_roadmap(context: &ProjectContext) -> Result<Vec<RoadmapCapability>> {
    context
        .pack
        .manifest
        .stages
        .iter()
        .enumerate()
        .map(|(index, stage)| {
            let source = context
                .pack
                .read_stage_file(&context.pack.instructions_path(stage))?;
            let sections = markdown_sections(&source);
            let status = if context.state.is_completed(&stage.id) {
                RoadmapStatus::Complete
            } else if stage.id == context.state.current_stage {
                RoadmapStatus::Current
            } else {
                RoadmapStatus::Upcoming
            };
            Ok(RoadmapCapability {
                id: stage.id.clone(),
                title: stage.title.clone(),
                summary: first_paragraph(required_section(&sections, "Goal")?),
                position: index + 1,
                status,
            })
        })
        .collect()
}

/// The stage's prediction prompt, with its leading `#` heading removed.
/// `None` when the stage declares none, which is every stage without
/// benchmarks.
pub fn read_prediction_prompt(
    context: &ProjectContext,
    stage: &crate::pack::StageSpec,
) -> Option<String> {
    let source = std::fs::read_to_string(context.pack.prediction_prompt_path(stage)).ok()?;
    let body = source
        .lines()
        .skip_while(|line| line.trim_start().starts_with('#') || line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();
    (!body.is_empty()).then_some(body)
}

pub fn load_help(context: &ProjectContext) -> Result<Vec<HelpLevel>> {
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
    let source = context
        .pack
        .read_stage_file(&context.pack.hints_path(stage))?;
    Ok(parse_help(&source))
}

fn markdown_sections(source: &str) -> BTreeMap<String, String> {
    let mut sections = BTreeMap::new();
    let mut heading = None::<String>;
    let mut lines = Vec::new();
    for line in source.lines() {
        if let Some(next) = line.strip_prefix("## ") {
            if let Some(previous) = heading.take() {
                sections.insert(previous, lines.join("\n").trim().to_string());
                lines.clear();
            }
            heading = Some(next.trim().to_string());
        } else if heading.is_some() {
            lines.push(line);
        }
    }
    if let Some(heading) = heading {
        sections.insert(heading, lines.join("\n").trim().to_string());
    }
    sections
}

fn required_section<'a>(sections: &'a BTreeMap<String, String>, name: &str) -> Result<&'a str> {
    sections
        .get(name)
        .map(String::as_str)
        .with_context(|| format!("capability instructions are missing the {name} section"))
}

fn first_paragraph(section: &str) -> String {
    section
        .split("\n\n")
        .find(|paragraph| !paragraph.trim().is_empty() && !paragraph.trim().starts_with("```"))
        .map(|paragraph| {
            strip_inline_markdown(
                &paragraph
                    .lines()
                    .map(str::trim)
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        })
        .unwrap_or_default()
}

fn narrative_paragraphs(section: &str, limit: usize) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut paragraph = Vec::new();
    let mut in_code_block = false;
    let flush = |paragraph: &mut Vec<&str>, paragraphs: &mut Vec<String>| {
        if paragraph.is_empty() {
            return;
        }
        let text = strip_inline_markdown(
            &paragraph
                .drain(..)
                .map(str::trim)
                .collect::<Vec<_>>()
                .join(" "),
        );
        if !text.is_empty() {
            paragraphs.push(text);
        }
    };
    for line in section.lines() {
        if line.trim_start().starts_with("```") {
            flush(&mut paragraph, &mut paragraphs);
            in_code_block = !in_code_block;
        } else if in_code_block {
            continue;
        } else if line.trim().is_empty() {
            flush(&mut paragraph, &mut paragraphs);
        } else {
            paragraph.push(line);
        }
        if paragraphs.len() >= limit {
            break;
        }
    }
    if paragraphs.len() < limit {
        flush(&mut paragraph, &mut paragraphs);
    }
    paragraphs.truncate(limit);
    paragraphs
}

fn bullet_items(section: &str) -> Vec<String> {
    section
        .lines()
        .filter_map(|line| line.trim().strip_prefix("- "))
        .map(strip_inline_markdown)
        .filter(|item| !item.is_empty())
        .collect()
}

fn first_code_block(section: &str) -> String {
    let mut inside = false;
    let mut lines = Vec::new();
    for line in section.lines() {
        if line.trim_start().starts_with("```") {
            if inside {
                break;
            }
            inside = true;
        } else if inside {
            lines.push(line);
        }
    }
    lines.join("\n").trim().to_string()
}

fn strip_inline_markdown(text: &str) -> String {
    text.replace(['`', '*'], "")
        .replace("  ", " ")
        .trim()
        .to_string()
}

fn parse_help(source: &str) -> Vec<HelpLevel> {
    let mut levels = Vec::new();
    let mut heading = None::<String>;
    let mut lines = Vec::new();
    for line in source.lines() {
        if let Some(next) = line.strip_prefix("# Hint ") {
            push_help(&mut levels, heading.take(), &mut lines);
            heading = Some(next.trim().to_string());
        } else if heading.is_some() {
            lines.push(line);
        }
    }
    push_help(&mut levels, heading, &mut lines);
    levels
}

fn push_help(levels: &mut Vec<HelpLevel>, heading: Option<String>, lines: &mut Vec<&str>) {
    let Some(heading) = heading else {
        return;
    };
    let (number, label) = heading
        .split_once(['—', '-'])
        .map_or((heading.as_str(), "Hint"), |(number, label)| {
            (number.trim(), label.trim())
        });
    let Ok(level) = number.parse::<usize>() else {
        lines.clear();
        return;
    };
    let content = lines.join("\n").trim().to_string();
    lines.clear();
    if !content.is_empty() {
        levels.push(HelpLevel {
            level,
            label: label.to_string(),
            content,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_structured_sections_and_help_levels() {
        let sections = markdown_sections(
            "# Stage\n\n## Goal\n\nBuild `scan`.\n\n## Requirements\n\n- Walk files.\n- Sort output.\n",
        );
        assert_eq!(first_paragraph(&sections["Goal"]), "Build scan.");
        assert_eq!(bullet_items(&sections["Requirements"]).len(), 2);

        let paragraphs = narrative_paragraphs(
            "First paragraph.\n\n```text\nskipped\n```\n\nSecond `paragraph`.",
            2,
        );
        assert_eq!(paragraphs, ["First paragraph.", "Second paragraph."]);

        let overview = parse_overview_sections(
            "# Project\n\n## What you build\n\nRead `input`.\n\n```text\ninput -> output\n```\n\n- First step\n- Second step\n",
        );
        assert_eq!(overview.len(), 1);
        assert_eq!(overview[0].title, "What you build");
        assert_eq!(overview[0].blocks.len(), 3);

        let help = parse_help(
            "# Hint 1 — Observation\n\nLook at output.\n\n# Hint 2 — Concept\n\nThink recursively.\n",
        );
        assert_eq!(help.len(), 2);
        assert_eq!(help[0].label, "Observation");
        assert_eq!(help[1].level, 2);
    }
}
