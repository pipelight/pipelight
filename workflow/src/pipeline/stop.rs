// Structs
use crate::types::{Pipeline, Status};
// Error Handling
use miette::{IntoDiagnostic, Result};
// Unix process manipiulation
use rustix::process::{kill_process_group, Signal};

/**
Abort process execution
Kil the process group
*/
impl Pipeline {
    pub fn stop(&mut self) -> Result<()> {
        if self.event.is_some() && self.status == Some(Status::Running) {
            let _pid = self.clone().event.unwrap().pid.unwrap();
            unsafe {
                let pgid_raw = self.event.clone().unwrap().pgid.unwrap();
                let pgid = rustix::process::Pid::from_raw(pgid_raw).unwrap();
                kill_process_group(pgid, Signal::Term).into_diagnostic()?
            }
            self.status = Some(Status::Aborted);
            self.log();
        }
        Ok(())
    }
}
