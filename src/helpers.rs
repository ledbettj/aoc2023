use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ParseError {
  message: String,
}

impl ParseError {
  pub fn new<S: Into<String>>(message: S) -> Self {
    Self {
      message: message.into(),
    }
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Parse error: {}", self.message)
  }
}

impl Error for ParseError {}

#[macro_export]
macro_rules! parse_error {
  ($($arg:tt)*) => {
    ParseError::new(format!($($arg)*))
  };
}
