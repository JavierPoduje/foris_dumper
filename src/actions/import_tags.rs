use std::process::Output;

use crate::{
    client::Client,
    logger::{logger::Logger, types::LogType},
};

use super::traits::{
    delete_model_extensions::DeleteModelExtensions, delete_tags::DeleteTags,
    import_scenarios_db::ImportScenariosDB,
};

pub struct ImportTags {}

impl ImportTags {
    pub fn new() -> Self {
        Self {}
    }

    pub fn perform(&self, client: &Client, remote_scenarios_db: &str, folder: &str) -> Output {
        Logger::send("deleting existing tags...".to_string(), LogType::Info);
        self.delete_tags(client);

        Logger::send("deleting existing model_extensions...".to_string(), LogType::Info);
        self.delete_model_extensions(client);

        Logger::send("importing tags and model_extensions...".to_string(), LogType::Info);
        self.import_scenarios_db(client, remote_scenarios_db, folder)
    }
}

impl DeleteTags for ImportTags {}
impl DeleteModelExtensions for ImportTags {}
impl ImportScenariosDB for ImportTags {}
