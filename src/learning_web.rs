use std::fs;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result};

use crate::context::ProjectContext;
use crate::fs_util::atomic_write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitialView<'a> {
    Overview,
    Stage(&'a str),
}

pub fn should_use_browser(terminal_requested: bool) -> bool {
    !terminal_requested
        && std::env::var_os("DELTAFORGE_NO_BROWSER").is_none()
        && std::io::stdin().is_terminal()
        && std::io::stdout().is_terminal()
}

pub fn generate_learning_page(
    context: &ProjectContext,
    overview: &str,
    initial_view: InitialView<'_>,
) -> Result<PathBuf> {
    let stages = context
        .pack
        .manifest
        .stages
        .iter()
        .map(|stage| {
            let path = context.pack.instructions_path(stage);
            let source = context.pack.read_stage_file(&path)?;
            let status = if context.state.is_completed(&stage.id) {
                "complete"
            } else if stage.id == context.state.current_stage {
                "current"
            } else {
                "upcoming"
            };
            let title = stage_display_title(&source, &stage.title);
            Ok(StagePage {
                id: stage.id.clone(),
                title,
                status,
                source,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let html = render_learning_page(
        &context.pack.manifest.name,
        &context.state.project,
        &context.state.language,
        overview,
        &stages,
        initial_view,
    );
    let output_dir = context.root.join(".deltaforge/ui");
    fs::create_dir_all(&output_dir).with_context(|| {
        format!(
            "failed to create learning-page directory {}",
            output_dir.display()
        )
    })?;
    let output = output_dir.join("learning.html");
    atomic_write(&output, &html)?;
    Ok(output)
}

pub fn open_learning_page(path: &Path) -> Result<()> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("failed to resolve learning page {}", path.display()))?;
    let mut command = browser_command(&canonical)?;
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
        .spawn()
        .with_context(|| format!("failed to open learning page {}", canonical.display()))?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn browser_command(path: &Path) -> Result<Command> {
    let mut command = Command::new("open");
    command.arg(path);
    Ok(command)
}

#[cfg(target_os = "linux")]
fn browser_command(path: &Path) -> Result<Command> {
    let mut command = Command::new("xdg-open");
    command.arg(path);
    Ok(command)
}

#[cfg(windows)]
fn browser_command(path: &Path) -> Result<Command> {
    let mut command = Command::new("rundll32");
    command.arg("url.dll,FileProtocolHandler").arg(path);
    Ok(command)
}

#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
fn browser_command(_path: &Path) -> Result<Command> {
    anyhow::bail!("opening a browser is not supported on this operating system")
}

struct StagePage {
    id: String,
    title: String,
    status: &'static str,
    source: String,
}

fn render_learning_page(
    pack_name: &str,
    project: &str,
    language: &str,
    overview: &str,
    stages: &[StagePage],
    initial_view: InitialView<'_>,
) -> String {
    let initial_target = match initial_view {
        InitialView::Overview => "overview".to_string(),
        InitialView::Stage(stage) => format!("stage-{stage}"),
    };
    let completed = stages
        .iter()
        .filter(|stage| stage.status == "complete")
        .count();
    let progress = if stages.is_empty() {
        0
    } else {
        completed * 100 / stages.len()
    };
    let mut sidebar = String::new();
    for (index, stage) in stages.iter().enumerate() {
        let marker = match stage.status {
            "complete" => "✓",
            "current" => "→",
            _ => "·",
        };
        sidebar.push_str(&format!(
            r#"<button class="stage-link {status}" data-target="stage-{id}" aria-label="Open stage {number}: {title}"><span class="stage-marker">{marker}</span><span><small>Stage {number}</small>{title}</span></button>"#,
            status = stage.status,
            id = escape_attr(&stage.id),
            number = index + 1,
            title = escape_html(&stage.title),
        ));
    }

    let overview_panel = render_overview_panel(overview, stages);
    let stage_panels = stages
        .iter()
        .enumerate()
        .map(|(index, stage)| render_stage_panel(stage, index, stages))
        .collect::<String>();

    format!(
        r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline'; img-src data:">
<title>{pack_name} · DeltaForge</title>
<style>{CSS}</style>
</head>
<body data-initial-target="{initial_target}">
<a class="skip-link" href="#main-content">Skip to content</a>
<div class="app-shell">
  <aside id="mobile-drawer" class="sidebar" aria-label="Course navigation">
    <div class="brand"><span class="brand-mark">Δ</span><span><strong>DeltaForge</strong><small>{pack_name}</small></span></div>
    <button class="overview-link" data-target="overview"><span>⌂</span> Project overview</button>
    <div class="progress-card"><div><span>Your progress</span><strong>{completed}/{total}</strong></div><div class="progress-track"><span style="width:{progress}%"></span></div></div>
    <nav class="stage-nav" aria-label="Stages">{sidebar}</nav>
    <div class="sidebar-meta"><span>{project}</span><span>{language}</span></div>
  </aside>
  <main id="main-content" class="main-content">
    <header class="mobile-header"><button id="menu-button" aria-expanded="false" aria-controls="mobile-drawer">☰</button><strong>{pack_name}</strong></header>
    {overview_panel}
    {stage_panels}
  </main>
</div>
<div id="toast" role="status" aria-live="polite"></div>
<script>{JS}</script>
</body>
</html>"##,
        pack_name = escape_html(pack_name),
        project = escape_html(project),
        language = escape_html(language),
        total = stages.len(),
        initial_target = escape_attr(&initial_target),
    )
}

fn render_overview_panel(overview: &str, stages: &[StagePage]) -> String {
    let sections = split_h2_sections(overview);
    let start = select_sections(
        &sections,
        &["What you are building", "Why this is useful", "Big picture"],
    );
    let practice = select_sections(&sections, &["Failure-analysis lab"]);
    let explore = select_sections(&sections, &["Optional extensions"]);
    let reference = sections
        .iter()
        .filter(|(title, _)| {
            !matches!(
                title.as_str(),
                "What you are building"
                    | "Why this is useful"
                    | "Big picture"
                    | "Failure-analysis lab"
                    | "Optional extensions"
                    | "What good looks like"
            )
        })
        .map(|(title, body)| format!("## {title}\n\n{body}"))
        .collect::<Vec<_>>()
        .join("\n\n");
    let good = select_sections(&sections, &["What good looks like"]);
    let roadmap = stages
        .iter()
        .enumerate()
        .map(|(index, stage)| {
            let summary = stage_summary(&stage.source);
            format!(
                r#"<article class="roadmap-card {status}"><span class="roadmap-number">{number}</span><div><small>{label}</small><h3>{title}</h3><p>{summary}</p></div></article>"#,
                status = stage.status,
                number = index + 1,
                label = status_label(stage.status),
                title = escape_html(&stage.title),
                summary = escape_html(&summary),
            )
        })
        .collect::<String>();
    format!(
        r#"<section id="overview" class="page-panel" hidden>
<div class="page-width">
  <div class="eyebrow">Project overview</div>
  <h1>Start with the big idea</h1>
  <p class="lede">See what you are building, why each step exists, and how the stages fit together. You can return here whenever the details start to feel disconnected.</p>
  {good}
  <div class="tabs" role="tablist" aria-label="Overview sections">
    <button role="tab" aria-selected="true" data-tab="overview-start">Start here</button>
    <button role="tab" aria-selected="false" data-tab="overview-roadmap">Roadmap</button>
    <button role="tab" aria-selected="false" data-tab="overview-practice">Practice</button>
    <button role="tab" aria-selected="false" data-tab="overview-reference">Reference</button>
  </div>
  <div id="overview-start" class="tab-panel active">{start}</div>
  <div id="overview-roadmap" class="tab-panel"><div class="roadmap-grid">{roadmap}</div></div>
  <div id="overview-practice" class="tab-panel">{practice}{explore}</div>
  <div id="overview-reference" class="tab-panel">{reference}</div>
</div></section>"#,
        start = markdown_to_html(&start),
        good = callout("What good looks like", &plain_text(&good), "success"),
        practice = markdown_to_html(&practice),
        explore = markdown_to_html(&explore),
        reference = markdown_to_html(&reference),
    )
}

fn render_stage_panel(stage: &StagePage, index: usize, stages: &[StagePage]) -> String {
    let sections = split_h2_sections(&stage.source);
    let goal = select_sections(&sections, &["Goal"]);
    let background = select_sections(&sections, &["Background"]);
    let requirements = select_sections(&sections, &["Requirements", "Edge cases"]);
    let example = select_sections(&sections, &["Example"]);
    let success = select_sections(&sections, &["Success criteria"]);
    let non_goals = select_sections(&sections, &["Non-goals"]);
    let previous = index
        .checked_sub(1)
        .and_then(|previous| stages.get(previous))
        .map_or("The project overview".to_string(), |stage| {
            stage.title.clone()
        });
    let next = stages
        .get(index + 1)
        .map_or("The finished project".to_string(), |stage| {
            stage.title.clone()
        });
    let status = status_label(stage.status);
    format!(
        r#"<section id="stage-{id}" class="page-panel" hidden>
<div class="page-width">
  <div class="stage-heading"><div><div class="eyebrow">Stage {number} · {status}</div><h1>{title}</h1><p class="lede">{summary}</p></div><span class="status-pill {status_class}">{status}</span></div>
  <div class="orientation-grid">
    <article><small>You already have</small><strong>{previous}</strong></article>
    <article class="accent"><small>This stage adds</small><strong>{summary}</strong></article>
    <article><small>Coming next</small><strong>{next}</strong></article>
  </div>
  <div class="tabs" role="tablist" aria-label="Stage {number} sections">
    <button role="tab" aria-selected="true" data-tab="{id}-task">Task</button>
    <button role="tab" aria-selected="false" data-tab="{id}-example">Example</button>
    <button role="tab" aria-selected="false" data-tab="{id}-why">Why?</button>
    <button role="tab" aria-selected="false" data-tab="{id}-reference">Reference</button>
  </div>
  <div id="{id}-task" class="tab-panel active">{goal}{requirements}{success}</div>
  <div id="{id}-example" class="tab-panel">{example}</div>
  <div id="{id}-why" class="tab-panel">{background}</div>
  <div id="{id}-reference" class="tab-panel">{non_goals}</div>
</div></section>"#,
        id = escape_attr(&stage.id),
        number = index + 1,
        status = status,
        status_class = stage.status,
        title = escape_html(&stage.title),
        summary = escape_html(&stage_summary(&stage.source)),
        previous = escape_html(&previous),
        next = escape_html(&next),
        goal = markdown_to_html(&goal),
        requirements = markdown_to_html(&requirements),
        success = markdown_to_html(&success),
        example = markdown_to_html(&example),
        background = markdown_to_html(&background),
        non_goals = markdown_to_html(&non_goals),
    )
}

fn status_label(status: &str) -> &'static str {
    match status {
        "complete" => "Complete",
        "current" => "Current",
        _ => "Later",
    }
}

fn split_h2_sections(source: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut title: Option<String> = None;
    let mut body = Vec::new();
    for line in source.lines() {
        if let Some(next) = line.strip_prefix("## ") {
            if let Some(title) = title.take() {
                sections.push((title, body.join("\n").trim().to_string()));
                body.clear();
            }
            title = Some(next.trim().to_string());
        } else if title.is_some() {
            body.push(line);
        }
    }
    if let Some(title) = title {
        sections.push((title, body.join("\n").trim().to_string()));
    }
    sections
}

fn select_sections(sections: &[(String, String)], names: &[&str]) -> String {
    names
        .iter()
        .filter_map(|name| {
            sections
                .iter()
                .find(|(title, _)| title.eq_ignore_ascii_case(name))
                .map(|(title, body)| format!("## {title}\n\n{body}"))
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn stage_summary(source: &str) -> String {
    let sections = split_h2_sections(source);
    let goal = sections
        .iter()
        .find(|(title, _)| title.eq_ignore_ascii_case("Goal"))
        .map(|(_, body)| body.as_str())
        .unwrap_or("Learn the next project behavior.");
    first_paragraph(goal)
}

fn stage_display_title(source: &str, fallback: &str) -> String {
    let Some(heading) = source
        .lines()
        .find_map(|line| line.strip_prefix("# ").map(str::trim))
    else {
        return fallback.to_string();
    };
    heading
        .split_once('—')
        .map(|(_, title)| title.trim().to_string())
        .filter(|title| !title.is_empty())
        .unwrap_or_else(|| heading.to_string())
}

fn first_paragraph(source: &str) -> String {
    source
        .split("\n\n")
        .find(|paragraph| !paragraph.trim().is_empty())
        .map(plain_text)
        .unwrap_or_default()
}

fn plain_text(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(|line| line.trim().trim_start_matches("- ").replace(['`', '*'], ""))
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn markdown_to_html(source: &str) -> String {
    let lines = source.lines().collect::<Vec<_>>();
    let mut html = String::new();
    let mut index = 0;
    let mut paragraph = Vec::new();
    let mut list: Option<&str> = None;
    let mut in_code = false;
    let mut code = String::new();

    while index < lines.len() {
        let raw = lines[index];
        let trimmed = raw.trim();
        if trimmed.starts_with("```") {
            flush_paragraph_html(&mut html, &mut paragraph);
            close_list(&mut html, &mut list);
            if in_code {
                html.push_str(&format!(
                    "<div class=\"code-block\"><button class=\"copy-button\" type=\"button\">Copy</button><pre><code>{}</code></pre></div>",
                    escape_html(code.trim_end())
                ));
                code.clear();
            }
            in_code = !in_code;
            index += 1;
            continue;
        }
        if in_code {
            code.push_str(raw);
            code.push('\n');
            index += 1;
            continue;
        }
        if trimmed.starts_with('|') && trimmed.ends_with('|') {
            flush_paragraph_html(&mut html, &mut paragraph);
            close_list(&mut html, &mut list);
            let mut rows = Vec::new();
            while index < lines.len() {
                let line = lines[index].trim();
                if !(line.starts_with('|') && line.ends_with('|')) {
                    break;
                }
                rows.push(
                    line.trim_matches('|')
                        .split('|')
                        .map(str::trim)
                        .collect::<Vec<_>>(),
                );
                index += 1;
            }
            render_table(&mut html, &rows);
            continue;
        }
        if trimmed.is_empty() {
            flush_paragraph_html(&mut html, &mut paragraph);
            close_list(&mut html, &mut list);
            index += 1;
            continue;
        }
        if let Some(title) = trimmed.strip_prefix("### ") {
            flush_paragraph_html(&mut html, &mut paragraph);
            close_list(&mut html, &mut list);
            html.push_str(&format!("<h3>{}</h3>", inline_html(title)));
        } else if let Some(title) = trimmed.strip_prefix("## ") {
            flush_paragraph_html(&mut html, &mut paragraph);
            close_list(&mut html, &mut list);
            html.push_str(&format!("<h2>{}</h2>", inline_html(title)));
        } else if let Some(title) = trimmed.strip_prefix("# ") {
            flush_paragraph_html(&mut html, &mut paragraph);
            close_list(&mut html, &mut list);
            html.push_str(&format!("<h1>{}</h1>", inline_html(title)));
        } else if let Some(item) = trimmed.strip_prefix("- ") {
            flush_paragraph_html(&mut html, &mut paragraph);
            ensure_list(&mut html, &mut list, "ul");
            html.push_str(&format!("<li>{}</li>", inline_html(item)));
        } else if let Some(item) = numbered_item(trimmed) {
            flush_paragraph_html(&mut html, &mut paragraph);
            ensure_list(&mut html, &mut list, "ol");
            html.push_str(&format!("<li>{}</li>", inline_html(item)));
        } else {
            close_list(&mut html, &mut list);
            paragraph.push(trimmed);
        }
        index += 1;
    }
    flush_paragraph_html(&mut html, &mut paragraph);
    close_list(&mut html, &mut list);
    html
}

fn render_table(html: &mut String, rows: &[Vec<&str>]) {
    if rows.len() < 2 {
        return;
    }
    html.push_str("<div class=\"table-wrap\"><table><thead><tr>");
    for cell in &rows[0] {
        html.push_str(&format!("<th>{}</th>", inline_html(cell)));
    }
    html.push_str("</tr></thead><tbody>");
    for row in rows.iter().skip(2) {
        html.push_str("<tr>");
        for cell in row {
            html.push_str(&format!("<td>{}</td>", inline_html(cell)));
        }
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");
}

fn numbered_item(value: &str) -> Option<&str> {
    let (number, item) = value.split_once(". ")?;
    (!number.is_empty() && number.chars().all(|ch| ch.is_ascii_digit())).then_some(item)
}

fn ensure_list<'a>(html: &mut String, list: &mut Option<&'a str>, kind: &'a str) {
    if *list == Some(kind) {
        return;
    }
    close_list(html, list);
    html.push_str(&format!("<{kind}>"));
    *list = Some(kind);
}

fn close_list(html: &mut String, list: &mut Option<&str>) {
    if let Some(kind) = list.take() {
        html.push_str(&format!("</{kind}>"));
    }
}

fn flush_paragraph_html(html: &mut String, paragraph: &mut Vec<&str>) {
    if paragraph.is_empty() {
        return;
    }
    html.push_str(&format!("<p>{}</p>", inline_html(&paragraph.join(" "))));
    paragraph.clear();
}

fn inline_html(source: &str) -> String {
    let mut html = String::new();
    let mut chars = source.chars().peekable();
    let mut code = false;
    let mut strong = false;
    while let Some(ch) = chars.next() {
        if ch == '`' {
            html.push_str(if code { "</code>" } else { "<code>" });
            code = !code;
        } else if ch == '*' && chars.peek() == Some(&'*') {
            chars.next();
            html.push_str(if strong { "</strong>" } else { "<strong>" });
            strong = !strong;
        } else {
            html.push_str(&escape_html(&ch.to_string()));
        }
    }
    if code {
        html.push_str("</code>");
    }
    if strong {
        html.push_str("</strong>");
    }
    html
}

fn callout(title: &str, body: &str, kind: &str) -> String {
    format!(
        "<aside class=\"callout {kind}\"><strong>{}</strong><p>{}</p></aside>",
        escape_html(title),
        escape_html(body)
    )
}

fn escape_html(source: &str) -> String {
    source
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn escape_attr(source: &str) -> String {
    escape_html(source)
}

const CSS: &str = r#"
:root{--ink:#18202b;--muted:#667085;--paper:#f7f5ef;--card:#fff;--line:#e5e1d8;--nav:#17243a;--nav2:#223451;--brand:#7756ff;--brand-soft:#eee9ff;--orange:#e56b3f;--green:#1f8a64;--shadow:0 18px 50px rgba(23,36,58,.10);font-family:Inter,ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif;color:var(--ink);background:var(--paper)}
*{box-sizing:border-box}body{margin:0;line-height:1.65}.skip-link{position:fixed;left:1rem;top:-5rem;z-index:20;background:#fff;padding:.8rem 1rem;border-radius:.5rem}.skip-link:focus{top:1rem}.app-shell{min-height:100vh}.sidebar{position:fixed;inset:0 auto 0 0;width:288px;background:linear-gradient(180deg,var(--nav),#101a2c);color:#fff;padding:1.4rem 1rem;display:flex;flex-direction:column;overflow:auto;z-index:10}.brand{display:flex;align-items:center;gap:.75rem;padding:.25rem .55rem 1.25rem}.brand-mark{display:grid;place-items:center;width:42px;height:42px;border-radius:13px;background:var(--brand);font-size:1.35rem;font-weight:800}.brand strong,.brand small{display:block}.brand small{color:#afbdd0}.overview-link,.stage-link{width:100%;border:0;background:transparent;color:#d7e0ec;text-align:left;border-radius:10px;cursor:pointer;font:inherit}.overview-link{padding:.75rem;margin-bottom:.65rem;display:flex;gap:.7rem;font-weight:700}.overview-link:hover,.overview-link.active,.stage-link:hover,.stage-link.active{background:var(--nav2);color:#fff}.progress-card{padding:.85rem;border:1px solid rgba(255,255,255,.12);border-radius:12px;margin-bottom:1rem;background:rgba(255,255,255,.04)}.progress-card>div:first-child{display:flex;justify-content:space-between;font-size:.82rem}.progress-track{height:6px;border-radius:9px;background:rgba(255,255,255,.12);margin-top:.65rem;overflow:hidden}.progress-track span{display:block;height:100%;background:#9d88ff}.stage-nav{display:grid;gap:.25rem}.stage-link{padding:.6rem .65rem;display:grid;grid-template-columns:24px 1fr;align-items:center;gap:.4rem}.stage-link small,.stage-link span span{display:block}.stage-link small{color:#8fa1b8;font-size:.68rem;text-transform:uppercase;letter-spacing:.08em}.stage-link.complete .stage-marker{color:#6ee7b7}.stage-link.current .stage-marker{color:#c4b5fd}.stage-marker{font-weight:900}.sidebar-meta{margin-top:auto;padding:1rem .6rem 0;color:#8295ad;font-size:.75rem;display:flex;justify-content:space-between}.main-content{margin-left:288px;min-height:100vh}.mobile-header{display:none}.page-panel{padding:3.6rem 3rem 6rem}.page-width{width:min(960px,100%);margin:0 auto}.eyebrow{color:var(--brand);font-size:.78rem;font-weight:800;text-transform:uppercase;letter-spacing:.12em;margin-bottom:.65rem}h1{font-size:clamp(2.15rem,5vw,3.7rem);line-height:1.05;letter-spacing:-.045em;margin:.2rem 0 1rem}h2{font-size:1.55rem;line-height:1.25;margin:2rem 0 .75rem}h3{font-size:1.08rem;margin:1.6rem 0 .5rem}.lede{font-size:1.16rem;color:#4d5968;max-width:760px;margin-bottom:2rem}.stage-heading{display:flex;gap:2rem;justify-content:space-between;align-items:flex-start}.status-pill{white-space:nowrap;border-radius:999px;padding:.45rem .8rem;font-size:.77rem;font-weight:800;text-transform:uppercase;letter-spacing:.07em;background:#eef1f5;color:#657084}.status-pill.current{background:var(--brand-soft);color:#5b3fd1}.status-pill.complete{background:#def7ec;color:#187252}.orientation-grid{display:grid;grid-template-columns:repeat(3,1fr);gap:.8rem;margin:2rem 0}.orientation-grid article{background:var(--card);border:1px solid var(--line);border-radius:14px;padding:1rem;box-shadow:0 6px 18px rgba(23,36,58,.04)}.orientation-grid article.accent{border-color:#c9bcff;background:#f6f3ff}.orientation-grid small,.orientation-grid strong{display:block}.orientation-grid small{color:var(--muted);font-size:.72rem;text-transform:uppercase;letter-spacing:.08em;font-weight:750;margin-bottom:.35rem}.orientation-grid strong{line-height:1.35}.tabs{display:flex;gap:.4rem;border-bottom:1px solid var(--line);margin:2.4rem 0 1.5rem;overflow:auto}.tabs button{appearance:none;border:0;background:transparent;padding:.8rem 1rem;border-bottom:3px solid transparent;font:inherit;font-weight:750;color:var(--muted);cursor:pointer}.tabs button:hover{color:var(--ink)}.tabs button[aria-selected=true]{color:var(--brand);border-color:var(--brand)}.tab-panel{display:none;animation:fade .16s ease}.tab-panel.active{display:block}.tab-panel>h2:first-child{margin-top:.5rem}.tab-panel p,.tab-panel li{max-width:780px}.tab-panel li{margin:.35rem 0}.callout{border-radius:14px;padding:1rem 1.15rem;margin:1.2rem 0;background:#eaf8f2;border-left:4px solid var(--green)}.callout p{margin:.35rem 0 0}.roadmap-grid{display:grid;gap:.75rem}.roadmap-card{display:grid;grid-template-columns:48px 1fr;gap:1rem;background:var(--card);border:1px solid var(--line);border-radius:14px;padding:1rem}.roadmap-card.current{border-color:#b8a8ff;box-shadow:0 0 0 3px var(--brand-soft)}.roadmap-number{display:grid;place-items:center;width:42px;height:42px;border-radius:12px;background:#eef0f4;font-weight:850}.roadmap-card h3{margin:.1rem 0}.roadmap-card p{margin:.25rem 0;color:var(--muted)}.roadmap-card small{color:var(--brand);font-weight:800;text-transform:uppercase;letter-spacing:.08em}.code-block{position:relative;margin:1rem 0}.code-block pre{overflow:auto;background:#111c2e;color:#e8edf5;padding:1.15rem;border-radius:12px;line-height:1.5}.copy-button{position:absolute;right:.6rem;top:.6rem;border:1px solid #40506a;background:#1d2d45;color:#fff;border-radius:7px;padding:.35rem .55rem;cursor:pointer}.tab-panel code:not(pre code){background:#eeeaf9;color:#563ec3;padding:.12rem .33rem;border-radius:4px}.table-wrap{overflow:auto;margin:1rem 0;border:1px solid var(--line);border-radius:12px}table{border-collapse:collapse;width:100%;background:#fff}th,td{text-align:left;padding:.75rem;border-bottom:1px solid var(--line);vertical-align:top}th{background:#f1efe9;font-size:.78rem;text-transform:uppercase;letter-spacing:.04em}#toast{position:fixed;right:1.2rem;bottom:1.2rem;background:#17243a;color:#fff;padding:.7rem 1rem;border-radius:9px;opacity:0;transform:translateY(8px);transition:.2s;pointer-events:none}#toast.show{opacity:1;transform:none}@keyframes fade{from{opacity:.3;transform:translateY(3px)}to{opacity:1;transform:none}}
@media(max-width:850px){.sidebar{transform:translateX(-100%);transition:.2s;width:min(88vw,300px)}.sidebar.open{transform:none}.main-content{margin-left:0}.mobile-header{display:flex;position:sticky;top:0;z-index:8;align-items:center;gap:1rem;background:rgba(247,245,239,.94);backdrop-filter:blur(12px);padding:.8rem 1rem;border-bottom:1px solid var(--line)}.mobile-header button{font-size:1.2rem;border:0;background:transparent}.page-panel{padding:2rem 1.15rem 4rem}.orientation-grid{grid-template-columns:1fr}.stage-heading{display:block}.status-pill{display:inline-block;margin-bottom:1rem}}
@media(prefers-reduced-motion:reduce){*{scroll-behavior:auto!important;animation:none!important;transition:none!important}}
"#;

const JS: &str = r#"
const sidebar=document.querySelector('.sidebar');
const menu=document.querySelector('#menu-button');
function showPanel(target){document.querySelectorAll('.page-panel').forEach(p=>p.hidden=p.id!==target);document.querySelectorAll('[data-target]').forEach(b=>b.classList.toggle('active',b.dataset.target===target));sidebar.classList.remove('open');menu?.setAttribute('aria-expanded','false');window.scrollTo({top:0,behavior:'instant'});}
document.querySelectorAll('[data-target]').forEach(button=>button.addEventListener('click',()=>showPanel(button.dataset.target)));
document.querySelectorAll('.tabs').forEach(tabs=>tabs.addEventListener('click',event=>{const button=event.target.closest('[data-tab]');if(!button)return;const page=tabs.closest('.page-panel');tabs.querySelectorAll('[data-tab]').forEach(tab=>tab.setAttribute('aria-selected',String(tab===button)));page.querySelectorAll('.tab-panel').forEach(panel=>panel.classList.toggle('active',panel.id===button.dataset.tab));}));
document.querySelectorAll('.copy-button').forEach(button=>button.addEventListener('click',async()=>{const text=button.nextElementSibling.innerText;try{await navigator.clipboard.writeText(text);button.textContent='Copied';setTimeout(()=>button.textContent='Copy',1200)}catch{const range=document.createRange();range.selectNodeContents(button.nextElementSibling);const selection=getSelection();selection.removeAllRanges();selection.addRange(range)}}));
menu?.addEventListener('click',()=>{const open=sidebar.classList.toggle('open');menu.setAttribute('aria-expanded',String(open))});
showPanel(document.body.dataset.initialTarget||'overview');
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_renderer_escapes_pack_html_and_formats_tables() {
        let rendered = markdown_to_html(
            "## Safe\n\n<script>alert(1)</script>\n\n| Name | Value |\n|---|---|\n| `a` | **b** |",
        );
        assert!(rendered.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
        assert!(!rendered.contains("<script>alert"));
        assert!(rendered.contains("<table>"));
        assert!(rendered.contains("<code>a</code>"));
        assert!(rendered.contains("<strong>b</strong>"));
    }

    #[test]
    fn section_selection_keeps_the_learning_order() {
        let sections = split_h2_sections(
            "# Stage\n\n## Goal\n\nStart small.\n\n## Requirements\n\nDo one thing.\n",
        );
        assert_eq!(sections.len(), 2);
        assert_eq!(stage_summary("## Goal\n\nStart small."), "Start small.");
        assert!(select_sections(&sections, &["Goal"]).contains("Start small"));
        assert_eq!(
            stage_display_title("# Stage 02 — Choose searchable files", "Fallback"),
            "Choose searchable files"
        );
    }
}
