use std::process::Output;

use crate::{
    client::Client,
    logger::{logger::Logger, types::LogType},
};

use super::traits::dump_scenario::DumpScenario as DumpScenarioTrait;

pub struct DumpScenario {}

impl DumpScenario {
    pub fn new() -> Self {
        Self {}
    }

    pub fn perform(&self, client: &Client, scenario: &str, ssh_alias: &str) -> Output {
        Logger::send(format!("dumping scenario '{}'...", scenario), LogType::Info);
        self.dump_scenario(client, scenario, ssh_alias)
    }
}

impl DumpScenarioTrait for DumpScenario {}
