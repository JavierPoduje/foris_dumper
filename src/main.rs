use clap::{Arg, Command};
use serde_json::{from_reader, Value as JsonValue};
use std::fs::File;
use std::process::{Command as PCommand, Output};

use dotenv;

const FILE_NAME: &str = "hosts.json";

fn hosts_file() -> JsonValue {
    let file = File::open(FILE_NAME).expect("file should open read only");
    let json: JsonValue = from_reader(file).expect("file should be proper JSON");
    json
}

fn dump(client_info: &JsonValue) -> Output {
    let ssh_alias = dotenv::var("SSH_ALIAS").unwrap();
    let host = client_info.get("host").unwrap();
    let username = client_info.get("username").unwrap();
    let password = client_info.get("password").unwrap();
    let scenarios_db = client_info.get("scenarios_db").unwrap();

    PCommand::new("ssh")
        .args([
            &ssh_alias,
            &format!("mysqldump -e --host={} --user={} --password={} --port=3306 --max_allowed_packet=1024M {} tags model_extensions", host, username, password, scenarios_db),
        ])
        .output()
        .expect("Couldn't get the dump, dude...")
}

fn main() -> Result<(), ()> {
    let args = Command::new("Tags Handler")
        .version("0.1")
        .about("Bring tags from Foris clients")
        .arg(
            Arg::new("client")
                .long("client")
                .help("name of the client (example: 'ueuropea-qa')")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let client = args.value_of("client").unwrap();
    let file = hosts_file();

    let hosts = file.get("hosts").unwrap();
    let client_info = hosts.get(client).unwrap();

    let output = dump(client_info);
    println!("{:?}", output);

    Ok(())
}
