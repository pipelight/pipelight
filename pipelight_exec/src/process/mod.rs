// Tests

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
    shell: Option<String>,
    uuid: Uuid,
    term: bool,
    background: bool,
    detach: bool,
    orphan: bool,
    fs: bool,
}
impl Default for Runner {
    fn default() -> Self {
        Runner {
            shell: None,
            uuid: Uuid::new_v4(),
            term: false,
            background: false,
            detach: false,
            orphan: false,
            fs: false,
        }
    }
}

impl Runner {
    fn new(uuid: Uuid) -> Self {
        Runner {
            shell: None,
            uuid,
            term: false,
            background: false,
            detach: false,
            orphan: false,
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
            // Add later
            // pgid: None,
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
    pub fn stdin(&mut self, stdin: &str) -> &mut Self {
        self.io.stdin = Some(stdin.to_owned());
        self
    }
    pub fn term(&mut self) -> &mut Self {
        self.config.term = true;
        self
    }

    pub fn shell(&mut self, shell: &str) -> &mut Self {
        self.config.shell = Some(shell.to_owned());
        self
    }
    /*
     * Will spawn the process and not wait for it to return (silent).
     * However,if the parent is killed, child will be killed.
     * background != detached
     *
     * Usually you want detach AND background.
     */
    pub fn background(&mut self) -> &mut Self {
        self.config.background = true;
        self
    }

    pub fn detach(&mut self) -> &mut Self {
        self.config.detach = true;
        self
    }
    /**
     * Detach child process from parent but keep in process group.
     * Killing parent won't kill child but killing parent group will.
     */
    pub fn soft_detach(&mut self) -> &mut Self {
        self.config.detach = true;
        self
    }
    /**
     * Detach child process from parent and remove from process group
     * Neither killing parent or parent group will kill the child.
     *
     * Must be root to create an orphan process.
     *
     */
    pub fn orphan(&mut self) -> &mut Self {
        self.config.orphan = true;
        self
    }
    pub fn fs(&mut self) -> &mut Self {
        self.config.fs = true;
        self
    }
}

impl Process {
    /**
     * Get process from Pid
     */
    pub fn get_from_pid(pid: &i32) -> Process {
        let mut s = System::new_all();
        s.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing()
                .with_cmd(sysinfo::UpdateKind::Always)
                .with_cwd(sysinfo::UpdateKind::Always)
                .with_root(sysinfo::UpdateKind::Always)
                .with_exe(sysinfo::UpdateKind::Always),
        );
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
            ..Process::new()
                .stdin(
                    &proc
                        .cmd()
                        .iter()
                        .map(|e| e.to_str().unwrap().to_owned())
                        .collect::<Vec<String>>()
                        .join(" "),
                )
                .to_owned()
        };
        if proc.cwd().is_some() {
            p.cwd = Some(proc.cwd().unwrap().to_str().unwrap().to_owned());
        }
        p
    }
}
