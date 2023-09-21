// File systeme crates
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum Style {
    #[default]
    Objects,
    Helpers,
    Javascript,
    Toml,
    Yaml,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub file_path: String,
    pub style: Style,
}

impl Default for Template {
    fn default() -> Self {
        Template {
            file_path: format!(
                "{}/pipelight.ts",
                &env::current_dir().unwrap().to_str().unwrap()
            ),
            style: Style::default(),
        }
    }
}
