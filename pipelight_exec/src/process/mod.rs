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
    term: bool,
    background: bool,
    detach: bool,
    save_outputs: bool,
}
impl Runner {
    fn new() {}
    /*
     * Request execution in a terminal.
     * Should be used if complex set of arguments can't be parsed with the default method.
     */
    fn term(&mut self) {
        self.term = true;
    }
    /*
     * Redirect and save process i/o to readable file.
     */
    fn (&mut self) {
        self.term = true;
    }
    /*
     * Request the process to be executed in the background,
     * - spawn the process and do not wait for completion.
     */
    fn background(&mut self) {
        self.background = true;
    }
}
/**
* A struct that stores the process attributes for further access and manipulation.
* ```rust
* let proc = Process::new()
*   .stdin("ls -al")
*   .term()
*   .save()
*   .background()
*   .detached()
*   .run()?;
*
* let stdout = proc.stdout();
* let stderr = proc.stderr();
* ```
*
* Process
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Process {
    #[serde(skip_serializing)]
    pub config: Runner,
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
            config: Default::default(),
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
}
#[bon]
impl Process {
    #[builder]
    fn run(&mut self, term: bool, background: bool, detached: bool, fs: Option<bool>) {
        if term && detached {}
        if let Some(term) = term {
            self.run_term()
        }
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
