// Templating
use handlebars::{Context, Handlebars};
// Error Handling
use log::{info, trace};
use miette::{IntoDiagnostic, Result};
// File systeme crates
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum Style {
    #[default]
    Objects,
    Helpers,
    Js,
    Toml,
    Yaml,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub file_name: String,
    pub style: Style,
}

impl Default for Template {
    fn default() -> Self {
        Template {
            file_name: "pipelight.ts".to_owned(),
            style: Style::default(),
        }
    }
}

impl Template {
    /**
    Create/Ensure a base `pipelight.ts` configuration file
    in the current directory
    */
    fn create_config_template(name: &str) -> Result<String> {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("helpers", "public/helpers_api.ts")
            .into_diagnostic()?;
        handlebars
            .register_template_file("objects", "public/objects_api.ts")
            .into_diagnostic()?;
        let rendered = handlebars
            .render_with_context(name, &Context::null())
            .into_diagnostic()?;
        Ok(rendered)
    }
    fn write_config_file(file_path: &str, code: &str) -> Result<()> {
        let path = Path::new(file_path);
        // Guard: don't overwrite existing file
        if !path.exists() {
            let owned_str = code.to_owned();
            let bytes = owned_str.as_bytes();
            let mut file = fs::File::create(path).into_diagnostic()?;
            file.write_all(bytes).into_diagnostic()?;
        }
        Ok(())
    }
    pub fn render() -> Result<()> {
        Ok(())
    }
}
