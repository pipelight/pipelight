#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug, trace};
    use serde_json::json;
    use serde_json::{Result, Value};
    #[test]
    fn internal() -> Result<()> {
        //test single step assertion json -> struct
        let s = r#"{ "name": "test" }"#;
        let step: Step = serde_json::from_str(s).unwrap();
        println!("{:#?}", step);

        //test config assertion
        let c = r#"{
            "pipelines" : [
            {
                "steps" : [
                    { "name": "test" },
                    { "name": "test2" }
                ]
            }
            ]        
        }"#;
        let config: Config = serde_json::from_str(c).unwrap();
        println!("{:#?}", config);

        Ok(())
    }
}
