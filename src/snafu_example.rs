use snafu::*;

struct Message {/* ... */}

struct Decoder {
  //...
}
#[derive(Debug, Snafu)]
enum DecodeError {
    EmptyMessage,
}

impl Decoder {
  //...
  pub fn decode(&self, message: &str) -> Result<Message, DecodeError> {
    ensure!(!message.is_empty(), EmptyMessage);
    //...
    Ok(Message{/*...*/})
  }
}
