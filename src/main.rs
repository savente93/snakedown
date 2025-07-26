use color_eyre::eyre::Result;
use snakedown::render_docs;
use tracing::subscriber::set_global_default;

mod cli;

use crate::cli::{Args, resolve_runtime_config};
use clap::Parser;

#[allow(clippy::missing_errors_doc)]
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(args.verbose.tracing_level_filter())
        .finish();

    set_global_default(subscriber)?;

    let config = resolve_runtime_config(args)?;
    render_docs(
        &config.pkg_path,
        &config.output_dir,
        config.skip_private,
        config.skip_undoc,
        config.exclude,
        &config.renderer,
    )?;

    Ok(())
}
