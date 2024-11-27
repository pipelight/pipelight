// Structs
use crate::Config;
// Filesystem - read file
use std::fs;
// Error Handling
use miette::{Error, IntoDiagnostic, Result};

// Filesystem
use pipelight_error::{CastError, HclError, JsonError, PipelightError, TomlError, YamlError};
use pipelight_utils::FileType;
use std::path::Path;

// Tests
mod test;

mod rules;
mod typescript;

impl Config {
    pub fn load_many(root_path: &str, args: Option<Vec<String>>) -> Result<()> {
        Ok(())
    }
    /**
     * Choose the appropriated method to load the config file
     * according to the file extension(.ts, .toml, .yml...).
     *
     * Arguments:
     * - file_path is the config file path
     * - args are only to be used with scripting language (typescript) to pass args to the underlying script.
     *
     * Languages coming next after v1.0.0:
     * - Rust, Hcl, Kcl, Python...
     */
    pub fn load(file_path: &str, args: Option<Vec<String>>) -> Result<Config> {
        let extension = &Path::new(file_path)
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        let file_type = FileType::from(extension);
        let mut config = match file_type {
            FileType::TypeScript | FileType::JavaScript => Config::ts(file_path, args)?,
            FileType::Json => Config::json(file_path)?,

            FileType::Toml | FileType::Tml => Config::tml(file_path)?,
            FileType::Yaml | FileType::Yml => Config::yml(file_path)?,
            // FileType::Kdl => Config::hcl(file_path)?,
            FileType::Hcl => Config::hcl(file_path)?,
            // FileType::Pkl => Config::pkl(file_path)?,
        };
        Ok(config.strict_check()?)
    }
    /**
    Returns a Config struct from a provided json file path.
    */
    pub fn json(file_path: &str) -> Result<Config> {
        let string = fs::read_to_string(file_path).into_diagnostic()?;
        let res = serde_json::from_str::<Config>(&string);
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                let err = CastError::JsonError(JsonError::new(e, &string));
                Err(err.into())
            }
        }
    }
    /**
    Returns a Config struct from a provided toml file path.
    */
    pub fn tml(file_path: &str) -> Result<Config> {
        let string = fs::read_to_string(file_path).into_diagnostic()?;
        let res = toml::from_str::<Config>(&string);
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                let err = CastError::TomlError(TomlError::new(e, &string));
                Err(err.into())
            }
        }
    }
    /**
    Returns a Config struct from a provided toml file path.
    */
    // pub fn kdl(file_path: &str) -> Result<Config> {
    //     let string = fs::read_to_string(file_path).into_diagnostic()?;
    //     let res: kdl::KdlDocument = string.parse().into_diagnostic()?;
    //     let nodes = res.nodes();
    //     let node = nodes[0].clone();
    //     let res = serde_json::to_value(node);
    //     println!("{:#?}", nodes);
    //     Ok(res);
    // }
    /**
    Returns a Config struct from a provided hcl file path.
    */
    pub fn hcl(file_path: &str) -> Result<Config> {
        let string = fs::read_to_string(file_path).into_diagnostic()?;
        let res = hcl::from_str::<Config>(&string);
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                let err = CastError::HclError(HclError::new(e, &string));
                Err(err.into())
            }
        }
    }
    /**
    Returns a Config struct from a provided yaml file path.
    */
    pub fn yml(file_path: &str) -> Result<Config> {
        let string = fs::read_to_string(file_path).into_diagnostic()?;
        let res = serde_yaml::from_str::<Config>(&string);
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                let err = CastError::YamlError(YamlError::new(e, &string));
                Err(err.into())
            }
        }
    }
}

#[cfg(test)]
mod cast {
    use super::*;
    use std::path::PathBuf;

    use crate::{Config, Logs};

    #[test]
    fn toml_configrs() -> Result<()> {
        // Get file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../examples");
        let path = path.display().to_string();

        let res = Config::load_many(&path, None);
        assert!(res.is_ok());
        Ok(())
    }

    #[test]
    fn typescript() -> Result<()> {
        // Get file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../examples/pipelight.ts");
        let path = path.display().to_string();

        let res = Config::load(&path, None);
        assert!(res.is_ok());
        Ok(())
    }
    #[test]
    fn javascript() -> Result<()> {
        // Get file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../examples/pipelight.js");
        let path = path.display().to_string();

        let res = Config::load(&path, None);
        assert!(res.is_ok());
        Ok(())
    }
    #[test]
    fn toml() -> Result<()> {
        // Get file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../examples/pipelight.toml");
        let path = path.display().to_string();

        let res = Config::load(&path, None);
        assert!(res.is_ok());
        Ok(())
    }
    #[test]
    fn hcl() -> Result<()> {
        // Get file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../examples/pipelight.hcl");
        let path = path.display().to_string();

        let res = Config::load(&path, None);
        assert!(res.is_ok());
        Ok(())
    }
    // #[test]
    // fn kdl() {
    //     let res = Config::load("./public/pipelight.kdl", None);
    //     println!("{:#?}", res);
    //     assert!(res.is_ok());
    // }

    // #[test]
    // fn pkl() {
    //     let res = Config::load("./public/pipelight.pkl", None);
    //     println!("{:#?}", res);
    //     assert!(res.is_ok());
    // }
    #[test]
    fn yaml() -> Result<()> {
        // Get file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../examples/pipelight.yaml");
        let path = path.display().to_string();

        let res = Config::load(&path, None);
        assert!(res.is_ok());
        Ok(())
    }
    #[test]
    fn logs() -> Result<()> {
        // Get file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("./public");
        let path = path.display().to_string();

        let res = Logs::read(&path);
        assert!(res.is_ok());
        Ok(())
    }
}
