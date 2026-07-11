use std::io::{IsTerminal, Write};
use std::process::{Command, Stdio};

use anyhow::{Context, Result};

pub struct Terminal {
    color: bool,
    width: usize,
    output: String,
}

impl Terminal {
    pub fn new() -> Self {
        let color = std::io::stdout().is_terminal()
            && std::env::var_os("NO_COLOR").is_none()
            && std::env::var("TERM").map_or(true, |term| term != "dumb");
        Self {
            color,
            width: 92,
            output: String::new(),
        }
    }

    pub fn display(self) -> Result<()> {
        if should_page() {
            page(&self.output)?;
        } else {
            print!("{}", self.output);
        }
        Ok(())
    }

    pub fn title(&mut self, title: &str) {
        self.line(&self.paint(title, "1;36"));
        self.line(&self.paint(&"─".repeat(title.chars().count().min(self.width)), "36"));
    }

    pub fn key_value(&mut self, key: &str, value: &str) {
        self.line(&format!(
            "{} {}",
            self.paint(&format!("{key}:"), "1"),
            value
        ));
    }

    pub fn section(&mut self, title: &str) {
        self.blank_line();
        self.line(&self.paint(title, "1;33"));
        self.line(&self.paint(&"─".repeat(title.chars().count()), "33"));
    }

    pub fn roadmap_line(&mut self, marker: &str, stage_id: &str, title: &str, status: &str) {
        let marker = match status {
            "complete" => self.paint(marker, "32"),
            "current" => self.paint(marker, "36;1"),
            _ => self.paint(marker, "2"),
        };
        let stage_id = self.paint(stage_id, "1");
        self.line(&format!("  {marker} {stage_id} - {title}"));
    }

    pub fn markdown(&mut self, source: &str) {
        let mut in_code = false;
        let mut pending_paragraph = Vec::new();

        for raw_line in source.trim().lines() {
            let line = raw_line.trim_end();
            if line.trim_start().starts_with("```") {
                self.flush_paragraph(&mut pending_paragraph);
                in_code = !in_code;
                continue;
            }

            if in_code {
                self.line(&format!("  {}", self.paint(line, "2")));
                continue;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                self.flush_paragraph(&mut pending_paragraph);
                self.blank_line();
                continue;
            }

            if let Some(title) = trimmed.strip_prefix("# ") {
                self.flush_paragraph(&mut pending_paragraph);
                self.line(&self.paint(title, "1;36"));
            } else if let Some(title) = trimmed.strip_prefix("## ") {
                self.flush_paragraph(&mut pending_paragraph);
                self.blank_line();
                self.line(&self.paint(title, "1;33"));
            } else if let Some(title) = trimmed.strip_prefix("### ") {
                self.flush_paragraph(&mut pending_paragraph);
                self.line(&self.paint(title, "1"));
            } else if let Some(item) = trimmed.strip_prefix("- ") {
                self.flush_paragraph(&mut pending_paragraph);
                self.line(&format!("  {} {}", self.paint("•", "36"), item));
            } else if is_numbered_item(trimmed) {
                self.flush_paragraph(&mut pending_paragraph);
                self.line(&format!("  {trimmed}"));
            } else {
                pending_paragraph.push(trimmed.to_string());
            }
        }

        self.flush_paragraph(&mut pending_paragraph);
    }

    pub fn blank_line(&mut self) {
        self.output.push('\n');
    }

    fn line(&mut self, value: &str) {
        self.output.push_str(value);
        self.output.push('\n');
    }

    fn flush_paragraph(&mut self, lines: &mut Vec<String>) {
        if lines.is_empty() {
            return;
        }
        let text = lines.join(" ");
        for line in wrap(&text, self.width) {
            self.line(&line);
        }
        lines.clear();
    }

    fn paint(&self, value: &str, code: &str) -> String {
        if self.color {
            format!("\x1b[{code}m{value}\x1b[0m")
        } else {
            value.to_string()
        }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

fn should_page() -> bool {
    std::io::stdin().is_terminal()
        && std::io::stdout().is_terminal()
        && std::env::var_os("DELTAFORGE_NO_PAGER").is_none()
}

fn page(output: &str) -> Result<()> {
    let pager = std::env::var("PAGER").unwrap_or_else(|_| "less -R".to_string());
    let mut parts = pager.split_whitespace();
    let Some(program) = parts.next() else {
        print!("{output}");
        return Ok(());
    };

    let mut child = match Command::new(program)
        .args(parts)
        .stdin(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => {
            print!("{output}");
            return Ok(());
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(output.as_bytes())
            .with_context(|| format!("failed to write output to pager {program}"))?;
    }

    let status = child
        .wait()
        .with_context(|| format!("failed to wait for pager {program}"))?;
    if !status.success() {
        print!("{output}");
    }
    Ok(())
}

fn is_numbered_item(value: &str) -> bool {
    let Some((number, rest)) = value.split_once(". ") else {
        return false;
    };
    !number.is_empty() && number.chars().all(|ch| ch.is_ascii_digit()) && !rest.is_empty()
}

fn wrap(value: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in value.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }
    lines
}
