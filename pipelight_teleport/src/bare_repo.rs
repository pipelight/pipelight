use std::fs;

// File
use super::types::Portal;
use pipelight_exec::Process;
use pipelight_utils::file::FileType;

// Error Handling
use miette::{Error, IntoDiagnostic, Result};
use pipelight_error::{LibError, PipelightError};
use pipelight_git::Git;

use tracing::{debug, error, info, trace};

#[cfg(test)]
mod test {
    use super::*;

    use gix::{self, remote::Direction};
    use pipelight_exec::{Process, Status};
    use std::env;

    pub fn exec(cmd: &str) -> Result<String, PipelightError> {
        println!("\n");
        println!("-> {}\n", cmd);
        let mut proc = Process::new();
        proc.term().stdin(&cmd).run()?;
        let res: Option<String> = match proc.state.status {
            Some(Status::Succeeded) => proc.io.stdout.clone(),
            Some(Status::Failed) => proc.io.stderr.clone(),
            _ => Some("Command is in an unknown state.".to_owned()),
        };
        debug!(
            "Command Status: {:#?}\n I/O: {:#?}\n",
            proc.state.status, proc.io
        );
        Ok(res.unwrap_or("null".to_owned()))
    }

    #[test]
    /// Get config form inside a bare repository.
    fn get_config_from_bare_repo() -> Result<()> {
        // Get info of current git dir for further cloning.
        let root = env::current_dir().unwrap();
        let repo = gix::discover(root).unwrap();
        let remote = repo.find_remote("origin").unwrap();
        let url = remote.url(Direction::Fetch).unwrap();

        let url = "https://github.com/crocuda/pipelight.git";

        // Create a testing directory
        let testdir = "./bare_repo_test_dir";
        fs::create_dir_all(testdir).into_diagnostic()?;

        // Clone current repo.
        exec(&format!("git clone {url} {testdir}"))?;

        // Go into new repo
        env::set_current_dir(testdir).unwrap();

        // Check that repo has been clone and we are working
        // on the right repo
        let root = env::current_dir().unwrap();
        let repo = gix::discover(root).unwrap();
        let remote = repo.find_remote("origin").unwrap();
        let url = remote.url(Direction::Fetch).unwrap();
        println!("{url}");

        // Search config file with git.
        // Expose config /tmp config to pipelight

        // Remove testing dir
        // fs::remove_dir_all("./bare_repo_test_dir").into_diagnostic()?;
        Ok(())
    }
}
