use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::logger::{logger::Logger, types::LogType};

pub struct FileManager {}

impl FileManager {
    pub fn write(folder: &str, content: Vec<u8>, filename: &str) -> Result<usize, Error> {
        Logger::send("writing dump file...".to_string(), LogType::Info);
        let path = PathBuf::from(&folder).join(&format!("{}.sql", filename));
        File::create(path)?.write(&content)
    }
}
