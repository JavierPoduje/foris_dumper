use clap::{Arg, ArgMatches, Command};
use dotenv;
use serde_json::{from_reader, Value as JsonValue};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command as PCommand, Output};
use std::str;

mod client;
use crate::client::Client;

const FILE_NAME: &str = "hosts.json";

fn hosts_file() -> JsonValue {
    let file = File::open(FILE_NAME).expect("file should open read only");
    let json: JsonValue = from_reader(file).expect("file should be proper JSON");
    json
}

fn dump(client: &Client) -> Output {
    println!("[INFO]: dumping db...");

    let ssh_alias = dotenv::var("SSH_ALIAS").unwrap();
    PCommand::new("ssh")
        .args([
            &ssh_alias,
            &format!("mysqldump -e --host={} --user={} --password={} --port=3306 --max_allowed_packet=1024M {} tags model_extensions", client.host, client.username, client.password, client.scenarios_db),
        ])
        .output()
        .expect("Couldn't get the dump...")
}

fn write(raw_output: Vec<u8>, filename: &str) -> Result<usize, std::io::Error> {
    println!("[INFO]: writing dump file...");
    let target_folder = dotenv::var("TARGET_FOLDER").unwrap();
    let path = PathBuf::from(target_folder.as_str()).join(&format!("{}.sql", filename));
    File::create(path)?.write(&raw_output)
}

fn perform_dump_tags(client_definition: &JsonValue) {
    let client = Client::new(client_definition);
    let output = dump(&client);
    if let Ok(_) = write(output.stdout, &client.scenarios_db) {
        println!("[INFO]: File successfully created");
    } else {
        println!("[ERROR]: Couldn't create file");
    }
}

fn perform_dump_scenario(client_definition: &JsonValue, args: ArgMatches) {
    let dump_scenario = args.value_of("scenario").unwrap();
    let client = Client::new(client_definition);
    let ssh_alias = dotenv::var("SSH_ALIAS").unwrap();

    println!("[INFO]: dumping db...");

    let output = PCommand::new("ssh")
        .args([
            &ssh_alias,
            &format!("mysqldump -e --host={} --user={} --password={} --port=3306 --max_allowed_packet=1024M {}", client.host, client.username, client.password, dump_scenario),
        ])
        .output()
        .expect("Couldn't get the dump...");

    if let Ok(_) = write(output.stdout, dump_scenario) {
        println!("[INFO]: File successfully created");
    } else {
        println!("[ERROR]: Couldn't create file");
    }
}

fn perform(client_definition: &JsonValue, args: ArgMatches) {
    match args.value_of("action") {
        Some(value) if value == "dump-tags" => {
            perform_dump_tags(client_definition);
        }
        Some(value) if value == "dump-scenario" => {
            perform_dump_scenario(client_definition, args);
        }
        _ => unreachable!(),
    }
}

fn main() -> Result<(), ()> {
    let args = Command::new("Tags Handler")
        .version("0.1")
        .about("Manage remote and local Darwin's DBs.")
        .arg(
            Arg::new("client")
                .long("client")
                .help("name of the client (example: 'hyades')")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("action")
                .long("action")
                .takes_value(true)
                .help(
                    "Action to perform. Options: \
                    * dump-tags
                    * dump-scenario
                ",
                )
                .possible_values(&["dump-tags", "dump-scenario"])
                .required(true),
        )
        .arg(
            Arg::new("scenario")
                .long("scenario")
                .help("Name of scenario to dump. Only used for action `dump-scenario`.")
                .takes_value(true),
        )
        .get_matches();

    // Assert commands integrity
    if args.value_of("action").unwrap() == "dump-scenario" {
        assert!(
            args.value_of("scenario").is_some(),
            "The `scenario` is necessary to perform the `dump-scenario` action"
        );
    };

    let file = hosts_file();

    match file.get(args.value_of("client").unwrap()) {
        Some(client_definition) => {
            perform(client_definition, args);
            Ok(())
        }
        None => {
            println!("[ERROR]: Client not found in the hosts.json file...");
            Err(())
        }
    }
}
