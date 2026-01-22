use color_eyre::eyre::Result;
use snakedown::{config::ConfigBuilder, render_docs};
use tracing::subscriber::set_global_default;

mod cli;

use crate::cli::{CliArgs, resolve_runtime_config};
use clap::Parser;

#[allow(clippy::missing_errors_doc)]
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = CliArgs::parse();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(args.verbose.tracing_level_filter())
        .finish();

    set_global_default(subscriber)?;

    tracing::debug_span!("resolving runtime config");
    let default_config = ConfigBuilder::default().init_with_defaults();
    let runtime_config = resolve_runtime_config(args)?;

    let config = default_config.merge(runtime_config).build()?;

    render_docs(config).await?;

    Ok(())
}
