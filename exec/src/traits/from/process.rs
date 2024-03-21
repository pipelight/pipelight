use crate::Process;
use sysinfo::{self, PidExt, ProcessExt};

impl From<&sysinfo::Process> for Process {
    fn from(proc: &sysinfo::Process) -> Process {
        Process {
            cwd: Some(proc.cwd().to_str().unwrap().to_owned()),
            pid: Some(proc.pid().as_u32() as i32),
            ..Process::new(&proc.cmd().join(" "))
        }
    }
}
