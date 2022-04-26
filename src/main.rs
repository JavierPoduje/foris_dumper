use clap::{Arg, Command};
use dotenv;
use serde_json::{from_reader, Value as JsonValue};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command as PCommand, Output};
use std::str;

mod client;

const FILE_NAME: &str = "hosts.json";

fn hosts_file() -> JsonValue {
    let file = File::open(FILE_NAME).expect("file should open read only");
    let json: JsonValue = from_reader(file).expect("file should be proper JSON");
    json
}

fn dump(client: &client::Client) -> Output {
    println!("[INFO]: dumping db...");

    let ssh_alias = dotenv::var("SSH_ALIAS").unwrap();
    PCommand::new("ssh")
        .args([
            &ssh_alias,
            &format!("mysqldump -e --host={} --user={} --password={} --port=3306 --max_allowed_packet=1024M {} tags model_extensions", client.host, client.username, client.password, client.scenarios_db),
        ])
        .output()
        .expect("Couldn't get the dump...")
}

fn write(raw_output: Vec<u8>, client: &client::Client) -> Result<usize, std::io::Error> {
    println!("[INFO]: writing dump file...");
    let target_folder = dotenv::var("TARGET_FOLDER").unwrap();
    let path = PathBuf::from(target_folder.as_str()).join(&format!("{}.sql", client.scenarios_db));
    File::create(path)?.write(&raw_output)
}

fn main() -> Result<(), ()> {
    let args = Command::new("Tags Handler")
        .version("0.1")
        .about("Bring tags from Foris clients")
        .arg(
            Arg::new("client")
                .long("client")
                .help("name of the client (example: 'hyades')")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let file = hosts_file();

    assert!(
        file.get(args.value_of("client").unwrap()).is_some(),
        "Client not found in the hosts.json file"
    );

    let client = client::Client::new(file.get(args.value_of("client").unwrap()).unwrap());

    let output = dump(&client);
    if let Ok(_) = write(output.stdout, &client) {
        println!("[INFO]: File successfully created");
    } else {
        println!("[ERROR]: Couldn't create file");
    }

    Ok(())
}
