use super::app_error::AppError;
use super::EnvSource;
use std::collections::HashMap;
use std::io;
use std::process;
use toml;

pub struct Pass<'a> {
    pass_dir: &'a str,
}

impl<'a> Pass<'a> {
    fn read_file(&self, file_name: &str) -> Result<String, AppError> {
        let mut pass_env_vars = HashMap::new();
        pass_env_vars.insert("PASSWORD_STORE_DIR", self.pass_dir);

        let output = process::Command::new("pass")
            .envs(&pass_env_vars)
            .args(&["show", file_name])
            .output()?;
        if output.status.success() {
            let str_output = String::from_utf8(output.stdout)?;
            Ok(str_output)
        } else {
            let str_err = String::from_utf8(output.stderr)?;
            let err = io::Error::new(
                io::ErrorKind::Other,
                format!("Command 'pass' returned with an error. {}", str_err),
            );
            Err(AppError::ExecutePassCmdError(err))
        }
    }
}

impl<'a> EnvSource<'a> for Pass<'a> {
    fn new(args: &'a HashMap<&'a str, &'a str>) -> Pass<'a> {
        Pass {
            pass_dir: args.get("pass_dir").map_or("~/.password-store", |arg| arg),
        }
    }
    fn get_env_vars(&self, app_name: &str) -> Result<HashMap<String, String>, AppError> {
        let pass_file_content = self.read_file(app_name)?;
        let parsed_vars: HashMap<String, String> = toml::from_str(&pass_file_content)?;
        Ok(parsed_vars)
    }
}
