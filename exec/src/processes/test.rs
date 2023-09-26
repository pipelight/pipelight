#[cfg(test)]
mod basic {
    use crate::globals::OUTDIR;
    use crate::types::{Io, Process, State, Status};
    use std::fs::remove_dir_all;
    #[test]
    fn run_piped() {
        let mut process = Process::new("echo test");
        process.run_piped().unwrap();
        assert_eq!(Some("test\n"), process.io.stdout.as_deref());
        assert_eq!(Some(Status::Succeeded), process.state.status);
    }
    #[test]
    fn run_muted() {
        let mut process = Process::new("echo test");
        process.run_muted().unwrap();
        assert_eq!(None, process.io.stdout.as_deref());
        assert_eq!(Some(Status::Succeeded), process.state.status);
    }
    #[test]
    fn run_fs() {
        let mut process = Process::new("echo test");
        process.run_fs().unwrap();
        // clean dirj
        remove_dir_all(&(*OUTDIR.lock().unwrap())).unwrap();
        assert_eq!(Some("test\n"), process.io.stdout.as_deref());
        assert_eq!(Some(Status::Succeeded), process.state.status);
    }
    #[test]
    fn run_detached() {
        let mut process = Process::new("echo test & sleep 10");
        process.run_detached().unwrap();
        assert_eq!(None, process.io.stdout.as_deref());
        assert_eq!(None, process.state.status);
    }
}
mod fs {
    use crate::globals::OUTDIR;
    use crate::types::{Io, Process, State, Status};
    use std::fs::remove_dir_all;

    #[test]
    fn run_fs_and_read() {
        let mut process = Process::new("echo test");
        process.run_fs().unwrap();
        // clean dirj
        remove_dir_all(&(*OUTDIR.lock().unwrap())).unwrap();
    }
}
