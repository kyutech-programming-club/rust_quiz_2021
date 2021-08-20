#![allow(dead_code)]
use anyhow::Result;
use std::io::{Read, Write};

fn input_num<Input: Read, Output: Write>(input: &mut Input, output: &mut Output) -> Result<()> {
  let mut buf = String::new();
  input.read_to_string(&mut buf)?;

  let v: Vec<&str> = buf.split(&['-', ' ', ':', '@'][..]).collect();
  let first = v[0].parse::<isize>()?;
  let second = v[1].parse::<isize>()?;

  writeln!(output, "{}", if first % second == 0 { "Y" } else { "N" })?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_visible() {
    let input = "57 3".to_owned();
    let mut stdin_mock = input.as_bytes();

    let mut stdout_mock = vec![];
    let result = input_num(&mut stdin_mock, &mut stdout_mock);

    assert!(result.is_ok());
    assert_eq!(stdout_mock, b"Y\n");
  }

  #[test]
  fn test_divisible() {
    let input = "57 5".to_owned();
    let mut stdin_mock = input.as_bytes();

    let mut stdout_mock = vec![];
    let result = input_num(&mut stdin_mock, &mut stdout_mock);

    assert!(result.is_ok());
    assert_eq!(stdout_mock, b"N\n");
  }

  #[test]
  #[allow(non_snake_case)]
  fn return_ParseIntError() {
    use matches::assert_matches;
    use std::num::ParseIntError;
    
    let mut stdin_mock = "aa".as_bytes();
    let mut stdout_mock = vec![];
    
    let result = input_num(&mut stdin_mock, &mut stdout_mock);

    assert!(result.is_err());
    let error = result.unwrap_err();

    // errorの種類を検証
    assert_matches!(error.root_cause().downcast_ref::<ParseIntError>(), Some(_));
  }
}
