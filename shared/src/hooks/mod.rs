use crate::types::GIT_HOOKS;
use log::{debug, error, info, trace, warn};
use std::error::Error;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

/// Create/Ensure git hooks file trees
pub fn is_hook() -> Result<bool, Box<dyn Error>> {
    Ok(true)
}

///Create directories
pub fn ensure_hooks() -> Result<(), Box<dyn Error>> {
    let root = ".git/hooks";
    let extension = ".d";
    let bin = "pipelight-trigger";

    let bin_path = format!("/usr/bin/{}", bin);
    let bin_path = Path::new(&bin_path);
    for hook in &GIT_HOOKS {
        let dir = format!("{}/{}{}", root, hook, extension);
        let dir = Path::new(&dir);

        let link = format!("{}/{}", dir.display(), bin);
        let link = Path::new(&link);

        ensure_dir(dir)?;
        ensure_symlink(bin_path, link)?;
    }
    Ok(())
}
fn ensure_dir(path: &Path) -> Result<(), Box<dyn Error>> {
    let dir_exists = path.exists();
    if dir_exists {
        fs::remove_dir_all(path)?;
    }
    let res_create = fs::create_dir(path);
    let res = match res_create {
        Ok(res) => {
            debug!("Hook folder created");
            return Ok(());
        }
        Err(e) => {
            warn!("Couldn't create {}", &path.display());
            return Err(Box::from(e));
        }
    };
}
fn ensure_symlink(src: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    let link_exists = dest.exists();
    if link_exists {
        fs::remove_file(dest)?;
    }
    let res_symlink = symlink(src, dest);
    let res = match res_symlink {
        Ok(res) => {
            debug!("Symlink created");
            return Ok(());
        }
        Err(e) => {
            warn!("Couldn't create {}", dest.display());
            return Err(Box::from(e));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
