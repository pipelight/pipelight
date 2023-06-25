use crate::types::{Process, Status};

pub trait Statuable {
    fn get_status(&self) -> Option<Status>;
    fn set_status(&mut self, status: Option<Status>);
}
impl Statuable for Process {
    fn get_status(&self) -> Option<Status> {
        return self.state.status.clone();
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.state.status = status;
    }
}
