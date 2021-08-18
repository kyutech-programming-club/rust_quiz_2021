#![allow(dead_code)]

use anyhow::Result;
use std::io::Read;

fn input_num<T: Read>(input: &mut T) -> Result<isize> {
  let mut buf = String::new();
  input.read_to_string(&mut buf)?;

  let num = buf.parse::<isize>()?;

  
  if num % 2 == 0 {
    Ok(0)
  } else {
    Ok(1)
  }


}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_even() {
    let input = "42".to_owned();
    let mut stdin_mock = input.as_bytes();

    let result = input_num(&mut stdin_mock);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
  }
  #[test]
  fn test_odd() {
    let input = "41".to_owned();
    let mut stdin_mock = input.as_bytes();

    let result = input_num(&mut stdin_mock);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
  }
}
