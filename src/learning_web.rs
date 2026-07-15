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
    open_in_browser(canonical.as_os_str())
}

/// Open a local file path or a `http://127.0.0.1` viewer URL in the system
/// browser.
pub fn open_in_browser(target: &std::ffi::OsStr) -> Result<()> {
    let mut command = browser_command(target)?;
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
        .spawn()
        .with_context(|| format!("failed to open {} in a browser", target.to_string_lossy()))?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn browser_command(target: &std::ffi::OsStr) -> Result<Command> {
    let mut command = Command::new("open");
    command.arg(target);
    Ok(command)
}

#[cfg(target_os = "linux")]
fn browser_command(target: &std::ffi::OsStr) -> Result<Command> {
    let mut command = Command::new("xdg-open");
    command.arg(target);
    Ok(command)
}

#[cfg(windows)]
fn browser_command(target: &std::ffi::OsStr) -> Result<Command> {
    let mut command = Command::new("rundll32");
    command.arg("url.dll,FileProtocolHandler").arg(target);
    Ok(command)
}

#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
fn browser_command(_target: &std::ffi::OsStr) -> Result<Command> {
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
    let total = stages.len();
    let completed = stages
        .iter()
        .filter(|stage| stage.status == "complete")
        .count();

    let notches = stages
        .iter()
        .map(|stage| format!(r#"<span class="notch {}"></span>"#, stage.status))
        .collect::<String>();

    let mut contents_rows = String::new();
    for (index, stage) in stages.iter().enumerate() {
        contents_rows.push_str(&format!(
            r#"<li><button class="toc-link {status}" data-target="stage-{id}" aria-label="Stage {number}: {title} — {label}"><span class="toc-num" aria-hidden="true">{numbered:02}</span><span class="toc-title">{title}</span><span class="toc-state" aria-hidden="true">{glyph}</span></button></li>"#,
            status = stage.status,
            id = escape_attr(&stage.id),
            number = index + 1,
            numbered = index + 1,
            title = escape_html(&stage.title),
            label = status_label(stage.status),
            glyph = status_glyph(stage.status),
        ));
    }

    let overview_panel = render_overview_panel(pack_name, overview, stages);
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
<meta name="color-scheme" content="light dark">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline'; img-src data:">
<title>{pack_name} · DeltaForge</title>
<style>{theme_tokens}{css}{theme_unify}</style>
</head>
<body data-initial-target="{initial_target}">
<a class="skip-link" href="#main-content">Skip to content</a>
<header class="masthead">
  <button id="menu-button" class="menu-button" aria-expanded="false" aria-controls="contents-nav">Contents</button>
  <div class="brand"><span class="mark" aria-hidden="true">Δ</span><span class="brand-text"><span class="brand-name">DeltaForge</span><span class="brand-pack">{pack_name}</span></span></div>
  <nav class="site-nav" aria-label="DeltaForge pages"><a class="site-link current" aria-current="page" href="learning.html">Learn</a><a class="site-link" href="test-report.html">Test report</a></nav>
  <div class="masthead-meta"><span class="workbench">{project} · {language}</span><div class="progress"><span class="notches" role="img" aria-label="{completed} of {total} stages complete">{notches}</span><span class="progress-count">{completed} of {total} complete</span></div></div>
</header>
<div class="layout">
<nav id="contents-nav" class="contents" aria-label="Chapters">
  <p class="contents-label" aria-hidden="true">Contents</p>
  <button class="toc-link front" data-target="overview" aria-label="Project overview"><span class="toc-num" aria-hidden="true">Δ</span><span class="toc-title">Project overview</span></button>
  <ol class="toc-list">{contents_rows}</ol>
</nav>
<main id="main-content">
{overview_panel}
{stage_panels}
</main>
</div>
<div id="scrim" class="scrim" aria-hidden="true"></div>
<script>{js}</script>
</body>
</html>"##,
        pack_name = escape_html(pack_name),
        project = escape_html(project),
        language = escape_html(language),
        initial_target = escape_attr(&initial_target),
        theme_tokens = crate::web_theme::TOKENS,
        theme_unify = crate::web_theme::UNIFY,
        css = CSS,
        js = JS,
    )
}

fn render_overview_panel(pack_name: &str, overview: &str, stages: &[StagePage]) -> String {
    let sections = split_h2_sections(overview);
    let lede = section_body(&sections, "What you are building")
        .map(markdown_to_html)
        .unwrap_or_default();

    let mut secs = String::new();
    let mut rail = String::new();
    let mut number = 0;

    if let Some(body) = section_body(&sections, "Why this is useful") {
        push_section(
            &mut secs,
            &mut rail,
            &mut number,
            "overview",
            "Why this is useful",
            &markdown_to_html(body),
            false,
        );
    }

    let roadmap_rows = stages
        .iter()
        .enumerate()
        .map(|(index, stage)| {
            format!(
                r#"<li><button class="roadmap-row {status}" data-target="stage-{id}"><span class="roadmap-head"><span class="roadmap-num" aria-hidden="true">{number:02}</span><span class="roadmap-title">{title}</span><span class="roadmap-leader" aria-hidden="true"></span><span class="roadmap-state">{state}</span></span><span class="roadmap-summary">{summary}</span></button></li>"#,
                status = stage.status,
                id = escape_attr(&stage.id),
                number = index + 1,
                title = escape_html(&stage.title),
                state = roadmap_state(stage.status),
                summary = escape_html(&stage_summary(&stage.source)),
            )
        })
        .collect::<String>();
    push_section(
        &mut secs,
        &mut rail,
        &mut number,
        "overview",
        "The road ahead",
        &format!(r#"<ol class="roadmap">{roadmap_rows}</ol>"#),
        false,
    );

    for (title, body) in &sections {
        if title.eq_ignore_ascii_case("What you are building")
            || title.eq_ignore_ascii_case("Why this is useful")
        {
            continue;
        }
        let boxed = title.eq_ignore_ascii_case("What good looks like");
        push_section(
            &mut secs,
            &mut rail,
            &mut number,
            "overview",
            title,
            &markdown_to_html(body),
            boxed,
        );
    }
    if sections.is_empty() && !overview.trim().is_empty() {
        push_section(
            &mut secs,
            &mut rail,
            &mut number,
            "overview",
            "About this project",
            &markdown_to_html(overview),
            false,
        );
    }

    let lede_block = if lede.is_empty() {
        String::new()
    } else {
        format!(r#"<div class="lede">{lede}</div>"#)
    };

    format!(
        r#"<section id="overview" class="chapter" hidden>
<div class="chapter-main">
<header class="chapter-head">
<p class="kicker"><span>Project overview</span></p>
<h1 class="chapter-title" tabindex="-1">{pack_name}</h1>
{lede_block}
</header>
<aside class="rail"><nav aria-label="Sections on this page"><p class="rail-label">On this page</p><ol>{rail}</ol></nav></aside>
<div class="prose">{secs}</div>
</div>
</section>"#,
        pack_name = escape_html(pack_name),
    )
}

fn render_stage_panel(stage: &StagePage, index: usize, stages: &[StagePage]) -> String {
    let sections = split_h2_sections(&stage.source);
    let panel_id = format!("stage-{}", stage.id);
    let goal = section_body(&sections, "Goal")
        .map(markdown_to_html)
        .unwrap_or_default();

    let mut secs = String::new();
    let mut rail = String::new();
    let mut number = 0;
    for (title, body) in &sections {
        if title.eq_ignore_ascii_case("Goal") {
            continue;
        }
        let boxed = title.eq_ignore_ascii_case("Non-goals");
        push_section(
            &mut secs,
            &mut rail,
            &mut number,
            &panel_id,
            title,
            &markdown_to_html(body),
            boxed,
        );
    }

    let (previous_target, previous_title) = index
        .checked_sub(1)
        .and_then(|previous| stages.get(previous))
        .map_or_else(
            || ("overview".to_string(), "Project overview".to_string()),
            |stage| (format!("stage-{}", stage.id), stage.title.clone()),
        );
    let next = stages
        .get(index + 1)
        .map(|stage| (format!("stage-{}", stage.id), stage.title.clone()));
    let next_waypoint = next
        .as_ref()
        .map_or("The finished project".to_string(), |(_, title)| {
            title.clone()
        });
    let turn_next = next.as_ref().map_or_else(
        || {
            r#"<p class="turn-end">This is the final stage. Everything past it is yours to design.</p>"#
                .to_string()
        },
        |(target, title)| {
            format!(
                r#"<button class="turn next" data-target="{target}"><span class="turn-label">Next →</span><span class="turn-title">{title}</span></button>"#,
                target = escape_attr(target),
                title = escape_html(title),
            )
        },
    );

    format!(
        r#"<section id="{panel_id}" class="chapter" hidden>
<div class="chapter-main">
<header class="chapter-head">
<p class="kicker"><span>Stage {number:02} of {total}</span><span aria-hidden="true">·</span><span class="st-{status}">{label}</span></p>
<h1 class="chapter-title" tabindex="-1">{title}</h1>
<div class="lede">{goal}</div>
</header>
<div class="waypoints">
<div class="waypoint"><span class="waypoint-label">Behind you</span><span class="waypoint-title">{previous_title}</span></div>
<div class="waypoint ahead"><span class="waypoint-label">Ahead</span><span class="waypoint-title">{next_waypoint}</span></div>
</div>
<aside class="rail"><nav aria-label="Sections in this chapter"><p class="rail-label">In this chapter</p><ol>{rail}</ol></nav></aside>
<div class="prose">{secs}</div>
<nav class="page-turn" aria-label="Chapter navigation"><button class="turn prev" data-target="{previous_target}"><span class="turn-label">← Previous</span><span class="turn-title">{previous_title}</span></button>{turn_next}</nav>
</div>
</section>"#,
        panel_id = escape_attr(&panel_id),
        number = index + 1,
        total = stages.len(),
        status = stage.status,
        label = status_label(stage.status),
        title = escape_html(&stage.title),
        previous_target = escape_attr(&previous_target),
        previous_title = escape_html(&previous_title),
        next_waypoint = escape_html(&next_waypoint),
    )
}

fn push_section(
    secs: &mut String,
    rail: &mut String,
    number: &mut usize,
    panel_id: &str,
    title: &str,
    body_html: &str,
    boxed: bool,
) {
    *number += 1;
    let anchor = format!("{}-{}", panel_id, slugify(title));
    secs.push_str(&format!(
        r##"<section class="sec{boxed}" id="{anchor}"><h2><span class="sec-num" aria-hidden="true">§{number}</span>{title}</h2>{body_html}</section>"##,
        boxed = if boxed { " sec-box" } else { "" },
        anchor = escape_attr(&anchor),
        title = inline_html(title),
    ));
    rail.push_str(&format!(
        r##"<li><a href="#{anchor}">{title}</a></li>"##,
        anchor = escape_attr(&anchor),
        title = inline_html(title),
    ));
}

fn status_label(status: &str) -> &'static str {
    match status {
        "complete" => "Done",
        "current" => "In progress",
        _ => "Ahead",
    }
}

fn status_glyph(status: &str) -> &'static str {
    match status {
        "complete" => "▪",
        "current" => "◆",
        _ => "▫",
    }
}

fn roadmap_state(status: &str) -> &'static str {
    match status {
        "complete" => "done",
        "current" => "you are here",
        _ => "ahead",
    }
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut pending_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            pending_dash = false;
            slug.push(ch.to_ascii_lowercase());
        } else {
            pending_dash = true;
        }
    }
    if slug.is_empty() {
        slug.push_str("section");
    }
    slug
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

fn section_body<'a>(sections: &'a [(String, String)], name: &str) -> Option<&'a str> {
    sections
        .iter()
        .find(|(title, _)| title.eq_ignore_ascii_case(name))
        .map(|(_, body)| body.as_str())
}

fn stage_summary(source: &str) -> String {
    let sections = split_h2_sections(source);
    let goal = section_body(&sections, "Goal").unwrap_or("Learn the next project behavior.");
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

const CSS: &str = r##"
/* Palette and font tokens come from the shared theme (web_theme::TOKENS). */
*{box-sizing:border-box}
html{background:var(--paper);scroll-behavior:smooth}
body{margin:0;font-family:var(--serif);font-size:1.0625rem;line-height:1.75;color:var(--ink);background:var(--paper)}
[hidden]{display:none!important}
:focus-visible{outline:2px solid var(--ember);outline-offset:2px}
button{font:inherit;color:inherit}
.skip-link{position:fixed;left:1rem;top:-4rem;z-index:40;background:var(--panel);border:1px solid var(--ink);padding:.6rem 1rem;font-family:var(--mono);font-size:.8rem;color:var(--ink);text-decoration:none}
.skip-link:focus{top:1rem}

/* Masthead */
.masthead{display:flex;align-items:center;gap:1.25rem;padding:1rem 2rem;border-bottom:3px double var(--line);background:var(--paper)}
.site-nav{display:flex;gap:1rem;margin-left:.5rem}
.site-nav a{color:var(--muted);text-decoration:none;font-size:.78rem;text-transform:uppercase;letter-spacing:.1em;font-weight:700;padding:.25rem 0;border-bottom:2px solid transparent}
.site-nav a:hover{color:var(--ink)}
.site-nav a.current{color:var(--ink);border-bottom-color:var(--ink)}
.menu-button{display:none}
.brand{display:flex;align-items:center;gap:.8rem}
.mark{display:grid;place-items:center;width:2.25rem;height:2.25rem;border:2px solid var(--ink);font-family:var(--mono);font-size:1.15rem;font-weight:700}
.brand-text{display:flex;flex-direction:column;line-height:1.3}
.brand-name{font-family:var(--mono);font-size:.66rem;font-weight:600;letter-spacing:.24em;text-transform:uppercase;color:var(--muted)}
.brand-pack{font-size:1.2rem;font-weight:700}
.masthead-meta{margin-left:auto;display:flex;flex-direction:column;align-items:flex-end;gap:.3rem}
.workbench{font-family:var(--mono);font-size:.72rem;color:var(--muted)}
.progress{display:flex;align-items:center;gap:.6rem}
.notches{display:flex;gap:3px}
.notch{width:13px;height:8px;border:1px solid var(--muted)}
.notch.complete{background:var(--ink);border-color:var(--ink)}
.notch.current{background:var(--ember);border-color:var(--ember)}
.progress-count{font-family:var(--mono);font-size:.72rem;color:var(--muted)}

/* Layout and contents */
.layout{display:grid;grid-template-columns:272px minmax(0,1fr);align-items:start}
.contents{position:sticky;top:0;max-height:100vh;overflow-y:auto;padding:2.2rem 1.4rem 2.2rem 2rem;border-right:1px solid var(--line)}
.contents-label{font-family:var(--mono);font-size:.66rem;letter-spacing:.24em;text-transform:uppercase;color:var(--muted);margin:0 0 .9rem .6rem}
.toc-list{list-style:none;margin:.7rem 0 0;padding:.7rem 0 0;border-top:1px solid var(--line-soft)}
.toc-link{display:grid;grid-template-columns:1.7rem 1fr auto;align-items:baseline;gap:.5rem;width:100%;padding:.42rem .55rem;border:0;background:none;text-align:left;cursor:pointer;border-left:3px solid transparent}
.toc-link:hover .toc-title{text-decoration:underline}
.toc-link.active{border-left-color:var(--ember);background:var(--panel)}
.toc-num{font-family:var(--mono);font-size:.72rem;color:var(--muted)}
.toc-title{font-size:.95rem;line-height:1.35}
.toc-link.upcoming .toc-title{color:var(--muted)}
.toc-state{font-size:.72rem;color:var(--muted)}
.toc-link.current .toc-state{color:var(--ember)}
.toc-link.current .toc-title{font-weight:700}

/* Chapter frame */
.chapter{display:flex;justify-content:center;padding:3.2rem 3rem 5.5rem}
.chapter-main{display:grid;grid-template-columns:minmax(0,66ch) minmax(150px,190px);column-gap:3.2rem;min-width:0}
.chapter-main>*{grid-column:1;min-width:0}
.rail{grid-column:2;grid-row:1/span 32;position:sticky;top:2.2rem;align-self:start;font-size:.85rem}
.rail-label{font-family:var(--mono);font-size:.64rem;letter-spacing:.22em;text-transform:uppercase;color:var(--muted);margin:0 0 .7rem}
.rail ol{list-style:none;margin:0;padding:0}
.rail li{margin:.4rem 0}
.rail a{display:block;color:var(--muted);text-decoration:none;border-left:2px solid var(--line-soft);padding-left:.65rem;line-height:1.4}
.rail a:hover{color:var(--ink)}
.rail a.active{color:var(--ember);border-left-color:var(--ember)}
.kicker{display:flex;flex-wrap:wrap;gap:.6rem;margin:0 0 .9rem;font-family:var(--mono);font-size:.72rem;letter-spacing:.16em;text-transform:uppercase;color:var(--muted)}
.kicker .st-current{color:var(--ember);font-weight:700}
.kicker .st-complete{color:var(--ink);font-weight:700}
.chapter-title{margin:0;font-size:clamp(1.9rem,4vw,2.55rem);line-height:1.12;font-weight:700;letter-spacing:-.01em}
.chapter-title:focus{outline:none}
.chapter-title::after{content:"";display:block;width:3.2rem;border-bottom:4px solid var(--ember);margin-top:1rem}
.lede{margin-top:1.3rem}
.lede p{margin:.4rem 0;font-size:1.2rem;line-height:1.62}

/* Waypoints */
.waypoints{display:flex;justify-content:space-between;gap:2rem;margin:2.2rem 0 .6rem;padding:.85rem 0;border-top:1px solid var(--line);border-bottom:1px solid var(--line)}
.waypoint{display:flex;flex-direction:column;gap:.1rem;min-width:0}
.waypoint.ahead{text-align:right}
.waypoint-label{font-family:var(--mono);font-size:.62rem;letter-spacing:.2em;text-transform:uppercase;color:var(--muted)}
.waypoint-title{font-size:.95rem}

/* Sections and prose */
.sec{position:relative;margin-top:2.9rem;scroll-margin-top:5rem}
.sec h2{position:relative;margin:0 0 .8rem;font-size:1.38rem;line-height:1.3}
.sec-num{font-family:var(--mono);font-size:.74rem;font-weight:600;color:var(--ember);margin-right:.65rem;letter-spacing:.05em}
.sec-box{border:1px solid var(--line);background:var(--panel);padding:1.2rem 1.5rem 1.35rem;margin-top:3.2rem}
.sec-box h2{font-size:1.05rem}
.prose p{margin:.9em 0}
.prose ul{padding-left:1.3rem;margin:.9em 0}
.prose ul li{list-style:square;margin:.45em 0}
.prose ol{padding-left:1.5rem;margin:.9em 0}
.prose ol li{margin:.45em 0}
.prose h3{font-size:1.06rem;margin:1.8em 0 .5em}
.prose :not(pre)>code,.lede :not(pre)>code{font-family:var(--mono);font-size:.85em;background:var(--inline-code);padding:.08em .35em;border-radius:2px}

/* Code */
.code-block{position:relative;margin:1.5rem 0}
.code-block pre{margin:0;padding:1.05rem 1.2rem;background:var(--code-bg);color:var(--code-ink);border-left:3px solid var(--ember);overflow-x:auto;font-family:var(--mono);font-size:.85rem;line-height:1.65}
.code-block code{font-family:inherit}
.copy-button{position:absolute;top:.55rem;right:.55rem;font-family:var(--mono);font-size:.64rem;letter-spacing:.12em;text-transform:uppercase;padding:.3rem .6rem;background:transparent;color:var(--code-ink);border:1px solid var(--code-line);cursor:pointer}
.copy-button:hover{border-color:var(--code-ink)}

/* Tables */
.table-wrap{overflow-x:auto;margin:1.5rem 0}
table{border-collapse:collapse;width:100%}
th{font-family:var(--mono);font-size:.68rem;letter-spacing:.1em;text-transform:uppercase;text-align:left;padding:.5rem .7rem;border-bottom:2px solid var(--ink)}
td{padding:.6rem .7rem;border-bottom:1px solid var(--line-soft);vertical-align:top}

/* Roadmap */
.roadmap{list-style:none;margin:0;padding:0}
.roadmap li{border-bottom:1px solid var(--line-soft)}
.roadmap-row{display:block;width:100%;padding:.95rem .2rem;border:0;background:none;text-align:left;cursor:pointer}
.roadmap-head{display:flex;align-items:baseline;gap:.8rem}
.roadmap-num{font-family:var(--mono);font-size:.76rem;color:var(--muted)}
.roadmap-title{font-size:1.06rem;font-weight:700}
.roadmap-row:hover .roadmap-title{text-decoration:underline}
.roadmap-row.upcoming .roadmap-title{color:var(--muted);font-weight:600}
.roadmap-leader{flex:1;min-width:2rem;border-bottom:1px dotted var(--line);transform:translateY(-.28em)}
.roadmap-state{font-family:var(--mono);font-size:.66rem;letter-spacing:.14em;text-transform:uppercase;color:var(--muted);white-space:nowrap}
.roadmap-row.current .roadmap-state,.roadmap-row.current .roadmap-num{color:var(--ember)}
.roadmap-row.current .roadmap-state{font-weight:700}
.roadmap-summary{display:block;margin:.3rem 0 0 2.4rem;color:var(--muted);font-size:.94rem;line-height:1.55}

/* Page turns */
.page-turn{display:flex;justify-content:space-between;gap:2rem;margin-top:4rem;border-top:3px double var(--line);padding-top:1.4rem}
.turn{display:flex;flex-direction:column;gap:.2rem;border:0;background:none;cursor:pointer;text-align:left;padding:0;max-width:46%}
.turn.next{margin-left:auto;text-align:right;align-items:flex-end}
.turn-label{font-family:var(--mono);font-size:.64rem;letter-spacing:.2em;text-transform:uppercase;color:var(--muted)}
.turn-title{font-size:1.02rem;font-weight:700}
.turn:hover .turn-title{text-decoration:underline}
.turn-end{margin:0 0 0 auto;max-width:46%;text-align:right;color:var(--muted);font-size:.92rem;font-style:italic}

/* Drawer scrim */
.scrim{position:fixed;inset:0;background:rgba(22,16,8,.42);z-index:20;opacity:0;pointer-events:none;transition:opacity .2s}
.scrim.show{opacity:1;pointer-events:auto}

@media(min-width:1280px){.sec-num{position:absolute;right:100%;margin-right:1.1rem;top:.45em}}
@media(max-width:960px){
  .masthead{position:sticky;top:0;z-index:10;padding:.6rem 1rem;gap:.8rem}
  .menu-button{display:inline-block;font-family:var(--mono);font-size:.7rem;letter-spacing:.14em;text-transform:uppercase;border:1px solid var(--ink);background:var(--paper);padding:.45rem .7rem;cursor:pointer}
  .mark{display:none}
  .brand-name{display:none}
  .brand-pack{font-size:1.02rem}
  .workbench,.progress-count{display:none}
  .layout{display:block}
  .contents{position:fixed;inset:0 auto 0 0;z-index:30;width:min(82vw,320px);background:var(--paper);transform:translateX(-103%);transition:transform .2s}
  .contents.open{transform:none;box-shadow:0 0 44px rgba(0,0,0,.3)}
  .chapter{display:block;padding:1.8rem 1.1rem 4rem}
  .chapter-main{display:block}
  .rail{position:static;margin:1.7rem 0;border:1px solid var(--line-soft);background:var(--panel);padding:.85rem 1rem}
  .rail ol{display:flex;flex-wrap:wrap;gap:.1rem 1.1rem}
  .rail a{border-left:0;padding-left:0}
  .sec{scroll-margin-top:4.2rem}
  .waypoints{flex-direction:column;gap:.6rem}
  .waypoint.ahead{text-align:left}
  .page-turn{flex-direction:column;gap:1.2rem}
  .turn,.turn.next,.turn-end{max-width:100%;margin-left:0;text-align:left;align-items:flex-start}
}
@media(prefers-reduced-motion:reduce){
  html{scroll-behavior:auto}
  *{transition:none!important;animation:none!important}
}
@media print{
  .masthead,.contents,.rail,.page-turn,.copy-button,.skip-link,.scrim{display:none!important}
  .chapter{display:block;padding:0}
}
"##;

const JS: &str = r##"
const menu=document.getElementById('menu-button');
const drawer=document.getElementById('contents-nav');
const scrim=document.getElementById('scrim');
function setDrawer(open){
  drawer.classList.toggle('open',open);
  scrim.classList.toggle('show',open);
  menu.setAttribute('aria-expanded',String(open));
}
menu.addEventListener('click',()=>setDrawer(!drawer.classList.contains('open')));
scrim.addEventListener('click',()=>setDrawer(false));
document.addEventListener('keydown',(event)=>{if(event.key==='Escape')setDrawer(false);});
function showPanel(target,moveFocus){
  document.querySelectorAll('.chapter').forEach((panel)=>{panel.hidden=panel.id!==target;});
  document.querySelectorAll('.toc-link').forEach((link)=>{
    const active=link.dataset.target===target;
    link.classList.toggle('active',active);
    if(active){link.setAttribute('aria-current','page');}else{link.removeAttribute('aria-current');}
  });
  setDrawer(false);
  window.scrollTo(0,0);
  if(moveFocus){
    const heading=document.querySelector('#'+CSS.escape(target)+' .chapter-title');
    if(heading)heading.focus({preventScroll:true});
  }
}
document.querySelectorAll('[data-target]').forEach((button)=>{
  button.addEventListener('click',()=>{
    showPanel(button.dataset.target,true);
    history.replaceState(null,'','#'+button.dataset.target);
  });
});
document.querySelectorAll('.copy-button').forEach((button)=>{
  button.addEventListener('click',async()=>{
    const pre=button.parentElement.querySelector('pre');
    try{
      await navigator.clipboard.writeText(pre.innerText);
      button.textContent='Copied';
    }catch{
      const range=document.createRange();
      range.selectNodeContents(pre);
      const selection=getSelection();
      selection.removeAllRanges();
      selection.addRange(range);
      button.textContent='Selected';
    }
    setTimeout(()=>{button.textContent='Copy';},1400);
  });
});
if('IntersectionObserver' in window){
  document.querySelectorAll('.chapter').forEach((chapter)=>{
    const links=new Map();
    chapter.querySelectorAll('.rail a').forEach((link)=>{
      links.set(link.getAttribute('href').slice(1),link);
    });
    if(links.size===0)return;
    const observer=new IntersectionObserver((entries)=>{
      entries.forEach((entry)=>{
        if(!entry.isIntersecting)return;
        links.forEach((link)=>link.classList.remove('active'));
        const link=links.get(entry.target.id);
        if(link)link.classList.add('active');
      });
    },{rootMargin:'-10% 0px -75% 0px'});
    chapter.querySelectorAll('.sec[id]').forEach((section)=>observer.observe(section));
  });
}
const linkedTarget=decodeURIComponent(location.hash.slice(1));
const linkedPanel=document.getElementById(linkedTarget);
showPanel(linkedPanel&&linkedPanel.classList.contains('chapter')?linkedTarget:(document.body.dataset.initialTarget||'overview'),false);
"##;

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
        assert_eq!(section_body(&sections, "goal"), Some("Start small."));
        assert_eq!(
            stage_display_title("# Stage 02 — Choose searchable files", "Fallback"),
            "Choose searchable files"
        );
    }

    #[test]
    fn slugs_are_stable_anchor_names() {
        assert_eq!(slugify("Edge cases"), "edge-cases");
        assert_eq!(slugify("Non-goals"), "non-goals");
        assert_eq!(slugify("§ ?!"), "section");
    }
}
