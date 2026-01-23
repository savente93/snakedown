use std::path::PathBuf;

use color_eyre::eyre::{Result, eyre};
use snakedown::{config::ConfigBuilder, render_docs};
use tracing::subscriber::set_global_default;

mod cli;

use crate::cli::{CliArgs, SubCommand, init::wizard as init_wizard, resolve_runtime_config};
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

    let command = args.subcommand.clone();

    match command {
        Some(SubCommand::Init) => {
            let output_toml_path = PathBuf::from("snakedown.toml");

            if output_toml_path.exists() {
                Err(eyre!("`snakedown.toml` already exists, exiting..."))?
            }
            let init_config = init_wizard()?;
            init_config.to_snakedown_toml(&output_toml_path)?;
        }

        None => {
            tracing::debug_span!("resolving runtime config");
            let default_config = ConfigBuilder::default().init_with_defaults();
            let runtime_config = resolve_runtime_config(args)?;

            let config_builder = default_config.merge(runtime_config);
            let config = config_builder.build()?;
            let _ = render_docs(config).await?;
        }
    };

    Ok(())
}
