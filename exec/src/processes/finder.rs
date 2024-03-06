// Unix process manipulation
use rustix::process::{getpgid, kill_process_group, test_kill_process_group, Signal};
// use std::collections::HashMap;
use sysinfo::get_current_pid;
use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};
// Error handling
use miette::{IntoDiagnostic, Result};
// Env
use itertools::Itertools;
use std::env;

// Utilities to find a running process easily
#[derive(Debug, Clone)]
pub struct Finder {
    // Search arguments
    pub seeds: Option<Vec<String>>,
    cwd: Option<String>,
    pid: Option<u32>,
    // Search results
    pub matches: Option<Vec<u32>>,
}

impl Default for Finder {
    fn default() -> Self {
        let path = env::current_dir().unwrap();
        let path = path.to_str().unwrap();
        Finder {
            seeds: None,
            pid: None,
            matches: None,
            cwd: Some(path.to_owned()),
        }
    }
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
        if let Some(mut seeds) = self.seeds.clone() {
            seeds.push(cmd.to_owned());
            self.seeds = Some(seeds);
        } else {
            self.seeds = Some(vec![cmd.to_owned()]);
        }
        self.to_owned()
    }
    /**
    Restrict search result by pid.
    */
    pub fn pid(&mut self, pid: &u32) -> Self {
        self.pid = Some(pid.to_owned());
        self.to_owned()
    }
    pub fn is_match_seeds(&mut self, process: &Process) -> Result<bool> {
        // Guard - Ensure command contains some seed(string)
        if let Some(seeds) = self.seeds.clone() {
            for seed in seeds {
                if !process.cmd().iter().join(" ").contains(&seed) {
                    // println!("{:#?}", process.cmd());
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /**
    Search matching processes and hydrate struct with matches.
    */
    pub fn search(&mut self) -> Result<Self> {
        let mut sys = System::new();
        sys.refresh_processes();
        // Loop through process list
        let mut matches: Vec<u32> = vec![];
        if let Some(pid) = self.pid {
            let sysinfo_pid = sysinfo::Pid::from_u32(pid);
            if let Some(process) = sys.processes().get(&sysinfo_pid) {
                if self.is_match_seeds(process)? {
                    matches.push(pid);
                }
            }
        } else {
            for (pid, process) in sys.processes() {
                // Guard - Ensure processes are running in same directory
                let mut cond_pwd: bool = false;
                if let Some(process_cwd) = self.cwd.clone() {
                    cond_pwd = process.cwd().to_str().unwrap() == self.cwd.clone().unwrap();
                }
                // Guard - Ensure command contains some seed(string)
                let mut cond_seed = false;
                if let Some(seeds) = self.seeds.clone() {
                    cond_seed = self.is_match_seeds(process)?;
                };

                // Guard - Ensure different process from the one alredy running
                let cond_other = pid != &get_current_pid().unwrap();

                // Final resolution
                if cond_pwd && cond_seed && cond_other {
                    matches.push(pid.as_u32());
                }
            }
        }
        if !matches.is_empty() {
            self.matches = Some(matches);
        }

        Ok(self.to_owned())
    }

    /**
    Kill processes if founded any.
    */
    pub fn kill(&self) -> Result<()> {
        if let Some(matches) = self.matches.clone() {
            for pid in matches {
                let rustix_pid = rustix::process::Pid::from_raw(pid.try_into().into_diagnostic()?);

                let pgid = getpgid(rustix_pid).into_diagnostic()?;
                if test_kill_process_group(pgid).is_ok() {
                    kill_process_group(pgid, Signal::Kill).into_diagnostic()?;
                }
            }
        }
        Ok(())
    }
}
