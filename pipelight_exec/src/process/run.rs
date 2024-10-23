use crate::dates::Duration;
use crate::{Io, Process, State, Status};

// Globals
use crate::globals::{get_shell, OUTDIR, SHELL};

// Unix process manipulation
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

// File manipulation
use std::fs::{create_dir_all, File};

// Error Handling
use log::info;
use miette::{IntoDiagnostic, Result};
use pipelight_error::{PipelightError, WrapError};

impl Process {
    fn to_command(&self) -> Command {
        // convert stdin
        let mut args: Vec<String> = self
            .io
            .stdin
            .clone()
            .unwrap()
            .split(" ")
            .map(|e| e.to_owned())
            .collect();

        // Ensure internal log dir exists
        let mut cmd = Command::new(args.remove(0));
        cmd.args(args);
        cmd
    }
    /**
     * Execute a process that inherit parent process outputs(stdout/stderr).
     *
     * Usually we want to catch process i/o for further manipulation,
     * but this function mainly leaks every output directly to the parent.
     *
     * Thus output won't be collected in the struct
     * and you won't have acces to i/o with `proc.io.stdout` (None)
     *
     * To be used in very specifice cases only,
     * like if you want to directly print to the terminal.
     */
    pub fn run_inherit(&mut self) -> Result<Self, PipelightError> {
        info!("Run subprocess piped to parent");
        get_shell()?;
        let mut duration = Duration::default();
        let mut cmd = self.to_command();
        cmd.stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let child = cmd.spawn()?;
        self.pid = Some(child.id().to_owned() as i32);
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
        Ok(self.to_owned())
    }

    /**
     * Execute a subprocess and pipe the outputs(stdout/stderr)
     * to the parent process.
     */
    pub fn run_piped(&mut self) -> Result<Self, PipelightError> {
        info!("Run subprocess piped to parent");
        get_shell()?;
        let mut cmd = self.to_command();
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = cmd.spawn()?;
        self.pid = Some(child.id().to_owned() as i32);

        let mut duration = Duration::default();
        duration.start();
        let output = child.wait_with_output()?;
        duration.stop();

        // Hydrate struct
        self.io = Io {
            uuid: self.io.uuid,
            stdin: self.io.stdin.to_owned(),
            ..Io::from(&output)
        };
        self.state = State {
            duration: Some(duration),
            status: Some(Status::from(&output)),
        };
        Ok(self.to_owned())
    }

    /**
     * Execute/Await a subprocess and pipe the outputs(stdout/stderr)
     * to files for further read/write while executing.
     * Suits long running processes for early inner inspection of outputs
     * whilst it still runs.
     */
    pub fn run_fs(&mut self) -> Result<Self, PipelightError> {
        info!("Run subprocess with output piped to pipelight managed files");
        get_shell()?;
        // path definition
        create_dir_all(&(*OUTDIR.lock().unwrap()))?;
        let stdout_path = format!("{}/{}_stdout", *OUTDIR.lock().unwrap(), self.uuid.unwrap());
        let stderr_path = format!("{}/{}_stderr", *OUTDIR.lock().unwrap(), self.uuid.unwrap());

        let mut cmd = self.to_command();
        cmd.stdin(Stdio::null())
            .stdout(File::create(stdout_path)?)
            .stderr(File::create(stderr_path)?);

        let child = cmd.spawn()?;
        self.pid = Some(child.id().to_owned() as i32);

        let mut duration = Duration::default();
        duration.start();
        let output = child.wait_with_output()?;
        duration.stop();

        // Hydrate struct
        self.io.read()?;
        self.io.clean()?;
        self.state = State {
            duration: Some(duration),
            status: Some(Status::from(&output)),
        };
        Ok(self.to_owned())
    }

    pub fn run_term_fs(&mut self) -> Result<Self, PipelightError> {
        info!("Run subprocess with output piped to pipelight managed files");
        get_shell()?;
        // path definition
        create_dir_all(&(*OUTDIR.lock().unwrap()))?;
        let stdout_path = format!("{}/{}_stdout", *OUTDIR.lock().unwrap(), self.uuid.unwrap());
        let stderr_path = format!("{}/{}_stderr", *OUTDIR.lock().unwrap(), self.uuid.unwrap());

        let mut cmd = Command::new(&(*SHELL.lock().unwrap()));
        cmd.arg("-c")
            .arg(self.io.stdin.as_ref().unwrap())
            .stdin(Stdio::null())
            .stdout(File::create(stdout_path)?)
            .stderr(File::create(stderr_path)?);

        let child = cmd.spawn()?;
        self.pid = Some(child.id().to_owned() as i32);

        let mut duration = Duration::default();
        duration.start();
        let output = child.wait_with_output()?;
        duration.stop();

        // Hydrate struct
        self.io.read()?;
        self.io.clean()?;
        self.state = State {
            duration: Some(duration),
            status: Some(Status::from(&output)),
        };
        Ok(self.to_owned())
    }

    pub fn run_term(&mut self) -> Result<Self, PipelightError> {
        let mut cmd = Command::new(&(*SHELL.lock().unwrap()));
        cmd.arg("-c")
            .arg(self.io.stdin.as_ref().unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = cmd.spawn()?;
        self.pid = Some(child.id().to_owned() as i32);

        let mut duration = Duration::default();
        duration.start();
        let output = child.wait_with_output()?;
        duration.stop();

        // Hydrate struct
        self.io = Io {
            uuid: self.io.uuid,
            stdin: self.io.stdin.to_owned(),
            ..Io::from(&output)
        };
        self.state = State {
            duration: Some(duration),
            status: Some(Status::from(&output)),
        };
        Ok(self.to_owned())
    }
    /**
     * Execute/NoAwait a subprocess and mute the input(stdin) and  outputs(stdout/stderr).
     * NoAwait means it immediatly returns once the subprocess is succesfully spawned and don't wait for output.
     */
    pub fn run_detached(&mut self) -> Result<Self, PipelightError> {
        info!("Run detached subprocess");
        get_shell()?;

        let mut cmd = self.to_command();
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd.process_group(0);

        let mut duration = Duration::default();
        duration.start();
        let child = cmd.spawn()?;
        self.pid = Some(child.id().to_owned() as i32);
        duration.stop();

        // Hydrate struct
        self.state = State {
            duration: Some(duration),
            status: Some(Status::Succeeded),
        };
        Ok(self.to_owned())
    }

    pub fn run_detached_fs(&mut self) -> Result<Self, PipelightError> {
        info!("Run subprocess with output piped to pipelight managed files");
        // path definition
        create_dir_all(&(*OUTDIR.lock().unwrap()))?;
        let stdout_path = format!("{}/{}_stdout", *OUTDIR.lock().unwrap(), self.uuid.unwrap());
        let stderr_path = format!("{}/{}_stderr", *OUTDIR.lock().unwrap(), self.uuid.unwrap());

        let mut cmd = self.to_command();
        cmd.stdin(Stdio::null())
            .stdout(File::create(stdout_path)?)
            .stderr(File::create(stderr_path)?);

        let mut duration = Duration::default();
        duration.start();
        let child = cmd.spawn()?;
        self.pid = Some(child.id().to_owned() as i32);
        duration.stop();

        // Hydrate struct
        self.state = State {
            duration: Some(duration),
            status: Some(Status::Succeeded),
        };
        Ok(self.to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{thread, time};

    #[test]
    fn attach_proc_piped_output() -> Result<()> {
        let proc = Process::new("echo test").run_piped()?;
        assert_eq!(proc.io.stdout, Some("test\n".to_owned()));
        Ok(())
    }

    #[test]
    fn attach_proc_w_inherited_output() -> Result<()> {
        let proc = Process::new("echo test").run_inherit()?;
        assert_eq!(proc.io.stdout, None);
        Ok(())
    }

    #[test]
    fn attach_proc_w_output_to_file() -> Result<()> {
        let proc = Process::new("echo test").run_fs()?;
        assert_eq!(proc.io.stdout, Some("test\n".to_owned()));
        Ok(())
    }

    #[test]
    fn detached_proc_fs_update_io() -> Result<(), PipelightError> {
        let mut proc = Process::new("echo test").run_detached_fs()?;

        let throttle = time::Duration::from_millis(1000);
        thread::sleep(throttle);
        proc.io.read()?;

        assert_eq!(proc.io.stdout, Some("test\n".to_owned()));
        Ok(())
    }
}
