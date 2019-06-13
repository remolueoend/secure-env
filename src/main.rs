extern crate secure_env;

use std::collections::HashMap;
use std::process;
use secure_env::pass::Pass;
use secure_env::EnvSource;

fn main() {
    let mut pass_args = HashMap::new();
    pass_args.insert("pass_dir", "/Users/remo/.secure-env");
    let pass = Pass::new(&pass_args);
    
    let env_vars = pass.get_env_vars("trello").unwrap_or_else(|err| {
            eprintln!("Failed to load environment variables: {:?}", err);
            process::exit(1)
    });
    
    for (key, value) in &env_vars {
        println!("{}={}", key, value);
    }
    
}