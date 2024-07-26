// Struct
use crate::exec::{Process, Status};

/**
In pipelight, many are the structs and enum substructs that have a status.
This trait is to query the deepenth status in a more standardize way.
*/
pub trait Statuable {
    fn get_status(&self) -> Option<Status>;
    fn set_status(&mut self, status: Option<Status>);
}

/**
Implementation of the Statuable trait for the process struct.
*/
impl Statuable for Process {
    fn get_status(&self) -> Option<Status> {
        self.state.status.to_owned()
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.state.status = status;
    }
}
