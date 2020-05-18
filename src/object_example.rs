use std::u64; 
use std::str::FromStr;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct UUID {
  most_sig_bits: u64,
  least_sig_bits: u64,
}
impl UUID {
  fn new(most_sig_bits: u64, least_sig_bits: u64) -> UUID {
    UUID{ most_sig_bits: most_sig_bits, least_sig_bits: least_sig_bits }
  }
  fn random() -> UUID {
    UUID{ most_sig_bits: rand::random(), least_sig_bits: rand::random() }
  }
}

impl FromStr for UUID {
    type Err = String;
    fn from_str(as_string: &str) -> Result<UUID, String> {
        unimplemented!("Todo: implement this") 
    }
}
impl ToString for UUID {
    fn to_string(&self) -> String {
      "Todo: implement this".to_string()
    }
}
