use clap::ArgMatches;
use serde_json::{from_reader, Value as JsonValue};
use std::fs::File;
use std::str;

mod actions;
mod client;
mod command;
mod config;
mod file_manager;
mod logger;

use actions::{Action, ActionType};
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
) -> Result<bool, &str> {
    let client = Client::new(client_definition);
    let ssh_alias = &config.ssh_alias;
    let scenario_db = client.scenarios_db.clone();
    let folder = &config.target_folder;

    let dump_created = match args.is_present("skip_dump_creation") {
        true => true,
        false => {
            let output = Action::new(&client, &scenario_db).perform(
                ActionType::DumpTags,
                folder,
                None,
                Some(ssh_alias),
            );

            matches!(
                FileManager::write(folder, output.stdout, &scenario_db),
                Ok(_)
            )
        }
    };

    if args.is_present("skip_insertion") {
        return Ok(true);
    }

    if !dump_created {
        return Err("Dump couldn't be created...");
    }

    if let Some(local_definition) = hosts_file().get("local") {
        let client = Client::new(local_definition);
        Action::new(&client, &scenario_db).perform(
            ActionType::ImportTags,
            folder,
            Some(&scenario_db),
            None,
        );

        Ok(true)
    } else {
        Err("Localhost not defined...")
    }
}

fn perform_dump_scenario(
    config: config::Config,
    client_definition: &JsonValue,
    args: ArgMatches,
) -> Result<bool, &str> {
    let client = Client::new(client_definition);
    let ssh_alias = &config.ssh_alias;
    let folder = &config.target_folder;

    // 1. dump_scenario
    let dump_scenario = args.value_of("db_name").unwrap();
    let dump_was_created = match args.is_present("skip_dump_creation") {
        true => true,
        false => {
            let output = Action::new(&client, dump_scenario).perform(
                ActionType::DumpScenario,
                folder,
                None,
                Some(ssh_alias),
            );
            matches!(
                FileManager::write(folder, output.stdout, dump_scenario),
                Ok(_)
            )
        }
    };

    if args.is_present("skip_insertion") {
        return Ok(true);
    }

    // 2. import scenario
    match dump_was_created {
        true => match hosts_file().get("local") {
            Some(local_definition) => {
                Logger::send("copying scenario...", LogType::Info);

                let client = Client::new(local_definition);
                Action::new(&client, dump_scenario).perform(
                    ActionType::ImportScenario,
                    folder,
                    None,
                    None,
                );

                Ok(true)
            }
            None => Err("Localhost not defined..."),
        },
        false => Err("Dump couldn't be created..."),
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

    if let Some(client_definition) = hosts_file().get(comm.args.value_of("client").unwrap()) {
        Logger::send("Start", LogType::Info);
        let response = match comm.args.value_of("action") {
            Some(value) if value == "tags" => {
                perform_dump_tags(config, client_definition, comm.args)
            }
            Some(value) if value == "scenarios" => {
                perform_dump_scenario(config, client_definition, comm.args)
            }
            _ => unreachable!(),
        };

        match response {
            Ok(_) => Logger::send("Process succesfully executed", LogType::Info),
            _ => Logger::send("Couldn't execute process correctly", LogType::Error),
        }
    } else {
        Logger::send("Client not found in the hosts.json file...", LogType::Error);
    }
}
