use git2::{Reference, Repository};
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::env;
use std::error::Error;

// File systeme crates
use std::fs;
use std::io::Write;
use std::os::unix::fs::symlink;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

// Enum workaround
use std::str::FromStr;
use std::string::ToString;
use strum::{EnumIter, EnumString, IntoEnumIterator};

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
    ///  Detect if there is a git repo in pwd
    fn exists(&mut self) -> bool {
        return self.repo.is_some();
    }
    /// Return actual attached branch
    pub fn get_branch(&self) -> Result<String, Box<dyn Error>> {
        // Only tested on attached HEAD
        // No edge case when head is a commit or else...
        let repo = self.repo.as_ref().unwrap();
        let head = repo.head()?;
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

impl Hook {
    /// Detect name of the hook that triggers script
    pub fn origin() -> Result<Hook, Box<dyn Error>> {
        let root = env::current_dir()?;
        let path_string = root.display().to_string();
        if path_string.contains("/.git/hooks/") {
            // Get hook name from folder name
            let name = root.file_stem().unwrap().to_str().unwrap().to_owned();
            let hook = Hook::from(&name);
            Ok(hook)
        } else {
            let message = "Can't trigger hook outside of repository hook folder";
            Err(Box::from(message))
        }
    }
    /// Ensure .git/hook folder
    pub fn new() -> Result<(), Box<dyn Error>> {
        let root = ".git/hooks";
        let extension = ".d";
        let bin = "pipelight";

        for hook in Hook::iter() {
            let file = format!("{}/{}", root, String::from(&hook));
            let file = Path::new(&file);

            let dir = format!("{}/{}{}", root, String::from(&hook), extension);
            let dir = Path::new(&dir);

            let executable = format!("{}/{}.sh", dir.display(), bin);
            let executable = Path::new(&executable);

            fs::create_dir_all(root)?;
            Hook::ensure_hook(file, &hook)?;
            Hook::ensure_directory(dir)?;
            Hook::create_script(&executable)?;
        }
        Ok(())
    }
    /// Create directories
    fn ensure_directory(path: &Path) -> Result<(), Box<dyn Error>> {
        let dir_exists = path.exists();
        if dir_exists {
            // fs::remove_dir_all(path)?;
            return Ok(());
        }
        fs::create_dir_all(path)?;
        Ok(())
    }
    /// Create a hook.d subfolder
    fn ensure_hook(path: &Path, hook: &Hook) -> Result<(), Box<dyn Error>> {
        Hook::create_subscripts_caller(path, hook)?;

        Ok(())
    }
    /// Create a hook that will call scrpts from a hook.d subfolder
    fn create_subscripts_caller(path: &Path, hook: &Hook) -> Result<(), Box<dyn Error>> {
        let git = Git::new();
        let action = String::from(hook);
        let root = git.repo.unwrap().path().display().to_string();
        let mut file = fs::File::create(path)?;
        let s = format!(
            "#!/bin/sh \n\
            dir=\"{root}hooks/{action}.d\" \n\
            for file in \"$dir/*\"; do \n\
              $file \n\
            done",
            root = root,
            action = action
        );
        let b = s.as_bytes();
        file.write_all(b)?;

        // Set permissions
        let metadata = file.metadata()?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;

        Ok(())
    }
    fn create_script(path: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = fs::File::create(path)?;
        let s = format!(
            "#!/bin/sh \n\
            pipelight trigger
            "
        );
        let b = s.as_bytes();
        file.write_all(b)?;

        // Set permissions
        let metadata = file.metadata()?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;

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
