use std::io::{Read, Write};
use anyhow::Result;

#[allow(dead_code)]
fn main(read_buf: &mut impl Read, write_buf: &mut impl Write) -> Result<()> {
    let mut s = String::new();
    read_buf.read_to_string(&mut s)?;
    let n = s.trim().parse::<i32>()?.rem_euclid(2);
    writeln!(write_buf, "{}", n)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("31", "1\n")]
    #[case("-1", "1\n")]
    fn should_write_1_when_input_is_odd(#[case] input: &str, #[case] expected: &str) {
        let expected = expected.as_bytes();
        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);
        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }

    #[rstest]
    #[case("42", "0\n")]
    #[case("0",  "0\n")]
    #[case("-2", "0\n")]
    fn should_write_0_when_input_is_even(#[case] input: &str, #[case] expected: &str) {
        let expected = expected.as_bytes();
        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);
        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_raise_ParseIntError_when_input_is_not_integer() {
        use matches::assert_matches;
        use std::num::ParseIntError;

        let mut stdin_mock = "ahiahi".as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_matches!(err.root_cause().downcast_ref::<ParseIntError>(), Some(_));
    }
}
