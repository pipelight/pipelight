// Struct
use crate::{Git, Hook};
// Trait - Enum iteration workaround
use strum::IntoEnumIterator;
// Filesystem manipulation
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
// Error Handling
use log::info;
use miette::{IntoDiagnostic, Result};

impl Hook {
    /**
    Ensure the .git/hooks directory
    with working hooks.
    */
    pub fn enable() -> Result<()> {
        info!("enabling git hooks.");
        for hook in Hook::iter() {
            if Git::new().repo.is_some() {
                Hook::create_script(&hook)?;
                Hook::create_subscripts_caller(&hook)?;
            }
        }
        Ok(())
    }

    /**
    Remove the entire .git/hooks directory
    and its pipelight auto-generated hooks.
    */
    pub fn disable() -> Result<()> {
        info!("disabling git hooks.");
        let dir = Path::new(".git/hooks/");
        if dir.exists() {
            fs::remove_dir_all(dir).into_diagnostic()?;
        }
        Ok(())
    }

    /**
    Generate the file structure under .git/hook for the given hook.
    - Creates a `.d` directory that can contain multiple scripts.
    - Creates a caller script to execute every script contained
    under the `.d`.

    .git/hooks
    ├── pre-push
    └── pre-psuh.d
    */
    fn create_subscripts_caller(hook: &Hook) -> Result<()> {
        let git = Git::new();

        let hook = String::from(hook);

        // Set the file path depending on the git repo type
        let hook_rel_dir;
        if git.repo.unwrap().is_bare() {
            hook_rel_dir = format!("./hooks/{}", &hook);
        } else {
            hook_rel_dir = format!(".git/hooks/{}", &hook);
        }
        let path = Path::new(&hook_rel_dir);

        let mut file = fs::File::create(path).into_diagnostic()?;
        let s = format!(
            "#!/bin/sh \n\
            dir=\"{root}.d\" \n\
            for file in \"$dir/*\"; do \n\
              ./$file \n\
            done",
            root = hook_rel_dir,
        );
        file.write_all(s.as_bytes()).into_diagnostic()?;

        // Set permissions
        let metadata = file.metadata().into_diagnostic()?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).into_diagnostic()?;

        Ok(())
    }

    /**
    Add a `_pipelight` caller script under the `.d`.
    This script calls pipelight with the flag corresponding to
    the actual hook.

    .git/hooks
    ├── pre-push
    └── pre-psuh.d
      └── _pipelight

    Note that pipeline will be attached to the standard output
    You can change this behavior globally are on a per pipeline basis.

    */
    fn create_script(hook: &Hook) -> Result<()> {
        let git = Git::new();

        let hook = String::from(hook);
        #[cfg(debug_assertions)]
        let script = format!(
            "#!/bin/sh \n\
            cargo run --bin \
            pipelight trigger \
                --flag {} \
                --attach\
            ",
            &hook,
        );
        #[cfg(not(debug_assertions))]
        let script = format!(
            "#!/bin/sh \n\
            pipelight trigger \
                --flag {} \
                --attach\
            ",
            &hook
        );

        // Set the file path depending on the git repo type
        let hook_rel_dir;
        if git.repo.unwrap().is_bare() {
            hook_rel_dir = format!("./hooks/{}.d", &hook);
        } else {
            hook_rel_dir = format!(".git/hooks/{}.d", &hook);
        }
        let dir_path = Path::new(&hook_rel_dir);
        fs::create_dir_all(dir_path).into_diagnostic()?;

        let file_path = format!("{}/_pipelight", &hook_rel_dir);
        // let file_path = Path::new(&file_path);
        let mut file = fs::File::create(file_path.clone()).into_diagnostic()?;

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
