// Tests

#[cfg(feature = "fd")]
mod fd;
mod finder;
mod run;

// Re-export
pub use finder::Finder;

use bon::{bon, builder};

// Unix process manipulation
use sysinfo::get_current_pid;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
// Struct
use crate::{Io, State};

/**
* A struct that stores the process attributes for further access and manipulation.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Runner {
    uuid: Uuid,
    term: bool,
    background: bool,
    detach: bool,
    fs: bool,
}
impl Default for Runner {
    fn default() -> Self {
        Runner {
            uuid: Uuid::new_v4(),
            term: false,
            background: false,
            detach: false,
            fs: false,
        }
    }
}

impl Runner {
    fn new(uuid: Uuid) -> Self {
        Runner {
            uuid,
            term: false,
            background: false,
            detach: false,
            fs: false,
        }
    }
}
/**
* A struct that stores the process attributes for further access and manipulation.
*
* Example:
*
* ```rust
* # use pipelight_exec::Process;
* # use miette::Report;
*
* let proc = Process::new()
*   .stdin("ls -al")
*   .fs()
*   .run()?;
*
* let stdout = proc.io.stdout;
* let stderr = proc.io.stderr;
*
* # Ok::<(), Report>(())
* ```
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Process {
    pub uuid: Uuid,
    #[serde(skip_serializing, skip_deserializing)]
    pub config: Runner,
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
        let uuid = Uuid::new_v4();
        Process {
            uuid,
            config: Runner::new(uuid),
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
     * Create a default process struct.
     */
    pub fn new() -> Self {
        Default::default()
    }
    pub fn stdin(&mut self, stdin: &str) -> Self {
        self.io.stdin = Some(stdin.to_owned());
        self.to_owned()
    }
    pub fn term(&mut self) -> Self {
        self.config.term = true;
        self.to_owned()
    }
    pub fn background(&mut self) -> Self {
        self.config.background = true;
        self.to_owned()
    }
    pub fn detach(&mut self) -> Self {
        self.config.detach = true;
        self.to_owned()
    }
    pub fn fs(&mut self) -> Self {
        self.config.fs = true;
        self.to_owned()
    }
}

impl Process {
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
            ..Process::new().stdin(
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
