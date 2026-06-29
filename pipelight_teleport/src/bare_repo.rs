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

use gix_dir::{
    EntryRef,
    walk::{self, Delegate},
};

#[derive(Default, Debug)]
struct Count {
    entries: u64,
}

impl Delegate for Count {
    fn emit(
        &mut self,
        _entry: EntryRef<'_>,
        _collapsed_directory_status: Option<gix::dir::entry::Status>,
    ) -> walk::Action {
        self.entries += 1;
        std::ops::ControlFlow::Continue(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use gix::{self, remote::Direction, validate::reference::branch_name};
    use std::sync::atomic::AtomicBool;

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
        let repo = gix::discover(&root).unwrap();
        let remote = repo.find_remote("origin").unwrap();
        let url = remote.url(Direction::Fetch).unwrap();

        let url = "https://github.com/crocuda/pipelight.git";

        // Create a testing directory
        let testdir = "./bare_repo_test_dir";
        fs::create_dir_all(testdir).into_diagnostic()?;

        // Bare clone current repo.
        // TODO: Move to gitoxide instead of relying on external tool.
        exec(&format!("git clone --bare {url} {testdir}"))?;

        // Go into new repo
        env::set_current_dir(testdir).unwrap();

        // Check that repo has been clone and we are working
        // on the right repo.
        let testing_root = env::current_dir().unwrap();
        let testing_repo = gix::discover(testing_root).unwrap();
        let testing_remote = testing_repo.find_remote("origin").unwrap();
        let url = testing_remote.url(Direction::Fetch).unwrap();
        println!("Working repo url: {url}");

        // Find a branch to take the <pipelight_config> file from.
        // Default to "master".
        let remote_names: Vec<String> = repo.branch_names().iter().map(|e| e.to_string()).collect();
        println!("Available branches: {:#?}", remote_names);
        let master_branch: String = if remote_names.contains(&"main".to_owned()) {
            String::from("main")
        } else if remote_names.contains(&"master".to_owned()) {
            String::from("master")
        } else {
            String::from("master")
        };

        // TODO: Move to gitoxide instead of relying on external tool.
        //
        // ATTEMPT #1 => failed. not a valide reference
        // let reference = testing_repo
        //     .find_reference("master:pipelight.ts")
        //     .into_diagnostic()?;
        // println!("{:#?}", reference);
        //
        // ATTEMPT #2 => failed. reason: need a worktree
        // let index = repo.index_or_empty().into_diagnostic()?;
        // let options = repo.dirwalk_options().into_diagnostic()?;
        // let mut delegate = Count::default();
        // let outcome = testing_repo
        //     .dirwalk(
        //         &index,
        //         vec!["pipelight"],
        //         &AtomicBool::default(),
        //         options,
        //         &mut delegate,
        //     )
        //     .into_diagnostic()?;
        // println!("{:#?}", delegate);

        let file = exec(&format!("git show {master_branch}:pipelight.ts"))?;
        let file = exec(&format!("git show {master_branch}:pipelight.js"))?;
        println!("{}", file);
        let file = exec(&format!("git show {master_branch}:pipelight.toml"))?;
        println!("{}", file);
        let file = exec(&format!("git show {master_branch}:pipelight.yaml"))?;
        println!("{}", file);

        // Search config file with git.
        // Expose config /tmp config to pipelight

        // Remove testing dir
        // env::set_current_dir(&root).unwrap();
        // fs::remove_dir_all("./bare_repo_test_dir").into_diagnostic()?;
        Ok(())
    }
}
