use super::app_error::AppError;
use super::EnvSource;
use std::collections::HashMap;
use std::io;
use std::process;
use toml;

/// Arguments passed to `Pass::new`
pub struct PassArgs<'a> {
    /// Path to the pass directory to use
    pub pass_dir: &'a str,
}

/// Environment variable source using GNU pass for retrieving and storing variables
pub struct Pass<'a> {
    /// Arguments provided when initializing a new `Pass` instance using `Pass::new`
    args: PassArgs<'a>,
}

impl<'a> Pass<'a> {
    /// Reads and returns the content of the file with the given name from
    /// the pass directory
    ///
    /// # Arguments
    /// * `file_name` - The name (path) of the file to read. Could also be a file path
    fn read_file(&self, file_name: &str) -> Result<String, AppError> {
        let mut pass_env_vars = HashMap::new();
        pass_env_vars.insert("PASSWORD_STORE_DIR", self.args.pass_dir);

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

impl<'a> EnvSource<'a, PassArgs<'a>> for Pass<'a> {
    /// Returns a new `Pass` instance with the given arguments
    ///
    /// # Arguments
    /// * `args` - Arguments struct to apply
    fn new(args: PassArgs<'a>) -> Pass<'a> {
        Pass { args }
    }
    /// Returns a hashmap containing all environment variables registered for the app
    /// with the given name. The name of the app has to correspond to an existing
    /// file name of path in the pass directory.
    /// Returns all environment variables listed in this file.
    ///
    /// # Arguments
    /// * `app_name` - The name of the app to get the env vars for
    fn get_env_vars(&self, app_name: &str) -> Result<HashMap<String, String>, AppError> {
        let pass_file_content = self.read_file(app_name)?;
        let parsed_vars: HashMap<String, String> = toml::from_str(&pass_file_content)?;
        Ok(parsed_vars)
    }
}
