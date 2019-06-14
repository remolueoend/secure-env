use super::AppError;
use super::EnvSource;
use std::io;
use std::process::Command;

pub fn start_command<'a, TSource, TSourceArgs>(name: &str, source: &TSource) -> Result<(), AppError>
where
    TSource: EnvSource<'a, TSourceArgs>,
{
    let env_vars = source.get_env_vars(name).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to load env vars for process '{}': {:?}", name, err),
        )
    })?;
    let process = Command::new(name).envs(&env_vars).spawn()?;
    
    for (key, value) in &env_vars {
        println!("{}={}", key, value);
    }
    Ok(())
}
