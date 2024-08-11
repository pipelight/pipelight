// Structs
use crate::dates::Duration;
use crate::error::PipelightError;
use crate::exec::{Io, Process, State, Status};
// Unix process manipulation
use std::process::{Command, Stdio};

// File manipulation
use std::fs::{create_dir_all, File};
// Globals
use crate::exec::globals::{get_shell, OUTDIR, SHELL};
// Error Handling
use log::info;
use miette::{IntoDiagnostic, Result};

impl Process {
    /**
    Execute/Await a subprocess and inherit the parent process output outputs(stdout/stderr)
    to the parent process.
    */
    pub fn run_inherit(&mut self) -> Result<(), PipelightError> {
        info!("Run subprocess piped to parent");
        get_shell()?;
        let mut duration = Duration::default();
        let child = Command::new(&(*SHELL.lock().unwrap()))
            .arg("-c")
            .arg(self.io.stdin.as_ref().unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Couldn't spawn a detached subprocess");

        // Hydrate struct
        duration.start();
        let output = child.wait_with_output()?;
        duration.stop();
        self.io = Io {
            uuid: self.io.uuid,
            stdin: self.io.stdin.to_owned(),
            ..Io::from(&output)
        };
        self.state = State {
            duration: Some(duration),
            status: Some(Status::from(&output)),
        };
        Ok(())
    }

    /**
    Execute/Await a subprocess and pipe the outputs(stdout/stderr)
    to the parent process.
    */
    pub fn run_piped(&mut self) -> Result<(), PipelightError> {
        info!("Run subprocess piped to parent");
        get_shell()?;
        let mut duration = Duration::default();
        let child = Command::new(&(*SHELL.lock().unwrap()))
            .arg("-c")
            .arg(self.io.stdin.as_ref().unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Couldn't spawn a detached subprocess");

        // Hydrate struct
        duration.start();
        let output = child.wait_with_output()?;
        duration.stop();
        self.io = Io {
            uuid: self.io.uuid,
            stdin: self.io.stdin.to_owned(),
            ..Io::from(&output)
        };
        self.state = State {
            duration: Some(duration),
            status: Some(Status::from(&output)),
        };
        Ok(())
    }

    /**
    Execute/Await a subprocess and pipe the outputs(stdout/stderr)
    to files for further read/write while executing.
    Suits long running processes for early inner inspection of outputs
    whilst it still runs.
    */
    pub fn run_fs(&mut self) -> Result<(), PipelightError> {
        info!("Run subprocess with output piped to pipelight managed files");
        get_shell()?;
        let mut duration = Duration::default();
        // path definition
        create_dir_all(&(*OUTDIR.lock().unwrap()))?;
        let stdout_path = format!("{}/{}_stdout", *OUTDIR.lock().unwrap(), self.uuid.unwrap());
        let stderr_path = format!("{}/{}_stderr", *OUTDIR.lock().unwrap(), self.uuid.unwrap());

        // Ensure internal log dir exists
        let child = Command::new(&(*SHELL.lock().unwrap()))
            .arg("-c")
            .arg(self.io.stdin.as_ref().unwrap())
            .stdin(Stdio::null())
            .stdout(File::create(stdout_path)?)
            .stderr(File::create(stderr_path)?)
            .spawn()?;

        // Hydrate struct
        duration.start();
        let output = child.wait_with_output()?;
        duration.stop();
        self.io.read()?;
        self.io.clean()?;
        self.state = State {
            duration: Some(duration),
            status: Some(Status::from(&output)),
        };
        Ok(())
    }

    /**
    Execute/NoAwait a subprocess and mute the input(stdin) and  outputs(stdout/stderr).
    NoAwait means it immediatly returns once the subprocess is succesfully spawned and don't wait for output.
    */
    pub fn run_detached(&mut self) -> Result<(), PipelightError> {
        info!("Run detached subprocess");
        get_shell()?;
        let mut duration = Duration::default();
        duration.start();
        Command::new(&(*SHELL.lock().unwrap()))
            .arg("-c")
            .arg(self.io.stdin.as_ref().unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        duration.stop();
        self.state = State {
            duration: Some(duration),
            status: Some(Status::Succeeded),
        };
        Ok(())
    }
}
