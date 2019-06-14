extern crate clap;
extern crate secure_env;

use clap::{App, Arg, SubCommand};
use secure_env::pass::{Pass, PassArgs};
use secure_env::start_command::start_command;
use secure_env::EnvSource;

fn main() {
    let arg_matches = App::new("secure-env")
        .version("1.0")
        .author("remolueoend")
        .about("injects environment variables from various sources into a target process")
        .subcommand(
            SubCommand::with_name("start")
                .about("starts the process with the given name")
                .arg(
                    Arg::with_name("PROCESS")
                        .help("name of the process to start")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();
    if let Some(start_args) = arg_matches.subcommand_matches("start") {
        let pass = Pass::new(PassArgs {
            pass_dir: "/Users/remo/.secure-env",
        });
        start_command(start_args.value_of("PROCESS").unwrap(), &pass);
    }
}
