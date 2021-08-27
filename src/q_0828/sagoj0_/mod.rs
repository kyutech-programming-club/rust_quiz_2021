#![allow(dead_code)]

use crate::utils::sagoj0_::error::QuizSolveError;
use crate::utils::sagoj0_::{io_util, parse_util};
use anyhow::{anyhow, Result};
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn count_divisor(n: usize) -> usize {
    (1..=n).filter(|&i| n % i == 0).count()
}

fn logic(input: &str) -> Result<usize> {
    let mut iter = input.split_whitespace();
    let n: usize = parse_util::parse(&mut iter)?;

    (1..=n)
        .max_by_key(|&i| count_divisor(i))
        .ok_or(anyhow!(QuizSolveError::invalid_input_error(
            0,
            "the argument must be greater than zero"
        )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sagoj0_::error::QuizSolveError;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 2)]
    #[case(4, 3)]
    #[case(13, 2)]
    #[case(360, 24)]
    fn 正_約数の個数を数える(#[case] input: usize, #[case] expected: usize) {
        let result = count_divisor(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("11", 10)]
    #[case("17", 12)]
    #[case("19", 18)]
    #[case("30", 30)]
    #[case("59", 48)]
    #[case("60", 60)]
    #[case("500", 480)]
    fn 正_入力以下の素数を数える(#[case] input: &str, #[case] expected: usize) {
        let result = logic(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("error")]
    #[case("###")]
    fn 誤_パースに失敗した際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        use std::num::ParseIntError;
        assert!(error.downcast_ref::<ParseIntError>().is_some());
    }

    #[rstest]
    #[case("")]
    #[case(" ")]
    fn 誤_入力が足りなければエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert!(error.downcast_ref::<QuizSolveError>().is_some());
        let error = error.downcast::<QuizSolveError>().unwrap();
        assert_eq!(error, QuizSolveError::LackOfInputOnParseError)
    }

    #[rstest]
    #[case("0")]
    fn 誤_引数が0の際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert!(error.downcast_ref::<QuizSolveError>().is_some());
        let error = error.downcast::<QuizSolveError>().unwrap();
        assert_eq!(
            error,
            QuizSolveError::invalid_input_error(0, "the argument must be greater than zero")
        );
    }
}
