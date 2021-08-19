use std::io::Read;
use anyhow::Result;

#[allow(dead_code)]
fn main(read_buf: &mut impl Read) -> Result<i32> {
    // Read from buf.
    let mut s = String::new();
    read_buf.read_to_string(&mut s)?;
    // Parse into i32.
    let n = s.trim().parse::<i32>()?;
    // Add one.
    Ok(n + 1)
}

#[cfg(test)]
mod tests {
    use super::main;
    use rstest::rstest;

    #[rstest]
    #[case("42", 43)]
    #[case("-1", 0)]
    #[case("-100", -99)]
    fn should_parse_int_and_add_one(#[case] input: &str, #[case] expected: i32) {
        let input = input.to_owned();
        let mut read_buf = input.as_bytes();

        let result = main(&mut read_buf);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("ahi")]
    #[case("-0.1")]
    #[case("0xff")]
    fn should_raise_error_for_invalid_input(#[case] input: &str) {
        use matches::assert_matches;
        use std::num::ParseIntError;

        let input = input.to_owned();
        let mut read_buf = input.as_bytes();

        let result = main(&mut read_buf);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_matches!(error.root_cause().downcast_ref::<ParseIntError>(), Some(_));
    }
}
