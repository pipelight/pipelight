// Unix process manipulation
use rustix::process::{getpid, kill_process, test_kill_process, Signal};
// use std::collections::HashMap;
use sysinfo::get_current_pid;
use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};
// Error handling
use crate::error::{LibError, PipelightError};
use miette::{Context, IntoDiagnostic, Result};
// Env
use itertools::Itertools;
use std::env;

// Utilities to find a running process easily
#[derive(Debug, Clone)]
pub struct Finder {
    // Search arguments
    pub seeds: Option<Vec<String>>,
    root: Option<String>,
    cwd: Option<String>,
    pid: Option<u32>,
    // Search results
    pub matches: Option<Vec<crate::exec::Process>>,
}

impl Default for Finder {
    fn default() -> Self {
        Finder {
            // A root directory to search from
            root: None,
            cwd: None,
            seeds: None,
            pid: None,
            matches: None,
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
    Restrict search result by a root directory.
    */
    pub fn root(&mut self, root: &str) -> Self {
        self.root = Some(root.to_owned());
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
        let mut matches: Vec<crate::exec::Process> = vec![];
        if let Some(pid) = self.pid {
            let sysinfo_pid = sysinfo::Pid::from_u32(pid);
            if let Some(process) = sys.processes().get(&sysinfo_pid) {
                if self.is_match_seeds(process)? {
                    matches.push(crate::exec::Process::from(process));
                }
            }
        } else {
            for (pid, process) in sys.processes() {
                // Guard - Ensure processes are running in same subdirectory
                let mut cond_root: bool = false;
                if let Some(root) = self.root.clone() {
                    cond_root = process.cwd().starts_with(self.root.clone().unwrap());
                } else {
                    cond_root = true;
                }
                // Guard - Ensure processes are running in same directory
                let mut cond_pwd: bool = false;
                if let Some(process_cwd) = self.cwd.clone() {
                    cond_pwd = process.cwd().to_str().unwrap() == self.cwd.clone().unwrap();
                } else {
                    cond_pwd = true;
                }
                // Guard - Ensure command contains some seed(string)
                let mut cond_seed = false;
                if let Some(seeds) = self.seeds.clone() {
                    cond_seed = self.is_match_seeds(process)?;
                };

                // Guard - Ensure different process from the one alredy running
                let cond_other = pid != &get_current_pid().unwrap();

                // Final resolution
                if cond_pwd && cond_seed && cond_other && cond_root {
                    println!("{:?}", process.cmd());
                    println!("{:?}", process.cwd());
                    // matches.push(pid.as_u32());
                    matches.push(crate::exec::Process::from(process));
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
    pub fn kill(&self) -> Result<(), PipelightError> {
        if let Some(matches) = self.matches.clone() {
            for process in matches {
                let pid = rustix::process::Pid::from_raw(process.pid.unwrap());
                if test_kill_process(pid.unwrap()).is_ok() {
                    match kill_process(pid.unwrap(), Signal::Kill) {
                        Ok(_) => return Ok(()),
                        Err(e) => {
                            return Err(LibError {
                                message: "Couldn't kill process".to_owned(),
                                help: "".to_owned(),
                            }
                            .into());
                        }
                    };
                }
            }
        }
        Ok(())
    }
}
