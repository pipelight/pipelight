use crate::types::config::Pipeline;
use crate::types::logs::{PipelineLog, PipelineStatus};
use colored::Colorize;
use log::{debug, error, info, trace, warn};
use rev_buf_reader::RevBufReader;
use std::error::Error;
use std::fs;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use uuid::Uuid;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn json_logs() {
    // let file_path = ".pipelight/logs/pipelines.json.log";
    // let contents = read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{}", contents);
}
pub fn raw_logs() {
    // let file_path = ".pipelight/logs/pipelines.raw.log";
    // let contents = read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{}", contents);
}
pub fn pretty_logs() -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir(".pipelight/logs").unwrap();
    for res in paths {
        let dir_entry = res?;
        let json = last_lines(&dir_entry.path())?;
        let pipeline = serde_json::from_str::<PipelineLog>(&json);
        println!("{:?}", pipeline);
    }
    // let logs = parse_logs()?;
    // let uuids: Vec<Uuid> = logs
    //     .iter()
    //     .filter(|log| log.status == PipelineStatus::Started)
    //     .map(|x| x.uuid)
    //     .collect();
    // let mut logs_by_uuid: Vec<PipelineLog> = vec![];
    // for uuid in uuids {
    //     logs_by_uuid = logs
    //         .clone()
    //         .into_iter()
    //         .filter(|log| log.uuid == uuid)
    //         .collect();
    // }
    // for log in logs_by_uuid {
    // println!("pipeline: {:?}", log.name);
    // if log.step.is_some() {
    //     let step = log.step.unwrap();
    //     info!(target:"pretty","\tstep: {0: <10}", step.name);
    //     debug!(target:"pretty","\t\tcommand: {}", step.command.stdin);
    //     trace!(target:"pretty","{}", step.command.stdout);
    //     error!(target:"pretty", "{}", step.command.stderr.red());
    // }
    // }
    Ok(())
}

// pub fn parse_logs() -> Result<Vec<PipelineLog>, Box<dyn Error>> {
//     let file_path = ".pipelight/logs";
//     let mut logs: Vec<PipelineLog> = vec![];
//     let lines = read_lines(file_path)?;
//     for line in lines {
//         // Parse from json file
//         let json = last_line_from_file(&file);
//         let log_result = serde_json::from_str::<PipelineLog>(&json);
//         let log = match log_result {
//             Ok(res) => res,
//             Err(e) => {
//                 let message = format!("Log file couldn't be parsed: {}", e);
//                 warn!("{}", message);
//                 return Err(Box::from(message));
//             }
//         };
//         logs.push(log);
//     }
//     Ok(logs)
// }

fn last_lines(path: &Path) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf = RevBufReader::new(file);
    let mut lines = buf.lines();
    let last_line = lines.next().unwrap().unwrap();
    Ok(last_line)
}
