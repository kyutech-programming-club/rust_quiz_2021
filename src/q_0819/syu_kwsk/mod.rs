#![allow(dead_code)]
use anyhow::{anyhow, Result};
use std::io::{Read, Write};

fn main<Input: Read, Output: Write>(input: &mut Input, output: &mut Output) -> Result<()> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    let mut iter = buf.split_whitespace();

    let first = match iter.next() {
        Some(x) => x.parse::<isize>()?,
        None => return Err(anyhow!("Index out of range")),
    };
    let second = match iter.next() {
        Some(x) => x.parse::<isize>()?,
        None => return Err(anyhow!("Index out of range")),
    };

    writeln!(output, "{}", if first % second == 0 { "Y" } else { "N" })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("4 2", "Y\n")]
    #[case("100 1", "Y\n")]
    #[case("57 3", "Y\n")]
    #[case("-4 -2", "Y\n")]
    #[case("0 1", "Y\n")]
    fn test_judge_divisible(#[case] input: &str, #[case] expected: &str) {
        let expected = expected.as_bytes();

        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }

    #[rstest]
    #[case("1 3", "N\n")]
    #[case("-5 4", "N\n")]
    fn test_judge_non_divisible(#[case] input: &str, #[case] expected: &str) {
        let expected = expected.as_bytes();

        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }

    #[rstest]
    #[case("", "Index out of range")]
    #[case(" ", "Index out of range")]
    #[case("1", "Index out of range")]
    fn should_return_error_missing_arguments(#[case] input: &str, #[case] expected: &str) {
        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected);
    }
}
