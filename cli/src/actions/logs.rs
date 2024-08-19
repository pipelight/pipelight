// Types
use pipelight_exec::{Statuable, Status};
use workflow::{pipeline::Filters, Getters, Logs, Node, Pipeline};
// Error Handling
use miette::{IntoDiagnostic, Result};

/**
Pretty print pipelines as a tree
*/
pub fn pretty(name: Option<String>) -> Result<()> {
    let mut pipelines = Logs::get()?;

    if let Some(name) = name {
        pipelines = Filters::filter_by_name(pipelines, &name)?;
    }

    for mut pipeline in pipelines {
        if pipeline.get_status() == Some(Status::Running) {
            pipeline.hydrate()?;
        }
        let node = Node::from(&pipeline.clone());
        println!("{}", node);
    }
    Ok(())
}

/**
Pretty print pipelines as json
*/
pub fn json(name: Option<String>) -> Result<()> {
    let mut pipelines = Logs::get()?;
    if let Some(name) = name {
        pipelines = Filters::filter_by_name(pipelines, &name)?;
    }
    for pipeline in pipelines {
        let pipeline_json =
            serde_json::to_string_pretty::<Pipeline>(&pipeline).into_diagnostic()?;
        println!("{}", pipeline_json);
    }
    Ok(())
}

/**
Clean
*/
pub fn clean() -> Result<()> {
    Logs::clean()?;
    Ok(())
}
