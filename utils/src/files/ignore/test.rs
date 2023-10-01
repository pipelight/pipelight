#[cfg(test)]
mod ignore {
    use crate::files::types::Ignore;

    #[tokio::test]
    async fn from_ignore_file() {
        let ignore = Ignore::new("./public/.pipelight_ignore").await.unwrap();
        // println!("{:#?}", ignore);
    }
}
