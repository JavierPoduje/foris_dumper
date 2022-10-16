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
                        * dump-tags
                        * dump-scenario
                ",
                    )
                    .possible_values(["dump-tags", "dump-scenario"])
                    .required(true),
            )
            .arg(
                Arg::new("scenario")
                    .long("scenario")
                    .help("Name of scenario to dump. Only used for action `dump-scenario`.")
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
            "dump-scenario" => {
                if self.args.value_of("scenario").is_some() {
                    return Ok(true);
                } else {
                    return Err(
                        "The `scenario` is necessary to perform the `dump-scenario` action",
                    );
                }
            }
            "dump-tags" => {
                if self.args.value_of("scenario").is_none() {
                    return Ok(true);
                } else {
                    return Err(
                        "The paremeter `scenario` only should be used with the `dump-scenario` action",
                    );
                }
            }
            _ => unreachable!(),
        }
    }
}
