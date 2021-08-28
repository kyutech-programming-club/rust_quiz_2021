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

fn logic(input: &str) -> Result<String> {
    let mut iter = input.split_whitespace();

    let x: isize = parse_util::parse_from_iter(&mut iter)?;
    let y: isize = parse_util::parse_from_iter(&mut iter)?;

    ensure!(
        y != 0,
        QuizSolveError::invalid_input_error(0, "the second number must not be 0")
    );

    Ok(if x % y == 0 {
        "Y".to_owned()
    } else {
        "N".to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use difference::assert_diff;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("4 2", "Y")]
    #[case("100 1", "Y")]
    #[case("57 3", "Y")]
    #[case("-4 -2", "Y")]
    #[case("0 1", "Y")]
    #[case("0\n1", "Y")]
    fn 正_割り切れた際に_yが出力される(#[case] input: &str, #[case] expected: &str) {
        let result = logic(input);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_diff!(result.as_str(), expected, "\n", 0);
    }

    #[rstest]
    #[case("1 3", "N")]
    #[case("-5 4", "N")]
    fn 正_割り切れなかった際に_nが出力される(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let result = logic(input);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.as_str(), expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn 誤_パースに失敗した際はParseIntErrorを返す() {
        use std::num::ParseIntError;
        let result = logic("aa");
        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert!(error.downcast_ref::<ParseIntError>().is_some());
    }

    #[rstest]
    #[allow(non_snake_case)]
    #[case(" ")]
    #[case("1")]
    fn 誤_入力が足りなければエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert!(error.downcast_ref::<QuizSolveError>().is_some());
        let error = error.downcast::<QuizSolveError>().unwrap();
        assert_eq!(error, QuizSolveError::LackOfInputOnParseError);
    }

    #[rstest]
    #[case("1 0")]
    #[case("0 0")]
    #[allow(non_snake_case)]
    fn 誤_二つ目の入力が0ならエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        assert!(error.downcast_ref::<QuizSolveError>().is_some());
        let error = error.downcast::<QuizSolveError>().unwrap();
        assert_eq!(
            error,
            QuizSolveError::invalid_input_error(0, "the second number must not be 0")
        );
    }
}
