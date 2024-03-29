use super::types::LogType;
use chrono::{Local, Timelike};
use colored::Colorize;

pub struct Logger {}

impl Logger {
    pub fn send(message: String, log_type: LogType) {
        match log_type {
            LogType::Info => {
                let stamp = format!("{} INFO", Logger::time()).green();
                println!("[{}]: {}", stamp, message);
            }
            LogType::Error => {
                let stamp = format!("{} ERROR", Logger::time()).red();
                println!("[{}]: {}", stamp, message);
            }
        }
    }

    fn time() -> String {
        let now = Local::now();
        format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
    }
}
