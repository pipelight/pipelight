// Unix process manipulation
use rustix::process::{getpgid, kill_process_group, test_kill_process_group, Pid, Signal};
use sysinfo::get_current_pid;
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
// Error handling
use miette::{IntoDiagnostic, Result};
// Env
use std::env;

// Utilities to find a running process easily
#[derive(Default, Debug, Clone)]
pub struct Finder {
    // Search arguments
    seed: Option<String>,
    cwd: Option<String>,
    pid: Option<u32>,
    // Search results
    pub matches: Option<Vec<u32>>,
}

impl Finder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Finder {
    /**
    Restrict search result by current working directory.
    */
    pub fn cwd(&mut self, cwd: &str) -> Self {
        self.cwd = Some(cwd.to_owned());
        self.to_owned()
    }

    /**
    Restrict search result by seed.
    */
    pub fn seed(&mut self, cmd: &str) -> Self {
        self.seed = Some(cmd.to_owned());
        self.to_owned()
    }
    /**
    Restrict search result by pid.
    */
    pub fn pid(&mut self, pid: &u32) -> Self {
        self.pid = Some(pid.to_owned());
        self.to_owned()
    }

    /**
    Search process and hydrate struct with matches.
    */
    pub fn search(&mut self) -> Result<Self> {
        let mut sys = System::new();
        sys.refresh_processes();
        // Loop through process list
        let mut matches: Vec<u32> = vec![];
        for (pid, process) in sys.processes() {
            // Guard - Ensure processes are running in same directory
            let mut cond_pwd = true;
            if let Some(cwd) = self.cwd.clone() {
                cond_pwd = cwd == env::current_dir().into_diagnostic()?.to_str().unwrap();
            }

            // Guard - Ensure command contains some seed(string)
            let mut cond_seed = true;
            if let Some(seed) = self.seed.clone() {
                cond_seed = process.cmd().contains(&seed);
            };

            // Guard - Ensure process has pid
            let mut cond_pid = true;
            if let Some(self_pid) = self.pid {
                let self_pid = sysinfo::Pid::from_u32(self_pid);
                cond_pid = pid == &self_pid;
            }
            // println!("{:#?}", cond_pid);

            // Guard - Ensure different process from the one alredy running
            let cond_other = pid != &get_current_pid().unwrap();

            // Guard: check if pid link to a running programm
            // test_kill_process(rustix_pid).into_diagnostic()?;

            // Final resolution
            if cond_pwd && cond_seed && cond_other && cond_pid {
                matches.push(pid.as_u32());
            }
        }
        if !matches.is_empty() {
            self.matches = Some(matches);
        }

        Ok(self.to_owned())
    }
    /**
    Kill processes if any.
    */
    pub fn kill(&self) -> Result<()> {
        if let Some(matches) = self.matches.clone() {
            for pid in matches {
                let rustix_pid;
                unsafe { rustix_pid = rustix::process::Pid::from_raw(pid) };

                let pgid = getpgid(rustix_pid).into_diagnostic()?;
                if test_kill_process_group(pgid).is_ok() {
                    kill_process_group(pgid, Signal::Term).into_diagnostic()?;
                }
            }
        }
        Ok(())
    }
}
