use serde_json::Value as JsonValue;
use std::process::{Command, Output, Stdio};

#[derive(Debug)]
pub struct Client {
    pub host: String,
    pub username: String,
    pub password: String,
    pub scenarios_db: String,
}

impl Client {
    pub fn new(client: &JsonValue) -> Self {
        assert!(client.get("host").is_some(), "'host' not found for client");
        assert!(
            client.get("username").is_some(),
            "'username' not found for client"
        );
        assert!(
            client.get("password").is_some(),
            "'password' not found for client"
        );
        assert!(
            client.get("scenarios_db").is_some(),
            "'scenarios_db' not found for client"
        );

        Client {
            host: Client::value_from_key(client, "host"),
            username: Client::value_from_key(client, "username"),
            password: Client::value_from_key(client, "password"),
            scenarios_db: Client::value_from_key(client, "scenarios_db"),
        }
    }

    pub fn dump_to_db(&self, dump_scenario: &str) -> Output {
        let target_folder = dotenv::var("TARGET_FOLDER").unwrap();

        // 1. delete db on localhost
        println!("[INFO]: deleting scenario...");
        Command::new("mysql")
            .args([
                format!("--host={}", self.host),
                format!("--user={}", self.username),
                format!("--password={}", self.password),
                format!("--port={}", "4006"),
                format!("-e DROP DATABASE IF EXISTS {}", dump_scenario),
            ])
            .output()
            .expect("Couldn't drop db");

        // 2. create db
        println!("[INFO]: creating scenario...");
        Command::new("mysql")
            .args([
                format!("--host={}", self.host),
                format!("--user={}", self.username),
                format!("--password={}", self.password),
                format!("--port={}", "4006"),
                format!("-e CREATE DATABASE {}", dump_scenario),
            ])
            .output()
            .expect("Couldn't drop db");

        // 3. import db
        println!("[INFO]: importing scenario...");
        let cat = Command::new("cat")
            .args([format!("{}/{}.sql", target_folder.as_str(), dump_scenario)])
            .stdout(Stdio::piped())
            .spawn();

        Command::new("mysql")
            .args([
                format!("--host={}", self.host),
                format!("--user={}", self.username),
                format!("--password={}", self.password),
                format!("--port={}", "4006"),
                format!("--max_allowed_packet={}", "1024M"),
                format!("{}", dump_scenario),
            ])
            .stdin(cat.ok().unwrap().stdout.unwrap())
            .output()
            .expect("Couldn't import db")
    }

    pub fn dump_tags(&self, ssh_alias: String) -> Output {
        Command::new("ssh")
        .args([
            ssh_alias,
            format!("mysqldump -e --host={} --user={} --password={} --port=3306 --max_allowed_packet=1024M {} tags model_extensions", self.host, self.username, self.password, self.scenarios_db),
        ])
        .output()
        .expect("Couldn't get the dump...")
    }

    pub fn dump_scenario(&self, ssh_alias: String, dump_scenario: &str) -> Output {
        Command::new("ssh")
        .args([
            ssh_alias,
            format!("mysqldump -e --host={} --user={} --password={} --port=3306 --max_allowed_packet=1024M {}", self.host, self.username, self.password, dump_scenario),
        ])
        .output()
        .expect("Couldn't get the dump...")
    }

    fn value_from_key(client: &JsonValue, raw_key: &str) -> String {
        client.get(raw_key).unwrap().as_str().unwrap().to_string()
    }
}
