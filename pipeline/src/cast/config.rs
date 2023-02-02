use super::Config;
use exec::Exec;
use log::{debug, error, trace, warn};

// standard lib
use std::env::current_dir;
use std::error::Error;
use std::path::Path;
use std::process::exit;

impl Config {
    pub fn load_from_string(js_object: &str) -> Result<Config, Box<dyn Error>> {
        let executable = "node -e";

        // Javascript with rust escape sequence (double curly braces)
        let script = format!(
            "\'
            const stock = console;
            console = {{}};
            console = stock;
            {};
            const json = JSON.stringify(config, null, 2);
            console.log(json);
        \'",
            js_object
        );

        let command = format!("{} {}", executable, script);
        let data = Exec::new().simple(&command)?;

        trace!("Config json output:\n{}", &data.stdout.clone().unwrap());

        let config = serde_json::from_str::<Config>(&data.stdout.clone().unwrap())?;
        Ok(config)
    }
    pub fn get() -> Config {
        let file = "pipelight.config.mjs";
        let res = Config::load_from_file(&file);
        match res {
            Ok(res) => {
                return res;
            }
            Err(e) => {
                let message = format!("From config file:\n{}", e);
                error!("{}", message);
                exit(1);
            }
        };
    }
    /// Ensure config file exists
    fn exists(file: &str) -> Result<bool, Box<dyn Error>> {
        let pwd = current_dir().unwrap();
        let string = format!("{}/{}", &pwd.display().to_string(), file);
        let path = Path::new(&string);
        let exist = Path::new(path).exists();
        if !exist {
            let message = "Config file not found.";
            error!("{}", message);
            exit(1);
        } else {
            return Ok(exist);
        }
    }

    /// Ensure that the node.js has no error
    fn lint(file: &str) -> Result<(), Box<dyn Error>> {
        debug!("Linting config file");
        let command = format!("node {}", file);
        let data = Exec::new().simple(&command)?;
        if data.stdout.is_none() {
            if data.stderr.is_none() {
                Ok(())
            } else {
                let message = format!("{}", data.stderr.unwrap());
                Err(Box::from(message))
            }
        } else {
            if data.stderr.is_none() {
                Ok(())
            } else {
                let message = format!("{}", data.stderr.unwrap());
                warn!("Node.js Output:\n{}", data.stdout.unwrap());
                Err(Box::from(message))
            }
        }
    }
    /// Return the config from .mjs file inside the working dir.
    fn load_from_file(file: &str) -> Result<Config, Box<dyn Error>> {
        Config::exists(file)?;
        Config::lint(file)?;

        let pwd = current_dir().unwrap();
        let string = format!("{}/{}", &pwd.display().to_string(), file);
        let path = Path::new(&string);

        let executable = "node -e";
        // Javascript with rust escape sequence (double curly braces)
        let script = format!(
            "\'
            const stock = console;
            console = {{}};
            const promess = import(`{}`);
            promess
              .then((res) => {{ 
                console = stock;
                const config = res.default;
                const json = JSON.stringify(config, null, 2);
                console.log(json);
              }})
              .catch((err) => {{
                console.log(err);
              }});
        \'",
            path.display().to_string()
        );
        let command = format!("{} {}", executable, script);
        let data = Exec::new().simple(&command)?;

        trace!("Config json output:\n{}", &data.stdout.clone().unwrap());

        let config = serde_json::from_str::<Config>(&data.stdout.clone().unwrap())?;
        Ok(config)
    }
}
