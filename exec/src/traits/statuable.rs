use crate::types::{Process, Status};

pub trait Statuable {
    fn get_status(&self) -> Option<Status>;
    fn set_status(&mut self, status: Option<Status>);
}
impl Statuable for Process {
    fn get_status(&self) -> Option<Status> {
        self.state.status.to_owned()
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.state.status = status;
    }
}
