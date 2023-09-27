#[cfg(test)]
mod pipeline {
    use crate::types::{Command, Pipeline, Step, StepOrParallel};
    #[test]
    fn can_run() {
        // Set a logger
        let mut p = Pipeline {
            steps: vec![StepOrParallel::Step(Step {
                commands: vec![Command::new("echo test")],
                ..Step::default()
            })],
            ..Pipeline::default()
        };
        assert!(p.run().is_ok());
    }
    #[test]
    fn concistent_logs() {
        // Set a logger
        let mut p = Pipeline {
            steps: vec![StepOrParallel::Step(Step {
                commands: vec![Command::new("echo test")],
                ..Step::default()
            })],
            ..Pipeline::default()
        };
        p.run().unwrap();
    }
}
