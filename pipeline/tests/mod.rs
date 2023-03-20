#[cfg(test)]
mod tests {
    use log::LevelFilter;
    use pipeline::cast;
    use pipeline::types;
    use utils::logger::logger;

    #[test]
    /// Loads a Good ts config with a simple pipeline
    fn load_good_config() {
        cast::Config::load_from_file_ts("./tests/files/good-config.ts").unwrap();
    }
    #[test]
    /// Loads a ts config with a simple pipeline
    /// With missing required fields
    fn load_config_missing_required() {
        let res = cast::Config::load_from_file_ts("./tests/files/bad-config.ts");
        assert!(res.is_err());
    }
}
