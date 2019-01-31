use failure::*;

struct Message {/* ... */}

struct Decoder {
  //...
}
impl Decoder {
  //...
  pub fn decode(&self, message: &str) -> Result<Message, Error> {
    ensure!(!message.is_empty(), "Message was empty");
    //...
    Ok(Message{/*...*/})
  }
}
