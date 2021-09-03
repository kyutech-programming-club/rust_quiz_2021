use anyhow::Result;
use crate::utils::tanacchi::parser::parse_from_iter;

#[allow(dead_code)]
fn main(input: &str) -> Result<isize> {
    let mut input = input.split_whitespace();
    let begin: isize = parse_from_iter(&mut input)?;
    let end: isize = parse_from_iter(&mut input)?;
    Ok((begin..=end).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("0 10", 55)]
    #[case("-10 10", 0)]
    #[case("-10 0", -55)]
    fn should_return_sum_of_range(#[case] input: &str, #[case] expected: isize) {
        let result = main(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
