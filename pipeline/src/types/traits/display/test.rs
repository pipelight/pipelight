#[cfg(test)]
mod tests {
    use crate::types::*;
    use log::LevelFilter;
    use utils::logger::logger;

    #[test]
    fn display_event() {
        logger.load().level(&LevelFilter::Trace);
        let event = Event::new();
        println!("{}", event);
    }
    #[test]
    fn display_command() {
        logger.load().level(&LevelFilter::Trace);
        let command = Command::default();
        println!("{}", command);
    }
    #[test]
    fn display_step() {
        logger.load().level(&LevelFilter::Trace);
        let step = Step::default();
        println!("{}", step);
    }
    #[test]
    fn display_pipeline() {
        logger.load().level(&LevelFilter::Trace);
        let pipeline = Pipeline::new();
        println!("{}", pipeline);
    }
}
