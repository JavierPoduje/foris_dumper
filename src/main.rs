use clap::{Arg, Command};
use serde_json::{from_reader, Value as JsonValue};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command as PCommand, Output};
use std::str;

use dotenv;

const FILE_NAME: &str = "hosts.json";

fn hosts_file() -> JsonValue {
    let file = File::open(FILE_NAME).expect("file should open read only");
    let json: JsonValue = from_reader(file).expect("file should be proper JSON");
    json
}

fn dump(client_info: &JsonValue) -> Output {
    let ssh_alias = dotenv::var("SSH_ALIAS").unwrap();

    let host = if let Some(host) = client_info.get("host") {
        host
    } else {
        panic!("Host is not defined for client");
    };
    let username = if let Some(username) = client_info.get("username") {
        username
    } else {
        panic!("Username is not defined for client");
    };
    let password = if let Some(password) = client_info.get("password") {
        password
    } else {
        panic!("Password is not defined for client");
    };
    let scenarios_db = if let Some(scenarios_db) = client_info.get("scenarios_db") {
        scenarios_db
    } else {
        panic!("Scenarios_db is not defined for client");
    };

    PCommand::new("ssh")
        .args([
            &ssh_alias,
            &format!("mysqldump -e --host={} --user={} --password={} --port=3306 --max_allowed_packet=1024M {} tags model_extensions", host, username, password, scenarios_db),
        ])
        .output()
        .expect("Couldn't get the dump, dude...")
}

fn write(dump_lines: std::str::Lines, client_info: &JsonValue) {
    let target_folder = dotenv::var("TARGET_FOLDER").unwrap();
    let scenarios_db = if let Some(scenarios_db) = client_info.get("scenarios_db") {
        scenarios_db.as_str()
    } else {
        panic!("Scenarios_db is not defined for client");
    };

    if let Some(scenarios_db) = scenarios_db {
        let path = PathBuf::from(target_folder.as_str()).join(&format!("{}.sql", scenarios_db));
        println!("{:?}", path);
        match File::create(path) {
            Ok(mut file) => {
                for line in dump_lines {
                    file.write(line.as_bytes());
                    file.write_all(b"\n");
                }
            }
            Err(e) => println!("{}", e),
        }
    }
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

    let dump_lines = match str::from_utf8(&output.stdout) {
        Ok(output) => output.lines(),
        Err(_) => panic!("todo..."), // TODO: handle this case correctly
    };

    write(dump_lines, client_info);

    Ok(())
}
