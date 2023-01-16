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
    pub fn new() -> Self {
        let mut e = Git { repo: None };
        if e.exists() {
            // Get the repo
        } else {
            // Maybe create the repo
        }
        return e;
    }
    ///  Detect if there is a git repo in pwd
    fn exists(&mut self) -> bool {
        // Seek git repo in current directory
        let root = env::current_dir().unwrap();
        let repo = Repository::discover(root).unwrap();
        // Set working dir
        let exist = repo.workdir().is_some();
        if exist {
            let wd = repo.workdir().unwrap().display().to_string();
            // Set working directory to .git parent
            // Use this function to teleport from hook folder to root
            // and read config file
            env::set_current_dir(wd).unwrap();
        }
        self.repo = Some(repo);
        return exist;
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
            let name = root
                .parent()
                .unwrap()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            let hook = Hook::from(&name);
            Ok(hook)
        } else {
            let message = "Can't trigger hook outside of repository hook folder";
            Err(Box::from(message))
        }
    }
    /// Ensure .git/hook folder
    pub fn ensure() -> Result<(), Box<dyn Error>> {
        let root = ".git/hooks";
        let extension = ".d";
        let bin = "pipelight-trigger";

        let bin_path = format!("/usr/bin/{}", bin);
        let bin_path = Path::new(&bin_path);
        for hook in Hook::iter() {
            let file = format!("{}/{}", root, hook.to_string());
            let file = Path::new(&file);

            let dir = format!("{}/{}{}", root, hook.to_string(), extension);
            let dir = Path::new(&dir);

            let link = format!("{}/{}", dir.display(), bin);
            let link = Path::new(&link);

            Hook::ensure_hook(file, &hook)?;
            Hook::ensure_directory(dir)?;
            Hook::ensure_symlink(bin_path, link)?;
        }
        Ok(())
    }
    /// Create directories
    fn ensure_directory(path: &Path) -> Result<(), Box<dyn Error>> {
        let dir_exists = path.exists();
        if dir_exists {
            fs::remove_dir_all(path)?;
        }
        fs::create_dir(path)?;
        Ok(())
    }
    /// Create a hook.d subfolder
    fn ensure_hook(path: &Path, hook: &Hook) -> Result<(), Box<dyn Error>> {
        let exists = path.exists();
        if exists {
            // fs::remove_file(path)?;
            return Ok(());
        }
        let file = fs::File::create(path)?;
        let metadata = file.metadata()?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;

        Hook::write(path, hook)?;
        Ok(())
    }
    /// Create a hook that will call scrpts from a hook.d subfolder
    fn write(path: &Path, hook: &Hook) -> Result<(), Box<dyn Error>> {
        let git = Git::new();
        let action = String::from(hook);
        let root = git.repo.unwrap().path().display().to_string();
        let mut file = fs::File::create(path)?;
        let s = format!(
            "#!/bin/sh \n\
                dir=\"{root}hooks/{action}.d\" \n\
                for f in \"$dir\"[>; do \n\
                  \"$f\" {action}\n\
                done",
            root = root,
            action = action
        );
        let b = s.as_bytes();
        file.write_all(b)?;
        Ok(())
    }
    fn ensure_symlink(src: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
        let link_exists = dest.exists();
        if link_exists {
            fs::remove_file(dest)?;
        }
        symlink(src, dest)?;
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
