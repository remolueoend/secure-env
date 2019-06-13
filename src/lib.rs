use std::collections::HashMap;
use app_error::AppError;

pub mod pass;
pub mod app_error;

pub trait EnvSource<'a> {
    fn new(args: &'a HashMap<&'a str, &'a str>) -> Self;
    fn get_env_vars(&self, names: &str) -> Result<HashMap<String, String>, AppError>;
}