use failure::*;

#[derive(Debug, Fail)]
#[fail(display = "Human readable error message {}", cause)]
pub struct PublicError {
  #[fail(cause)]
  cause: Error,
}
#[derive(Debug, Fail)]
#[fail(display = "Internal error message")]
struct InternalError();
#[derive(Debug, Fail)]
#[fail(display = "Other internal error message")]
struct OtherInternalError();
impl From<Error> for PublicError {
  fn from(err: Error) -> Self {
    PublicError { cause: err }
  }
}
impl From<InternalError> for PublicError {
  fn from(err: InternalError) -> Self {
    PublicError { cause: err.into() }
  }
}
impl From<OtherInternalError> for PublicError {
  fn from(err: OtherInternalError) -> Self {
    PublicError { cause: err.into() }
  }
}


//Then elsewhere:
pub fn public_function() -> Result<(), PublicError> {
  let value: String = do_some_stuff()?;
  do_some_other_stuff(value)?;
  Ok(())
}

fn do_some_stuff() -> Result<String, InternalError> {
  //Dummy logic
  Ok("Hello".to_string())
}
fn do_some_other_stuff(value: String) -> Result<(), OtherInternalError> {
  //...
  Ok(())
}
