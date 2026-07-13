use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::context::ProjectContext;
use crate::fs_util::atomic_write;
use crate::runner::{
    FixtureEntry, FixtureEntryKind, FixturePreviewKind, TestDiagnostic, TestInput, TestResult,
    TestRunSummary,
};

pub fn generate_test_report(
    context: &ProjectContext,
    summaries: &[TestRunSummary],
) -> Result<PathBuf> {
    let html = render_test_report(context, summaries);
    let output_dir = context.root.join(".deltaforge/ui");
    fs::create_dir_all(&output_dir).with_context(|| {
        format!(
            "failed to create test-report directory {}",
            output_dir.display()
        )
    })?;
    let output = output_dir.join("test-report.html");
    atomic_write(&output, &html)?;
    Ok(output)
}

fn render_test_report(context: &ProjectContext, summaries: &[TestRunSummary]) -> String {
    let passed: usize = summaries.iter().map(|summary| summary.passed).sum();
    let failed: usize = summaries.iter().map(|summary| summary.failed).sum();
    let total = passed + failed;
    let status = if failed == 0 { "passed" } else { "failed" };
    let headline = if failed == 0 {
        "Everything passed".to_string()
    } else if failed == 1 {
        "One test needs attention".to_string()
    } else {
        format!("{failed} tests need attention")
    };
    let summary_line = if failed == 0 {
        format!("{passed} of {total} tests passed")
    } else {
        format!("{passed} passed · {failed} failed · {total} run")
    };

    let mut navigation = String::new();
    let mut main = String::new();
    let mut first_failure = None;

    for summary in summaries {
        let stage_title = context
            .pack
            .manifest
            .stage(&summary.stage_id)
            .map(|stage| stage.title.as_str())
            .unwrap_or(&summary.stage_id);
        let stage_anchor = format!("stage-{}", safe_id(&summary.stage_id));
        let failed_results = summary
            .results
            .iter()
            .enumerate()
            .filter(|(_, result)| !result.passed)
            .collect::<Vec<_>>();
        let passed_results = summary
            .results
            .iter()
            .enumerate()
            .filter(|(_, result)| result.passed)
            .collect::<Vec<_>>();
        let failure_links = failed_results
            .iter()
            .map(|(index, result)| {
                let id = format!("result-{}-{index}", safe_id(&summary.stage_id));
                format!(
                    r##"<li><a href="#{id}"><span aria-hidden="true">×</span><span>{name}</span></a></li>"##,
                    id = escape_attr(&id),
                    name = escape_html(&result.name),
                )
            })
            .collect::<String>();
        let failure_navigation = if failure_links.is_empty() {
            String::new()
        } else {
            format!(r#"<ul class="failure-nav">{failure_links}</ul>"#)
        };
        let _ = write!(
            navigation,
            r##"<li><a href="#{stage_anchor}"><span>{stage_title}</span><small>{passed}/{total}</small></a>{failure_navigation}</li>"##,
            stage_anchor = escape_attr(&stage_anchor),
            stage_title = escape_html(stage_title),
            passed = summary.passed,
            total = summary.results.len(),
        );

        let _ = write!(
            main,
            r#"<section class="stage-block" id="{stage_anchor}">
<header class="stage-head">
  <div><p class="eyebrow">Stage {stage_id}</p><h2>{stage_title}</h2></div>
  <a class="text-link" href="learning.html#stage-{stage_id}">Read the instructions <span aria-hidden="true">→</span></a>
</header>"#,
            stage_anchor = escape_attr(&stage_anchor),
            stage_id = escape_attr(&summary.stage_id),
            stage_title = escape_html(stage_title),
        );

        if !failed_results.is_empty() {
            main.push_str(
                r#"<div class="result-group"><h3 class="group-title">Needs attention</h3>"#,
            );
            for (index, result) in failed_results {
                let id = format!("result-{}-{index}", safe_id(&summary.stage_id));
                first_failure.get_or_insert_with(|| id.clone());
                main.push_str(&render_failed_result(&summary.stage_id, result, &id));
            }
            main.push_str("</div>");
        }

        if !passed_results.is_empty() {
            main.push_str(r#"<div class="result-group passed-group"><h3 class="group-title">Passed</h3><div class="passed-list">"#);
            for (index, result) in passed_results {
                let id = format!("result-{}-{index}", safe_id(&summary.stage_id));
                main.push_str(&render_passed_result(&summary.stage_id, result, &id));
            }
            main.push_str("</div></div>");
        }

        main.push_str("</section>");
    }

    let first_failure_attr = first_failure.unwrap_or_default();
    format!(
        r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="color-scheme" content="light dark">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline'">
<title>Test report · {pack_name} · DeltaForge</title>
<style>{css}{input_css}</style>
</head>
<body class="status-{status}" data-first-failure="{first_failure}">
<a class="skip-link" href="#results">Skip to results</a>
<header class="topbar">
  <div class="brand"><span class="mark" aria-hidden="true">Δ</span><span><strong>DeltaForge</strong><small>Test report</small></span></div>
  <div class="project-meta"><span>{project}</span><span aria-hidden="true">/</span><span>{pack_name}</span><span aria-hidden="true">/</span><span>{language}</span></div>
</header>
<div class="shell">
  <aside class="sidebar">
    <p class="sidebar-label">Run summary</p>
    <div class="mini-summary"><strong>{passed}/{total}</strong><span>tests passed</span></div>
    <nav aria-label="Stages in this report"><ol>{navigation}</ol></nav>
  </aside>
  <main id="results">
    <header class="report-head">
      <p class="status-label"><span class="status-dot" aria-hidden="true"></span>{status_label}</p>
      <h1>{headline}</h1>
      <p class="summary-line">{summary_line}</p>
      <div class="report-actions">
        <button id="whitespace-toggle" class="button" type="button" aria-pressed="false">Show whitespace</button>
        <a class="button secondary" href="learning.html">Open learning guide</a>
      </div>
    </header>
    {main}
  </main>
</div>
<script>{js}</script>
</body>
</html>"##,
        pack_name = escape_html(&context.pack.manifest.name),
        project = escape_html(&context.state.project),
        language = escape_html(&context.state.language),
        status_label = if failed == 0 {
            "Run complete"
        } else {
            "Run failed"
        },
        first_failure = escape_attr(&first_failure_attr),
        css = CSS,
        input_css = INPUT_CSS,
        js = JS,
    )
}

fn render_failed_result(stage_id: &str, result: &TestResult, id: &str) -> String {
    let mut diagnostics = String::new();
    for diagnostic in &result.diagnostics {
        diagnostics.push_str(&render_diagnostic(diagnostic));
    }
    if diagnostics.is_empty() {
        for failure in &result.failures {
            let _ = write!(
                diagnostics,
                r#"<article class="diagnostic"><p class="diagnostic-kind">Mismatch</p><h4>Test expectation failed</h4><p>{failure}</p></article>"#,
                failure = escape_html(failure),
            );
        }
    }

    let output = render_streams(result);
    let input = render_test_input(result.input.as_ref());
    let expectations = render_expectations(&result.expectations);
    let rerun = rerun_command(stage_id, &result.name);
    let duration = duration_label(result.duration_ms);

    format!(
        r#"<article class="test-card failed" id="{id}" tabindex="-1">
<header class="test-head"><span class="result-icon" aria-hidden="true">×</span><div><h3>{name}</h3><p>{duration}</p></div></header>
<div class="tabs" data-tabs>
  <div class="tab-list" role="tablist" aria-label="Test details">
    <button id="{id}-tab-diagnosis" role="tab" aria-selected="true" aria-controls="{id}-panel-diagnosis" data-tab="diagnosis">Diagnosis</button>
    <button id="{id}-tab-input" role="tab" aria-selected="false" aria-controls="{id}-panel-input" data-tab="input" tabindex="-1">Test input</button>
    <button id="{id}-tab-output" role="tab" aria-selected="false" aria-controls="{id}-panel-output" data-tab="output" tabindex="-1">Program output</button>
    <button id="{id}-tab-contract" role="tab" aria-selected="false" aria-controls="{id}-panel-contract" data-tab="contract" tabindex="-1">Test contract</button>
  </div>
  <section id="{id}-panel-diagnosis" class="tab-panel" role="tabpanel" aria-labelledby="{id}-tab-diagnosis" data-panel="diagnosis">{diagnostics}</section>
  <section id="{id}-panel-input" class="tab-panel" role="tabpanel" aria-labelledby="{id}-tab-input" data-panel="input" hidden>{input}</section>
  <section id="{id}-panel-output" class="tab-panel" role="tabpanel" aria-labelledby="{id}-tab-output" data-panel="output" hidden>{output}</section>
  <section id="{id}-panel-contract" class="tab-panel" role="tabpanel" aria-labelledby="{id}-tab-contract" data-panel="contract" hidden>{expectations}</section>
</div>
<footer class="rerun"><div><span>Run only this test</span><code>{rerun}</code></div><button class="copy-command" type="button">Copy command</button></footer>
</article>"#,
        id = escape_attr(id),
        name = escape_html(&result.name),
        duration = escape_html(&duration),
        rerun = escape_html(&rerun),
    )
}

fn render_passed_result(stage_id: &str, result: &TestResult, id: &str) -> String {
    let rerun = rerun_command(stage_id, &result.name);
    format!(
        r#"<details class="passed-row" id="{id}"><summary><span class="result-icon" aria-hidden="true">✓</span><span>{name}</span><small>{duration}</small></summary><div class="passed-detail">{input}<h4>Test contract</h4>{expectations}<p><code>{rerun}</code></p></div></details>"#,
        id = escape_attr(id),
        name = escape_html(&result.name),
        duration = escape_html(&duration_label(result.duration_ms)),
        input = render_test_input(result.input.as_ref()),
        expectations = render_expectations(&result.expectations),
        rerun = escape_html(&rerun),
    )
}

fn render_diagnostic(diagnostic: &TestDiagnostic) -> String {
    let comparison = match (&diagnostic.expected, &diagnostic.actual) {
        (None, None) => String::new(),
        (expected, actual) => {
            let differing_line = match (expected, actual) {
                (Some(expected), Some(actual))
                    if matches!(diagnostic.kind, "stdout-exact" | "json" | "exit-code") =>
                {
                    first_differing_line(expected, actual)
                }
                _ => None,
            };
            let difference_note = differing_line.map_or_else(String::new, |line| {
                format!(r#"<p class="difference-note">First difference on line {line}</p>"#)
            });
            format!(
                r#"{difference_note}<div class="comparison">{expected}{actual}</div>"#,
                expected = expected.as_ref().map_or_else(String::new, |value| {
                    format!(
                        r#"<section><h5>Expected</h5>{}</section>"#,
                        output_block_highlight(value, differing_line)
                    )
                }),
                actual = actual.as_ref().map_or_else(String::new, |value| {
                    format!(
                        r#"<section><h5>Actual</h5>{}</section>"#,
                        output_block_highlight(value, differing_line)
                    )
                }),
            )
        }
    };
    format!(
        r#"<article class="diagnostic"><p class="diagnostic-kind">{kind}</p><h4>{title}</h4><p>{summary}</p>{comparison}</article>"#,
        kind = escape_html(&diagnostic.kind.replace('-', " ")),
        title = escape_html(&diagnostic.title),
        summary = escape_html(&diagnostic.summary),
    )
}

fn render_test_input(input: Option<&TestInput>) -> String {
    let Some(input) = input else {
        return r#"<div class="input-empty"><h4>Test setup unavailable</h4><p>The runner stopped before it could capture this test's inputs.</p></div>"#.to_string();
    };
    let command = input
        .command
        .iter()
        .map(|argument| shell_word(argument))
        .collect::<Vec<_>>()
        .join(" ");
    let stdin = input.stdin.as_ref().map_or_else(
        || r#"<div class="empty-input">No standard input</div>"#.to_string(),
        |value| output_block(value),
    );
    let environment = if input.env.is_empty() {
        r#"<div class="empty-input">No additional environment variables</div>"#.to_string()
    } else {
        let rows = input
            .env
            .iter()
            .map(|(key, value)| {
                format!(
                    "<tr><th>{}</th><td><code>{}</code></td></tr>",
                    escape_html(key),
                    escape_html(value)
                )
            })
            .collect::<String>();
        format!(r#"<table class="environment"><tbody>{rows}</tbody></table>"#)
    };
    let fixture = render_fixture(input);

    format!(
        r#"<div class="test-input">
<section class="input-intro"><h4>How this test starts</h4><p>This is the setup DeltaForge prepared before launching your program.</p></section>
<div class="input-grid">
  <section class="input-card command-input"><h5>Command</h5><code>{command}</code></section>
  <section class="input-card"><h5>Run settings</h5><dl><div><dt>Working directory</dt><dd><code>{working_directory}</code></dd></div><div><dt>Timeout</dt><dd>{timeout_ms} ms</dd></div></dl></section>
</div>
<section class="input-section"><h5>Standard input</h5>{stdin}</section>
<section class="input-section"><h5>Test environment</h5>{environment}</section>
{fixture}
</div>"#,
        command = escape_html(&command),
        working_directory = escape_html(&input.working_directory),
        timeout_ms = input.timeout_ms,
    )
}

fn render_fixture(input: &TestInput) -> String {
    let Some(snapshot) = &input.fixture else {
        return r#"<section class="input-section fixture-section"><h5>Starting files</h5><div class="empty-input">No fixture was declared. The test begins with an empty temporary workspace.</div></section>"#.to_string();
    };
    let mut root = FixtureTreeNode::default();
    for entry in &snapshot.entries {
        root.insert(entry);
    }
    let tree = render_fixture_nodes(&root.children, 0);
    let omitted = if snapshot.omitted_entries == 0 {
        String::new()
    } else {
        format!(
            r#"<p class="fixture-limit">{} more fixture entries are omitted to keep this report responsive.</p>"#,
            snapshot.omitted_entries
        )
    };
    let name = input.fixture_name.as_deref().unwrap_or("fixture");
    format!(
        r#"<section class="input-section fixture-section"><div class="fixture-heading"><div><h5>Starting files</h5><p><code>{name}</code> is copied into a fresh workspace for this test.</p></div><span>{count} entries shown</span></div><div class="fixture-browser"><ul class="fixture-tree">{tree}</ul></div>{omitted}</section>"#,
        name = escape_html(name),
        count = snapshot.entries.len(),
    )
}

#[derive(Default)]
struct FixtureTreeNode<'a> {
    children: BTreeMap<String, FixtureTreeNode<'a>>,
    entry: Option<&'a FixtureEntry>,
}

impl<'a> FixtureTreeNode<'a> {
    fn insert(&mut self, entry: &'a FixtureEntry) {
        let mut node = self;
        for part in entry.path.split('/') {
            node = node.children.entry(part.to_string()).or_default();
        }
        node.entry = Some(entry);
    }
}

fn render_fixture_nodes(nodes: &BTreeMap<String, FixtureTreeNode<'_>>, depth: usize) -> String {
    nodes
        .iter()
        .map(|(name, node)| {
            let is_file = node
                .entry
                .is_some_and(|entry| entry.kind == FixtureEntryKind::File);
            if is_file {
                render_fixture_file(name, node.entry.expect("file node has an entry"))
            } else {
                let children = render_fixture_nodes(&node.children, depth + 1);
                let open = if depth == 0 { " open" } else { "" };
                format!(
                    r#"<li class="fixture-directory"><details{open}><summary><span class="tree-icon" aria-hidden="true">▸</span><span>{name}</span></summary><ul>{children}</ul></details></li>"#,
                    name = escape_html(name),
                )
            }
        })
        .collect()
}

fn render_fixture_file(name: &str, entry: &FixtureEntry) -> String {
    let size = entry
        .size_bytes
        .map_or_else(|| "size unavailable".to_string(), format_file_size);
    let preview = match (&entry.preview, entry.preview_kind) {
        (Some(value), Some(kind)) => {
            let kind_label = if kind == FixturePreviewKind::Binary {
                "Binary preview (hexadecimal)"
            } else {
                "File contents"
            };
            let truncation = if entry.preview_truncated {
                r#"<p class="preview-note">Preview truncated to keep the report responsive.</p>"#
            } else {
                ""
            };
            format!(
                r#"<div class="file-preview"><p class="preview-label">{kind_label}</p>{output}{truncation}</div>"#,
                output = output_block(value),
            )
        }
        _ if entry.preview_truncated => r#"<div class="file-preview"><div class="empty-input">Preview omitted because the report's fixture budget was reached.</div></div>"#.to_string(),
        _ => r#"<div class="file-preview"><div class="empty-input">This file could not be previewed.</div></div>"#.to_string(),
    };
    format!(
        r#"<li class="fixture-file"><details><summary><span class="tree-icon" aria-hidden="true">·</span><span>{name}</span><small>{size}</small></summary>{preview}</details></li>"#,
        name = escape_html(name),
        size = escape_html(&size),
    )
}

fn format_file_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    }
}

fn render_streams(result: &TestResult) -> String {
    let stdout = result.report_stdout.as_deref().unwrap_or(&result.stdout);
    let stderr = result.report_stderr.as_deref().unwrap_or(&result.stderr);
    format!(
        r#"<div class="streams"><section><h4>Standard output</h4>{stdout}</section><section><h4>Standard error</h4>{stderr}</section><dl class="process-facts"><div><dt>Exit code</dt><dd>{exit}</dd></div><div><dt>Runtime</dt><dd>{duration}</dd></div></dl></div>"#,
        stdout = output_block(stdout),
        stderr = output_block(stderr),
        exit = result
            .actual_exit_code
            .map_or_else(|| "unavailable".to_string(), |code| code.to_string()),
        duration = escape_html(&duration_label(result.duration_ms)),
    )
}

fn render_expectations(expectations: &[String]) -> String {
    if expectations.is_empty() {
        return "<p>No structured expectations were recorded.</p>".to_string();
    }
    let items = expectations
        .iter()
        .map(|expectation| format!("<li>{}</li>", escape_html(expectation)))
        .collect::<String>();
    format!(r#"<div class="contract"><p>The test succeeds when:</p><ul>{items}</ul></div>"#)
}

fn output_block(value: &str) -> String {
    output_block_highlight(value, None)
}

fn output_block_highlight(value: &str, highlighted_line: Option<usize>) -> String {
    if value.is_empty() {
        return r#"<div class="empty-output">empty output</div>"#.to_string();
    }
    let display = truncate_for_report(value);
    format!(
        r#"<div class="output-pair"><pre class="output plain-output">{plain}</pre><pre class="output visible-output" hidden>{visible}</pre></div>"#,
        plain = render_output_lines(&display, highlighted_line, false),
        visible = render_output_lines(&display, highlighted_line, true),
    )
}

fn render_output_lines(value: &str, highlighted_line: Option<usize>, visible: bool) -> String {
    value
        .split_inclusive('\n')
        .enumerate()
        .map(|(index, line)| {
            let text = if visible {
                visible_whitespace(line)
            } else {
                line.to_string()
            };
            if highlighted_line == Some(index + 1) {
                format!(r#"<span class="diff-line">{}</span>"#, escape_html(&text))
            } else {
                escape_html(&text)
            }
        })
        .collect()
}

fn first_differing_line(expected: &str, actual: &str) -> Option<usize> {
    let expected_lines = expected.split('\n').collect::<Vec<_>>();
    let actual_lines = actual.split('\n').collect::<Vec<_>>();
    let length = expected_lines.len().max(actual_lines.len());
    (0..length)
        .find(|index| expected_lines.get(*index) != actual_lines.get(*index))
        .map(|index| index + 1)
}

fn truncate_for_report(value: &str) -> String {
    const LIMIT: usize = 20_000;
    if value.chars().count() <= LIMIT {
        return value.to_string();
    }
    let mut display = value.chars().take(LIMIT).collect::<String>();
    display.push_str(
        "\n[DeltaForge: report view truncated; use --verbose for full terminal output]\n",
    );
    display
}

fn visible_whitespace(value: &str) -> String {
    let mut visible = String::new();
    for ch in value.chars() {
        match ch {
            ' ' => visible.push('·'),
            '\t' => visible.push_str("→\t"),
            '\n' => visible.push_str("↵\n"),
            '\r' => visible.push('␍'),
            _ => visible.push(ch),
        }
    }
    visible
}

fn rerun_command(stage_id: &str, test_name: &str) -> String {
    format!(
        "deltaforge test --stage {} --filter {}",
        shell_word(stage_id),
        shell_word(test_name)
    )
}

fn shell_word(value: &str) -> String {
    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return value.to_string();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn duration_label(duration_ms: Option<u128>) -> String {
    duration_ms.map_or_else(
        || "runtime unavailable".to_string(),
        |duration| format!("{duration} ms"),
    )
}

fn safe_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                ch
            } else {
                '-'
            }
        })
        .collect()
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn escape_attr(value: &str) -> String {
    escape_html(value)
}

const CSS: &str = r#"
:root{color-scheme:light dark;--paper:#f4f1e9;--surface:#fffdf8;--ink:#20211f;--muted:#6c6c65;--line:#d7d1c4;--soft:#e9e4da;--fail:#a43c2f;--fail-bg:#f6e8e3;--pass:#2e684d;--pass-bg:#e5efe7;--code:#1f211f;--code-ink:#ecebe5;--focus:#1e5d78}
*{box-sizing:border-box}html{scroll-behavior:smooth}body{margin:0;background:var(--paper);color:var(--ink);font:16px/1.55 ui-sans-serif,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif}.skip-link{position:fixed;left:1rem;top:-5rem;z-index:50;background:var(--ink);color:var(--surface);padding:.65rem 1rem}.skip-link:focus{top:1rem}.topbar{height:68px;display:flex;align-items:center;justify-content:space-between;border-bottom:1px solid var(--line);padding:0 2rem;background:var(--surface);position:sticky;top:0;z-index:20}.brand{display:flex;gap:.7rem;align-items:center}.brand .mark{font:700 1.45rem Georgia,serif}.brand strong,.brand small{display:block;line-height:1.1}.brand small{color:var(--muted);font-size:.72rem;text-transform:uppercase;letter-spacing:.12em;margin-top:.25rem}.project-meta{display:flex;gap:.55rem;color:var(--muted);font:13px ui-monospace,SFMono-Regular,Consolas,monospace}.shell{display:grid;grid-template-columns:250px minmax(0,1fr);max-width:1500px;margin:0 auto}.sidebar{padding:2.3rem 1.5rem;border-right:1px solid var(--line);min-height:calc(100vh - 68px);position:sticky;top:68px;align-self:start}.sidebar-label,.eyebrow,.diagnostic-kind,.status-label{font-size:.72rem;text-transform:uppercase;letter-spacing:.13em;color:var(--muted);font-weight:700}.mini-summary{padding:1rem 0 1.6rem;border-bottom:1px solid var(--line);display:flex;align-items:baseline;gap:.5rem}.mini-summary strong{font:600 2rem Georgia,serif}.mini-summary span{color:var(--muted)}.sidebar ol{list-style:none;padding:1rem 0;margin:0}.sidebar a{display:flex;justify-content:space-between;gap:1rem;color:var(--ink);text-decoration:none;padding:.65rem 0;border-bottom:1px solid var(--soft)}.sidebar a:hover{text-decoration:underline}.sidebar small{color:var(--muted)}.sidebar .failure-nav{padding:.25rem 0 .85rem .65rem}.sidebar .failure-nav a{justify-content:flex-start;gap:.45rem;border:0;padding:.28rem 0;color:var(--fail);font-size:.78rem;line-height:1.3}main{min-width:0;padding:3.5rem clamp(1.4rem,5vw,5.5rem) 7rem}.report-head{max-width:900px;padding-bottom:3rem;border-bottom:2px solid var(--ink)}.status-label{display:flex;align-items:center;gap:.5rem}.status-dot{width:.58rem;height:.58rem;border-radius:50%;background:var(--fail)}.status-passed .status-dot{background:var(--pass)}h1{font:500 clamp(2.5rem,5vw,5rem)/1.02 Georgia,"Times New Roman",serif;letter-spacing:-.035em;margin:.75rem 0 1rem;max-width:850px}.summary-line{font-size:1.12rem;color:var(--muted)}.report-actions{display:flex;gap:.7rem;margin-top:1.8rem}.button{appearance:none;border:1px solid var(--ink);background:var(--ink);color:var(--surface);padding:.68rem .9rem;font:600 .8rem ui-sans-serif,sans-serif;text-decoration:none;cursor:pointer}.button.secondary{background:transparent;color:var(--ink)}.stage-block{padding:3.5rem 0 1rem;scroll-margin-top:90px}.stage-head{display:flex;justify-content:space-between;gap:2rem;align-items:flex-end;margin-bottom:1.8rem}.stage-head h2{font:500 clamp(1.8rem,3vw,2.7rem)/1.12 Georgia,serif;margin:.25rem 0}.eyebrow{margin:0}.text-link{color:var(--ink);font-weight:650;text-underline-offset:.2em;padding-bottom:.4rem}.result-group{margin-top:1.8rem}.group-title{font-size:.8rem;text-transform:uppercase;letter-spacing:.12em;margin:0 0 .8rem}.test-card{background:var(--surface);border:1px solid var(--line);margin-bottom:1.25rem;scroll-margin-top:90px}.test-card.failed{border-top:4px solid var(--fail)}.test-card:focus{outline:3px solid var(--focus);outline-offset:3px}.test-head{display:flex;align-items:flex-start;gap:.9rem;padding:1.2rem 1.35rem;border-bottom:1px solid var(--line)}.test-head h3{margin:0;font-size:1.05rem}.test-head p{margin:.2rem 0 0;color:var(--muted);font-size:.82rem}.result-icon{display:inline-grid;place-items:center;width:1.55rem;height:1.55rem;border-radius:50%;font-weight:800;background:var(--fail-bg);color:var(--fail);flex:0 0 auto}.tab-list{display:flex;border-bottom:1px solid var(--line);padding:0 1.35rem;gap:1.4rem}.tab-list button{appearance:none;border:0;border-bottom:3px solid transparent;background:transparent;padding:.85rem 0 .7rem;color:var(--muted);font-weight:650;cursor:pointer}.tab-list button[aria-selected=true]{color:var(--ink);border-color:var(--ink)}.tab-panel{padding:1.35rem}.diagnostic{padding:0 0 1.4rem;margin-bottom:1.4rem;border-bottom:1px solid var(--soft)}.diagnostic:last-child{border-bottom:0;margin-bottom:0;padding-bottom:0}.diagnostic-kind{margin:0 0 .35rem;color:var(--fail)}.diagnostic h4{font:600 1.35rem Georgia,serif;margin:0 0 .35rem}.diagnostic>p:last-of-type{margin:.3rem 0;color:var(--muted)}.difference-note{display:inline-block!important;background:var(--fail-bg);color:var(--fail)!important;padding:.25rem .5rem;margin:.65rem 0 0!important;font:700 .72rem ui-monospace,monospace}.comparison{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:1px;background:var(--line);border:1px solid var(--line);margin-top:1rem}.comparison section{background:var(--surface);min-width:0}.comparison h5,.streams h4{margin:0;padding:.55rem .75rem;background:var(--soft);font-size:.72rem;text-transform:uppercase;letter-spacing:.1em}.output{margin:0;background:var(--code);color:var(--code-ink);padding:1rem;min-height:4rem;max-height:360px;overflow:auto;font:13px/1.55 ui-monospace,SFMono-Regular,Consolas,monospace;white-space:pre-wrap;overflow-wrap:anywhere}.diff-line{display:block;background:#53332d;margin:0 -.35rem;padding:0 .35rem;border-left:3px solid #e98c76}.empty-output{padding:1rem;background:var(--soft);color:var(--muted);font-style:italic}.streams{display:grid;gap:1rem}.process-facts{display:flex;gap:2rem;margin:.5rem 0}.process-facts div{display:flex;gap:.5rem}.process-facts dt{color:var(--muted)}.process-facts dd{margin:0;font-family:ui-monospace,monospace}.contract p{margin-top:0}.contract li{margin:.35rem 0}.rerun{border-top:1px solid var(--line);padding:1rem 1.35rem;display:flex;justify-content:space-between;align-items:center;gap:1rem;background:var(--soft)}.rerun span{display:block;color:var(--muted);font-size:.75rem;margin-bottom:.25rem}.rerun code,.passed-detail code{font:12px ui-monospace,SFMono-Regular,Consolas,monospace}.copy-command{border:1px solid var(--ink);background:transparent;padding:.5rem .7rem;cursor:pointer}.passed-list{border-top:1px solid var(--line)}.passed-row{border-bottom:1px solid var(--line);background:var(--surface)}.passed-row summary{display:grid;grid-template-columns:auto 1fr auto;align-items:center;gap:.75rem;padding:.85rem 1rem;cursor:pointer;list-style:none}.passed-row summary::-webkit-details-marker{display:none}.passed-row .result-icon{background:var(--pass-bg);color:var(--pass)}.passed-row small{color:var(--muted)}.passed-detail{padding:0 1rem 1rem 3.3rem;color:var(--muted)}button:focus-visible,a:focus-visible,summary:focus-visible{outline:3px solid var(--focus);outline-offset:3px}
@media(max-width:800px){.topbar{padding:0 1rem}.project-meta{display:none}.shell{display:block}.sidebar{position:static;min-height:0;border-right:0;border-bottom:1px solid var(--line);padding:1rem}.sidebar nav{display:none}main{padding:2rem 1rem 5rem}.stage-head{align-items:flex-start;flex-direction:column;gap:.5rem}.comparison{grid-template-columns:1fr}.report-actions{flex-wrap:wrap}.rerun{align-items:flex-start;flex-direction:column}.copy-command{width:100%}}
@media(prefers-reduced-motion:reduce){html{scroll-behavior:auto}}
@media print{.topbar,.sidebar,.report-actions,.tab-list,.rerun{display:none!important}.shell{display:block}main{padding:0}.tab-panel[hidden]{display:block}.test-card{break-inside:avoid}}
"#;

const INPUT_CSS: &str = r#"
.test-input{display:grid;gap:1.4rem}.input-intro h4{font:600 1.35rem Georgia,serif;margin:0 0 .25rem}.input-intro p,.fixture-heading p{margin:.25rem 0;color:var(--muted)}.input-grid{display:grid;grid-template-columns:minmax(0,1.5fr) minmax(220px,.7fr);gap:1px;background:var(--line);border:1px solid var(--line)}.input-card{background:var(--surface);padding:1rem;min-width:0}.input-card h5,.input-section>h5,.fixture-heading h5,.passed-detail>h4{font-size:.72rem;text-transform:uppercase;letter-spacing:.1em;margin:0 0 .7rem}.command-input code{display:block;background:var(--code);color:var(--code-ink);padding:1rem;overflow:auto;white-space:pre-wrap;overflow-wrap:anywhere;font:13px/1.55 ui-monospace,SFMono-Regular,Consolas,monospace}.input-card dl{margin:0}.input-card dl div{display:flex;justify-content:space-between;gap:1rem;padding:.35rem 0;border-bottom:1px solid var(--soft)}.input-card dl div:last-child{border-bottom:0}.input-card dt{color:var(--muted)}.input-card dd{margin:0;text-align:right}.input-section{border-top:1px solid var(--line);padding-top:1.2rem}.empty-input,.input-empty{padding:1rem;background:var(--soft);color:var(--muted)}.environment{width:100%;border-collapse:collapse}.environment th,.environment td{text-align:left;padding:.55rem .7rem;border:1px solid var(--line)}.environment th{width:30%;font:600 12px ui-monospace,SFMono-Regular,Consolas,monospace}.fixture-heading{display:flex;align-items:flex-end;justify-content:space-between;gap:1rem;margin-bottom:.8rem}.fixture-heading h5{margin-bottom:.2rem}.fixture-heading>span{font-size:.75rem;color:var(--muted)}.fixture-browser{border:1px solid var(--line);background:var(--surface);max-height:520px;overflow:auto;padding:.45rem 0}.fixture-tree,.fixture-tree ul{list-style:none;margin:0;padding:0}.fixture-tree ul{padding-left:1.15rem;border-left:1px solid var(--soft);margin-left:1rem}.fixture-tree details>summary{display:flex;align-items:center;gap:.45rem;min-height:2rem;padding:.25rem .75rem;cursor:pointer}.fixture-tree details>summary:hover{background:var(--soft)}.fixture-tree summary::marker{content:""}.fixture-tree small{margin-left:auto;color:var(--muted);font-size:.72rem}.tree-icon{display:inline-block;width:.75rem;color:var(--muted);font-family:ui-monospace,monospace}.fixture-directory>details[open]>summary .tree-icon{transform:rotate(90deg)}.fixture-file>details[open]>summary{background:var(--soft);font-weight:650}.file-preview{margin:.25rem .75rem 1rem 1.95rem;border:1px solid var(--line)}.file-preview .output{max-height:300px}.preview-label,.preview-note{margin:0;padding:.45rem .7rem;background:var(--soft);color:var(--muted);font-size:.72rem}.preview-note{border-top:1px solid var(--line)}.fixture-limit{color:var(--muted);font-size:.8rem;margin:.6rem 0 0}.passed-detail .test-input{color:var(--ink);margin:1rem 0 1.5rem}.passed-detail>h4{color:var(--ink);margin-top:1.5rem}
@media(max-width:800px){.input-grid{grid-template-columns:1fr}.fixture-heading{align-items:flex-start;flex-direction:column}.fixture-browser{max-height:420px}.file-preview{margin-left:.75rem}}
"#;

const JS: &str = r#"
document.querySelectorAll('[data-tabs]').forEach((tabs)=>{
  const buttons=[...tabs.querySelectorAll('[role="tab"]')];
  const panels=[...tabs.querySelectorAll('[data-panel]')];
  const select=(button)=>{
    buttons.forEach((candidate)=>{
      const selected=candidate===button;
      candidate.setAttribute('aria-selected',String(selected));
      candidate.tabIndex=selected?0:-1;
    });
    panels.forEach((panel)=>{panel.hidden=panel.dataset.panel!==button.dataset.tab;});
  };
  buttons.forEach((button,index)=>{
    button.addEventListener('click',()=>select(button));
    button.addEventListener('keydown',(event)=>{
      let next;
      if(event.key==='ArrowRight')next=(index+1)%buttons.length;
      if(event.key==='ArrowLeft')next=(index-1+buttons.length)%buttons.length;
      if(event.key==='Home')next=0;
      if(event.key==='End')next=buttons.length-1;
      if(next===undefined)return;
      event.preventDefault();select(buttons[next]);buttons[next].focus();
    });
  });
});
const whitespace=document.getElementById('whitespace-toggle');
whitespace.addEventListener('click',()=>{
  const showing=whitespace.getAttribute('aria-pressed')!=='true';
  whitespace.setAttribute('aria-pressed',String(showing));
  whitespace.textContent=showing?'Hide whitespace':'Show whitespace';
  document.querySelectorAll('.plain-output').forEach((item)=>{item.hidden=showing;});
  document.querySelectorAll('.visible-output').forEach((item)=>{item.hidden=!showing;});
});
document.querySelectorAll('.copy-command').forEach((button)=>button.addEventListener('click',async()=>{
  const command=button.closest('.rerun').querySelector('code').textContent;
  try{await navigator.clipboard.writeText(command);button.textContent='Copied';}
  catch{
    const range=document.createRange();range.selectNodeContents(button.closest('.rerun').querySelector('code'));
    const selection=getSelection();selection.removeAllRanges();selection.addRange(range);button.textContent='Selected';
  }
  setTimeout(()=>{button.textContent='Copy command';},1400);
}));
const first=document.body.dataset.firstFailure;
if(first){const target=document.getElementById(first);if(target)target.focus({preventScroll:true});}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_markers_make_line_endings_and_tabs_visible() {
        assert_eq!(visible_whitespace("a b\t\r\n"), "a·b→\t␍↵\n");
    }

    #[test]
    fn shell_words_quote_test_names() {
        assert_eq!(shell_word("simple_name"), "simple_name");
        assert_eq!(shell_word("finds user's file"), "'finds user'\\''s file'");
    }

    #[test]
    fn report_output_is_bounded() {
        let output = truncate_for_report(&"x".repeat(21_000));
        assert!(output.contains("report view truncated"));
        assert!(output.len() < 21_000);
    }

    #[test]
    fn first_difference_uses_human_line_numbers() {
        assert_eq!(
            first_differing_line("same\nleft\n", "same\nright\n"),
            Some(2)
        );
        assert_eq!(first_differing_line("same", "same"), None);
    }

    #[test]
    fn input_view_shows_stdin_environment_and_empty_workspace() {
        let input = TestInput {
            command: vec!["tinyhttp".to_string(), "serve".to_string()],
            stdin: Some("GET / HTTP/1.1\r\n\r\n".to_string()),
            env: BTreeMap::from([("MODE".to_string(), "strict".to_string())]),
            timeout_ms: 750,
            working_directory: "{project_root}".to_string(),
            fixture_name: None,
            fixture: None,
        };

        let html = render_test_input(Some(&input));

        assert!(html.contains("tinyhttp serve"));
        assert!(html.contains("GET / HTTP/1.1"));
        assert!(html.contains("␍↵"));
        assert!(html.contains("MODE"));
        assert!(html.contains("strict"));
        assert!(html.contains("750 ms"));
        assert!(html.contains("empty temporary workspace"));
    }
}
