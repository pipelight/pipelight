// Structs
use crate::types::{Io, Process, State};
use uuid::Uuid;

impl Process {
    /**
    The prefered way to create a process struct.
    Pass the stdin deep down the Io substruct.
    and Pass the process uuid to its underlying Io substruct.
    */
    pub fn new(stdin: &str) -> Process {
        let uuid = Some(Uuid::new_v4());
        Process {
            uuid,
            pid: None,
            cwd: None,
            io: Io {
                uuid,
                stdin: Some(stdin.to_owned()),
                ..Io::default()
            },
            state: State::default(),
        }
    }
}
impl Default for Process {
    /**
    Pass the process uuid to its underlying Io substruct.
    */
    fn default() -> Process {
        let uuid = Some(Uuid::new_v4());
        Process {
            uuid,
            pid: None,
            cwd: None,
            io: Io {
                uuid,
                ..Io::default()
            },
            state: State::default(),
        }
    }
}
