// Structs
use pipelight_utils::teleport::Portal;
// Globals
use std::sync::Arc;
// Watchexec
use ignore_files::{IgnoreFile, IgnoreFilter};
use std::future::Future;
use watchexec::{action::ActionHandler, Config, Watchexec};
use watchexec_events::Event;
use watchexec_filterer_ignore::IgnoreFilterer;
use watchexec_signals::Signal;
// Env
use std::env;
use std::path::Path;

// Error handling
use miette::{Diagnostic, IntoDiagnostic, Result};

/**
* Retrieve an ignore file fullpath if any.
*/
pub fn get_ignore_path() -> Result<String> {
    // Search an ignore file to set a filter
    let mut portal = Portal::new()?;
    portal.seed(".pipelight_ignore");
    match portal.search_file() {
        Ok(_) => Ok(portal.target.file_path.unwrap()),
        Err(_) => {
            let mut portal = Portal::new()?;
            portal.seed(".gitignore");
            match portal.search_file() {
                Ok(_) => Ok(portal.target.file_path.unwrap()),
                Err(err) => return Err(err.into()),
            }
        }
    }
}

/**
* Self reconfigure when the IgnoreFile changes.
*/
pub async fn reconfigure(watchexec: &Watchexec, action: &ActionHandler) -> Result<()> {
    if let Some(ignore_path) = get_ignore_path().ok() {
        for event in action.events.iter() {
            if event
                .paths()
                .any(|(p, _)| p.to_str().unwrap() == ignore_path)
            {
                // Set Filter
                let filterer = make_filter_configuration(&ignore_path).await?;
                watchexec.config.filterer(Arc::new(filterer));
                break;
            }
        }
    }
    Ok(())
}

/**
Create an action filter based on provided ignore file path.
It blacklists some files to avoid recursive file watching.
 */
pub async fn make_filter_configuration(path: &str) -> Result<IgnoreFilterer> {
    let path = Path::new(path);
    // Set Filter
    let applies_in = env::current_dir().into_diagnostic()?;
    let file = IgnoreFile {
        path: path.into(),
        applies_in: Some(applies_in.clone()),
        applies_to: None,
    };
    let globs = [".pipelight/", ".git/", ".cargo/", "target", ".node_modules"];
    let mut filter: IgnoreFilter = IgnoreFilter::empty(applies_in.clone());
    filter
        .add_globs(&globs, Some(&applies_in))
        .into_diagnostic()?;
    filter.add_file(&file).await.into_diagnostic()?;

    let filterer = IgnoreFilterer(filter);
    Ok(filterer)
}

/**
Create a default action filter.
To be used when no ignore file is provided.
It blacklists some files to avoid recursive file watching.
 */
pub async fn make_default_filter_configuration() -> Result<IgnoreFilterer> {
    // Set Filter
    let applies_in = env::current_dir().into_diagnostic()?;

    let globs = [".pipelight/", ".git/", ".cargo/", ".node_modules/"];
    let mut filter: IgnoreFilter = IgnoreFilter::empty(applies_in.clone());
    filter
        .add_globs(&globs, Some(&applies_in))
        .into_diagnostic()?;

    let filterer = IgnoreFilterer(filter);
    Ok(filterer)
}
