use std::collections::BTreeMap;

use anyhow::{Context, Result};
use serde::Serialize;

use crate::context::ProjectContext;

#[derive(Debug, Clone, Serialize)]
pub struct CapabilityContent {
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

    Ok(CapabilityContent {
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

        let help = parse_help(
            "# Hint 1 — Observation\n\nLook at output.\n\n# Hint 2 — Concept\n\nThink recursively.\n",
        );
        assert_eq!(help.len(), 2);
        assert_eq!(help[0].label, "Observation");
        assert_eq!(help[1].level, 2);
    }
}
