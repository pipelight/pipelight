pub trait Statuable {
    fn get_status(&self) -> Option<Status>;
    fn set_status(&mut self, status: Option<Status>);
}
