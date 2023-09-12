use crate::types::{Environment, Process, State};
use std::env;
use std::fs;
use std::path::Path;
use uuid::Uuid;

impl Default for Process {
    fn default() -> Self {
        Process {
            uuid: Uuid::new_v4(),
            state: State::default(),
            os: Environment::default(),
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
        let dir_path = ".pipelight/_internals/out";
        let path = Path::new(&dir_path);
        fs::create_dir_all(path).unwrap();
        Environment {
            directory: dir_path.to_owned(),
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
        os_env
    }
    /// Return user session shell when possible
    fn get_shell(&mut self) -> String {
        let shell_result = env::var("SHELL");
        match shell_result {
            Ok(res) => {
                self.shell = res;
                self.shell.clone()
            }
            Err(_) => self.shell.clone(),
        }
    }
}

impl State {
    pub fn new() -> State {
        Self::default()
    }
}
