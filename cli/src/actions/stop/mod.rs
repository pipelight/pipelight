use pipeline::Logs;

// Error Handling
use miette::Result;

/// Stop pipeline and attached pipelines/subprocesses
pub fn stop(pipeline_name: &String) -> Result<()> {
    // Get pipelines with provided name
    let pipelines = Logs::get_many_by_name(&pipeline_name)?;
    for mut pipeline in pipelines {
        pipeline.stop()
    }
    Ok(())
}
