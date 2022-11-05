use std::process::Output;

use crate::{
    client::Client,
    logger::{logger::Logger, types::LogType},
};

use super::traits::dump_tags::DumpTags as DumpTagsTrait;

pub struct DumpTags {}

impl DumpTags {
    pub fn new() -> Self {
        Self {}
    }

    pub fn perform(&self, client: &Client, ssh_alias: &str) -> Output {
        Logger::send("dumping tags and model_extensions...".to_string(), LogType::Info);
        self.dump_tags(client, ssh_alias)
    }
}

impl DumpTagsTrait for DumpTags {}
