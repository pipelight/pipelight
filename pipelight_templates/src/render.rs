// Templating
use handlebars::{Context, Handlebars};
// Error Handling
use miette::{IntoDiagnostic, Result};
// File systeme crates
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::types::{Assets, Style, Template};
use pipelight_utils::file::{is_filename, FileType};

impl Template {
    /**
     * Generate a base `pipelight.<extension>` configuration file
     * in the current directory
     */
    pub fn new(style: Option<String>, file: Option<String>) -> Result<Self> {
        let mut e = Template::default();
        let mut extension = "toml".to_owned();

        // If file defined, set the api style
        // by looking up to the file extension
        if let Some(file) = file {
            let file_extension = &Path::new(&file).extension();
            if let Some(file_extension) = file_extension {
                extension = file_extension.to_str().unwrap().to_owned();
                e.style = Style::from(&FileType::from(&extension));
            } else {
                extension = String::from(&FileType::default());
            }
            e.file_path = file;
        }
        // If style defined, set the api style and modify file extension
        if let Some(style) = style {
            let style = Style::from(&style);
            extension = String::from(&FileType::from(&style));
            e.style = style;
        }
        // Set the appropriate file extension
        e.file_path = Path::new(&e.file_path)
            .parent()
            .unwrap()
            .join(format!(
                "{}.{}",
                &Path::new(&e.file_path)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                &extension
            ))
            .to_str()
            .unwrap()
            .to_owned();

        // If the provided path is a filename
        // Generate a file path exploitable by Handlebars
        if is_filename(Path::new(&e.file_path)).is_ok() {
            let absolute_path = format!(
                "{}/{}",
                env::current_dir().unwrap().to_str().unwrap(),
                &e.file_path.clone()
            );
            e.file_path = absolute_path;
        }
        Ok(e)
    }
    pub fn create(&self) -> Result<()> {
        let rendered = self.create_config_template()?;
        self.write_config_file(&rendered)?;
        Ok(())
    }
    pub fn create_ignore(&self) -> Result<()> {
        let rendered = self.create_ignore_template()?;
        self.write_ignore_file(&rendered)?;
        Ok(())
    }
    pub fn try_delete(&self) -> Result<()> {
        let path = Path::new(&self.file_path)
            .canonicalize()
            .into_diagnostic()?;
        _ = fs::remove_file(path).into_diagnostic().is_ok();
        Ok(())
    }
    /**
    Generate in memory the config file template.
    */
    pub fn create_config_template(&self) -> Result<String> {
        let style = &String::from(&self.style);
        let extension = &String::from(&FileType::from(&self.style));
        let path = format!("{}.{}", style, extension);
        let mut handlebars = Handlebars::new();
        handlebars
            .register_embed_templates::<Assets>()
            .into_diagnostic()?;
        let rendered_string = handlebars
            .render_with_context(&path, &Context::null())
            .into_diagnostic()?;
        Ok(rendered_string)
    }
    /**
    Write the config file template to filesystem.
    */
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
    /**
    Generate in memory the config file template.
    */
    pub fn create_ignore_template(&self) -> Result<String> {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_embed_templates::<Assets>()
            .into_diagnostic()?;
        let rendered_string = handlebars
            .render_with_context("pipelight_ignore", &Context::null())
            .into_diagnostic()?;
        Ok(rendered_string)
    }
    /**
    Write the config file template to filesystem.
    */
    fn write_ignore_file(&self, code: &String) -> Result<()> {
        let ignore_path = format!(
            "{}/.pipelight_ignore",
            env::current_dir().unwrap().to_str().unwrap(),
        );
        let path = Path::new(&ignore_path);
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

#[cfg(test)]
mod test {
    use crate::Template;
    use std::env;
    use std::path::Path;

    /**
    Retrieve the parent directory of this test file
    */
    pub fn get_test_dir() -> String {
        // Test module dir
        let test_dir = Path::new(file!())
            .parent()
            .unwrap()
            .strip_prefix("templates")
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        println!("test_dir={:?}", test_dir);
        // Test pwd
        let pwd = env::current_dir().unwrap();
        let pwd = pwd.to_str().unwrap();
        println!("pwd={:?}", pwd);

        test_dir
    }

    // #[test]
    fn handlebars_find_template_files() {
        let res = Template::default().create_config_template();
        assert!(res.is_ok())
    }
    // #[test]
    fn create_default_file() {
        let e = Template::new(
            None,
            Some(format!("{}/default.pipelight.ts", get_test_dir())),
        )
        .unwrap();
        println!("{:?}", e);
        assert!(e.create().is_ok());
    }
    // #[test]
    fn create_helpers_api_file() {
        let e = Template::new(
            Some("helpers".to_owned()),
            Some(format!("{}/helper.pipelight.ts", get_test_dir())),
        )
        .unwrap();
        println!("{:?}", e);
        assert!(e.create().is_ok());
    }
    // #[test]
    fn create_toml_file() {
        let e = Template::new(
            Some("toml".to_owned()),
            Some(format!("{}/toml.pipelight.toml", get_test_dir())),
        )
        .unwrap();
        println!("{:?}", e);
        assert!(e.create().is_ok());
    }
    // #[test]
    fn create_yaml_file() {
        let e = Template::new(
            Some("yaml".to_owned()),
            Some(format!("{}/yaml.pipelight.yaml", get_test_dir())),
        )
        .unwrap();
        println!("{:?}", e);
        assert!(e.create().is_ok());
    }
    // #[test]
    fn create_with_wrong_file_extension() {
        let e = Template::new(None, Some(format!("{}/pipelight.fail", get_test_dir()))).unwrap();
        println!("{:?}", e);
        assert!(e.create().is_ok());
    }
    // #[test]
    fn create_with_default_params() {
        let e = Template::new(None, None);
        println!("{:?}", e);
        assert!(e.is_ok());
    }
    // #[test]
    fn create_with_style_only() {
        let e = Template::new(Some("helpers".to_owned()), None);
        println!("{:?}", e);
        assert!(e.is_ok());
    }
}
