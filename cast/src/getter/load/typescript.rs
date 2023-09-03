use crate::Config;

impl Config {
    /// Return a Config struct from a provided typescript file path
    fn ts(file_path: &str, args: Option<Vec<String>>) -> Result<Config> {
        // Fail safe guards
        Config::lint(file_path)?;
        Config::check(file_path, args.clone())?;

        let executable = "deno eval";
        let script = main_script(file_path);
        let command = if args.is_some() {
            format!("{} {} -- {}", executable, script, args.unwrap().join(" "))
        } else {
            format!("{} {}", executable, script)
        };
        let p = Process::new(&command).simple()?;
        let json = p.state.stdout.unwrap();
        let res = serde_json::from_str::<Config>(&json);
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                let err = JsonError::new(e, &json);
                Err(err.into())
            }
        }
    }
    /// Check if the deno script contains syntax errors
    fn lint(file: &str) -> Result<()> {
        // debug!("Linting config file");
        let command = format!(
            "deno lint \
            --rules-exclude=no-explicit-any,no-unused-vars \
            --quiet {}",
            file
        );
        let p = Process::new(&command).simple()?;
        if p.state.stderr.is_none() {
            Ok(())
        } else {
            let message = p.state.stderr.unwrap();
            Err(Error::msg(message))
        }
    }
    /// Run the script to detect runtime errors
    fn check(file: &str, args: Option<Vec<String>>) -> Result<()> {
        // debug!("Linting config file");
        let mut command = format!(
            "deno run \
            --allow-net \
            --allow-read \
            --allow-env \
            --allow-run \
            --check \
            --quiet \
            {}",
            file,
        );
        if args.is_some() {
            command = format!("{} {}", command, args.unwrap().join(" "));
        }

        let p = Process::new(&command).simple()?;

        if p.state.stderr.is_none() {
            Ok(())
        } else {
            let message = p.state.stderr.unwrap();
            Err(Error::msg(message))
        }
    }
}
