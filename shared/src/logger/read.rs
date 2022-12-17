use crate::types::logs::PipelineLog;
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn json_logs() {
    let file_path = ".pipelight/logs/pipelines.json.log";
    let contents = read_to_string(file_path).expect("Should have been able to read the file");
    println!("{}", contents);
}
pub fn raw_logs() {
    let file_path = ".pipelight/logs/pipelines.raw.log";
    let contents = read_to_string(file_path).expect("Should have been able to read the file");
    println!("{}", contents);
}

pub fn pretty_logs() -> Result<(), Box<dyn Error>> {
    let file_path = ".pipelight/logs/pipelines.json.log";
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(json) = line {
                let log = serde_json::from_str::<PipelineLog>(&json)?;

                println!("{:?}", log);
            }
        }
    }
    Ok(())
}
