use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum AppError {
    EnvFileReadError(FromUtf8Error),
    ExecutePassCmdError(io::Error),
    ParsePassFileError(toml::de::Error),
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
impl From<toml::de::Error> for AppError {
    fn from(error: toml::de::Error) -> Self {
        AppError::ParsePassFileError(error)
    }
}
