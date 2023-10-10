use workflow::Logs;
// Error Handling
use miette::Result;

/**
Stop the pipeline and its attached subprocesses.
*/
pub fn launch(pipeline_name: &str) -> Result<()> {
    // Get pipelines from the provided name.
    let pipelines = Logs::new().hydrate()?.get_many_by_name(pipeline_name)?;
    for mut pipeline in pipelines {
        pipeline.stop()?;
    }
    Ok(())
}
