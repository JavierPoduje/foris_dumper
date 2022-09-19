use super::types::LogType;
use chrono::{Local, Timelike};
use colored::Colorize;

pub struct Logger {}

impl Logger {
    pub fn send(message: &str, log_type: LogType) {
        match log_type {
            LogType::Info => {
                let stamp = format!("{} INFO", Logger::time()).green();
                println!("[{}]: {}", stamp, message);
            }
            LogType::Error => {
                let stamp = format!("{} ERROR", Logger::time()).red();
                println!("[{}]: {}", stamp, message);
            }
            LogType::Warning => {
                let stamp = format!("{} WARNING", Logger::time()).yellow();
                println!("[{}]: {}", stamp, message);
            }
        }
    }

    fn time() -> String {
        let now = Local::now();
        format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
    }
}
