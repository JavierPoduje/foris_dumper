use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::logger::{logger::Logger, types::LogType};

pub struct FileManager {}

impl FileManager {
    pub fn write(content: Vec<u8>, filename: &str) -> Result<usize, Error> {
        Logger::send("writing dump file...", LogType::Info);
        let target_folder = dotenv::var("TARGET_FOLDER").unwrap();
        let path = PathBuf::from(&target_folder).join(&format!("{}.sql", filename));
        File::create(path)?.write(&content)
    }
}
