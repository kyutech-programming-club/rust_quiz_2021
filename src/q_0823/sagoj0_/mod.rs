#![allow(dead_code)]

use crate::utils::sagoj0_::error::QuizSolveError;
use crate::utils::sagoj0_::{io_util, parse_util};
use anyhow::{anyhow, ensure, Result};
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)?;
    Ok(())
}

fn logic(input: &str) -> Result<i64> {
    let mut iter = input.split_whitespace();

    let num1: i64 = parse_util::parse(&mut iter)?;
    let num2: i64 = parse_util::parse(&mut iter)?;

    ensure!(
        num1 < num2,
        QuizSolveError::invalid_input_error(
            num2,
            "the second argument must be greater than the first."
        )
    );

    (num1..=num2)
        .try_fold(1_i64, |acc, x| acc.checked_mul(x))
        .ok_or(anyhow!(QuizSolveError::OverflowError(
            "multiplication overflow".to_owned()
        )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sagoj0_::error::QuizSolveError;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("-3 10", 0)]
    #[case("1 5", 120)]
    #[case("7 9", 504)]
    fn 正_num1からnum2までの総乗を求める(#[case] input: &str, #[case] expected: i64) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("1 error")]
    #[case("### 0")]
    fn 誤_パースに失敗した際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        use std::num::ParseIntError;
        assert!(error.downcast_ref::<ParseIntError>().is_some());
    }

    #[rstest]
    #[case("1  ")]
    #[case("0")]
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
    #[case("1 1", 1)]
    #[case("0 -5", -5)]
    #[allow(non_snake_case)]
    fn 誤_num1がnum2以上ならエラーを返す(#[case] input: &str, #[case] num2: isize) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

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
    #[rstest]
    #[case("1000 1010")]
    #[allow(non_snake_case)]
    fn 誤_オーバーフローを検知する(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert!(error.downcast_ref::<QuizSolveError>().is_some());
        let error = error.downcast::<QuizSolveError>().unwrap();
        assert_eq!(
            error,
            QuizSolveError::OverflowError("multiplication overflow".to_owned())
        );
    }
}
