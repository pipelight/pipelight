// Tests
mod test;

mod finder;
mod run;
mod self_process;

// Re-export
pub use finder::Finder;

use serde::{Deserialize, Serialize};
use sysinfo::{self, PidExt, ProcessExt};
use uuid::Uuid;
// Struct
use crate::{Io, State};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SelfProcess;

/**
* A struct that stores the process attributes for further access and manipulation.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Process {
    pub uuid: Option<Uuid>,
    pub pid: Option<i32>,
    pub state: State,
    pub io: Io,
    pub cwd: Option<String>,
}
impl Default for Process {
    /**
     * Create a process struct without an initial command.
     */
    fn default() -> Process {
        let uuid = Some(Uuid::new_v4());
        Process {
            uuid,
            pid: None,
            cwd: None,
            io: Io {
                uuid,
                stdin: None,
                ..Io::default()
            },
            state: State::default(),
        }
    }
}
impl Process {
    /**
     * Create a process struct with an initial command.
     */
    pub fn new(stdin: &str) -> Process {
        let uuid = Some(Uuid::new_v4());
        Process {
            uuid,
            pid: None,
            cwd: None,
            io: Io {
                uuid,
                stdin: Some(stdin.to_owned()),
                ..Io::default()
            },
            state: State::default(),
        }
    }
}

/**
* Convert a sysinfo::Process struct into a pipelight::Process struct
*/
impl From<&sysinfo::Process> for Process {
    fn from(proc: &sysinfo::Process) -> Process {
        Process {
            cwd: Some(proc.cwd().to_str().unwrap().to_owned()),
            pid: Some(proc.pid().as_u32() as i32),
            ..Process::new(&proc.cmd().join(" "))
        }
    }
}
