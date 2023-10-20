use crate::types::Io;
use std::process::Output;

impl From<&Output> for Io {
    fn from(output: &Output) -> Io {
        let stdout_str = String::from_utf8(output.stdout.to_owned()).unwrap();
        // .strip_suffix("\r\n")
        // .unwrap()
        let stderr_str = String::from_utf8(output.stderr.to_owned()).unwrap();
        // .strip_suffix("\r\n")
        // .unwrap()

        let mut stdout = None;
        if !stdout_str.is_empty() {
            stdout = Some(stdout_str);
        }
        let mut stderr = None;
        if !stderr_str.is_empty() {
            stderr = Some(stderr_str);
        }
        Io {
            stdin: None,
            uuid: None,
            stdout,
            stderr,
        }
    }
}
