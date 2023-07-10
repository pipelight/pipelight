use crate::interface::types::ColoredOutput;
use convert_case::{Case, Casing};

use log::error;
use std::process::exit;

impl From<&String> for ColoredOutput {
    fn from(action: &String) -> ColoredOutput {
        let cased: &str = &action.to_case(Case::Kebab);
        match cased {
            "always" => ColoredOutput::Always,
            "never" => ColoredOutput::Never,
            _ => {
                let message = "Couldn't convert arg to exploitable enum";
                error!("{}", message);
                exit(1);
            }
        }
    }
}
impl From<&ColoredOutput> for String {
    fn from(action: &ColoredOutput) -> String {
        match action {
            ColoredOutput::Always => "always".to_owned(),
            ColoredOutput::Never => "never".to_owned(),
        }
    }
}
