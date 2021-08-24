use std::num::ParseIntError;
use std::error::Error;

#[allow(dead_code)]
pub fn convert(input: &str) -> Result<i32, ParseIntError> {
  input.parse::<i32>()
}

#[allow(dead_code)]
pub fn add_one(input: Result<i32, ParseIntError>) -> Result<i32, Box<dyn Error>> {
    Ok(input.unwrap() + 1)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case("0", 1)]
    #[case("42", 43)]
    #[case("-6", -5)]
    fn added_nomally(#[case] input: &str, #[case] expected: i32) {
        assert!(convert(&input).is_ok());
        assert_eq!(add_one(convert(&input)).unwrap(), expected);
    }

    #[rstest]
    #[case("0", 0)]
    #[case("42", 42)]
    #[case("-6", -6)]
    fn added_abnomaly(#[case] input: &str, #[case] expected: i32) {
        assert!(convert(&input).is_ok());
        assert_ne!(add_one(convert(&input)).unwrap(), expected);
    }

    #[rstest]
    #[case("ahiahi")]
    #[case("proken216")]
    #[case("!#")]
    fn parse_error(#[case] input: &str) {
        assert!(convert(&input).is_err());
    }
}
