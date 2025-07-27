use color_eyre::Result;
use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{LogLevel, Verbosity, VerbosityFilter};
use snakedown::{
    config::{Config, ConfigBuilder},
    render::SSG,
};

#[allow(dead_code)]
pub struct CustomLogLevel {}

impl LogLevel for CustomLogLevel {
    fn default_filter() -> VerbosityFilter {
        VerbosityFilter::Error
    }
    fn quiet_help() -> Option<&'static str> {
        Some("suppress all logging output")
    }
    fn quiet_long_help() -> Option<&'static str> {
        Some("Suppress the logging output of the application, including errors.")
    }
    fn verbose_help() -> Option<&'static str> {
        Some("Increase verbosity of the logging (can be specified multiple times).")
    }
    fn verbose_long_help() -> Option<&'static str> {
        Some(
            "Increase the logging verbosity of the application by one level (ERROR, WARN, INFO, DEBUG, TRACE)",
        )
    }
}

pub fn resolve_runtime_config(args: Args) -> Result<Config> {
    let mut config_builder = ConfigBuilder::default();

    if let Some(config_file_path) = discover_config_file(args.config_file) {
        let file_config_builder = ConfigBuilder::from_path(&config_file_path)?;
        config_builder = config_builder.merge(file_config_builder);
    }

    let cli_args_builder = ConfigBuilder::default()
        .with_output_dir(args.output_dir)
        .with_pkg_path(args.pkg_path)
        .with_skip_undoc(if args.skip_undoc { Some(true) } else { None })
        .with_skip_private(if args.skip_private { Some(true) } else { None })
        .with_exclude(args.exclude)
        .with_ssg(args.ssg);

    config_builder = config_builder.merge(cli_args_builder);

    config_builder.build()
}

pub fn discover_config_file(arg_config_path: Option<PathBuf>) -> Option<PathBuf> {
    let mut candidates = vec![];

    if let Some(args_path) = arg_config_path {
        candidates.push(args_path);
    }

    candidates.push(PathBuf::from("snakedown.toml"));
    candidates.push(PathBuf::from("$HOME/.config/snakedown/snakedown.toml"));
    candidates
        .into_iter()
        .find(|candidate| candidate.exists() && candidate.is_file())
}

#[derive(Parser)]
#[command(version, about, long_about= None)]
pub struct Args {
    #[command(flatten)]
    pub verbose: Verbosity,

    /// The path of the root of the package
    pub pkg_path: Option<PathBuf>,

    /// The directory where to put the rendered docs
    pub output_dir: Option<PathBuf>,

    /// The path to the configuration file
    #[arg(long, short)]
    pub config_file: Option<PathBuf>,

    /// Do not render undocumented functions, classes or modules
    #[arg(long, default_value_t = false)]
    pub skip_undoc: bool,

    /// Do not render private functions, classes or modules (who's name start with _)
    #[arg(long, default_value_t = false)]
    pub skip_private: bool,

    /// Any files that should be excluded, can be file or directories and specific multiple times but currently globs are not supported
    #[arg(short, long)]
    pub exclude: Option<Vec<PathBuf>>,

    /// What format to render the front matter in, (zola, hugo, plain markdown, etc.)
    #[arg(short, long, value_enum)]
    pub ssg: Option<SSG>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use clap_verbosity_flag::log::Level;
    use color_eyre::Result;

    #[test]
    fn test_custom_log_level_interface() -> Result<()> {
        // Explicitly call each method to ensure they are covered
        assert_eq!(CustomLogLevel::default_filter(), VerbosityFilter::Error);
        assert_eq!(
            CustomLogLevel::quiet_help(),
            Some("suppress all logging output")
        );
        assert_eq!(
            CustomLogLevel::quiet_long_help(),
            Some("Suppress the logging output of the application, including errors.")
        );
        assert_eq!(
            CustomLogLevel::verbose_help(),
            Some("Increase verbosity of the logging (can be specified multiple times).")
        );
        assert_eq!(
            CustomLogLevel::verbose_long_help(),
            Some(
                "Increase the logging verbosity of the application by one level (ERROR, WARN, INFO, DEBUG, TRACE)"
            )
        );
        Ok(())
    }

    #[test]
    fn test_args_defaults() -> Result<()> {
        let args = Args::parse_from(["snakedown"]);
        assert!(args.pkg_path.is_none());
        assert!(args.output_dir.is_none());
        assert!(!args.skip_undoc);
        assert!(!args.skip_private);
        assert!(args.exclude.is_none());
        assert!(args.ssg.is_none());
        Ok(())
    }

    #[test]
    fn test_args_all_flags() -> Result<()> {
        let args = Args::parse_from([
            "snakedown",
            "src/pkg",
            "dist",
            "--skip-undoc",
            "--skip-private",
            "--exclude",
            "path/to/exclude1",
            "--exclude",
            "path/to/exclude2",
            "--ssg",
            "markdown",
            "-v",
            "-v",
        ]);
        assert_eq!(args.pkg_path, Some(PathBuf::from("src/pkg")));
        assert_eq!(args.output_dir, Some(PathBuf::from("dist")));
        assert!(args.skip_undoc);
        assert!(args.skip_private);
        assert_eq!(
            args.exclude,
            Some(vec![
                PathBuf::from("path/to/exclude1"),
                PathBuf::from("path/to/exclude2")
            ])
        );
        // Verbosity should be INFO with -v -v (test indirectly)
        let level = args.verbose.log_level();
        assert_eq!(level, Some(Level::Info));
        assert_eq!(args.ssg, Some(SSG::Markdown));
        Ok(())
    }

    #[test]
    fn test_args_exclude_short_flag() -> Result<()> {
        let args = Args::parse_from(["mybin", "-e", "excluded"]);
        assert_eq!(args.exclude, Some(vec![PathBuf::from("excluded")]));
        Ok(())
    }
}
