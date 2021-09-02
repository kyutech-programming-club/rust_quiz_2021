#![allow(dead_code)]

use crate::utils::sagoj0_::error::QuizSolveError;
use crate::utils::sagoj0_::{io_util, parse_util};
use anyhow::{ensure, Result};
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<isize> {
    let mut iter = input.split_whitespace();

    let num1: isize = parse_util::parse_from_iter(&mut iter)?;
    let num2: isize = parse_util::parse_from_iter(&mut iter)?;

    ensure!(
        num1 < num2,
        QuizSolveError::invalid_input_error(
            num2,
            "the second argument must be greater than the first."
        )
    );

    Ok((num1..=num2).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sagoj0_::error::QuizSolveError;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("0 10", 55)]
    #[case("1 100", 5050)]
    #[case("-100 100", 0)]
    fn 正_num1からnum2までの総和を求める(#[case] input: &str, #[case] expected: isize) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("1 error")]
    #[case("### 0")]
    fn 異_パースに失敗した際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        use std::num::ParseIntError;
        assert!(error.downcast_ref::<ParseIntError>().is_some());
    }

    #[rstest]
    #[case("1  ")]
    #[case("0")]
    fn 異_入力が足りなければエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert!(error.downcast_ref::<QuizSolveError>().is_some());
        let error = error.downcast::<QuizSolveError>().unwrap();
        assert_eq!(error, QuizSolveError::LackOfInputOnParseError)
    }

    #[rstest]
    #[case("1 1", 1)]
    #[case("0 -5", -5)]
    #[allow(non_snake_case)]
    fn 異_num1がnum2以上ならエラーを返す(#[case] input: &str, #[case] num2: isize) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        //       assert_eq!(error.to_string().as_str(), "Invalid input")

        // errorの種類を検証
        assert!(error.downcast_ref::<QuizSolveError>().is_some());
        let error = error.downcast::<QuizSolveError>().unwrap();
        assert_eq!(
            error,
            QuizSolveError::invalid_input_error(
                num2,
                "the second argument must be greater than the first."
            )
        );
    }
}
