use serde_json::{from_reader, Value as JsonValue};
use std::fs::File;
use clap::{Arg, Command};

const FILE_NAME: &str = "hosts.json";

fn hosts_file() -> JsonValue {
    let file = File::open(FILE_NAME).expect("file should open read only");
    let json: JsonValue = from_reader(file).expect("file should be proper JSON");
    json
}

fn main() {
    let args = Command::new("Tags Handler")
        .version("0.1")
        .about("Bring tags from Foris clients")
        .arg(
            Arg::new("client")
                .long("client")
                .help("name of the client (example: 'ueuropea-qa')")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    let client = args.value_of("client").unwrap();
    let file = hosts_file();

    let hosts = file.get("hosts").unwrap();
    let info = hosts.get(client);
    println!("{:?}", info);
}
