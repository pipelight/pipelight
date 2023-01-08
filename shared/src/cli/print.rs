use pipeline::types::Pipelines;
use std::error::Error;
/// Pretty print logs from json log file
pub fn pretty() -> Result<(), Box<dyn Error>> {
    let pipelines = Pipelines::get()?;
    for pipeline in pipelines {
        println!("{}", pipeline);
    }
    Ok(())
}

/// Print json log file
pub fn json() -> Result<(), Box<dyn Error>> {
    let pipelines = Pipelines::get()?;
    for pipeline in pipelines {
        println!("{:?}", pipeline);
    }
    Ok(())
}
