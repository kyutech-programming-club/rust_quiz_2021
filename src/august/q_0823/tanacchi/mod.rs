use anyhow::{ensure, Result};
use crate::utils::tanacchi::parser::parse_from_iter;
use crate::utils::tanacchi::error::Error as MyError;

#[allow(dead_code)]
fn main(input: &str) -> Result<isize> {
    let mut input = input.split_whitespace();
    let begin: isize = parse_from_iter(&mut input)?;
    let end: isize = parse_from_iter(&mut input)?;
    ensure!(
        begin < end,
        MyError::InvalidInputError("The second number must be greater than the first one.")
    );
    Ok((begin..=end).into_iter().fold(1, |sum, n| sum * n))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1 10", 3628800)]
    #[case("0 10", 0)]
    #[case("-5 5", 0)]
    #[case("3 10", 1814400)]
    fn should_return_mul_of_range(#[case] input: &str, #[case] expected: isize) {
        let result = main(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
