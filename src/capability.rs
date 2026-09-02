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
    /// The stage's authored sections, in reading order, each parsed into
    /// paragraphs, lists, and code blocks.
    ///
    /// These were once flattened to a first paragraph and a few bullet items.
    /// Packs write requirements as prose with a fenced list as often as they
    /// write them as bullets, so that flattening silently emptied the two
    /// panels a learner relies on most on thirteen of FlashIndex's fourteen
    /// stages. The workbench now renders what the pack actually says.
    pub sections: Vec<CapabilitySection>,
    pub capability_statement: String,
    pub next: Option<CapabilityPreview>,
    pub help_levels: usize,
    /// How many of those levels may be revealed right now. Sent explicitly so
    /// the workbench renders the reveal control from the rule the service
    /// actually enforces instead of re-deriving it and disagreeing.
    pub available_help_levels: usize,
    pub revealed_help: Vec<HelpLevel>,
}

/// One authored section of a stage guide.
#[derive(Debug, Clone, Serialize)]
pub struct CapabilitySection {
    /// Stable identifier the workbench places into a specific panel:
    /// `background`, `requirements`, `example`, `expected`, `edge_cases`, or
    /// `non_goals`.
    pub key: &'static str,
    /// The heading the pack used, kept so an unusual pack still reads
    /// correctly.
    pub title: String,
    pub blocks: Vec<OverviewBlock>,
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

/// A run of inline text, in reading order.
pub type InlineText = Vec<InlineSpan>;

/// One inline run inside a block.
///
/// 1.0 had no such thing. Inline markup was deleted by a blanket
/// `replace(['`', '*'], "")`, which is how ByteForgeVM's multiplication stage
/// came to teach `push left right`: the operator lived inside a code span and
/// the strip took it with the backticks. Nothing may be dropped from a
/// specification silently, so every construct the packs use is represented
/// here and anything else is a `validate-pack --strict` failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InlineSpan {
    /// Ordinary prose.
    Text { text: String },
    /// `` `like this` ``. Its content is verbatim: no markup is recognised
    /// inside a code span, which is exactly the rule that was missing.
    Code { text: String },
    /// `**like this**`.
    Strong { spans: InlineText },
    /// `*like this*`.
    Emphasis { spans: InlineText },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OverviewBlock {
    /// A sub-heading inside a section. `level` is the authored depth, so a
    /// pack that nests further still renders in order rather than collapsing
    /// into the surrounding prose — which is what 1.0 did to 22 headings
    /// across 21 stage files, taking the hierarchy of everything beneath them
    /// with it.
    Heading {
        level: u8,
        spans: InlineText,
    },
    Paragraph {
        spans: InlineText,
    },
    Code {
        language: String,
        content: String,
    },
    /// `ordered` distinguishes `1. 2. 3.` from `- - -`. Packs write 164
    /// numbered items across the four of them, all of which used to render as
    /// bullets: a sequence of steps presented as an unordered set.
    List {
        ordered: bool,
        items: Vec<InlineText>,
    },
    Table {
        headers: Vec<InlineText>,
        /// One per column, from the delimiter row. Authored alignment is a
        /// decision about how a column reads, so it is carried rather than
        /// dropped.
        alignments: Vec<ColumnAlignment>,
        rows: Vec<Vec<InlineText>>,
    },
}

impl OverviewBlock {
    /// The block's text with no markup at all. What a surface that cannot
    /// show structure — a rail summary, a terminal line — should print.
    pub fn plain_text(&self) -> String {
        match self {
            Self::Heading { spans, .. } | Self::Paragraph { spans } => plain_text(spans),
            Self::Code { content, .. } => content.clone(),
            Self::List { items, .. } => items
                .iter()
                .map(|item| plain_text(item))
                .collect::<Vec<_>>()
                .join("\n"),
            Self::Table { headers, rows, .. } => std::iter::once(headers)
                .chain(rows.iter())
                .map(|row| {
                    row.iter()
                        .map(|cell| plain_text(cell))
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}

/// Inline text with nothing but its words: code spans keep their content,
/// emphasis keeps its words, and no marker characters survive.
pub fn plain_text(spans: &[InlineSpan]) -> String {
    let mut out = String::new();
    for span in spans {
        match span {
            InlineSpan::Text { text } | InlineSpan::Code { text } => out.push_str(text),
            InlineSpan::Strong { spans } | InlineSpan::Emphasis { spans } => {
                out.push_str(&plain_text(spans));
            }
        }
    }
    out
}

/// Inline text rendered back to the markdown it was parsed from. Used by
/// `deltaforge pack content`, which publishes what a learner can see as
/// markdown, and by the round-trip assertion in `validate-pack --strict`.
pub fn to_markdown(spans: &[InlineSpan]) -> String {
    let mut out = String::new();
    for span in spans {
        match span {
            InlineSpan::Text { text } => out.push_str(&escape_markdown(text)),
            InlineSpan::Code { text } => {
                // A code span containing a backtick is fenced with more of
                // them, which is how CommonMark spells it and how it survives
                // being parsed again.
                let fence = "`".repeat(longest_backtick_run(text) + 1);
                let padding = if text.starts_with('`') || text.ends_with('`') {
                    " "
                } else {
                    ""
                };
                out.push_str(&format!("{fence}{padding}{text}{padding}{fence}"));
            }
            InlineSpan::Strong { spans } => out.push_str(&format!("**{}**", to_markdown(spans))),
            InlineSpan::Emphasis { spans } => out.push_str(&format!("*{}*", to_markdown(spans))),
        }
    }
    out
}

/// Render blocks back to the markdown they were parsed from.
///
/// One emitter, with three jobs: it is what `deltaforge pack content`
/// publishes, it is how `validate-pack --strict` proves a pack survives the
/// renderer, and it is the reason those two can never disagree.
pub fn blocks_to_markdown(blocks: &[OverviewBlock]) -> String {
    let mut out = String::new();
    for block in blocks {
        match block {
            OverviewBlock::Heading { level, spans } => {
                let hashes = "#".repeat(usize::from(*level));
                out.push_str(&format!("{hashes} {}\n\n", to_markdown(spans)));
            }
            OverviewBlock::Paragraph { spans } => {
                out.push_str(&format!("{}\n\n", to_markdown(spans)));
            }
            OverviewBlock::Code { language, content } => {
                out.push_str(&format!("```{language}\n{content}\n```\n\n"));
            }
            OverviewBlock::List { ordered, items } => {
                for (index, item) in items.iter().enumerate() {
                    let marker = if *ordered {
                        format!("{}.", index + 1)
                    } else {
                        "-".to_string()
                    };
                    out.push_str(&format!("{marker} {}\n", to_markdown(item)));
                }
                out.push('\n');
            }
            OverviewBlock::Table {
                headers,
                alignments,
                rows,
            } => {
                let row = |cells: &[InlineText]| {
                    format!(
                        "| {} |\n",
                        cells
                            .iter()
                            .map(|cell| to_markdown(cell))
                            .collect::<Vec<_>>()
                            .join(" | ")
                    )
                };
                out.push_str(&row(headers));
                out.push_str(&format!(
                    "|{}\n",
                    alignments
                        .iter()
                        .map(|alignment| format!("{}|", alignment.delimiter()))
                        .collect::<String>()
                ));
                for cells in rows {
                    out.push_str(&row(cells));
                }
                out.push('\n');
            }
        }
    }
    out
}

/// Escape the three characters `parse_inline` treats as markup, and nothing
/// else. Escaping more would turn ordinary prose ("well-known") into noise;
/// escaping less would stop the round trip from being exact, which is the
/// whole basis of the fidelity check.
fn escape_markdown(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        if matches!(character, '\\' | '`' | '*') {
            out.push('\\');
        }
        out.push(character);
    }
    out
}

/// Every place a pack writes markdown the workbench cannot render.
///
/// This is the rule that keeps P0-3 from happening again. A pack is not valid
/// under `--strict` if any of its learner-facing prose loses characters on the
/// way to the screen — the corruption that shipped in 1.0 would have failed
/// here, on the stage that teaches multiplication.
///
/// It covers every file a learner can reach: the project README that becomes
/// the guide, each stage's instructions, its hints, and its prediction prompt.
pub fn pack_render_defects(pack: &crate::pack::LoadedPack) -> Vec<String> {
    let mut problems = Vec::new();
    let mut check = |label: String, source: &str| {
        for problem in unrepresentable_markdown(source) {
            problems.push(format!("{label}: {problem}"));
        }
    };

    if let Ok(readme) = fs::read_to_string(pack.root.join("README.md")) {
        for (title, body) in markdown_sections(&readme) {
            check(format!("README.md, section {title}"), &body);
        }
    }
    for stage in &pack.manifest.stages {
        for (name, path) in [
            ("instructions.md", pack.instructions_path(stage)),
            ("hints.md", pack.hints_path(stage)),
            ("prediction.md", pack.prediction_prompt_path(stage)),
        ] {
            let Ok(source) = fs::read_to_string(&path) else {
                continue;
            };
            // Headings delimit; their own bodies are what gets rendered.
            let sections = markdown_sections(&source);
            if sections.is_empty() {
                check(format!("{}/{name}", stage.id), &source);
            }
            for (title, body) in sections {
                check(format!("{}/{name}, section {title}", stage.id), &body);
            }
        }
    }
    problems
}

/// Everything in `source` the workbench renderer cannot represent.
///
/// The test is a round trip: parse the markdown into blocks, render the blocks
/// back to markdown, and compare what is left after whitespace. Anything that
/// does not come back is a character a learner will never see — which is how
/// ByteForgeVM stage 4 came to teach `push left right` — and anything that
/// comes back escaped is a marker the pack meant literally but wrote as
/// markup.
///
/// This is deliberately a property, not a list of banned constructs. A list
/// only refuses what someone thought of; a round trip refuses everything the
/// renderer does not actually handle, including whatever is added next.
pub fn unrepresentable_markdown(source: &str) -> Vec<String> {
    let mut problems = ignored_constructs(source);
    let blocks = parse_blocks(&source.lines().collect::<Vec<_>>());
    let authored = fidelity_form(source);
    let rendered = fidelity_form(&blocks_to_markdown(&blocks));
    if authored != rendered {
        problems.extend(first_divergence(&authored, &rendered));
    }
    problems
}

/// Block constructs CommonMark defines and this renderer has no block for.
///
/// The round trip cannot see these: their markers are ordinary characters, so
/// they survive as literal text and compare equal. A learner reads `>` at the
/// start of a line, or `[the docs](https://…)` with its brackets and URL
/// showing. The list is short because it is exhaustive over what the parser
/// does not handle — add a block above and the entry here comes out.
fn ignored_constructs(source: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let mut in_code = false;
    let lines: Vec<&str> = source.lines().collect();
    for (index, raw) in lines.iter().enumerate() {
        let line = raw.trim();
        if line.starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code || line.is_empty() {
            continue;
        }
        let mut say = |what: &str| {
            problems.push(format!("line {}: {what}: {line}", index + 1));
        };
        if line.starts_with('>') {
            say("a block quotation has no block in the workbench renderer");
        }
        if link_like(line) {
            say("a link renders as its own brackets and URL");
        }
        if line
            .chars()
            .all(|c| c == '-' || c == '=' || c == '*' || c == '_')
            && line.len() >= 3
        {
            say("a rule or underlined heading renders as punctuation");
        }
        if raw.starts_with("  ") && markdown_list_item(line).is_some() {
            say("a nested list renders flattened into its parent");
        }
        if line.starts_with('<') && line.contains('>') {
            say("raw HTML renders as text");
        }
    }
    problems
}

/// `[text](target)` or `![alt](target)`, outside a code span.
fn link_like(line: &str) -> bool {
    let bare: String = {
        let mut out = String::new();
        let mut in_code = false;
        for character in line.chars() {
            if character == '`' {
                in_code = !in_code;
            } else if !in_code {
                out.push(character);
            }
        }
        out
    };
    let characters: Vec<char> = bare.chars().collect();
    characters.iter().enumerate().any(|(index, character)| {
        *character == '['
            && find_from(&characters, index, ']')
                .is_some_and(|close| characters.get(close + 1) == Some(&'('))
    })
}

/// Both sides reduced to the words and markers they carry, with layout
/// removed: paragraphs are re-wrapped by the parser, so line breaks and runs
/// of spaces cannot be part of the comparison. Everything else is.
fn fidelity_form(source: &str) -> Vec<String> {
    source
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .split(' ')
        .map(str::to_string)
        .collect()
}

/// The words around the first place the two forms part company, so the report
/// points at the sentence rather than at the document.
fn first_divergence(authored: &[String], rendered: &[String]) -> Vec<String> {
    let at = authored
        .iter()
        .zip(rendered)
        .position(|(left, right)| left != right)
        .unwrap_or_else(|| authored.len().min(rendered.len()));
    let from = at.saturating_sub(6);
    let window = |words: &[String]| {
        words[from.min(words.len())..(at + 7).min(words.len())]
            .join(" ")
            .trim()
            .to_string()
    };
    vec![format!(
        "the pack says \"…{}…\" but the workbench renders \"…{}…\"",
        window(authored),
        window(rendered)
    )]
}

fn longest_backtick_run(text: &str) -> usize {
    let mut longest = 0;
    let mut run = 0;
    for character in text.chars() {
        if character == '`' {
            run += 1;
            longest = longest.max(run);
        } else {
            run = 0;
        }
    }
    longest
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadmapCapability {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub position: usize,
    pub status: RoadmapStatus,
    /// Whether this is the step the learner is on. Orthogonal to `status`: a
    /// step that has passed but has not been advanced past is both complete
    /// and current, and the interface needs to say both.
    pub current: bool,
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
    /// The hint as authored, and as structure. Hints used to reach the
    /// workbench as one flat string, so a hint that named a function in
    /// backticks displayed the backticks — a third disagreeing renderer in a
    /// product about writing code.
    pub content: RichText,
}

/// How one table column is aligned, as its delimiter row spelled it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnAlignment {
    Start,
    Center,
    End,
}

impl ColumnAlignment {
    fn parse(cell: &str) -> Self {
        match (cell.starts_with(':'), cell.ends_with(':')) {
            (true, true) => Self::Center,
            (false, true) => Self::End,
            _ => Self::Start,
        }
    }

    fn delimiter(self) -> &'static str {
        match self {
            Self::Start => "---",
            Self::Center => ":---:",
            Self::End => "---:",
        }
    }
}

/// Authored markdown, together with the blocks the workbench renders from it.
///
/// One source and one parser. Surfaces that can show structure read `blocks`;
/// the terminal and the exported record, which render markdown themselves,
/// read `source`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RichText {
    pub source: String,
    pub blocks: Vec<OverviewBlock>,
}

impl RichText {
    pub fn parse(source: impl Into<String>) -> Self {
        let source: String = source.into();
        let blocks = parse_blocks(&source.lines().collect::<Vec<_>>());
        Self { source, blocks }
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
}

/// Exactly what a learner can see in the workbench for one stage, and nothing
/// else: no `tests.yaml`, no fixtures, no reference solution.
///
/// This is the fixture for the content-sufficiency practice in
/// `docs/product/content-sufficiency.md`: if a stage cannot be implemented from
/// this alone, the specification is incomplete, however complete it looks to
/// its author.
#[derive(Debug, Clone, Serialize)]
pub struct PublishedStageContent {
    pub pack: String,
    pub pack_description: String,
    pub stage_id: String,
    pub position: usize,
    pub total_stages: usize,
    pub title: String,
    pub mission: String,
    pub sections: Vec<CapabilitySection>,
    pub help: Vec<HelpLevel>,
    /// The titles of every earlier stage, which a learner has already built and
    /// may therefore rely on.
    pub established: Vec<String>,
    /// Build and run commands for the chosen language, since the contract is
    /// expressed in terms of an invocation.
    pub run_command: Vec<String>,
}

pub fn load_published_content(
    pack: &crate::pack::LoadedPack,
    stage_id: &str,
    language_id: &str,
) -> Result<PublishedStageContent> {
    let position = pack
        .manifest
        .stages
        .iter()
        .position(|stage| stage.id == stage_id)
        .with_context(|| {
            format!(
                "pack {} does not contain stage {stage_id}",
                pack.manifest.id
            )
        })?;
    let stage = &pack.manifest.stages[position];
    let language = pack.manifest.language(language_id).with_context(|| {
        format!(
            "pack {} does not support language {language_id}",
            pack.manifest.id
        )
    })?;
    let source = pack.read_stage_file(&pack.instructions_path(stage))?;
    let sections = markdown_sections(&source);
    let help = fs::read_to_string(pack.hints_path(stage))
        .map(|source| parse_help(&source))
        .unwrap_or_default();

    Ok(PublishedStageContent {
        pack: pack.manifest.name.clone(),
        pack_description: pack.manifest.description.clone(),
        stage_id: stage.id.clone(),
        position: position + 1,
        total_stages: pack.manifest.stages.len(),
        title: stage.title.clone(),
        mission: first_paragraph(required_section(&sections, "Goal")?),
        sections: capability_sections(&sections)?,
        help,
        established: pack.manifest.stages[..position]
            .iter()
            .map(|earlier| earlier.title.clone())
            .collect(),
        run_command: language.run.command.clone(),
    })
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
        sections: capability_sections(&sections)?,
        capability_statement: sections
            .get("Capability acquired")
            .map(|section| first_paragraph(section))
            .unwrap_or_else(|| format!("Your program can now {}.", stage.title.to_lowercase())),
        next,
        help_levels: hints.len(),
        available_help_levels: available_help_levels(
            hints.len(),
            context.state.is_completed(&stage.id),
        ),
        revealed_help,
    })
}

/// The six panels the workbench shows for a stage, in reading order, paired
/// with the `##` heading each one is authored under.
const PANEL_SECTIONS: [(&str, &str); 6] = [
    ("background", "Background"),
    ("requirements", "Requirements"),
    ("example", "Example"),
    ("expected", "Success criteria"),
    ("edge_cases", "Edge cases"),
    ("non_goals", "Non-goals"),
];

/// Every `##` heading a stage guide must carry to render, in reading order.
/// `Goal` is first because it supplies the mission line above the panels.
pub const REQUIRED_STAGE_SECTIONS: [&str; 7] = [
    "Goal",
    "Background",
    "Requirements",
    "Example",
    "Success criteria",
    "Edge cases",
    "Non-goals",
];

/// The required headings this stage guide is missing or leaves empty.
///
/// `capability_sections` below refuses to build content without them, so a
/// pack that fails this check cannot be opened in the workbench at all. The
/// authoring doctor calls the same function, rather than its own partial list,
/// so `pack doctor` reporting a clean pack means the pack actually renders.
pub fn missing_stage_sections(source: &str) -> Vec<&'static str> {
    let sections = markdown_sections(source);
    REQUIRED_STAGE_SECTIONS
        .into_iter()
        .filter(|heading| {
            sections
                .get(*heading)
                .is_none_or(|body| parse_blocks(&body.lines().collect::<Vec<_>>()).is_empty())
        })
        .collect()
}

/// The six sections the workbench shows for a stage, in reading order, each
/// parsed into blocks. A pack that omits one of the required headings fails
/// here rather than rendering an empty panel.
fn capability_sections(sections: &BTreeMap<String, String>) -> Result<Vec<CapabilitySection>> {
    PANEL_SECTIONS
        .into_iter()
        .map(|(key, heading)| {
            let body = required_section(sections, heading)?;
            Ok(CapabilitySection {
                key,
                title: heading.to_string(),
                blocks: parse_blocks(&body.lines().collect::<Vec<_>>()),
            })
        })
        .collect()
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
                    blocks: parse_blocks(&lines),
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
            blocks: parse_blocks(&lines),
        });
    }
    sections.retain(|section| !section.blocks.is_empty());
    sections
}

/// Parse markdown lines into headings, paragraphs, bullet or numbered lists,
/// fenced code blocks, and tables. Shared by the project overview and every
/// stage section, so all three workbench surfaces agree about what a pack
/// says.
fn parse_blocks(lines: &[&str]) -> Vec<OverviewBlock> {
    let mut blocks = Vec::new();
    let mut paragraph: Vec<&str> = Vec::new();
    let mut list: Vec<InlineText> = Vec::new();
    let mut list_ordered = false;
    let mut table: Vec<&str> = Vec::new();
    let mut code: Vec<&str> = Vec::new();
    let mut code_language = String::new();
    let mut in_code = false;

    let flush_paragraph = |paragraph: &mut Vec<&str>, blocks: &mut Vec<OverviewBlock>| {
        if paragraph.is_empty() {
            return;
        }
        let spans = parse_inline(
            &paragraph
                .drain(..)
                .map(str::trim)
                .collect::<Vec<_>>()
                .join(" "),
        );
        if !spans.is_empty() {
            blocks.push(OverviewBlock::Paragraph { spans });
        }
    };
    let flush_list =
        |list: &mut Vec<InlineText>, ordered: bool, blocks: &mut Vec<OverviewBlock>| {
            if !list.is_empty() {
                blocks.push(OverviewBlock::List {
                    ordered,
                    items: std::mem::take(list),
                });
            }
        };
    let flush_table = |table: &mut Vec<&str>, blocks: &mut Vec<OverviewBlock>| {
        if let Some(block) = parse_table(table) {
            blocks.push(block);
        } else {
            // Not a table after all — pipes in prose. Keep the lines rather
            // than dropping them.
            for line in table.iter() {
                blocks.push(OverviewBlock::Paragraph {
                    spans: parse_inline(line.trim()),
                });
            }
        }
        table.clear();
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
                flush_list(&mut list, list_ordered, &mut blocks);
                flush_table(&mut table, &mut blocks);
                code_language = fence.trim().to_string();
                in_code = true;
            }
        } else if in_code {
            code.push(line);
        } else if let Some((level, heading)) = markdown_heading(trimmed) {
            flush_paragraph(&mut paragraph, &mut blocks);
            flush_list(&mut list, list_ordered, &mut blocks);
            flush_table(&mut table, &mut blocks);
            blocks.push(OverviewBlock::Heading {
                level,
                spans: parse_inline(heading),
            });
        } else if is_table_row(trimmed) {
            flush_paragraph(&mut paragraph, &mut blocks);
            flush_list(&mut list, list_ordered, &mut blocks);
            table.push(trimmed);
        } else if let Some((ordered, item)) = markdown_list_item(trimmed) {
            flush_paragraph(&mut paragraph, &mut blocks);
            flush_table(&mut table, &mut blocks);
            // A list that changes marker is two lists, not one mislabelled one.
            if !list.is_empty() && ordered != list_ordered {
                flush_list(&mut list, list_ordered, &mut blocks);
            }
            list_ordered = ordered;
            list.push(parse_inline(item));
        } else if trimmed.is_empty() {
            flush_paragraph(&mut paragraph, &mut blocks);
            flush_list(&mut list, list_ordered, &mut blocks);
            flush_table(&mut table, &mut blocks);
        } else {
            flush_list(&mut list, list_ordered, &mut blocks);
            flush_table(&mut table, &mut blocks);
            paragraph.push(line);
        }
    }
    flush_paragraph(&mut paragraph, &mut blocks);
    flush_list(&mut list, list_ordered, &mut blocks);
    flush_table(&mut table, &mut blocks);
    blocks
}

/// An ATX heading and its depth. `## ` never reaches here: it delimits the
/// sections these blocks live inside.
fn markdown_heading(line: &str) -> Option<(u8, &str)> {
    let hashes = line.len() - line.trim_start_matches('#').len();
    (1..=6).contains(&hashes).then(|| {
        let rest = line[hashes..].strip_prefix(' ')?;
        Some((hashes as u8, rest.trim()))
    })?
}

fn is_table_row(line: &str) -> bool {
    line.starts_with('|') && line.ends_with('|') && line.len() > 1
}

/// A GitHub-style table: a header row, a delimiter row, then body rows.
/// Returns `None` when the lines are not one, so the caller can keep them as
/// prose instead of losing them.
fn parse_table(lines: &[&str]) -> Option<OverviewBlock> {
    let delimiter = lines.get(1)?;
    let delimiter_cells = table_cells(delimiter);
    let is_delimiter = delimiter_cells
        .iter()
        .all(|cell| cell.contains('-') && cell.chars().all(|c| c == '-' || c == ':' || c == ' '));
    if !is_delimiter {
        return None;
    }
    let alignments = delimiter_cells
        .iter()
        .map(|cell| ColumnAlignment::parse(cell))
        .collect();
    let headers: Vec<InlineText> = table_cells(lines.first()?)
        .iter()
        .map(|cell| parse_inline(cell))
        .collect();
    let rows = lines[2..]
        .iter()
        .map(|line| {
            table_cells(line)
                .iter()
                .map(|cell| parse_inline(cell))
                .collect()
        })
        .collect();
    Some(OverviewBlock::Table {
        headers,
        alignments,
        rows,
    })
}

fn table_cells(line: &str) -> Vec<String> {
    line.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

/// Parse one line of inline markdown into spans.
///
/// Code spans bind tightest and their content is verbatim, which is the rule
/// whose absence deleted a multiplication operator from a stage that teaches
/// multiplication. An unclosed marker is not markup: it stays as the character
/// it is, and `validate-pack --strict` reports it rather than letting it reach
/// a learner as punctuation nobody meant.
fn parse_inline(source: &str) -> InlineText {
    let characters: Vec<char> = source.chars().collect();
    let mut spans = Vec::new();
    let mut literal = String::new();
    let mut index = 0;

    let flush = |literal: &mut String, spans: &mut InlineText| {
        if !literal.is_empty() {
            spans.push(InlineSpan::Text {
                text: std::mem::take(literal),
            });
        }
    };

    while index < characters.len() {
        let character = characters[index];
        if character == '\\'
            && let Some(next) = characters.get(index + 1)
            && is_markdown_punctuation(*next)
        {
            literal.push(*next);
            index += 2;
            continue;
        }
        if character == '`'
            && let Some(end) = find_from(&characters, index + 1, '`')
        {
            flush(&mut literal, &mut spans);
            spans.push(InlineSpan::Code {
                text: characters[index + 1..end].iter().collect(),
            });
            index = end + 1;
            continue;
        }
        if character == '*' {
            let marker = if characters.get(index + 1) == Some(&'*') {
                "**"
            } else {
                "*"
            };
            if let Some(end) = find_marker(&characters, index + marker.len(), marker) {
                let inner: String = characters[index + marker.len()..end].iter().collect();
                if !inner.trim().is_empty() {
                    flush(&mut literal, &mut spans);
                    let inner = parse_inline(&inner);
                    spans.push(if marker == "**" {
                        InlineSpan::Strong { spans: inner }
                    } else {
                        InlineSpan::Emphasis { spans: inner }
                    });
                    index = end + marker.len();
                    continue;
                }
            }
        }
        literal.push(character);
        index += 1;
    }
    flush(&mut literal, &mut spans);
    spans
}

fn is_markdown_punctuation(character: char) -> bool {
    matches!(
        character,
        '\\' | '`' | '*' | '_' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '.' | '!' | '|'
    )
}

fn find_from(characters: &[char], start: usize, wanted: char) -> Option<usize> {
    (start..characters.len()).find(|index| characters[*index] == wanted)
}

/// The next occurrence of `marker` at or after `start`. `**` must not match
/// the first star of a `***`-style run, and a lone `*` must not match the
/// first star of a `**`.
fn find_marker(characters: &[char], start: usize, marker: &str) -> Option<usize> {
    let wanted: Vec<char> = marker.chars().collect();
    let mut index = start;
    while index + wanted.len() <= characters.len() {
        if characters[index..index + wanted.len()] == wanted[..] {
            let next = characters.get(index + wanted.len());
            if wanted.len() == 2 || next != Some(&'*') {
                return Some(index);
            }
        }
        index += 1;
    }
    None
}

/// A list item and whether its marker was a number. `None` for anything that
/// is not a list item at all.
fn markdown_list_item(line: &str) -> Option<(bool, &str)> {
    if let Some(item) = line.strip_prefix("- ") {
        return Some((false, item));
    }
    let (number, item) = line.split_once(". ")?;
    (!number.is_empty() && number.chars().all(|character| character.is_ascii_digit()))
        .then_some((true, item))
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
                current: stage.id == context.state.current_stage,
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
) -> Option<RichText> {
    let source = std::fs::read_to_string(context.pack.prediction_prompt_path(stage)).ok()?;
    let body = source
        .lines()
        .skip_while(|line| line.trim_start().starts_with('#') || line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();
    (!body.is_empty()).then(|| RichText::parse(body))
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
            plain_line(
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
        let text = plain_line(
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

/// A markdown line reduced to the words it contains.
///
/// This replaces a blanket `replace(['`', '*'], "")`, which deleted every
/// backtick and asterisk in the document — including the ones that were the
/// content rather than the markup.
fn plain_line(text: &str) -> String {
    plain_text(&parse_inline(text)).trim().to_string()
}

/// Parse a stage's `hints.md` into its help ladder.
///
/// This is the only parser of that file. There used to be three — this one,
/// the terminal `hint` command's, and the authoring doctor's heading count —
/// and they disagreed about what counts as a level. The terminal one treated
/// any prose before the first `# Hint` heading as level 1, which pushed
/// `hint_state` one rung too high and revealed the gated retrospective in the
/// workbench, because the two surfaces write and read the same counter.
///
/// A level is numbered by its position in reading order, not by the digits in
/// its heading, so `hint_state` (a count) and `HelpLevel::level` (an index)
/// cannot drift apart when a pack numbers its headings `1, 2, 4`. The authored
/// number is still required — it is what marks the line as a rung rather than
/// an ordinary `# Hint`-prefixed sentence — and `validate-pack` reports
/// numbering that does not run 1..n.
pub fn parse_help(source: &str) -> Vec<HelpLevel> {
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

/// The numbers a pack authored on its `# Hint N` headings, in reading order.
/// `parse_help` renumbers by position, so this is what tells an author their
/// ladder skips or repeats a rung.
pub fn authored_help_numbers(source: &str) -> Vec<usize> {
    source
        .lines()
        .filter_map(|line| help_heading_parts(line.strip_prefix("# Hint ")?.trim()))
        .map(|(number, _)| number)
        .collect()
}

/// How many help levels may be revealed for a stage right now.
///
/// The last authored level is the retrospective and stays gated until the
/// capability is acquired; a pack with a single level has no separate
/// retrospective to gate, so that one level stays visible. The workbench used
/// to re-derive this as `min(total, 4)`, which happened to match the flagship's
/// five-rung ladder and was wrong for every three-rung pack: the reveal button
/// offered a level the service then refused.
pub fn available_help_levels(total: usize, completed: bool) -> usize {
    if completed {
        total
    } else {
        total.saturating_sub(1).max(total.min(1))
    }
}

/// Split `1 — Observation` into its number and its label. `None` when the
/// heading carries no parsable number, which is what separates a ladder rung
/// from an ordinary line that happens to start with `# Hint `.
fn help_heading_parts(heading: &str) -> Option<(usize, &str)> {
    let (number, label) = heading
        .split_once(['—', '-'])
        .map_or((heading, "Hint"), |(number, label)| {
            (number.trim(), label.trim())
        });
    Some((number.parse().ok()?, label))
}

fn push_help(levels: &mut Vec<HelpLevel>, heading: Option<String>, lines: &mut Vec<&str>) {
    let Some(heading) = heading else {
        return;
    };
    let parts = help_heading_parts(&heading).map(|(_, label)| label.to_string());
    let content = lines.join("\n").trim().to_string();
    lines.clear();
    let Some(label) = parts else {
        return;
    };
    if !content.is_empty() {
        levels.push(HelpLevel {
            level: levels.len() + 1,
            label,
            content: RichText::parse(content),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Markdown markers that reached the learner as punctuation.
    ///
    /// Only prose is checked. A code span's content is verbatim by
    /// definition — `left * right` is the specification, not a stray marker —
    /// and that distinction is the entire difference between this renderer
    /// and the blanket `replace` it replaced.
    fn leaked_markers(block: &OverviewBlock) -> Vec<String> {
        fn walk(spans: &[InlineSpan], found: &mut Vec<String>) {
            for span in spans {
                match span {
                    InlineSpan::Text { text } => {
                        for marker in ['`', '*', '#'] {
                            if text.contains(marker) {
                                found.push(format!("a literal {marker:?} in prose: {text}"));
                            }
                        }
                    }
                    InlineSpan::Code { .. } => {}
                    InlineSpan::Strong { spans } | InlineSpan::Emphasis { spans } => {
                        walk(spans, found);
                    }
                }
            }
        }
        let mut found = Vec::new();
        match block {
            OverviewBlock::Heading { spans, .. } | OverviewBlock::Paragraph { spans } => {
                walk(spans, &mut found);
            }
            OverviewBlock::List { items, .. } => {
                for item in items {
                    walk(item, &mut found);
                }
            }
            OverviewBlock::Table { headers, rows, .. } => {
                for cell in headers.iter().chain(rows.iter().flatten()) {
                    walk(cell, &mut found);
                }
            }
            OverviewBlock::Code { .. } => {}
        }
        found
    }

    /// One unstyled run, for the many assertions that are about block
    /// structure rather than inline markup.
    fn text_spans(text: &str) -> InlineText {
        vec![InlineSpan::Text {
            text: text.to_string(),
        }]
    }

    /// Every panel the workbench shows must have authored content behind it,
    /// for every stage of every shipped pack. This is the guard for a defect
    /// that hid in plain sight: requirements and expected behavior were
    /// extracted as bullet items only, so thirteen of FlashIndex's fourteen
    /// stages rendered those two panels empty.
    #[test]
    fn every_shipped_stage_fills_every_panel() {
        for pack_id in ["flashindex", "minikv", "tinyhttp", "byteforgevm"] {
            let pack = crate::pack::load_builtin_pack(pack_id).unwrap();
            for stage in &pack.manifest.stages {
                let published = load_published_content(&pack, &stage.id, "rust")
                    .unwrap_or_else(|error| panic!("{pack_id}/{}: {error:#}", stage.id));
                assert!(
                    !published.mission.trim().is_empty(),
                    "{pack_id}/{} has no goal",
                    stage.id
                );
                assert_eq!(
                    published.sections.len(),
                    6,
                    "{pack_id}/{} is missing a section",
                    stage.id
                );
                for section in &published.sections {
                    assert!(
                        !section.blocks.is_empty(),
                        "{pack_id}/{} would render an empty {} panel",
                        stage.id,
                        section.key
                    );
                    // Not empty is not the same as right. 1.0 fixed sections
                    // rendering blank and shipped them rendering wrong: a
                    // learner saw `### Reflection` as a sentence and
                    // `push left right` where the pack said `left * right`.
                    // Nothing a learner reads may carry a marker that was
                    // meant as markup.
                    let leaked = section
                        .blocks
                        .iter()
                        .flat_map(leaked_markers)
                        .collect::<Vec<_>>();
                    assert!(
                        leaked.is_empty(),
                        "{pack_id}/{}: the {} panel renders {}",
                        stage.id,
                        section.key,
                        leaked.join("; ")
                    );
                }
                let expected_help = if pack_id == "flashindex" { 5 } else { 3 };
                assert!(
                    published.help.len() >= expected_help,
                    "{pack_id}/{} has {} help levels, expected at least {expected_help}",
                    stage.id,
                    published.help.len()
                );
                for (index, level) in published.help.iter().enumerate() {
                    assert_eq!(level.level, index + 1, "{pack_id}/{}", stage.id);
                    assert!(
                        !level.content.is_empty(),
                        "{pack_id}/{} help level {} is empty",
                        stage.id,
                        level.level
                    );
                }

                // And the whole pack survives its own renderer. This is the
                // same rule `validate-pack --strict` enforces, asserted here
                // so a pack cannot be changed without running it.
                assert_eq!(
                    pack_render_defects(&pack),
                    Vec::<String>::new(),
                    "{pack_id} writes markdown the workbench cannot render"
                );
            }
        }
    }

    /// The flagship's help ladder is labelled at every rung. A missing label
    /// falls back to the literal word "Hint", which is what thirteen stages
    /// used to show.
    #[test]
    fn the_flagship_help_ladder_is_labelled_end_to_end() {
        const LADDER: [&str; 5] = [
            "Observation",
            "Concept",
            "Experiment",
            "Structure",
            "Retrospective",
        ];
        let pack = crate::pack::load_builtin_pack("flashindex").unwrap();
        for stage in &pack.manifest.stages {
            let published = load_published_content(&pack, &stage.id, "rust").unwrap();
            let labels = published
                .help
                .iter()
                .map(|level| level.label.as_str())
                .collect::<Vec<_>>();
            assert_eq!(labels, LADDER, "{}", stage.id);
        }
    }

    /// Prose before the first `# Hint` heading is not a rung. The terminal
    /// `hint` command used to count it as one, which pushed the shared
    /// `hint_state` counter a level too high and revealed the gated
    /// retrospective in the workbench.
    #[test]
    fn a_preamble_is_not_a_help_level_and_numbering_follows_position() {
        let ladder = parse_help(concat!(
            "Read the instructions again before opening a hint.\n\n",
            "# Hint 1 — Observation\n\nLook at the output.\n\n",
            "# Hint 2 — Concept\n\nThink recursively.\n\n",
            "# Hint 3 — Retrospective\n\nThe whole answer.\n",
        ));
        assert_eq!(ladder.len(), 3);
        assert_eq!(ladder[0].label, "Observation");
        assert_eq!(ladder[0].content.source, "Look at the output.");

        // A ladder numbered 1, 2, 4 still climbs 1, 2, 3: `hint_state` counts
        // rungs, so a level that skipped would never be revealed.
        let misnumbered =
            parse_help("# Hint 1\n\nfirst\n\n# Hint 2\n\nsecond\n\n# Hint 4\n\nthird\n");
        assert_eq!(
            misnumbered
                .iter()
                .map(|hint| hint.level)
                .collect::<Vec<_>>(),
            [1, 2, 3]
        );
        assert_eq!(authored_help_numbers("# Hint 1\na\n# Hint 4\nb\n"), [1, 4]);
    }

    /// The reveal ceiling the service enforces. The workbench used to re-derive
    /// it as `min(levels, 4)`, which matched the flagship's five-rung ladder
    /// and offered a refused level on every three-rung pack.
    #[test]
    fn the_retrospective_is_the_only_gated_level() {
        assert_eq!(available_help_levels(5, false), 4);
        assert_eq!(available_help_levels(5, true), 5);
        assert_eq!(available_help_levels(3, false), 2);
        assert_eq!(available_help_levels(3, true), 3);
        // A single-rung ladder has no separate retrospective to withhold.
        assert_eq!(available_help_levels(1, false), 1);
        assert_eq!(available_help_levels(0, false), 0);
    }

    /// P0-3. `strip_inline_markdown` was a blanket
    /// `replace(['`', '*'], "")`, which deleted the multiplication operator
    /// from the ByteForgeVM stage that teaches multiplication. Nothing may be
    /// dropped from a specification silently.
    #[test]
    fn inline_markup_is_parsed_rather_than_deleted() {
        assert_eq!(
            parse_inline("`MUL`: pop right, then push `left * right`."),
            [
                InlineSpan::Code {
                    text: "MUL".to_string()
                },
                InlineSpan::Text {
                    text: ": pop right, then push ".to_string()
                },
                InlineSpan::Code {
                    text: "left * right".to_string()
                },
                InlineSpan::Text {
                    text: ".".to_string()
                },
            ]
        );
        // Reduced to words, the operator is still there — which is what the
        // rail summary and the terminal print.
        assert_eq!(
            plain_line("`MUL`: pop right, then push `left * right`."),
            "MUL: pop right, then push left * right."
        );
    }

    #[test]
    fn emphasis_nests_and_code_spans_are_verbatim() {
        assert_eq!(
            parse_inline("Checks the **`shape`** of it"),
            [
                InlineSpan::Text {
                    text: "Checks the ".to_string()
                },
                InlineSpan::Strong {
                    spans: vec![InlineSpan::Code {
                        text: "shape".to_string()
                    }]
                },
                InlineSpan::Text {
                    text: " of it".to_string()
                },
            ]
        );
        // No markup is recognised inside a code span, so the stars stay.
        assert_eq!(
            parse_inline("`a ** b`"),
            [InlineSpan::Code {
                text: "a ** b".to_string()
            }]
        );
    }

    #[test]
    fn an_unclosed_marker_stays_the_character_it_is() {
        assert_eq!(
            parse_inline("2 * 3 and a stray ` tick"),
            [InlineSpan::Text {
                text: "2 * 3 and a stray ` tick".to_string()
            }]
        );
    }

    #[test]
    fn headings_lists_and_tables_are_blocks_of_their_own() {
        let blocks = parse_blocks(&[
            "### Reflection",
            "",
            "1. First",
            "2. Second",
            "",
            "- A",
            "",
            "| Head | Count |",
            "|---|---:|",
            "| `ADD` | 2 |",
        ]);
        assert_eq!(
            blocks,
            [
                OverviewBlock::Heading {
                    level: 3,
                    spans: text_spans("Reflection")
                },
                OverviewBlock::List {
                    ordered: true,
                    items: vec![text_spans("First"), text_spans("Second")],
                },
                OverviewBlock::List {
                    ordered: false,
                    items: vec![text_spans("A")],
                },
                OverviewBlock::Table {
                    headers: vec![text_spans("Head"), text_spans("Count")],
                    alignments: vec![ColumnAlignment::Start, ColumnAlignment::End],
                    rows: vec![vec![
                        vec![InlineSpan::Code {
                            text: "ADD".to_string()
                        }],
                        text_spans("2"),
                    ]],
                },
            ]
        );
    }

    /// Pipes in prose are not a table, and must not be swallowed as one.
    #[test]
    fn a_pipe_row_without_a_delimiter_stays_prose() {
        let blocks = parse_blocks(&["| not a table |"]);
        assert_eq!(
            blocks,
            [OverviewBlock::Paragraph {
                spans: text_spans("| not a table |")
            }]
        );
    }

    /// The rule `validate-pack --strict` enforces, stated as the property it
    /// is: everything a pack writes must survive being rendered.
    #[test]
    fn the_fidelity_check_finds_what_the_renderer_would_drop() {
        assert!(unrepresentable_markdown("Push `left * right` onto the stack.").is_empty());
        assert!(unrepresentable_markdown("### Reflection\n\n- Why?\n").is_empty());
        assert!(
            unrepresentable_markdown("| A | B |\n|---|---:|\n| 1 | 2 |\n").is_empty(),
            "column alignment must survive"
        );
        // A code span that never closes: the backtick reaches the learner.
        assert!(!unrepresentable_markdown("Call `scan without closing it.").is_empty());
        // A construct the renderer has no block for.
        assert!(!unrepresentable_markdown("> A quotation.\n").is_empty());
        assert!(!unrepresentable_markdown("A [link](https://example.com) here.\n").is_empty());
    }

    #[test]
    fn parses_structured_sections_and_help_levels() {
        let sections = markdown_sections(
            "# Stage\n\n## Goal\n\nBuild `scan`.\n\n## Requirements\n\n- Walk files.\n- Sort output.\n",
        );
        assert_eq!(first_paragraph(&sections["Goal"]), "Build scan.");
        assert_eq!(
            parse_blocks(&sections["Requirements"].lines().collect::<Vec<_>>()),
            [OverviewBlock::List {
                ordered: false,
                items: vec![text_spans("Walk files."), text_spans("Sort output.")],
            }]
        );

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

        // Requirements are as often prose with a fenced list as they are
        // bullets. Both must reach the learner.
        let prose = parse_blocks(
            &"Only print files with these extensions:\n\n```text\n.rs .md\n```\n"
                .lines()
                .collect::<Vec<_>>(),
        );
        assert_eq!(prose.len(), 2);
        assert!(matches!(prose[0], OverviewBlock::Paragraph { .. }));
        assert!(matches!(prose[1], OverviewBlock::Code { .. }));

        let help = parse_help(
            "# Hint 1 — Observation\n\nLook at output.\n\n# Hint 2 — Concept\n\nThink recursively.\n",
        );
        assert_eq!(help.len(), 2);
        assert_eq!(help[0].label, "Observation");
        assert_eq!(help[1].level, 2);
    }
}
