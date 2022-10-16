use std::process::Output;

use crate::{
    client::Client,
    logger::{logger::Logger, types::LogType},
};

use super::traits::{
    create_scenario::CreateScenario, delete_scenario::DeleteScenario, import::Import,
};

pub struct ImportScenario {}

impl ImportScenario {
    pub fn new() -> Self {
        Self {}
    }

    pub fn perform(&self, client: &Client, scenario: &str, folder: &str) -> Output {
        Logger::send("deleting scenario...", LogType::Info);
        self.delete_scenario(client, scenario);

        Logger::send("creating scenario...", LogType::Info);
        self.create_scenario(client, scenario);

        Logger::send("importing scenario...", LogType::Info);
        self.import(client, scenario, folder)
    }
}

impl DeleteScenario for ImportScenario {}
impl CreateScenario for ImportScenario {}
impl Import for ImportScenario {}
