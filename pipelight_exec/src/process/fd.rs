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
     * Read process I/O file descriptors (stdout/stderr)
     * and update struct field accordingly.
     */
    pub fn read_fds(&mut self) -> Result<Self, PipelightError> {
        if let Some(pid) = self.pid {
            let proc = UnixProcess::new(pid).unwrap();
            println!("{:#?}", proc);
            println!("{:#?}", proc.fd_from_fd(1));
            match proc.fd_from_fd(1).unwrap().target {
                FDTarget::Path(x) => {
                    let f = File::open(x)?;
                    let mut buf_reader = BufReader::new(f);
                    let mut stdout = String::new();
                    buf_reader.read_to_string(&mut stdout)?;
                    self.io.stdout = Some(stdout);
                }
                FDTarget::Pipe(x) => {
                    println!("{:#?}", x);
                }
                x => {
                    println!("{:#?}", x);
                }
            };
        }
        Ok(self.to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
