// Templating
use handlebars::{Context, Handlebars};
// Error Handling
use log::{info, trace};
use miette::{Error, IntoDiagnostic, Result};
// File systeme crates
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

use utils::files::FileType;

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
            file_path: "pipelight.ts".to_owned(),
            style: Style::default(),
        }
    }
}

impl Template {
    /**
    Create/Ensure a base `pipelight.ts` configuration file
    in the current directory
    */
    pub fn new(style: Option<String>, file: Option<String>) -> Result<Self> {
        let mut e = Template::default();
        let mut extension = "ts".to_owned();

        if let Some(file) = file {
            let file_extension = &Path::new(&file).extension();
            if let Some(file_extension) = file_extension {
                extension = file_extension.to_str().unwrap().to_owned();
                e.style = Style::from(&FileType::from(&extension));
            } else {
                extension = String::from(&FileType::default());
            }
        }
        if let Some(style) = style {
            let style = Style::from(&style);
            extension = String::from(&FileType::from(&style));
            e.style = style;
        }
        e.file_path = format!(
            "{}/{}.{}",
            &Path::new(&e.file_path).parent().unwrap().to_str().unwrap(),
            &Path::new(&e.file_path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap(),
            &extension
        );
        Ok(e)
    }
    pub fn create(&self) -> Result<()> {
        let rendered = self.create_config_template()?;
        self.write_config_file(&rendered)?;
        Ok(())
    }
    fn create_config_template(&self) -> Result<String> {
        let style = &String::from(&self.style);
        let extension = &String::from(&FileType::from(&self.style));
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file(style, format!("public/{}.{}", style, extension))
            .into_diagnostic()?;
        let rendered_string = handlebars
            .render_with_context(&style, &Context::null())
            .into_diagnostic()?;
        Ok(rendered_string)
    }
    fn write_config_file(&self, code: &String) -> Result<()> {
        let path = Path::new(&self.file_path);
        // Guard: don't overwrite existing file
        if !path.exists() {
            let owned_str = code.to_owned();
            let bytes = owned_str.as_bytes();
            let mut file = fs::File::create(path).into_diagnostic()?;
            file.write_all(bytes).into_diagnostic()?;
        }
        Ok(())
    }
}
