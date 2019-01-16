use std::fs::File;
use std::io::Read;
use failure::*;

struct Config;
fn parse_config(s : &str) -> Config { Config }

#[derive(Debug, Fail)]
#[fail(display = "Invalid config in file {}", filename)]
struct InvalidConfigError {
  filename : String,
}
//...
fn load_config(path : String) -> Result<Config, InvalidConfigError> {
  let serializedConfig = File::open(path)?.read_to_string(&mut path)?;
  Ok(parse_config(serializedConfig))
}