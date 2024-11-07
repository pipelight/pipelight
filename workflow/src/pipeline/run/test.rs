#[cfg(test)]
mod test {
    use crate::types::{Command, Parallel, Pipeline, Step, StepOrParallel};
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
    fn run_default() -> Result<()> {
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
    #[test]
    fn run_parallel() -> Result<()> {
        let mut p = Pipeline {
            name: "test".to_owned(),
            steps: vec![StepOrParallel::Parallel(Parallel {
                steps: vec![
                    Step {
                        name: "test".to_owned(),
                        commands: vec![Command::new("sleep 3")],
                        ..Default::default()
                    },
                    Step {
                        name: "test".to_owned(),
                        commands: vec![Command::new("sleep 3")],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            })],
            ..Default::default()
        };
        p.run()?;
        println!("{:#?}", p);
        Ok(())
    }
}
