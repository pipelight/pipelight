#[cfg(test)]
mod convert {
    use crate::dates::convert::*;
    use crate::dates::types::Duration;
    use std::time;

    #[test]
    fn string_to_duration() {
        let string = "PT120.025298910S";
        iso8601_to_std_duration(&string).unwrap();
    }
    #[test]
    fn duration_to_string() {
        let std = time::Duration::new(5, 0);
        std_duration_to_iso8601(&std).unwrap();
    }
    // Date and time
    use chrono::{DateTime, NaiveDateTime};
    #[test]
    fn iso8601_to_date() {
        let iso = "2023-07-28 09:20:00.228245012 +0200";
        let res = DateTime::parse_from_str(iso, "%Y-%m-%d %H:%M:%S%.9f %z");
        assert!(res.is_ok());
    }
    #[test]
    fn simple_iso8601_to_date() {
        let iso = "2023-08-06 14:54:30.221";
        let res = NaiveDateTime::parse_from_str(iso, "%Y-%m-%d %H:%M:%S%.3f");
        assert!(res.is_ok());
    }
}
