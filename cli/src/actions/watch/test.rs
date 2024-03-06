#[cfg(test)]
mod watcher {
    // Env
    use std::env;
    use std::fs;
    use std::fs::remove_dir_all;
    // Globals
    use crate::actions::watch::build::get_ignore_path;
    use crate::actions::watch::{build, Watcher};
    use utils::teleport::Portal;
    use exec::Process;
    // Error handling
    use miette::{Diagnostic, IntoDiagnostic, Result};
    use thiserror::Error;
    // Logger
    use log::warn;
    // Process finder
    use exec::processes::Finder;

    #[tokio::test]
    async fn builder() -> Result<()> {
        // Teleport
        Portal::new()?.seed("test.pipelight").search()?.teleport()?;
        // Build watcher
        let res = build().await;
        assert!(res.is_ok());
        Ok(())
    }
    // #[tokio::test]
    async fn try_start() -> Result<()> {
        // Teleport
        Portal::new()?.seed("test.pipelight").search()?.teleport()?;
        // Watcher::start()?;
        let (we, runtime) = build().await?;
        we.main().await.into_diagnostic()?;
        Ok(())
    }

    #[test]
    /**
    Try to retrieve an ignore file
    */
    fn test_utils() -> Result<()> {
        let res = get_ignore_path()?;
        println!("{}", res);
        Ok(())
    }

    fn run_watcher(dir: &str) -> Result<()> {
        let root = env::current_dir().into_diagnostic()?;
        let root = root.to_str().unwrap();
        fs::create_dir_all(dir.clone()).into_diagnostic()?;

        env::set_current_dir(dir).into_diagnostic()?;

        // Run watcher
        let mut process = Process::new("cargo run --bin pipelight init --template toml");
        process.run_detached()?;
        let mut process = Process::new("cargo run --bin pipelight watch");
        process.run_detached()?;
        let res = Watcher::has_homologous_already_running()?;

        env::set_current_dir(root).into_diagnostic()?;
        remove_dir_all(dir);

        Ok(())
    }


    #[test]
    pub fn test_single_watcher() -> Result<()> {
        // Create tmp dir
        let test_dir = "./test_dir_tmp/watcher".to_owned();
        let a = test_dir.clone() + "/a";
        run_watcher(&a);
        Ok(())
    }

    #[test]
    pub fn test_multiple_watcher() -> Result<()> {
        // Create tmp dir to run isolated watchers
        let test_dir = "./test_dir_tmp/watcher".to_owned();
        let a = test_dir.clone() + "/a";
        let b = test_dir.clone() + "/b";

        for dir in vec![a,b] {
            run_watcher(&dir);
        }

        // find and kill processes
        let finder = Finder::new()
            .seed("pipelight")
            .seed("watch")
            .search()?;

        println!("{:?}", finder);

        Ok(())
    }
}
