// Call ts-node on mjs/ts files

use log::{debug, error, info, trace, warn};
use subprocess::{Exec, Popen, PopenConfig, PopenError};

pub fn ts_node(mut working_dir: &str) -> Result<(), PopenError> {
    if working_dir.trim().is_empty() {
        working_dir = "./"
    }
    let command: String = "ts-node ".to_owned();
    let entrypoint: &str = "pipelight.ts";
    // Popen::create(&[command, entrypoint], PopenConfig::default())?;
    let res = { Exec::shell(command + entrypoint) }
        .cwd(working_dir)
        .capture()?
        .stdout_str();

    println!("{}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal() {
        // set cwd to test in other folder
        ts_node("./test");
    }
}
