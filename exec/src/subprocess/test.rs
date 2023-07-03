#[cfg(test)]
mod process {
    use crate::types::{Environment, Process, State, Status};
    // Test Cli struct to bash string reversion
    #[test]
    fn run_and_read() {
        // Define a cli struct
        let mut process = Process::new("echo my_read_test");
        process.run().unwrap();
        assert_eq!(Some("my_read_test\n".to_owned()), process.state.stdout);
    }
    #[test]
    fn simple_attached_run() {
        // Define a cli struct
        let mut process = Process::new("echo my_read_test");
        process.simple().unwrap();
        assert_eq!(Some(Status::Succeeded), process.state.status);
    }
}
