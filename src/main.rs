use clap::{Arg, ArgMatches, Command};
use dotenv;
use serde_json::{from_reader, Value as JsonValue};
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::str;

mod action;
mod client;
mod file_manager;

use action::Action;
use client::Client;
use file_manager::FileManager;

const FILE_NAME: &str = "hosts.json";

fn hosts_file() -> JsonValue {
    let file = File::open(FILE_NAME).expect("file should open read only");
    let json: JsonValue = from_reader(file).expect("file should be proper JSON");
    json
}

fn perform_dump_tags(client_definition: &JsonValue, args: ArgMatches) -> Result<usize, Error> {
    println!("[INFO]: dumping tags...");
    let client = Client::new(client_definition);
    let ssh_alias = dotenv::var("SSH_ALIAS").unwrap();
    let scenario_db = client.scenarios_db.clone();
    let folder = dotenv::var("TARGET_FOLDER").unwrap();

    let dump_created = match args.is_present("skip_dump_creation") {
        true => true,
        false => {
            println!("[INFO]: dumping scenario...");
            let output = Action::new(client).dump_tags(ssh_alias);
            match FileManager::write(output.stdout, &scenario_db) {
                Ok(_) => true,
                _ => false,
            }
        }
    };

    match dump_created {
        true => match hosts_file().get("local") {
            Some(local_definition) => {
                println!("[INFO]: creating tags on local scenarios_db");
                Action::new(Client::new(local_definition)).import_tags(folder, &scenario_db);
                Ok(1)
            }
            None => Err(Error::new(
                ErrorKind::Interrupted,
                "Localhost not defined...",
            )),
        },
        false => Err(Error::new(
            ErrorKind::Interrupted,
            "Dump couldn't be created...",
        )),
    }
}

fn perform_dump_scenario(client_definition: &JsonValue, args: ArgMatches) -> Result<usize, Error> {
    let client = Client::new(client_definition);
    let ssh_alias = dotenv::var("SSH_ALIAS").unwrap();
    let folder = dotenv::var("TARGET_FOLDER").unwrap();

    // 1. dump_scenario
    let dump_scenario = args.value_of("scenario").unwrap();
    let dump_was_created = match args.is_present("skip_dump_creation") {
        true => true,
        false => {
            println!("[INFO]: dumping scenario...");
            let output = Action::new(client).dump_scenario(ssh_alias, dump_scenario);
            match FileManager::write(output.stdout, dump_scenario) {
                Ok(_) => true,
                _ => false,
            }
        }
    };

    // 2. import scenario
    match dump_was_created {
        true => match hosts_file().get("local") {
            Some(local_definition) => {
                println!("[INFO]: copying scenario...");
                Action::new(Client::new(local_definition)).import_scenario(folder, dump_scenario);
                Ok(1)
            }
            None => Err(Error::new(
                ErrorKind::Interrupted,
                "Localhost not defined...",
            )),
        },
        false => Err(Error::new(
            ErrorKind::Interrupted,
            "Dump couldn't be created...",
        )),
    }
}

fn perform(client_definition: &JsonValue, args: ArgMatches) {
    let response = match args.value_of("action") {
        Some(value) if value == "dump-tags" => perform_dump_tags(client_definition, args),
        Some(value) if value == "dump-scenario" => perform_dump_scenario(client_definition, args),
        _ => unreachable!(),
    };
    match response {
        Ok(_) => println!("[INFO]: Process succesfully executed"),
        _ => println!("[ERROR]: Couldn't create file"),
    }
}

fn main() -> Result<(), ()> {
    let args = Command::new("Foris Dumps Handler")
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
        .arg(
            Arg::new("skip_dump_creation")
                .long("skip_dump_creation")
                .help("Skip the creation of the dump.")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("skip_insertion")
                .long("skip_insertion")
                .help("Skip the insertion of the dump content on the DB")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    // Assert commands integrity
    match args.value_of("action").unwrap() {
        "dump-scenario" => {
            assert!(
                args.value_of("scenario").is_some(),
                "The `scenario` is necessary to perform the `dump-scenario` action"
            );
        }
        "dump-tags" => {
            assert!(
                args.value_of("scenario").is_none(),
                "The paremeter `scenario` only should be used with the `dump-scenario` action"
            )
        }
        _ => unreachable!(),
    }

    // Execute
    match hosts_file().get(args.value_of("client").unwrap()) {
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
