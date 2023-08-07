#[cfg(test)]
mod date {
    use crate::Event;
    // Date and time
    use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};
    // Error Handling
    use miette::{IntoDiagnostic, Result};

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
