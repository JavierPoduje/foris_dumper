use crate::client::Client;
use std::process::{Command, Output};

pub trait CreateScenario {
    fn create_scenario(&self, client: &Client, scenario: &str) -> Output {
        Command::new("mysql")
            .args([
                format!("--host={}", client.host),
                format!("--user={}", client.username),
                format!("--password={}", client.password),
                format!("--port={}", "4006"),
                format!("-e CREATE DATABASE {}", scenario),
            ])
            .output()
            .unwrap_or_else(|_| panic!("Couldn't create DB {}", scenario))
    }
}
