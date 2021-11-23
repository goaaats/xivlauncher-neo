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
  #[must_use]
  pub fn from(message: &str) -> Self {
    Self { message: message.to_string() }
  }

  /// Create a new error.
  /// # Arguments:
  /// * `message` - The error message
  pub fn new(message: String) -> Self {
    Self { message }
  }
}

impl Error for XlError {}

impl fmt::Display for XlError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.message.as_str())
  }
}

pub type XlResult<T> = std::result::Result<T, XlError>;
