#[cfg(test)]
mod get_config {
    use crate::Config;
    use utils::teleport::Teleport;
    // Error Handling
    use miette::{IntoDiagnostic, Result};

    #[test]
    fn get_config_file() -> Result<()> {
        let config = Config::get(None, None);
        assert!(config.is_ok());
        Ok(())
    }
}
