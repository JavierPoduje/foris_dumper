use crate::client::Client;
use std::process::{Command, Output};

pub trait DumpScenario {
    fn dump_scenario(&self, client: &Client, dump_scenario: &str, ssh_alias: &str) -> Output {
        Command::new("ssh")
            .args([
                ssh_alias.to_string(),
                format!(
                    "
                mysqldump \
                -e \
                --host={} \
                --user={} \
                --password={} \
                --port=3306 \
                --max_allowed_packet=1024M \
                {}",
                    client.host, client.username, client.password, dump_scenario
                ),
            ])
            .output()
            .expect("Couldn't get the dump...")
    }
}
