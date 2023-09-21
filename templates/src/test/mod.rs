#[cfg(test)]
mod template {
    use crate::Template;
    use std::env;
    use std::path::Path;

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
    #[test]
    fn handlebars_find_template_files() {
        let res = Template::default().create_config_template();
        assert!(res.is_ok())
    }
    #[test]
    fn create_default_file() {
        let e = Template::new(
            None,
            Some(format!("{}/default.pipelight.ts", get_test_dir())),
        )
        .unwrap();
        println!("{:?}", e);
        e.create().unwrap();
    }
    #[test]
    fn create_helpers_api_file() {
        let e = Template::new(
            Some("helpers".to_owned()),
            Some(format!("{}/helper.pipelight.ts", get_test_dir())),
        )
        .unwrap();
        println!("{:?}", e);
        e.create().unwrap();
    }
}
