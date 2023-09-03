#[cfg(test)]
mod get_config {
    use crate::Config;

    #[test]
    fn get_config_file() {
        let config = Config::get(None, None);
        assert!(config.is_ok());
    }
}
