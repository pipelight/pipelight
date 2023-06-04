use exec::types::Status;
use exec::Exec;
use log::trace;
use pipeline::types::Logs;
use std::env;

//sys
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;

/// Stop pipeline and attached pipelines/subprocesses
pub fn stop(pipeline_name: &String) -> Result<()> {
    // Get pipelines with provided name
    let pipelines = Logs::get_many_by_name(&pipeline_name)?;
    for pipeline in pipelines {
        // Get sid
        if pipeline.event.is_some() {
            if pipeline.event.clone().unwrap().sid.is_some() {
                let sid = pipeline.clone().event.unwrap().sid.unwrap();

                // Get every pipeline with same sid
                let pipelines_sid = Logs::get_many_by_sid(&sid)?;
                for mut p in pipelines_sid {
                    // Set pipeline status
                    p.status = Some(Status::Aborted);
                    p.log();
                }

                // Get every process with same sid
                // and kill process
                let mut sys = System::new_all();
                sys.refresh_all();

                for (pid, process) in sys.processes() {
                    if process.session_id().is_some() {
                        if process.session_id().unwrap() == Pid::from_u32(sid) {
                            process.kill();
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
