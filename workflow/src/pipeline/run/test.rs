#[cfg(test)]
mod test {
    use crate::types::{Command, Pipeline, Step, StepOrParallel};
    use miette::Result;

    #[test]
    fn can_run() {
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
    fn default() -> Result<()> {
        let mut p = Pipeline {
            name: "test".to_owned(),
            steps: vec![StepOrParallel::Step(Step {
                name: "test".to_owned(),
                commands: vec![Command::new("sleep 5"), Command::new("pwd")],
                ..Default::default()
            })],
            ..Default::default()
        };
        p.run()?;
        println!("{:#?}", p);
        Ok(())
    }
}
