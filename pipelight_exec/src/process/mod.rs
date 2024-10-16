// Tests

// #[cfg(feature = "procfs")]
mod default;

mod test;

mod finder;
mod run;
mod self_process;

// Unix process manipulation
use sysinfo::get_current_pid;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

// Re-export
pub use finder::Finder;

use serde::{Deserialize, Serialize};
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
    // process parent id
    pub ppid: Option<i32>,
    pub gid: Option<i32>,
    pub sid: Option<i32>,
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
            ppid: None,
            gid: None,
            sid: None,
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
            ppid: None,
            gid: None,
            sid: None,
            cwd: None,
            io: Io {
                uuid,
                stdin: Some(stdin.to_owned()),
                ..Io::default()
            },
            state: State::default(),
        }
    }
    /**
     * Get process from Pid
     */
    pub fn get_from_pid(pid: &i32) -> Process {
        let mut s = System::new_all();
        s.refresh_processes_specifics(ProcessesToUpdate::All, ProcessRefreshKind::new());
        let res = s
            .process(sysinfo::Pid::from_u32(
                u32::try_from(pid.to_owned()).unwrap(),
            ))
            .unwrap()
            .to_owned();

        res.into()
    }
}

/**
* Convert a rustix::Process struct into a pipelight::Process struct
*/
impl From<&sysinfo::Process> for Process {
    fn from(proc: &sysinfo::Process) -> Process {
        let mut p = Process {
            pid: Some(proc.pid().as_u32() as i32),
            ppid: Some(proc.parent().unwrap().as_u32() as i32),
            gid: Some(*proc.group_id().unwrap().to_owned() as i32),
            sid: Some(proc.session_id().unwrap().as_u32() as i32),
            ..Process::new(
                &proc
                    .cmd()
                    .iter()
                    .map(|e| e.to_str().unwrap().to_owned())
                    .collect::<Vec<String>>()
                    .join(" "),
            )
        };
        if proc.cwd().is_some() {
            p.cwd = Some(proc.cwd().unwrap().to_str().unwrap().to_owned());
        }
        p
    }
}
