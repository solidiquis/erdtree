use crate::error::prelude::*;
use chrono::Utc;
use log::{LevelFilter, Log, Metadata, Record};
use std::fmt::Write;

pub static mut BUFFER: String = String::new();

pub struct LoggityLog;

impl LoggityLog {
    pub fn new() -> Self {
        LoggityLog {}
    }

    pub fn init() -> Result<&'static LoggityLog> {
        let logger = Box::new(LoggityLog::new());
        let leak: &'static LoggityLog = Box::leak(logger);
        log::set_logger(leak).into_report(ErrorCategory::Internal)?;
        log::set_max_level(LevelFilter::Info);
        Ok(leak)
    }
}

impl Log for LoggityLog {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        unsafe {
            let _ = writeln!(
                BUFFER,
                "[{}] {} {}",
                Utc::now().to_rfc3339(),
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {
        unsafe { println!("{BUFFER}") }
    }
}
