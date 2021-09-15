#![allow(dead_code)]
use crate::utils::sagoj0_::io_util::io_handler;
use crate::utils::sagoj0_::parse_util::parse_from_iter;
use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<&'static str> {
    let mut input = input.split_whitespace();
    let s: String = parse_from_iter(&mut input)?;

    if s.chars().map(|c| c.to_digit(10).unwrap() as usize).sum::<usize>() % 9 == 0 {
        Ok("Yes")
    } else {
        Ok("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("123456789", "Yes")]
    #[case("0", "Yes")]
    #[case("31415926535897932384626433832795028841971693993751058209749445923078164062862089986280", "No")]
    fn 正_atcoderの入出力例において成功する(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let result = logic(input);

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
