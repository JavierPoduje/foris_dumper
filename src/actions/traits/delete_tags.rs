use crate::client::Client;
use std::process::{Command, Output};

pub trait DeleteTags {
    fn delete_tags(&self, client: &Client) -> Output {
        Command::new("mysql")
            .args([
                format!("--host={}", client.host),
                format!("--user={}", client.username),
                format!("--password={}", client.password),
                format!("--port={}", "4006"),
                format!("-e DELETE from {}.tags", client.scenarios_db),
            ])
            .output()
            .expect("Couldn't delete tags")
    }
}
