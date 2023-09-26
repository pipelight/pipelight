use crate::types::{Io, Process, State};
use std::env;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use crate::globals::SHELL;

impl Process {
    pub fn new(stdin: &str) -> Process {
        let uuid = Some(Uuid::new_v4());
        Process {
            uuid,
            pid: None,
            io: Io {
                uuid,
                stdin: Some(stdin.to_owned()),
                ..Io::default()
            },
            state: State::default(),
        }
    }
    pub fn default() -> Process {
        let uuid = Some(Uuid::new_v4());
        Process {
            uuid,
            pid: None,
            io: Io {
                uuid,
                ..Io::default()
            },
            state: State::default(),
        }
    }
}
