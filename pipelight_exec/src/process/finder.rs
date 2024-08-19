// Unix process manipulation
use rustix::process::{getpid, kill_process, test_kill_process, Signal};
// use std::collections::HashMap;
use sysinfo::get_current_pid;
use sysinfo::{Process, ProcessRefreshKind, ProcessesToUpdate, System};
// Error handling
use miette::{Context, IntoDiagnostic, Result};
use pipelight_utils::error::{LibError, PipelightError};
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
    pub matches: Option<Vec<crate::Process>>,
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
     * Restrict search result by pid.
     */
    pub fn pid(&mut self, pid: &u32) -> Self {
        self.pid = Some(pid.to_owned());
        self.to_owned()
    }
    /**
     * Guard - Ensure command contains some seed(string)
     */
    fn is_match_seeds(&mut self, process: &Process) -> Result<bool> {
        if let Some(seeds) = self.seeds.clone() {
            for seed in seeds {
                if !process
                    .cmd()
                    .iter()
                    .map(|e| e.to_str().unwrap())
                    .join(" ")
                    .contains(&seed)
                {
                    // println!("{:#?}", process.cmd());
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
    /**
     * Guard -Ensure processes are running in same subdirectory
     */
    fn is_same_root(&mut self, process: &Process) -> Result<bool> {
        if process.cwd().is_some() && self.root.is_some() {
            return Ok(process
                .cwd()
                .unwrap()
                .starts_with(self.root.clone().unwrap()));
        } else {
            return Ok(false);
        }
    }
    /**
     * Guard -Ensure processes are running in same directory
     */
    fn is_same_cwd(&mut self, process: &Process) -> Result<bool> {
        if process.cwd().is_some() && self.root.is_some() {
            return Ok(process.cwd().unwrap().to_str().unwrap() == self.root.clone().unwrap());
        } else {
            return Ok(false);
        }
    }

    /**
     * Search matching processes and hydrate struct with matches.
     */
    pub fn search(&mut self) -> Result<Self> {
        let mut s = System::new_all();
        s.refresh_processes_specifics(ProcessesToUpdate::All, ProcessRefreshKind::new());

        // Loop through process list
        let mut matches: Vec<crate::Process> = vec![];
        if let Some(pid) = self.pid {
            let sysinfo_pid = sysinfo::Pid::from_u32(pid);
            if let Some(process) = s.processes().get(&sysinfo_pid) {
                if self.is_match_seeds(process)? {
                    matches.push(crate::Process::from(process));
                }
            }
        } else {
            for (pid, process) in s.processes() {
                // println!("{:?}", process.cmd());
                // println!("{:?}", process.cwd());

                // Guard - Ensure processes are running in same subdirectory
                let cond_root: bool = self.is_same_root(process)?;

                // Guard - Ensure processes are running in same directory
                let cond_pwd: bool = self.is_same_cwd(process)?;

                // Guard - Ensure command contains some seed(string)
                let mut cond_seed = false;
                if let Some(seeds) = self.seeds.clone() {
                    cond_seed = self.is_match_seeds(process)?;
                };

                // Guard - Ensure different process from the one alredy running
                let cond_other = pid != &get_current_pid().unwrap();

                // println!("{:?}", cond_pwd);
                // Final resolution
                if cond_pwd && cond_seed && cond_other && cond_root {
                    // println!("{:?}", process.cmd());
                    // println!("{:?}", process.cwd());
                    // matches.push(pid.as_u32());
                    matches.push(crate::Process::from(process));
                }
            }
        }
        if !matches.is_empty() {
            self.matches = Some(matches);
        }

        Ok(self.to_owned())
    }

    /**
     * Kill matches if any.
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

#[cfg(test)]
mod test {
    use crate::{Finder, Process};
    use std::env;
    // Error handling
    use miette::{IntoDiagnostic, Result};

    #[test]
    /**
     * Run a simple process, detach it and find it back.
     */
    fn find_and_kill_random_process() -> Result<()> {
        let mut process = Process::new("sleep 12");
        process.run_detached()?;

        let finder = Finder::new()
            .root(env::current_dir().into_diagnostic()?.to_str().unwrap())
            .seed("sleep")
            .search()?;
        finder.kill()?;

        assert_eq!(finder.clone().matches.unwrap().len(), 1);

        Ok(())
    }
}
