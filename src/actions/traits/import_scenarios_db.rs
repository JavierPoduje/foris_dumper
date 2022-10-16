use crate::client::Client;
use std::process::{Command, Output, Stdio};

pub trait ImportScenariosDB {
    fn import_scenarios_db(
        &self,
        client: &Client,
        remote_scenarios_db: &str,
        folder: &str,
    ) -> Output {
        let cat = Command::new("cat")
            .args([format!("{}{}.sql", folder, remote_scenarios_db)])
            .stdout(Stdio::piped())
            .spawn();

        Command::new("mysql")
            .args([
                format!("--host={}", client.host),
                format!("--user={}", client.username),
                format!("--password={}", client.password),
                format!("--port={}", "4006"),
                client.scenarios_db.to_string(),
            ])
            .stdin(cat.ok().unwrap().stdout.unwrap())
            .output()
            .expect("Couldn't import tags/model_extensions")
    }
}
