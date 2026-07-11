use anyhow::Result;

use crate::cli::PortfolioArgs;
use crate::commands::bench::{history_path, read_history};
use crate::context::{GlobalOptions, ProjectContext};
use crate::fs_util::atomic_write;

pub fn run(args: PortfolioArgs, options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;
    let history = read_history(&history_path(&context))?;
    let output = render(&context, history.as_slice());
    atomic_write(&args.output, output)?;
    println!("Wrote portfolio summary: {}", args.output.display());
    Ok(())
}

fn render(context: &ProjectContext, history: &[crate::commands::bench::BenchmarkRecord]) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", context.pack.manifest.name));
    out.push_str("## Project Summary\n\n");
    out.push_str(&format!(
        "Built `{}` in `{}` as a staged systems project with DeltaForge.\n\n",
        context.state.project, context.state.language
    ));

    out.push_str("## Completed Features\n\n");
    let completed = context
        .pack
        .manifest
        .stages
        .iter()
        .filter(|stage| context.state.is_completed(&stage.id))
        .collect::<Vec<_>>();
    if completed.is_empty() {
        out.push_str("No stages have been completed yet.\n\n");
    } else {
        for stage in completed {
            out.push_str(&format!("- {}: {}\n", stage.id, stage.title));
        }
        out.push('\n');
    }

    out.push_str("## Architecture Highlights\n\n");
    out.push_str("- Command-line behavior is validated through black-box stage tests.\n");
    out.push_str(
        "- Progress, hints, test summaries, and benchmark history are stored locally.\n\n",
    );

    out.push_str("## Benchmark Highlights\n\n");
    if history.is_empty() {
        out.push_str("No benchmark data has been recorded yet.\n\n");
    } else {
        for record in history.iter().rev().take(5) {
            if record.results.success {
                out.push_str(&format!(
                    "- `{}` / `{}`: median {:.2} ms",
                    record.stage,
                    record.benchmark,
                    record.results.runtime_median_ms.unwrap_or_default()
                ));
                if let Some(throughput) = record.results.throughput_mb_s {
                    out.push_str(&format!(", {:.2} MB/s", throughput));
                }
                out.push('\n');
            }
        }
        out.push('\n');
    }

    out.push_str("## Future Improvements\n\n");
    out.push_str("- Broaden correctness coverage and edge-case handling.\n");
    out.push_str("- Profile benchmark hot paths before optimizing.\n");
    out
}
