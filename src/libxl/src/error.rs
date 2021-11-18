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
  pub fn new(message: &str) -> Self {
    return Self {
      message: message.to_string(),
    };
  }
}

impl Error for XlError {}

impl fmt::Display for XlError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.message.as_str())
  }
}

/// Create a new XlError with the given message
#[macro_export]
macro_rules! xl_error {
  ($msg:expr) => {
    XlError::new($msg)
  };
}
