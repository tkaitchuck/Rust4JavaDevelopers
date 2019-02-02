use failure::*;
use std::fs;
use std::io;

struct Config;
fn parse_config(s: &str) -> Result<Config, ConfigReadError> {
  Ok(Config)
}

#[derive(Debug, Fail)]
enum ConfigReadError {
  #[fail(display = "Invalid config in file {}", 0)]
  FailedToReadFile(String, #[fail(cause)] io::Error),
  #[fail(display = "Unable to parese config because {}", 0)]
  InvalidConfig(String),
}
//...
fn load_config(path: String) -> Result<Config, ConfigReadError> {
  let serialized_config = fs::read_to_string(&path).map_err(|err| ConfigReadError::FailedToReadFile(path, err))?;
  parse_config(&serialized_config)
}
