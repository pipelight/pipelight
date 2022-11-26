#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use std::fmt;

// #[derive(Debug)]
// // #[serde(rename_all = “camelCase”)]
// pub struct Config<'a> {
//     pipelines: [&'a Pipeline<'a>],
// }
// #[derive(Debug)]
// pub struct Pipeline<'a> {
//     steps: [&'a Step<'a>],
// }
// #[derive(Debug)]
// pub struct Step<'a> {
//     name: &'a str,
//     commands: [&'a str],
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pipelines: [Pipeline; 1],
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    name: Option<String>,
    steps: [Step; 1],
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    name: String,
    commands: Option<[String; 10]>,
}
// impl fmt::Display for Step {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

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
                    { "name": "test" }
                ]
            }
            ]        
        }"#;
        let config: Config = serde_json::from_str(c).unwrap();
        println!("{:#?}", config);

        Ok(())
    }
}
