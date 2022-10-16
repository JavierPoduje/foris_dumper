use crate::client::Client;
use std::process::{Command, Output};

pub trait DeleteScenario {
    fn delete_scenario(&self, client: &Client, scenario: &str) -> Output {
        Command::new("mysql")
            .args([
                format!("--host={}", client.host),
                format!("--user={}", client.username),
                format!("--password={}", client.password),
                format!("--port={}", "4006"),
                format!("-e DROP DATABASE IF EXISTS {}", scenario),
            ])
            .output()
            .unwrap_or_else(|_| panic!("Couldn't drop DB {}", scenario))
    }
}
