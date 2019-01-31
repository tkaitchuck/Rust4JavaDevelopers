use failure::Fail;

struct Message {/* ... */}

#[derive(Debug, Fail)]
enum DecodeFailure {
  #[fail(display = "Failure decoding message")]
  CorruptData,
}

struct Decoder {
  //...
}
impl Decoder {
  //...
  pub fn decode(&self, message: &str) -> Result<Message, DecodeFailure> {
    //...
    Ok(Message{/*...*/})
  }
}
