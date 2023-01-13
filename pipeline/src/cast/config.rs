use super::Config;
use exec::Exec;
use log::{debug, error, info, trace, warn};

// standard lib
use std::env::current_dir;
use std::error::Error;
use std::path::Path;

impl Config {
    pub fn new() -> Self {
        // Git::new();
        let file = "pipelight.config.mjs";
        let mut config = Config {
            file: file.to_owned(),
            pipelines: None,
        };
        if config.exists() {
            config = Config::load().unwrap();
        }
        return config;
    }
    /// Ensure config file exists
    fn exists(&self) -> bool {
        let pwd = current_dir().unwrap();
        let string = format!("{}/{}", &pwd.display().to_string(), self.file);
        let path = Path::new(&string);
        let exist = Path::new(path).exists();
        if !exist {
            let message = "Config file not found.";
            let hint =
                "Use \"pipelight init\" to generate config file\n or move to the right directory";
            error!("{}", message);
            debug!("{}", hint);
        }
        return exist;
    }

    /// Return the config from .mjs file inside the working dir.
    fn load() -> Result<Config, Box<dyn Error>> {
        let executable = "node -e";
        let script = r#"'
            const stock = console;
            console = {};
            const promess = import(`./pipelight.config.mjs`);
            promess
              .then((res) => {
                console = stock;
                const config = res.default;
                const json = JSON.stringify(config, null, 2);
                console.log(json);
              })
              .catch((err) => {
                console.log(err);
              });
        '"#;
        let command = format!("{} {}", executable, script);
        let data = Exec::new().attached(&command)?;
        let config_result = serde_json::from_str::<Config>(&data);
        match config_result {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                let message = format!("From config file: {}", e);
                warn!("{}", message);
                debug!("Json output:\n{}", data);
                return Err(Box::from(message));
            }
        };
    }
}
