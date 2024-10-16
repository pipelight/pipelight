// Structs
use crate::dates::Duration;
use crate::{Io, Process, State, Status};

use pipelight_error::PipelightError;
// Unix process manipulation
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

// File manipulation
use procfs::process::{FDTarget, Process as UnixProcess};
use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::io::Read;
use std::os::fd::FromRawFd;

// Globals
use crate::globals::{get_shell, OUTDIR, SHELL};

// Error Handling
use log::info;
use miette::{IntoDiagnostic, Result};

impl Process {
    /**
     * Run a detached process
     */
    pub fn run(&mut self) -> Result<Self, PipelightError> {
        info!("Run detached subprocess");
        get_shell()?;
        let mut duration = Duration::default();

        let mut args: Vec<String> = self
            .io
            .stdin
            .clone()
            .unwrap()
            .split(" ")
            .map(|e| e.to_owned())
            .collect();

        let mut cmd = Command::new(args.remove(0));
        cmd.args(args);
        cmd.process_group(0);

        duration.start();
        let child = cmd.spawn()?;

        // Update proc pid
        self.pid = Some(child.id() as i32);

        duration.stop();
        self.state = State {
            duration: Some(duration),
            status: Some(Status::Succeeded),
        };
        Ok(self.to_owned())
    }

    /**
     * Read process I/O file descriptors (stdout/stderr)
     * and update struct field accordingly.
     */
    pub fn update(&mut self) -> Result<Self, PipelightError> {
        if let Some(pid) = self.pid {
            let proc = UnixProcess::new(pid).unwrap();
            match proc.fd_from_fd(1).unwrap().target {
                FDTarget::Path(x) => {
                    let f = File::open(x)?;
                    let mut buf_reader = BufReader::new(f);
                    let mut stdout = String::new();
                    buf_reader.read_to_string(&mut stdout)?;
                    self.io.stdout = Some(stdout);
                }
                _ => {}
            };
        }

        Ok(self.to_owned())
    }
}
