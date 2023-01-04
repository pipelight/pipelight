use super;
use cast;
use std::convert::From;

impl From<&cast::Pipeline> for Pipeline {
    fn from(e: &cast::Pipeline) -> Self {
        let steps = e.steps.iter().map(|e| Step::from(e)).collect::<Vec<Step>>();
        let triggers = e
            .triggers
            .iter()
            .map(|e| Triggers::from(e))
            .collect::<Vec<Triggers>>()
            .flatten()
            .collect();
        let p = Pipeline {
            pid: None,
            uuid: Uuid::new_v4(),
            date: Some(Utc::now().to_string()),
            name: e.name.to_owned(),
            steps: steps,
            status: None,
            triggers: Some(triggers),
        };
        return p;
    }
}

impl From<&cast::Step> for Step {
    fn from(e: &Step) -> Self {
        let commands = e
            .commands
            .iter()
            .map(|e| Command::from(e))
            .collect::<Vec<Command>>();
        Step {
            name: e.clone().name,
            commands: commands,
            non_blocking: e.non_blocking,
            on_failure: e.clone().on_failure,
        }
    }
}

impl From<&String> for Command {
    fn from(s: &String) -> Self {
        Command {
            stdin: s.to_owned(),
            output: None,
        }
    }
}

impl From<&Output> for StrOutput {
    fn from(s: &Output) -> Self {
        let stdout = String::from_utf8(s.clone().stdout).unwrap().to_owned();
        let stderr = String::from_utf8(s.clone().stderr).unwrap().to_owned();
        return StrOutput {
            status: s.status.success(),
            stdout: Some(stdout),
            stderr: Some(stderr),
        };
    }
}

impl From<&cast::Trigger> for Trigger {
    fn from(e: &cast::Trigger) -> Self {
        let mut list: Vec<Trigger> = vec![];
        for branch in e.branches {
            for action in e.actions {
                list.push(Trigger {
                    branch: Some(branch.to_owned()),
                    action: Some(action.to_owned()),
                })
            }
        }
        return list;
    }
}
