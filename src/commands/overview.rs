use anyhow::Result;
use serde::Serialize;

use crate::cli::OverviewArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::learning_web::{
    InitialView, generate_learning_page, open_learning_page, should_use_browser,
};
use crate::pack::LoadedPack;
use crate::terminal::Terminal;

pub fn run(args: OverviewArgs, options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;
    let overview = read_pack_overview(&context.pack);
    let roadmap = context
        .pack
        .manifest
        .stages
        .iter()
        .map(|stage| RoadmapStage {
            id: stage.id.clone(),
            title: stage.title.clone(),
            status: if context.state.is_completed(&stage.id) {
                "complete".to_string()
            } else if stage.id == context.state.current_stage {
                "current".to_string()
            } else {
                "upcoming".to_string()
            },
        })
        .collect::<Vec<_>>();

    let document = OverviewDocument {
        project: context.state.project.clone(),
        name: context.pack.manifest.name.clone(),
        description: context.pack.manifest.description.clone(),
        language: context.state.language.clone(),
        current_stage: context.state.current_stage.clone(),
        overview,
        roadmap,
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&document)?);
    } else if args.no_open {
        let path = generate_learning_page(&context, &document.overview, InitialView::Overview)?;
        println!("Generated learning page: {}", path.display());
    } else if should_use_browser(args.terminal) {
        let path = generate_learning_page(&context, &document.overview, InitialView::Overview)?;
        if let Err(error) = open_learning_page(&path) {
            eprintln!("warning: {error:#}; showing the terminal view instead");
            render_terminal(&document)?;
        } else {
            println!("Opened {} overview in your browser.", document.name);
        }
    } else {
        render_terminal(&document)?;
    }

    Ok(())
}

fn render_terminal(document: &OverviewDocument) -> Result<()> {
    let mut terminal = Terminal::new();
    terminal.title(&format!("{} Overview", document.name));
    terminal.key_value("Project", &document.project);
    terminal.key_value("Language", &document.language);
    terminal.key_value("Current stage", &document.current_stage);
    terminal.section("Project Context");
    terminal.markdown(&document.overview);
    terminal.section("Stage roadmap:");
    for stage in &document.roadmap {
        terminal.roadmap_line(
            marker(&stage.status),
            &stage.id,
            &stage.title,
            &stage.status,
        );
    }
    terminal.display()
}

pub fn read_pack_overview(pack: &LoadedPack) -> String {
    std::fs::read_to_string(pack.root.join("README.md"))
        .unwrap_or_else(|_| pack.manifest.description.clone())
}

fn marker(status: &str) -> &'static str {
    match status {
        "complete" => "✓",
        "current" => "→",
        _ => "○",
    }
}

#[derive(Debug, Serialize)]
struct OverviewDocument {
    project: String,
    name: String,
    description: String,
    language: String,
    current_stage: String,
    overview: String,
    roadmap: Vec<RoadmapStage>,
}

#[derive(Debug, Serialize)]
struct RoadmapStage {
    id: String,
    title: String,
    status: String,
}
