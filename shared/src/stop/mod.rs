use exec::types::Status;
use exec::Exec;
use log::trace;
use pipeline::types::Logs;
use std::env;

//sys
use rustix::process::{kill_process_group, Pid, Signal};

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;

/// Stop pipeline and attached pipelines/subprocesses
pub fn stop(pipeline_name: &String) -> Result<()> {
    // Get pipelines with provided name
    let pipelines = Logs::get_many_by_name(&pipeline_name)?;
    for mut pipeline in pipelines {
        pipeline.stop()
    }
    Ok(())
}
