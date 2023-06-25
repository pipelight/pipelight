use crate::types::{Environment, Process, State};
use std::env;

impl Default for Process {
    fn default() -> Self {
        Process {
            state: State::default(),
            env: Environment::default(),
        }
    }
}
impl Process {
    pub fn new(stdin: &str) -> Process {
        Process {
            state: State {
                stdin: Some(stdin.to_owned()),
                ..State::default()
            },
            ..Self::default()
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            shell: "sh".to_owned(),
            attached: true,
            pid: None,
        }
    }
}
impl Environment {
    pub fn new() -> Environment {
        let mut os_env = Environment { ..Self::default() };
        os_env.get_shell();
        return os_env;
    }
    /// Return user session shell when possible
    fn get_shell(&mut self) -> String {
        let shell_result = env::var("SHELL");
        match shell_result {
            Ok(res) => {
                self.shell = res;
                return self.shell.clone();
            }
            Err(_) => {
                return self.shell.clone();
            }
        };
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            status: None,
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }
}
impl State {
    pub fn new() -> State {
        Self::default()
    }
}
