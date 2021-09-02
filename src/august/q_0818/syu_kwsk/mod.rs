#![allow(dead_code)]

use std::io::{Read, Write};

fn main<Input: Read, Output: Write>(input: &mut Input, output: &mut Output) -> anyhow::Result<()> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    let num = buf.parse::<isize>()?;

    writeln!(output, "{}", if num % 2 == 1 { 1 } else { 0 })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn test_judge_odd() {
        let input = "1".to_owned();
        let mut stdin_mock = input.as_bytes();

        let mut stdout_mock = vec![];
        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, b"1\n");
    }

    #[test]
    fn test_judge_even() {
        let input = "2".to_owned();
        let mut stdin_mock = input.as_bytes();

        let mut stdout_mock = vec![];
        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, b"0\n");
    }
}
