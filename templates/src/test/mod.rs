#[cfg(test)]
mod template {
    use crate::render::Template;

    #[test]
    fn create_default_file() {
        let e = Template::default();
        println!("{:?}", e);
        e.create().unwrap();
    }
    #[test]
    fn create_file() {
        let e = Template::default();
        println!("{:?}", e);
        e.create().unwrap();
    }
}
