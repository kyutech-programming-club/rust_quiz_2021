#![allow(dead_code)]

use crate::utils::sagoj0_::error::QuizSolveError;
use crate::utils::sagoj0_::{io_util, parse_util::parse_from_iter};
use anyhow::{anyhow, Result};
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<isize> {
    let mut iter = input.split_whitespace();
    let n: isize = parse_from_iter(&mut iter)?;
    let m: u32 = parse_from_iter(&mut iter)?;

    power(n, m)
}

pub fn power(n: isize, m: u32) -> Result<isize> {
    n.checked_pow(m)
        .ok_or(anyhow!(QuizSolveError::OverflowError(
            "overflow at power".to_owned()
        )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use QuizSolveError::LackOfInputOnParseError;

    #[rstest]
    #[case("2 5", 32)]
    #[case("3 5", 243)]
    #[case("-3 3", -27)]
    fn 正_累乗計算を行う(#[case] input: &str, #[case] expected: isize) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("10 100")]
    #[case("2 2000")]
    fn 異_オーバーフローを検知する(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_error_match!(
            error,
            QuizSolveError,
            QuizSolveError::OverflowError("overflow at power".to_owned())
        );
    }

    #[rstest]
    #[case("")]
    #[case(" ")]
    #[case("1")]
    fn 異_入力が足りなければエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert_eq!(result.is_err(), true);
        let error = result.unwrap_err();
        assert_error_match!(error, QuizSolveError, LackOfInputOnParseError);
    }
}
