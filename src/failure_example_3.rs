use std::fs::File;
use std::io::Read;
use std::io;
use failure::*;

struct Config;
fn parse_config(s : &str) -> Config { Config }

#[derive(Debug, Fail)]
#[fail(display = "Invalid config in file {}", filename)]
struct InvalidConfigError {
  filename : String,
}
impl InvalidConfigError {
  fn new(filename: String) -> InvalidConfigError {
    InvalidConfigError{filename:filename}
  }
}
impl From<io::Error> for InvalidConfigError {
    fn from(err: io::Error) -> Self {
        InvalidConfigError::new(err.)
    }
}
//...
fn load_config(path : String) -> Result<Config, InvalidConfigError> {
  let serializedConfig = File::open(path).or_else(|err| Err(InvalidConfigError::new(path)).read_to_string(&mut path)?;
  Ok(parse_config(serializedConfig))
}
