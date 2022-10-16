use clap::ArgMatches;
use serde_json::{from_reader, Value as JsonValue};
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::str;

mod action;
mod client;
mod command;
mod config;
mod file_manager;
mod logger;

use action::Action;
use client::Client;
use file_manager::FileManager;
use logger::{logger::Logger, types::LogType};

const FILE_NAME: &str = "hosts.json";

fn hosts_file() -> JsonValue {
    let file = File::open(FILE_NAME).expect("file should open read only");
    let json: JsonValue = from_reader(file).expect("file should be proper JSON");
    json
}

fn perform_dump_tags(
    config: config::Config,
    client_definition: &JsonValue,
    args: ArgMatches,
) -> Result<usize, Error> {
    let client = Client::new(client_definition);
    let ssh_alias = &config.ssh_alias;
    let scenario_db = client.scenarios_db.clone();
    let folder = &config.target_folder;

    let dump_created = match args.is_present("skip_dump_creation") {
        true => true,
        false => {
            Logger::send(
                &format!("dumping '{}.sql' in target folder...", scenario_db),
                LogType::Info,
            );
            let output = Action::new(client).dump_tags(ssh_alias);
            matches!(
                FileManager::write(folder, output.stdout, &scenario_db),
                Ok(_)
            )
        }
    };

    if args.is_present("skip_insertion") {
        return Ok(1);
    }

    match dump_created {
        true => match hosts_file().get("local") {
            Some(local_definition) => {
                Logger::send("creating tags on local scenarios_db", LogType::Info);
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

fn perform_dump_scenario(
    config: config::Config,
    client_definition: &JsonValue,
    args: ArgMatches,
) -> Result<usize, Error> {
    let client = Client::new(client_definition);
    let ssh_alias = &config.ssh_alias;
    let folder = &config.target_folder;

    // 1. dump_scenario
    let dump_scenario = args.value_of("scenario").unwrap();
    let dump_was_created = match args.is_present("skip_dump_creation") {
        true => true,
        false => {
            Logger::send("dumping scenario...", LogType::Info);
            let output = Action::new(client).dump_scenario(ssh_alias, dump_scenario);
            matches!(
                FileManager::write(folder, output.stdout, dump_scenario),
                Ok(_)
            )
        }
    };

    if args.is_present("skip_insertion") {
        return Ok(1);
    }

    // 2. import scenario
    match dump_was_created {
        true => match hosts_file().get("local") {
            Some(local_definition) => {
                Logger::send("copying scenario...", LogType::Info);
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

fn perform(config: config::Config, client_definition: &JsonValue, args: ArgMatches) {
    Logger::send("Start", LogType::Info);
    let response = match args.value_of("action") {
        Some(value) if value == "tags" => perform_dump_tags(config, client_definition, args),
        Some(value) if value == "scenarios" => {
            perform_dump_scenario(config, client_definition, args)
        }
        _ => unreachable!(),
    };
    match response {
        Ok(_) => Logger::send("Process succesfully executed", LogType::Info),
        _ => Logger::send("Couldn't create file", LogType::Error),
    }
}

fn main() {
    let config = config::Config::new();
    let comm = command::Command::new();

    // validate commands integrity
    if let Err(message) = comm.validate() {
        Logger::send(message, LogType::Error);
        std::process::exit(0x0100);
    }

    match hosts_file().get(comm.args.value_of("client").unwrap()) {
        Some(client_definition) => {
            perform(config, client_definition, comm.args);
        }
        None => {
            Logger::send("Client not found in the hosts.json file...", LogType::Error);
        }
    }
}
