// Call ts-node on mjs/ts files

use log::{debug, error, info, trace, warn};
use subprocess::Exec;

pub fn ts_node() {
    let command: String = "ts-node ".to_owned();
    let entrypoint: &str = "simp.config.mjs";
    let dir_checksum = { Exec::shell(command + entrypoint) }.capture();
    // debug!("{dir_checksum}")
}
