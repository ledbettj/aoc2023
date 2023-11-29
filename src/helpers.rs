use std::str::FromStr;

pub struct Input(&'static str);

impl Input {
  pub fn lines(&self) -> Vec<&str> {
    self.0.lines().collect()
  }

  pub fn lines_into<T>(&self) -> Vec<T>
  where
    T: From<&'static str>,
  {
    self.0.lines().map(|line| line.into()).collect()
  }

  pub fn lines_parse<T>(&self) -> Result<Vec<T>, T::Err>
  where
    T: FromStr,
  {
    self.0.lines().map(|line| line.parse()).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_input_lines() {
    let i = Input("Hello\nWorld\n");
    assert_eq!(i.lines(), vec!["Hello", "World"]);
  }

  #[test]
  fn test_input_lines_into() {
    let i = Input("Hello\nWorld\n");
    assert_eq!(
      i.lines_into::<String>(),
      vec!["Hello".to_string(), "World".to_string()]
    );
  }

  #[test]
  fn test_input_lines_parse() {
    let i = Input("1\n2\n");
    assert_eq!(i.lines_parse::<i32>(), Ok(vec![1, 2]));
  }
}
