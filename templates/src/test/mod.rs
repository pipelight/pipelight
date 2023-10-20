#[cfg(test)]
mod template {
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
