use std::env;

/// Launch detached subprocess
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let _pipeline: String = args[1].to_owned();
    println!("child output")
    // for step in pipeline.steps {
    // exec_attah(pi)
    // println!("{}", step)
    // }
}
