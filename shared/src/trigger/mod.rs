// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_must_use)]
// use crate::git::Git;
// use crate::logger::Logs;
// use crate::types::logs::PipelineLog;
// use crate::types::{Config, Pipeline, TriggerTuple};
// use log::{debug, error, info, trace, warn};
// #[allow(dead_code)]
// use project_root::get_project_root;
// use std::env;
// use std::env::current_dir;
// use std::error::Error;
// use std::path::Path;
// use std::process::exit;
//
// /// Launch attached subprocess
// pub fn trigger(env: &TriggerTuple) -> Result<(), Box<dyn Error>> {
//     get_event();
//
//     let handle = Logs::new().get()?;
//     let config = Config::new()?;
//
//     for pipeline in &config.pipelines {
//         if pipeline.triggers.is_none() {
//             let message = format!("No triggers defined for pipeline: {:?}", &pipeline.name);
//         } else {
//             for trigger in &pipeline.clone().triggers.unwrap() {
//                 let tuples = trigger.to_tuples()?;
//                 if tuples.contains(env) {
//                     let mut pipeline = PipelineLog::from(pipeline);
//                 }
//             }
//         }
//     }
//     Ok(())
// }
// fn get_event() -> Result<(), Box<dyn Error>> {
//     let mut mode = "";
//     let root = current_dir()?;
//     let path_string = root.display().to_string();
//     if path_string.contains("/.git/hooks/") {
//         let hook = root
//             .parent()
//             .unwrap()
//             .file_stem()
//             .unwrap()
//             .to_str()
//             .unwrap();
//         mode = hook;
//     } else {
//         mode = "manual";
//     }
//     debug!("mode: {:?}", &mode);
//     Ok(())
// }
