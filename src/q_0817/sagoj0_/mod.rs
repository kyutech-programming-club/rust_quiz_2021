#![allow(dead_code)]

use anyhow::Result;
use std::io::Read;

fn input_plus1<T: Read>(input: &mut T) -> Result<isize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    let num = buf.parse::<isize>()?;

    Ok(num + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_plus1() {
        let stdin_mock = "42".to_owned();
        let mut stdin_mock = stdin_mock.as_bytes();

        let result = input_plus1(&mut stdin_mock);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42 + 1);
    }
}
