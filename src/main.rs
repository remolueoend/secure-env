use std::collections::HashMap;
use std::io;
use std::process;
use std::string::FromUtf8Error;

#[derive(Debug)]
enum AppError {
    EnvFileReadError(FromUtf8Error),
    ExecutePassCmdError(io::Error),
}
impl From<FromUtf8Error> for AppError {
    fn from(error: FromUtf8Error) -> Self {
        AppError::EnvFileReadError(error)
    }
}
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::ExecutePassCmdError(error)
    }
}

fn main() {
    let pass_cmd = "pass";
    let file_name = "default.env";
    let env_vars =
        get_env_file_content(pass_cmd, "/Users/remo/.secure-env", file_name).unwrap_or_else(|err| {
            eprintln!("Failed to load environment variables: {:?}", err);
            process::exit(1)
        });
    print!("{}", env_vars);
}

fn execute_cmd(
    cmd: &str,
    args: &Vec<&str>,
    env_args: &HashMap<&str, &str>,
) -> Result<String, AppError> {
    let output = process::Command::new(cmd)
        .envs(env_args)
        .args(args)
        .output()?;
    if output.status.success() {
        let str_output = String::from_utf8(output.stdout)?;
        Ok(str_output)
    } else {
        let str_err = String::from_utf8(output.stderr)?;
        let err = io::Error::new(
            io::ErrorKind::Other,
            format!("Command \"{}\" returned with an error. {}", cmd, str_err),
        );
        Err(AppError::ExecutePassCmdError(err))
    }
}

fn get_env_file_content<'a>(
    pass_cmd: &str,
    pass_dir: &str,
    file_name: &str,
) -> Result<String, AppError> {
    let mut pass_env_vars = HashMap::new();
    pass_env_vars.insert("PASSWORD_STORE_DIR", pass_dir);

    execute_cmd(pass_cmd, &vec!["show", file_name], &pass_env_vars)
}
