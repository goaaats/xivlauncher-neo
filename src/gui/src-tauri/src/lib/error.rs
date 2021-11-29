use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct XlError {
  /// A serializable generic error
  pub message: String,
}

impl XlError {
  /// Create a new error.
  /// # Arguments:
  /// * `message` - The error message
  pub fn new(message: String) -> Self {
    Self { message }
  }
}

impl Error for XlError {}

impl From<anyhow::Error> for XlError {
  fn from(e: anyhow::Error) -> Self {
    XlError::new(format!("{:#?}", e))
  }
}

impl fmt::Display for XlError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.message.as_str())
  }
}

pub type XlResult<T> = std::result::Result<T, XlError>;
