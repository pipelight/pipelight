#[cfg(test)]
mod tests {
    use crate::types::Pipeline;
    use log::LevelFilter;
    use utils::logger::logger;

    #[test]
    fn run() {
        logger.level(&LevelFilter::Trace);
        let pipeline = Pipeline::new();
        pipeline.run()
    }
}
