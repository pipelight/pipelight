// Unix process manipulation
use rustix::process::{getgid, getpid, kill_process, test_kill_process, Signal};
use sysinfo::get_current_pid;
use sysinfo::{Process, ProcessRefreshKind, ProcessesToUpdate, System};
// Error handling
use log::{trace, warn};
use miette::{Context, IntoDiagnostic, Result};
use pipelight_error::{LibError, PipelightError, WrapError};
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
     * Restrict search result by seed.
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
            let mut matches = vec![];
            for seed in seeds {
                matches.push(
                    process
                        .cmd()
                        .iter()
                        .map(|e| e.to_str().unwrap())
                        .join(" ")
                        .contains(&seed),
                )
            }
            if matches.contains(&false) {
                return Ok(false);
            } else {
                return Ok(true);
            }
        } else {
            Ok(true)
        }
    }
    /**
     * Guard -Ensure processes are running in a same parent directory.
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
     * Guard -Ensure processes are running in the same directory
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
            true,
            // ProcessRefreshKind::everything(),
            ProcessRefreshKind::nothing()
                .with_cmd(sysinfo::UpdateKind::Always)
                .with_cwd(sysinfo::UpdateKind::Always)
                .with_root(sysinfo::UpdateKind::Always)
                .with_exe(sysinfo::UpdateKind::Always),
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
                let cond_seed = self.is_match_seeds(process)?;

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
            true,
            ProcessRefreshKind::nothing()
                .with_cmd(sysinfo::UpdateKind::Always)
                .with_cwd(sysinfo::UpdateKind::Always)
                .with_root(sysinfo::UpdateKind::Always)
                .with_exe(sysinfo::UpdateKind::Always),
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
     * Try to kill matches if any.
     * And stop if one of the processes couldn't be killed.
     *
     * When you want to silently fail use:
     *
     * ```rs
     * kill().ok()
     *
     * ```
     */
    pub fn kill(&self) -> Result<(), PipelightError> {
        if let Some(matches) = self.matches.clone() {
            for process in matches {
                let pid = rustix::process::Pid::from_raw(process.pid.unwrap());
                if test_kill_process(pid.unwrap()).is_ok() {
                    match kill_process(pid.unwrap(), Signal::KILL) {
                        Ok(_) => {
                            trace!("killed process: {:#?}", pid.unwrap());
                        }
                        Err(e) => {
                            warn!("couldn't kill process: {:#?}", pid.unwrap());
                            return Err(LibError {
                                message: "Couldn't kill process".to_owned(),
                                help: e.to_string(),
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
    use super::*;
    use crate::{Finder, Process};
    use std::env;

    use serial_test::serial;

    use std::fs;
    use std::{thread, time};

    /**
     * Run a simple process, detach it and find it back.
     */
    #[test]
    #[serial]
    fn short_seed() -> Result<(), PipelightError> {
        let mut process = Process::new()
            .stdin("sleep 14")
            .background()
            .detach()
            .to_owned();
        process.run()?;
        let mut process = Process::new()
            .stdin("sleep 10")
            .background()
            .detach()
            .to_owned();
        process.run()?;

        let finder = Finder::new()
            .root(env::current_dir()?.to_str().unwrap())
            .seed("sleep 1")
            .search()?;

        finder.kill()?;
        assert_eq!(finder.clone().matches.unwrap().len(), 2);

        Ok(())
    }

    #[test]
    #[serial]
    fn long_seed() -> Result<(), PipelightError> {
        let mut process = Process::new()
            .stdin("sleep 22")
            .background()
            .detach()
            .to_owned();
        process.run()?;

        let mut process = Process::new()
            .stdin("sleep 20")
            .background()
            .detach()
            .to_owned();
        process.run()?;

        let finder = Finder::new()
            .root(env::current_dir()?.to_str().unwrap())
            .seed("sleep 22")
            .search()?;

        // println!("{:#?}", finder);
        finder.kill()?;

        assert_eq!(finder.clone().matches.unwrap().len(), 1);

        Ok(())
    }

    #[test]
    #[serial]
    fn same_root() -> Result<(), PipelightError> {
        let root = env::current_dir()?;
        let root = root.to_str().unwrap();

        let test_dir = "./test_dir_tmp/finder".to_owned();
        let a = test_dir.clone() + "/a";
        let b = test_dir.clone() + "/b";
        for dir in vec![a, b] {
            fs::create_dir_all(&dir)?;
            env::set_current_dir(&dir)?;
            let mut process = Process::new()
                .stdin("sleep 31")
                .background()
                .detach()
                .to_owned();
            process.run()?;
            env::set_current_dir(&root)?;
        }

        let finder = Finder::new().root(root).seed("sleep 31").search()?;
        finder.kill()?;
        println!("{:#?}", finder);
        assert_eq!(finder.clone().matches.unwrap().len(), 2);
        Ok(())
    }

    #[test]
    #[serial]
    fn different_cwd() -> Result<(), PipelightError> {
        let root = env::current_dir()?;
        let root = root.to_str().unwrap().to_owned();

        let test_dir = root.clone() + "./test_dir_tmp/finder";
        let a = test_dir.clone() + "/a";
        let b = test_dir.clone() + "/b";
        for dir in vec![a.clone(), b] {
            fs::create_dir_all(&dir)?;
            env::set_current_dir(&dir)?;
            let mut process = Process::new()
                .stdin("sleep 41")
                .background()
                .detach()
                .to_owned();
            process.run()?;
            env::set_current_dir(&root)?;
        }
        // Wait for propagation
        let throttle = time::Duration::from_millis(1000);
        thread::sleep(throttle);

        // let path = Path::from()
        let finder = Finder::new().cwd(&a).seed("sleep 41").search()?;
        finder.kill()?;

        println!("{:#?}", finder);
        assert_eq!(finder.clone().matches.unwrap().len(), 1);
        Ok(())
    }
}
