extern crate secure_env;

use secure_env::pass::{Pass, PassArgs};
use secure_env::EnvSource;
use std::process;

fn main() {
    let pass = Pass::new(PassArgs {
        pass_dir: "/Users/remo/.secure-env",
    });
    let env_vars = pass.get_env_vars("trello").unwrap_or_else(|err| {
        eprintln!("Failed to load environment variables: {:?}", err);
        process::exit(1)
    });
    for (key, value) in &env_vars {
        println!("{}={}", key, value);
    }
}
