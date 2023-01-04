use pipeline::types::{Pipeline, Pipelines};
use rev_buf_reader::RevBufReader;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn read_last_line(path: &Path) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf = RevBufReader::new(file);
    let mut lines = buf.lines();
    let last_line = lines.next().unwrap().unwrap();
    Ok(last_line)
}

/// Pretty print logs from json log file
pub fn pretty() -> Result<(), Box<dyn Error>> {
    let pipelines = Pipelines::get()?;
    for pipeline in pipelines {
        println!("{}", pipeline);
    }
    Ok(())
}
/// Pretty print logs from json log file
pub fn get_pipelines() -> Result<Vec<Pipeline>, Box<dyn Error>> {
    let paths = fs::read_dir(".pipelight/logs").unwrap();
    let mut pipelines = vec![];
    for res in paths {
        let dir_entry = res?;
        let json = read_last_line(&dir_entry.path())?;
        let pipeline = serde_json::from_str::<Pipeline>(&json)?;
        pipelines.push(pipeline);
    }
    Ok(pipelines)
}
pub fn json() -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir(".pipelight/logs").unwrap();
    for res in paths {
        let dir_entry = res?;
        let json = read_last_line(&dir_entry.path())?;
        let pipeline = serde_json::from_str::<Pipeline>(&json)?;
        println!("{:?}", pipeline);
    }
    Ok(())
}
