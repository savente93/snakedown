use color_eyre::eyre::Result;
use snakedown::{indexing::fetch::cache_remote_objects_inv, render_docs};
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
    for (key, external_index) in config.externals {
        tracing::debug!("fetching: {}", key);
        cache_remote_objects_inv(
            &external_index.url,
            external_index.name.unwrap_or(key),
            None,
            false,
        )
        .await?;
    }
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
