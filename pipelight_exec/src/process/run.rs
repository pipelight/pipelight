use crate::dates::Duration;
use crate::{Io, Process, State, Status};

// Globals
use crate::globals::{OUTDIR, SHELL};

// Unix process manipulation
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

// File manipulation
use std::fs::{create_dir_all, File};

// Error Handling
use log::info;
use miette::Result;
use pipelight_error::{LibError, PipelightError};

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

        match self.config.detach {
            true => {
                // cmd.process_group(0);

                // Soft detach
                cmd.stdout(Stdio::null()).stderr(Stdio::null());
            }
            false => {}
        };

        // Hard detach child processes
        if self.config.orphan {
            cmd.process_group(0);
        }

        // Process execution

        // Read process output if available
        let mut duration = Duration::default();

        if self.config.background {
            duration.start();
            let child = cmd.spawn()?;
            self.pid = Some(child.id().to_owned() as i32);
            duration.stop();

            // After spawn process modifications
            if self.config.orphan {
                rustix::process::setpgid(
                    Some(rustix::process::Pid::from_child(&child)),
                    Some(rustix::process::Pid::from_raw(2824).unwrap()),
                )
                .map_err(|e| LibError {
                    message: "Couldn't create orphan process.\n".to_owned() + &e.to_string(),
                    help: "".to_string(),
                })?;
                cmd.process_group(0);
            }
        } else {
            // Catch child pid
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
                // self.io.clean()?;
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
    use crate::process::Finder;
    use serial_test::serial;

    use pretty_assertions::{assert_eq, assert_ne};
    use std::{thread, time};
    // Error Handling
    use miette::IntoDiagnostic;

    #[test]
    #[serial]
    fn test_run_default() -> Result<()> {
        let proc = Process::new().stdin("echo test").run()?;
        assert_eq!(proc.io.stdout, Some("test\n".to_owned()));
        Ok(())
    }
    #[test]
    #[serial]
    fn test_run_orphan() -> Result<()> {
        let proc = Process::new()
            .stdin("sleep 10")
            .background()
            .detach()
            .orphan()
            .run()?;

        println!("{:#?}", proc);

        // Wait until process is executed
        let throttle = time::Duration::from_millis(1000);
        thread::sleep(throttle);

        // let finder = Finder::new().seed("sleep 10").search()?;
        // let matches = finder.matches.unwrap();
        // let reproc = matches.first().unwrap();

        let reproc = Process::get_from_pid(&proc.pid.unwrap());

        assert_ne!(reproc.ppid, Some(std::process::id() as i32));
        Ok(())
    }
    #[test]
    fn test_run_wait_for_output() -> Result<()> {
        let proc = Process::new().stdin("sleep 3").run()?;
        assert_eq!(proc.io.stdout, None);
        assert_eq!(proc.state.status, Some(Status::Succeeded));
        Ok(())
    }
    #[test]
    fn test_run_term_wait_for_output() -> Result<()> {
        let proc = Process::new().stdin("sleep 3").run()?;
        assert_eq!(proc.io.stdout, None);
        assert_eq!(proc.state.status, Some(Status::Succeeded));
        Ok(())
    }
    #[test]
    fn test_run_write_to_fs() -> Result<()> {
        let proc = Process::new().stdin("echo test").fs().run()?;
        assert_eq!(proc.io.stdout, Some("test\n".to_owned()));
        Ok(())
    }
    #[test]
    fn test_run_in_background() -> Result<()> {
        let proc = Process::new().stdin("sleep 3").background().run()?;
        assert_eq!(proc.io.stdout, None);
        Ok(())
    }
    #[test]
    fn test_run_term_in_background() -> Result<()> {
        let proc = Process::new().stdin("sleep 3").term().background().run()?;
        assert_eq!(proc.io.stdout, None);
        Ok(())
    }
    #[test]
    fn test_run_in_background_and_detach() -> Result<()> {
        let proc = Process::new()
            .stdin("sleep 3")
            .background()
            .detach()
            .run()?;
        assert_eq!(proc.io.stdout, None);
        Ok(())
    }
    #[test]
    fn test_run_in_background_write_to_fs() -> Result<()> {
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
