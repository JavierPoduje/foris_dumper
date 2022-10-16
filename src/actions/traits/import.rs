use crate::client::Client;
use std::process::{Command, Output, Stdio};

pub trait Import {
    fn import(&self, client: &Client, db: &str, folder: &str) -> Output {
        let cat = Command::new("cat")
            .args([format!("{}/{}.sql", folder, db)])
            .stdout(Stdio::piped())
            .spawn();

        Command::new("mysql")
            .args([
                format!("--host={}", client.host),
                format!("--user={}", client.username),
                format!("--password={}", client.password),
                format!("--port={}", "4006"),
                format!("--max_allowed_packet={}", "1024M"),
                db.to_string(),
            ])
            .stdin(cat.ok().unwrap().stdout.unwrap())
            .output()
            .unwrap_or_else(|_| panic!("Couldn't import DB {}", db))
    }
}
