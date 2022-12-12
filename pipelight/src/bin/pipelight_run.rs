use shared::exec::exec_attach;
use std::env;

/// Launch detached subprocess
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let pipeline: String = args[1].to_owned();
    println!("child output")
    // for step in pipeline.steps {
    //     for command in step.commands{
    //         exec_attach(command);
    //     }
    // }
}
