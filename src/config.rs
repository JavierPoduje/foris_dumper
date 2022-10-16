pub struct Config {
    pub ssh_alias: String,
    pub target_folder: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            ssh_alias: dotenv::var("SSH_ALIAS").unwrap(),
            target_folder: dotenv::var("TARGET_FOLDER").unwrap(),
        }
    }
}
