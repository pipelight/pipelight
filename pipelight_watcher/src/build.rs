// Structs
use pipelight_teleport::Portal;
// Globals
use std::sync::Arc;
// Watchexec
use ignore_files::{IgnoreFile, IgnoreFilter};
use watchexec_filterer_ignore::IgnoreFilterer;

use std::future::Future;
use watchexec::{action::ActionHandler, filter::Filterer, Config, Watchexec};
use watchexec_events::Event;
use watchexec_signals::Signal;
// Env
use super::Watcher;
use std::env;
use std::path::Path;

// Error handling
use miette::{Diagnostic, IntoDiagnostic, Result};

impl Watcher {
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

    pub async fn make_filterer() -> Result<IgnoreFilterer> {
        match Self::get_ignore_path() {
            Ok(res) => Self::make_filter_configuration(&res).await,
            Err(_) => Self::make_default_filter_configuration(),
        }
    }

    /**
     * Create an action filter based on provided ignore file path.
     * It blacklists some files to avoid recursive file watching.
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
        let globs = [
            ".pipelight/",
            ".git/",
            ".jj",
            ".cargo/",
            "target",
            "result",
            ".node_modules",
        ];

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
    pub fn make_default_filter_configuration() -> Result<IgnoreFilterer> {
        // Set Filter
        let applies_in = env::current_dir().into_diagnostic()?;

        let globs = [
            ".pipelight/",
            ".git/",
            ".jj",
            ".cargo/",
            "target",
            "result",
            ".node_modules",
        ];
        let mut filter: IgnoreFilter = IgnoreFilter::empty(applies_in.clone());
        filter
            .add_globs(&globs, Some(&applies_in))
            .into_diagnostic()?;

        let filterer = IgnoreFilterer(filter);
        Ok(filterer)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_ignore_path() -> Result<()> {
        let res = Watcher::get_ignore_path()?;
        println!("{:#?}", res);
        Ok(())
    }
    #[tokio::test]
    async fn ignore_file_detection() -> Result<()> {
        let ignore_filterer = Watcher::make_filterer().await?;
        // println!("{:#?}", ignore_filterer);
        Ok(())
    }
}
