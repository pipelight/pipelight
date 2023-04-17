// External Import
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::env;

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;
// use std::error::Error;

// File systeme crates
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

// Enum workaround
use std::string::ToString;
use strum::{EnumIter, IntoEnumIterator};

// Internal Import
mod from;

pub struct Git {
    pub repo: Option<Repository>,
}
impl Git {
    pub fn new() -> Git {
        let root = env::current_dir().unwrap();
        return Git {
            repo: Repository::discover(root).ok(),
        };
    }
    pub fn teleport(&mut self) {
        if self.exists() {
            let wd = self
                .repo
                .as_mut()
                .unwrap()
                .workdir()
                .unwrap()
                .display()
                .to_string();
            env::set_current_dir(wd).unwrap();
        }
    }
    ///  Detect if inside a git repo
    pub fn exists(&mut self) -> bool {
        return self.repo.is_some();
    }
    /// Return actual attached branch
    pub fn get_branch(&self) -> Result<String> {
        // Only tested on attached HEAD
        // No edge case when head is a commit or else...
        let repo = self.repo.as_ref().unwrap();
        let head = repo.head().into_diagnostic()?;
        let name = head.shorthand().unwrap().to_owned();
        Ok(name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, EnumIter, Eq, Ord)]
pub enum Hook {
    ApplypatchMsg,
    PreApplypatch,
    PostApplypatch,
    PreCommit,
    PrepareCommitMsg,
    CommitMsg,
    PostCommit,
    PreRebase,
    PostCheckout,
    PostMerge,
    PreReceive,
    Update,
    PostReceive,
    PostUpdate,
    PreAutoGc,
    PostRewrite,
    PrePush,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Flag {
    Hook(Hook),
    Manual,
}

impl Hook {
    /// Detect name of the hook that triggers script
    pub fn origin() -> Result<Flag> {
        let root = env::current_dir().into_diagnostic()?;
        let path_string = root.display().to_string();
        if path_string.contains("/.git/hooks/") {
            // Get hook name from folder name
            let name = root.file_stem().unwrap().to_str().unwrap().to_owned();
            let hook = Flag::Hook(Hook::from(&name));
            Ok(hook)
        } else {
            Ok(Flag::Manual)
            // let message = "Can't trigger hook outside of repository hook folder";
            // Err(Box::from(message))
        }
    }
    /// Ensure .git/hook folder
    pub fn new() -> Result<()> {
        let root = ".git/hooks";
        let extension = ".d";
        let bin = "pipelight";

        for hook in Hook::iter() {
            let caller = format!("{}/{}", root, String::from(&hook));
            let caller_path = Path::new(&caller);

            let dot_d_dir = format!("{}/{}{}", root, String::from(&hook), extension);
            let dot_d_dir_path = Path::new(&dot_d_dir);

            let script = format!("{}/{}", dot_d_dir, bin);
            let script_path = Path::new(&script);

            if Git::new().repo.is_some() {
                Hook::create_script(&dot_d_dir_path, &script_path)?;
                Hook::create_subscripts_caller(&caller_path, &hook)?;
            }
        }
        Ok(())
    }
    /// Create a hook that will call scrpts from a hook.d subfolder
    fn create_subscripts_caller(path: &Path, hook: &Hook) -> Result<()> {
        let git = Git::new();
        let action = String::from(hook);
        let root = git.repo.unwrap().path().display().to_string();
        let mut file = fs::File::create(path).into_diagnostic()?;
        let s = format!(
            "#!/bin/sh \n\
            dir=\"{root}hooks/{action}.d\" \n\
            for file in \"$dir/*\"; do \n\
              cd $dir
              $file \n\
            done",
            root = root,
            action = action
        );
        let b = s.as_bytes();
        file.write_all(b).into_diagnostic()?;

        // Set permissions
        let metadata = file.metadata().into_diagnostic()?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).into_diagnostic()?;

        Ok(())
    }
    fn create_script(directory_path: &Path, file_path: &Path) -> Result<()> {
        fs::create_dir_all(directory_path).into_diagnostic()?;
        let mut file = fs::File::create(file_path).into_diagnostic()?;
        #[cfg(debug_assertions)]
        let s = format!(
            "#!/bin/sh \n\
            cargo run --bin pipelight trigger \
            "
        );
        #[cfg(not(debug_assertions))]
        let s = format!(
            "#!/bin/sh \n\
            pipelight trigger \
            "
        );
        let b = s.as_bytes();
        file.write_all(b).into_diagnostic()?;

        // Set permissions
        let metadata = file.metadata().into_diagnostic()?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(file_path, perms).into_diagnostic()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
