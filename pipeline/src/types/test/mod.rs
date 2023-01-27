#[cfg(test)]
mod tests {
    use crate::types::Pipeline;
    use log::LevelFilter;
    use utils::logger::logger;

    #[test]
    fn run() {
        logger.load().level(&LevelFilter::Trace);
        let pipeline = Pipeline::new();
        pipeline.run()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Parallel;
    use log::LevelFilter;
    use utils::logger::logger;

    #[test]
    fn run_parallel() {
        // Set logger
        logger.load().level(&LevelFilter::Trace);

        let Parallel = Parallel::new();
        parallel.run()
    }
}
