use snafu::*;

#[derive(Debug, Snafu)]
pub struct PublicError(InternalError);

#[derive(Debug, Snafu)]
enum InternalError {
    #[snafu(display("Internal error message"))]
    SomeInternalError,
    #[snafu(display("Other internal error message"))]
    OtherInternalError,
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
fn do_some_other_stuff(value: String) -> Result<(), InternalError> {
  //...
  Ok(())
}
