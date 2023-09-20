#[cfg(test)]
mod template {
    use crate::Template;
    use std::path::Path;
    pub fn get_test_dir() -> String {
        let test_dir = Path::new(file!())
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        println!("test dir {}", test_dir);
        test_dir
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
