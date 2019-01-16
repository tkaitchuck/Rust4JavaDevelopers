use derive_more::Display;
use failure::*;

struct Message{/* ... */}

#[derive(Debug, Display, Fail)]
enum DecodeFailure {
  CorruptData,
}

struct Decoder {
  //...
}
impl Decoder {
  //...
  pub fn decode(&self, message : &str) -> Result<Message, DecodeFailure> {
    //...
    Ok(Message{/*...*/})
  }
}
