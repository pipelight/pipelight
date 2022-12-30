use crate::git::Git;
use crate::types::GIT_HOOKS;
use log::{debug, error, info, trace, warn};
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::os::unix::fs::symlink;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub struct Hooks {
    name: String,
}
impl Hooks {
    pub fn is() -> Result<bool, Box<dyn Error>> {
        let root = env::current_dir()?;
        let path_string = root.display().to_string();
        let my_bool = path_string.contains("/.git/hooks/");
        let hook = root
            .parent()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        Ok(my_bool)
    }
    /// Create/Ensure git hooks file trees
    pub fn ensure() -> Result<(), Box<dyn Error>> {
        let root = ".git/hooks";
        let extension = ".d";
        let bin = "pipelight-trigger";

        let bin_path = format!("/usr/bin/{}", bin);
        let bin_path = Path::new(&bin_path);
        for hook in &GIT_HOOKS {
            let file = format!("{}/{}", root, hook);
            let file = Path::new(&file);

            let dir = format!("{}/{}{}", root, hook, extension);
            let dir = Path::new(&dir);

            let link = format!("{}/{}", dir.display(), bin);
            let link = Path::new(&link);

            Hooks::ensure_hook(file, hook)?;
            Hooks::ensure_directory(dir)?;
            Hooks::ensure_symlink(bin_path, link)?;
        }
        Ok(())
    }
    /// Create directories
    fn ensure_directory(path: &Path) -> Result<(), Box<dyn Error>> {
        let dir_exists = path.exists();
        if dir_exists {
            fs::remove_dir_all(path)?;
        }
        fs::create_dir(path);
        Ok(())
    }
    /// Create a hook that will call subfolder script
    fn ensure_hook(path: &Path, hook: &str) -> Result<(), Box<dyn Error>> {
        let exists = path.exists();
        if exists {
            fs::remove_file(path)?;
        }
        let file = fs::File::create(path)?;
        let metadata = file.metadata()?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;

        Hooks::write(path, hook)?;
        Ok(())
    }
    fn write(path: &Path, hook: &str) -> Result<(), Box<dyn Error>> {
        let git = Git::new();
        let root = git.repo.unwrap().path().display().to_string();
        let mut file = fs::File::create(path)?;
        let s = format!(
            "#!/bin/sh \n\
                dir=\"{}hooks/{}.d\" \n\
                for f in \"$dir\"/*; do \n\
                  exec \"$f\" \n\
                done",
            root, hook
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
