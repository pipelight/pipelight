#[cfg(test)]
mod convert {
    use crate::dates::types::Duration;

    #[test]
    fn string_to_duration() {
        let mut d = Duration::default();
        d.start().unwrap();
        d.stop().unwrap();
        assert!(d.computed.is_some());
    }
}
