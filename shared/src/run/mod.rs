use exec::Exec;
use log::trace;
use pipeline::types::Pipeline;
use std::env;
use std::error::Error;
use std::thread;

/// To be called from the cli.
/// Either spawn a detached new process or spawn an attached thread
/// to run the pipeline
pub fn run_bin(pipeline_name: String, attach: bool) -> Result<(), Box<dyn Error>> {
    let bin = "pipelight-run";

    let pipeline = Pipeline::name(&pipeline_name)?;

    #[cfg(debug_assertions)]
    let command = format!("cargo run --bin {} {}", bin, pipeline_name);

    #[cfg(not(debug_assertions))]
    let command = format!("{} {}", bin, pipeline_name);

    match attach {
        true => {
            // Lauch attach thread
            run_in_thread(&pipeline_name)?;
        }
        false => {
            // Lauch detached process
            trace!("Create detached subprocess");
            Exec::new().detached(&command)?;
        }
    }
    Ok(())
}

/// Get command line args and run pipeline
pub fn run() -> Result<(), Box<dyn Error>> {
    // Collect Args
    let args = env::args().collect::<Vec<String>>();
    let pipeline_name: String = args[1].to_owned();

    let mut pipeline = Pipeline::name(&pipeline_name)?;
    pipeline.run();
    Ok(())
}

/// Launch attached thread
pub fn run_in_thread(name: &str) -> Result<(), Box<dyn Error>> {
    let name = name.to_owned();
    let thread = thread::spawn(move || {
        let mut pipeline = Pipeline::name(&name).unwrap();
        pipeline.run();
    });
    thread.join().unwrap();
    Ok(())
}
