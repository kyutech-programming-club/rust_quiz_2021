#![allow(dead_code)]
use anyhow::Result;
use std::io::{Read, Write};

fn main<Input: Read, Output: Write>(input: &mut Input, output: &mut Output) -> Result<()> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    let v: Vec<&str> = buf.split(' ').collect();
    let first = v[0].parse::<isize>()?;
    let second = v[1].parse::<isize>()?;

    writeln!(output, "{}", if first % second == 0 { "Y" } else { "N" })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_divisible() {
        let input = "10 2".to_owned();
        let mut stdin_mock = input.as_bytes();

        let mut stdout_mock = vec![];
        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, b"Y\n");
    }

    #[test]
    fn test_judge_non_divisible() {
        let input = "3 2".to_owned();
        let mut stdin_mock = input.as_bytes();

        let mut stdout_mock = vec![];
        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, b"N\n");
    }
}
