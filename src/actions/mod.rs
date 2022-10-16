use crate::client::Client;
use std::process::Output;

pub mod traits;

use crate::actions::dump_scenario::DumpScenario;
use crate::actions::dump_tags::DumpTags;
use crate::actions::import_scenario::ImportScenario;
use crate::actions::import_tags::ImportTags;

mod dump_scenario;
mod dump_tags;
mod import_scenario;
mod import_tags;

pub enum ActionType {
    ImportScenario,
    ImportTags,
    DumpTags,
    DumpScenario,
}

pub struct Action<'a> {
    client: &'a Client,
    scenario: &'a str,
}

impl<'a> Action<'a> {
    pub fn new(client: &'a Client, scenario: &'a str) -> Self {
        Self { client, scenario }
    }

    pub fn perform(
        self,
        action_type: ActionType,
        folder: &str,
        remote_scenarios_db: Option<&str>,
        ssh_alias: Option<&str>,
    ) -> Output {
        match action_type {
            ActionType::ImportScenario => {
                ImportScenario::new().perform(self.client, self.scenario, folder)
            }
            ActionType::ImportTags => {
                ImportTags::new().perform(self.client, remote_scenarios_db.unwrap(), folder)
            }
            ActionType::DumpTags => DumpTags::new().perform(self.client, ssh_alias.unwrap()),
            ActionType::DumpScenario => {
                DumpScenario::new().perform(self.client, self.scenario, ssh_alias.unwrap())
            }
        }
    }
}
