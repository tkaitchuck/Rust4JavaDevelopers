use snafu::*;

struct Message {/* ... */}

#[derive(Debug, Snafu)]
enum DecodeError {
  #[snafu(display("Error decoding message due to {}", error_message))]
  CorruptData {
    error_message: String,
  },
}

struct Decoder {
  //...
}
impl Decoder {
  //...
  pub fn decode(&self, message: &str) -> Result<Message, DecodeError> {
    //...
    Ok(Message{/*...*/})
  }
}
