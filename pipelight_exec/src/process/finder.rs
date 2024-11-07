// Unix process manipulation
use rustix::process::{getgid, getpid, kill_process, test_kill_process, Signal};
use sysinfo::get_current_pid;
use sysinfo::{Process, ProcessRefreshKind, ProcessesToUpdate, System};
// Error handling
use miette::{Context, IntoDiagnostic, Result};
use pipelight_error::{LibError, PipelightError};
// Env
use itertools::Itertools;
use std::ops::Deref;
use std::{env, i32};

// Utilities to find a running process easily
#[derive(Debug, Clone)]
pub struct Finder {
    // Search arguments
    pub seeds: Option<Vec<String>>,
    root: Option<String>,
    cwd: Option<String>,
    pid: Option<u32>,
    gid: Option<u32>,
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
            gid: None,
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
    fn is_match_seeds(&mut self, process: &Process) -> Result<bool, PipelightError> {
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
     * Permissive: Return true if can not figure out
     */
    fn is_same_root(&mut self, process: &Process) -> Result<bool, PipelightError> {
        if process.cwd().is_some() && self.root.is_some() {
            return Ok(process
                .cwd()
                .unwrap()
                .starts_with(self.root.clone().unwrap()));
        }
        return Ok(true);
    }
    /**
     * Guard -Ensure processes are running in same directory
     * Permissive: Return true if can not figure out
     */
    fn is_same_cwd(&mut self, process: &Process) -> Result<bool, PipelightError> {
        if process.cwd().is_some() && self.cwd.is_some() {
            return Ok(process.cwd().unwrap().to_str().unwrap() == self.cwd.clone().unwrap());
        }
        return Ok(true);
    }

    /**
     * Search matching processes and hydrate struct with matches.
     */
    pub fn search(&mut self) -> Result<Self, PipelightError> {
        let mut s = System::new_all();
        s.refresh_processes_specifics(
            ProcessesToUpdate::All,
            ProcessRefreshKind::new()
                .without_cpu()
                .without_memory()
                .without_disk_usage()
                .without_environ(),
        );

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
            let self_proc =
                crate::Process::get_from_pid(&(get_current_pid().unwrap().as_u32() as i32));

            for (pid, process) in s.processes() {
                // Guard - Ensure processes are running in same subdirectory
                let cond_root: bool = self.is_same_root(process)?;

                // Guard - Ensure processes are running in same directory
                let cond_pwd: bool = self.is_same_cwd(process)?;

                // Guard - Ensure different process from the one already running (pid)
                let cond_other_pid = pid != &get_current_pid().unwrap();

                // Guard - Ensure command contains some seed(string)
                let mut cond_seed = false;
                if let Some(seeds) = self.seeds.clone() {
                    cond_seed = self.is_match_seeds(process)?;
                };

                // Final resolution
                if cond_root && cond_pwd && cond_seed && cond_other_pid {
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
     * Search matching processes and hydrate struct with matches.
     * Ensure the matching process is not a parent of this one.
     */
    pub fn search_no_parents(&mut self) -> Result<Self, PipelightError> {
        let mut s = System::new_all();
        s.refresh_processes_specifics(
            ProcessesToUpdate::All,
            ProcessRefreshKind::new()
                .without_cpu()
                .without_memory()
                .without_disk_usage()
                .without_environ(),
        );

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
            let self_proc =
                crate::Process::get_from_pid(&(get_current_pid().unwrap().as_u32() as i32));

            // println!("{:#?}", self_proc);

            for (pid, process) in s.processes() {
                // Guard - Ensure processes are running in same subdirectory
                let cond_root: bool = self.is_same_root(process)?;

                // Guard - Ensure processes are running in same directory
                let cond_pwd: bool = self.is_same_cwd(process)?;

                // Guard - Ensure different process from the one already running (pid)
                let cond_other_pid = pid != &get_current_pid().unwrap();

                // Guard - Ensure this process is not the parent (ppid)
                let mut cond_other_ppid = false;
                if process.parent().is_some() && self_proc.pid.is_some() {
                    cond_other_ppid =
                        self_proc.pid.unwrap() != process.parent().unwrap().as_u32() as i32;
                }

                // Guard - Ensure different process from the one already running (gid)
                // let mut cond_other_gid = false;
                // if process.group_id().is_some() && self_proc.gid.is_some() {
                //     cond_other_gid = self_proc.gid.unwrap()
                //         != process.group_id().unwrap().deref().to_owned() as i32;
                // }

                // Guard - Ensure different process from the one already running (sid)
                // let mut cond_other_sid = false;
                // if process.session_id().is_some() && self_proc.sid.is_some() {
                //     cond_other_gid =
                //         self_proc.sid.unwrap() != process.session_id().unwrap().as_u32() as i32;
                // }

                // Guard - Ensure command contains some seed(string)
                let mut cond_seed = false;
                if let Some(seeds) = self.seeds.clone() {
                    cond_seed = self.is_match_seeds(process)?;
                };

                // Final resolution
                if cond_root && cond_pwd && cond_seed && cond_other_pid && cond_other_ppid {
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
    use pipelight_error::PipelightError;

    /**
     * Run a simple process, detach it and find it back.
     */
    #[test]
    fn find_and_kill_random_process() -> Result<(), PipelightError> {
        let mut process = Process::new().stdin("sleep 12").background();
        process.detach().run()?;

        let finder = Finder::new()
            .root(env::current_dir()?.to_str().unwrap())
            .seed("sleep 12")
            .search()?;

        println!("{:#?}", finder);
        finder.kill()?;

        assert_eq!(finder.clone().matches.unwrap().len(), 1);

        Ok(())
    }
}
