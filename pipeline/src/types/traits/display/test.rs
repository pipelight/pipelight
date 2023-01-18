#[cfg(test)]
mod tests {
    use crate::types::*;
    use log::LevelFilter;
    use utils::logger::logger;

    #[test]
    fn display_event() {
        logger.level(&LevelFilter::Trace);
        let event = Event::new();
        println!("");
        println!("{}", event);
    }
    #[test]
    fn display_command() {
        logger.level(&LevelFilter::Trace);
        let command = Command {
            stdin: "ls".to_owned(),
            output: None,
        };
        println!("");
        println!("{}", command);
    }
    #[test]
    fn display_step() {
        logger.level(&LevelFilter::Trace);
        let command = Command {
            stdin: "ls".to_owned(),
            output: None,
        };
        let step = Step {
            name: "my_step".to_owned(),
            commands: vec![command.clone()],
            non_blocking: None,
            on_failure: None,
        };
        println!("");
        println!("{}", step);
    }
    #[test]
    fn display_pipeline() {
        logger.level(&LevelFilter::Trace);
        let pipeline = Pipeline::new();
        println!("");
        println!("{}", pipeline);
    }
}
