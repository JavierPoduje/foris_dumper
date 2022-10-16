use crate::client::Client;
use std::process::{Command, Output};

pub trait DeleteModelExtensions {
    fn delete_model_extensions(&self, client: &Client) -> Output {
        Command::new("mysql")
            .args([
                format!("--host={}", client.host),
                format!("--user={}", client.username),
                format!("--password={}", client.password),
                format!("--port={}", "4006"),
                format!("-e DELETE from {}.model_extensions", client.scenarios_db),
            ])
            .output()
            .expect("Couldn't delete model_extensions")
    }
}
