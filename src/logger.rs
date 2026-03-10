use std::time::SystemTime;

use strum::Display;

#[derive(Debug, Default)]
pub struct Logger {
    pub logs: Vec<LogMessage>,
}

#[derive(Debug, Clone)]
pub struct LogMessage {
    pub log_level: LogLevel,
    pub timestamp: SystemTime,
    pub message: String,
}

#[derive(Debug, Display, Clone)]
#[strum(serialize_all = "UPPERCASE")]
pub enum LogLevel {
    Debug,
    Info,
    Error,
    Fatal,
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn log(&mut self, message: &str, log_level: LogLevel) {
        self.logs.push(LogMessage {
            log_level: log_level.clone(),
            timestamp: SystemTime::now(),
            message: log_level.to_string() + ": " + message,
        });
    }

    pub fn log_debug(&mut self, message: &str) {
        self.log(message, LogLevel::Debug);
    }

    pub fn log_info(&mut self, message: &str) {
        self.log(message, LogLevel::Info);
    }

    pub fn log_error(&mut self, message: &str) {
        self.log(message, LogLevel::Error);
    }

    pub fn log_fatal(&mut self, message: &str) {
        self.log(message, LogLevel::Fatal);
    }
}
