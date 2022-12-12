#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#[allow(dead_code)]
pub mod actions;
pub mod cli;
pub mod config;
pub mod exec;
pub mod logger;
pub mod types;

#[macro_export]
macro_rules! duration {
    ($function: expr) => {
        use logger::trace;
        use std::time::Instant;
        let start = Instant::now();
        $function;
        let duration = start.elapsed();
        trace!("{:?}", duration)
    };
}
