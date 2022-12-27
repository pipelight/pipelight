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
        let pipeline = serde_json::from_str::<PipelineLog>(&json)?;
        println!("{}", pipeline);
    }
    Ok(())
}

fn last_lines(path: &Path) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf = RevBufReader::new(file);
    let mut lines = buf.lines();
    let last_line = lines.next().unwrap().unwrap();
    Ok(last_line)
}
