use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

pub struct FileManager {}

impl FileManager {
    pub fn write(content: Vec<u8>, filename: &str) -> Result<usize, Error> {
        println!("[INFO]: writing dump file...");
        let target_folder = dotenv::var("TARGET_FOLDER").unwrap();
        let path = PathBuf::from(&target_folder).join(&format!("{}.sql", filename));
        File::create(path)?.write(&content)
    }
}
