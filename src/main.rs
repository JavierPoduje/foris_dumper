use serde_json::{Value, from_reader};
use std::fs::File;

const FILE_NAME: &str = "hosts.json";

fn main() {
    let file = File::open(FILE_NAME).expect("file should open read only");
    let json: Value = from_reader(file).expect("file should be proper JSON");
    println!("{}", json);
}
