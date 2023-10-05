#[cfg(test)]
mod ignore {
    use crate::files::types::Ignore;

    #[tokio::test]
    async fn from_ignore_file() {
        let res = Ignore::new("./public/.pipelight_ignore");
        // println!("{:#?}", ignore);
        assert!(res.is_ok());
    }
}
