use crate::git::{Git, Hook};
use git2::Repository;
use std::env;
// Enum workaround
use std::string::ToString;
use strum::IntoEnumIterator;

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

// File systeme crates
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

impl Default for Git {
    fn default() -> Git {
        let root = env::current_dir().unwrap();
        Git {
            // recursively browse through fs
            repo: Repository::discover(root).ok(),
        }
    }
}
impl Git {
    pub fn new() -> Self {
        Self::default()
    }
    ///  Detect if inside a git repo
    pub fn exists(&mut self) -> bool {
        self.repo.is_some()
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
    /// Return tag if its is latest commit
    pub fn get_tag(&self) -> Result<String> {
        let repo = self.repo.as_ref().unwrap();
        let head = repo.head().into_diagnostic()?;
        if head.is_tag() {
            let tag = head.name().unwrap().to_string();
            Ok(tag)
        } else {
            Err(Error::msg("The current HEAD is not a tag"))
        }
    }
    /// Return tag if its is latest commit
    pub fn get_commit(&self) -> Result<String> {
        let repo = self.repo.as_ref().unwrap();
        let head = repo.head().into_diagnostic()?;
        let commit_id = head.peel_to_commit().into_diagnostic()?.id().to_string();
        Ok(commit_id)
    }
}

impl Hook {
    /// Ensure .git/hook folder
    pub fn enable() -> Result<()> {
        for hook in Hook::iter() {
            if Git::new().repo.is_some() {
                Hook::create_script(&hook)?;
                Hook::create_subscripts_caller(&hook)?;
            }
        }
        Ok(())
    }
    /// Create a hook that will call scrpts from a hook.d subfolder
    fn create_subscripts_caller(hook: &Hook) -> Result<()> {
        let git = Git::new();
        let action = String::from(hook);
        let root = git.repo.unwrap().path().display().to_string();

        let path = &format!(".git/hooks/{}", String::from(hook));
        let path = Path::new(path);

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
    fn create_script(hook: &Hook) -> Result<()> {
        let hook = String::from(hook);
        #[cfg(debug_assertions)]
        let script = format!(
            "#!/bin/sh \n\
            cargo run --bin pipelight trigger --flag {}\
            ",
            &hook,
        );
        #[cfg(not(debug_assertions))]
        let script = format!(
            "#!/bin/sh \n\
            pipelight trigger --flag {}\
            ",
            &hook
        );

        let dir_path = format!(".git/hooks/{}.d", &hook);
        let dir_path = Path::new(&dir_path);

        fs::create_dir_all(dir_path).into_diagnostic()?;

        let file_path = format!(".git/hooks/{}.d/_pipelight", &hook);
        let file_path = Path::new(&file_path);
        let mut file = fs::File::create(file_path).into_diagnostic()?;

        let bytes = script.as_bytes();
        file.write_all(bytes).into_diagnostic()?;

        // Set permissions
        let metadata = file.metadata().into_diagnostic()?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(file_path, perms).into_diagnostic()?;

        Ok(())
    }
}
