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

    tracing::debug_span!("resolving runtime config");
    let config = resolve_runtime_config(args)?;

    tracing::debug!("fetching external indices");

    render_docs(config).await?;

    Ok(())
}
