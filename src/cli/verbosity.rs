use clap_verbosity_flag::{LogLevel, VerbosityFilter};

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

#[cfg(test)]
mod test {
    use super::CustomLogLevel;
    use clap_verbosity_flag::{LogLevel, VerbosityFilter};
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
}
