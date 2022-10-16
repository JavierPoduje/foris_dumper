use clap::{Arg, ArgMatches, Command as ClapCommand};

pub struct Command {
    pub args: ArgMatches,
}

impl Command {
    pub fn new() -> Self {
        let args = ClapCommand::new("Foris Dumps Handler")
            .version("0.1")
            .about("Manage remote and local Darwin's DBs.")
            .arg(
                Arg::new("client")
                    .long("client")
                    .help("name of the client (example: 'hyades')")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::new("action")
                    .long("action")
                    .takes_value(true)
                    .help(
                        "Action to perform. Options: \
                        * tags
                        * scenarios
                ",
                    )
                    .possible_values(["tags", "scenarios"])
                    .required(true),
            )
            .arg(
                Arg::new("scenario")
                    .long("scenario")
                    .help("Name of scenario to dump. Only used for action `scenarios`.")
                    .takes_value(true),
            )
            .arg(
                Arg::new("skip_dump_creation")
                    .long("skip_dump_creation")
                    .help("Skip the creation of the dump.")
                    .takes_value(false)
                    .required(false),
            )
            .arg(
                Arg::new("skip_insertion")
                    .long("skip_insertion")
                    .help("Skip the insertion of the dump content on the DB")
                    .takes_value(false)
                    .required(false),
            )
            .get_matches();
        Self { args }
    }

    pub fn validate(&self) -> Result<bool, &str> {
        match self.args.value_of("action").unwrap() {
            "scenarios" => {
                if self.args.value_of("scenario").is_some() {
                    Ok(true)
                } else {
                    Err("The `scenario` is necessary to perform the `scenarios` action")
                }
            }
            "tags" => {
                if self.args.value_of("scenario").is_none() {
                    Ok(true)
                } else {
                    Err("The paremeter `scenario` only should be used with the `scenarios` action")
                }
            }
            _ => unreachable!(),
        }
    }
}
