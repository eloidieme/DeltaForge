use anyhow::Result;

use crate::context::GlobalOptions;
use crate::pack::{PackSearchOptions, discover_packs_with_options};

pub fn run(options: &GlobalOptions) -> Result<()> {
    let discovery = discover_packs_with_options(&PackSearchOptions {
        packs_dir: options.packs_dir.clone(),
    })?;

    for problem in &discovery.problems {
        eprintln!(
            "warning: skipping invalid pack {}: {}",
            problem.path.display(),
            problem.error
        );
    }

    let packs = discovery.packs;
    if packs.is_empty() {
        println!("No project packs found.");
        println!("Set DELTAFORGE_PACKS_DIR or install bundled packs.");
        return Ok(());
    }

    println!("Available projects:");
    println!();

    for pack in packs {
        let topics = if pack.manifest.topics.is_empty() {
            "none".to_string()
        } else {
            pack.manifest.topics.join(", ")
        };
        let languages = pack
            .manifest
            .languages
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");

        println!("{}", pack.manifest.id);
        println!("  {}", pack.manifest.description);
        println!("  Languages: {languages}");
        println!("  Topics: {topics}");
        println!("  Source: {}", pack.root.display());
        println!();
    }

    Ok(())
}
