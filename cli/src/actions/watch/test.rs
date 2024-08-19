#[cfg(test)]
mod watcher {
    // Env
    use std::env;
    use std::fs;
    use std::fs::remove_dir_all;
    // Globals
    use crate::actions::watch::build::get_ignore_path;
    use crate::actions::watch::{build, Watcher};
    use pipelight_exec::Process;
    use pipelight_utils::teleport::Portal;
    // Error handling
    use miette::{Diagnostic, IntoDiagnostic, Result};
    // Logger
    use log::warn;
    // Fancy color
    use colored::Colorize;
    // Process finder
    use pipelight_exec::Finder;

    fn print_cwd() -> Result<()> {
        let path = env::current_dir().into_diagnostic()?;
        let string = path.to_str().unwrap();
        println!("$pwd is {}", string.blue());
        Ok(())
    }

    #[tokio::test]
    async fn builder() -> Result<()> {
        // Teleport
        Portal::new()?.seed("test.pipelight").search()?.teleport()?;
        // Build watcher
        let res = build().await;
        assert!(res.is_ok());
        Ok(())
    }
    // #[trun_detached
    async fn try_start() -> Result<()> {
        // Teleport
        Portal::new()?.seed("test.pipelight").search()?.teleport()?;
        // Watcher::start()?;
        let watchexec = build().await?;
        watchexec.main().await.into_diagnostic()?;
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
        fs::create_dir_all(dir).into_diagnostic()?;
        print_cwd()?;

        env::set_current_dir(dir).into_diagnostic()?;
        print_cwd()?;

        // Run watcher
        let mut process = Process::new("cargo run --bin pipelight init --template toml");
        process.run_piped()?;
        let mut process = Process::new("cargo run --bin pipelight watch --attach");
        process.run_detached()?;
        // let res = Watcher::has_homologous_already_running()?;

        Ok(())
    }

    #[test]
    pub fn test_single_watcher() -> Result<()> {
        let root = env::current_dir().into_diagnostic()?;
        let root = root.to_str().unwrap();

        // Create tmp dir
        let test_dir = "./test_dir_tmp/watcher".to_owned();
        let a = test_dir.clone() + "/a";
        for dir in vec![a.clone(), a] {
            run_watcher(&dir)?;
            env::set_current_dir(&root).into_diagnostic()?;
        }

        env::set_current_dir(&test_dir).into_diagnostic()?;

        // Wait for propagation
        Process::new("sleep 1").run_piped()?;

        let finder = Watcher::find_all()?;
        println!("{:#?}", finder);

        // Clean
        // Bug or feature?
        // The action of removing directories that are watched stops the watcher.
        env::set_current_dir(root).into_diagnostic()?;
        remove_dir_all(test_dir).into_diagnostic()?;

        assert_eq!(finder.clone().matches.unwrap().len(), 1);

        finder.kill()?;
        Ok(())
    }

    #[test]
    /**
    Run watchers in unrelated projects
    */
    pub fn test_multiple_watcher() -> Result<()> {
        let root = env::current_dir().into_diagnostic()?;
        let root = root.to_str().unwrap();

        // Create tmp dir to run isolated watchers
        let test_dir = "./test_dir_tmp/watcher".to_owned();
        let a = test_dir.clone() + "/a";
        let b = test_dir.clone() + "/b";

        for dir in vec![a, b] {
            run_watcher(&dir)?;
            env::set_current_dir(&root).into_diagnostic()?;
        }

        env::set_current_dir(&test_dir).into_diagnostic()?;

        // Wait for propagation
        Process::new("sleep 1").run_piped()?;

        let finder = Watcher::find_all()?;
        println!("{:#?}", finder);

        // Clean
        // Bug or feature?
        // The action of removing directories that are watched stops the watcher.
        env::set_current_dir(root).into_diagnostic()?;
        remove_dir_all(test_dir).into_diagnostic()?;

        assert_eq!(finder.clone().matches.unwrap().len(), 2);

        finder.kill()?;
        Ok(())
    }
}
