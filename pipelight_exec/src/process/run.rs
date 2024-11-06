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
    pub fn run(&mut self) -> Result<Self, PipelightError> {
        // Generate command
        let mut cmd = match self.config.term {
            false => self.to_command(),
            true => {
                let mut e = Command::new(&(*SHELL.lock().unwrap()));
                e.arg("-c").arg(self.io.stdin.as_ref().unwrap());
                e
            }
        };
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Output redirection
        match self.config.detach {
            true => {
                cmd.process_group(0);
            }
            false => {}
        };
        match self.config.fs {
            true => {
                let proc_path = format!("{}/{}", *OUTDIR.lock().unwrap(), self.uuid);
                create_dir_all(&proc_path)?;

                let stdout_path = format!("{proc_path}/1");
                let stderr_path = format!("{proc_path}/2");
                cmd.stdout(File::create(stdout_path)?)
                    .stderr(File::create(stderr_path)?);
            }
            false => {}
        }

        // Process execution
        // and catch child pid

        // Read process output if available
        let mut duration = Duration::default();

        if self.config.background {
            cmd.spawn()?;
        } else {
            let child = cmd.spawn()?;
            self.pid = Some(child.id().to_owned() as i32);

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
            if self.config.fs {
                self.io.read()?;
                self.io.clean()?;
            }
        }
        Ok(self.to_owned())
    }
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
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{thread, time};

    #[test]
    fn default() -> Result<()> {
        let proc = Process::new().stdin("echo test").run()?;
        assert_eq!(proc.io.stdout, Some("test\n".to_owned()));
        Ok(())
    }
    #[test]
    fn default_wait_for_output() -> Result<()> {
        let proc = Process::new().stdin("sleep 3").run()?;
        assert_eq!(proc.io.stdout, None);
        Ok(())
    }
    #[test]
    fn fs() -> Result<()> {
        let proc = Process::new().stdin("echo test").fs().run()?;
        assert_eq!(proc.io.stdout, Some("test\n".to_owned()));
        Ok(())
    }
    #[test]
    fn background() -> Result<()> {
        let proc = Process::new().stdin("sleep 3").background().run()?;
        assert_eq!(proc.io.stdout, None);
        Ok(())
    }
    #[test]
    fn background_term() -> Result<()> {
        let proc = Process::new().stdin("sleep 3").term().background().run()?;
        assert_eq!(proc.io.stdout, None);
        Ok(())
    }
    #[test]
    fn background_detach() -> Result<()> {
        let proc = Process::new()
            .stdin("sleep 3")
            .background()
            .detach()
            .run()?;
        assert_eq!(proc.io.stdout, None);
        Ok(())
    }
    #[test]
    fn background_fs() -> Result<()> {
        let mut proc = Process::new().stdin("echo test").background().fs().run()?;
        assert_eq!(proc.io.stdout, None);

        // Wait until process is executed
        let throttle = time::Duration::from_millis(1000);
        thread::sleep(throttle);

        proc.io.read().into_diagnostic()?;
        assert_eq!(proc.io.stdout, Some("test\n".to_owned()));
        Ok(())
    }
}
