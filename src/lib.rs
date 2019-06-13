use std::collections::HashMap;
use app_error::AppError;

pub mod pass;
pub mod app_error;

pub trait EnvSource<'a, TArgs> {
    fn new(args: TArgs) -> Self;
    fn get_env_vars(&self, names: &str) -> Result<HashMap<String, String>, AppError>;
}