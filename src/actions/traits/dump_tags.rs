use crate::client::Client;
use std::process::{Command, Output};

pub trait DumpTags {
    fn dump_tags(&self, client: &Client, ssh_alias: &str) -> Output {
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
                --no-create-info \
                --complete-insert \
                --compact {} \
                tags model_extensions",
                    client.host, client.username, client.password, client.scenarios_db
                ),
            ])
            .output()
            .expect("Couldn't get the dump...")
    }
}
