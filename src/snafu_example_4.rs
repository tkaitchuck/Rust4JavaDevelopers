use snafu::*;
use std::fs;
use std::io;

struct Config;
fn parse_config(s: &str) -> Result<Config, ConfigReadError> {
  Ok(Config)
}

#[derive(Debug, Snafu)]
enum ConfigReadError {
  #[snafu(display("Invalid config in file {}", path))]
  FailedToReadFile {
    path: String,
    source: io::Error,
  },
  #[snafu(display("Unable to parese config because {}", msg))]
  InvalidConfig{
    msg: String,
  },
}
//...
fn load_config(path: String) -> Result<Config, ConfigReadError> {
  let serialized_config = fs::read_to_string(&path).context(FailedToReadFile{path})?;
  parse_config(&serialized_config)
}
