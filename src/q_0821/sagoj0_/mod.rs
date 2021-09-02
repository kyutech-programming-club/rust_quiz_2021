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

    let height: f64 = parse_util::parse_from_iter(&mut iter)?;
    let width: f64 = parse_util::parse_from_iter(&mut iter)?;

    ensure!(
        height > 0.,
        QuizSolveError::invalid_input_error(0, "the height value must be greater than zero")
    );

    Ok(match width / height / height {
        bmi if bmi < 18.5 => "瘦せ型".to_owned(),
        bmi if bmi < 25. => "普通".to_owned(),
        _ => "肥満".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use difference::assert_diff;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("1.6 60", "普通")]
    #[case("1.7 50", "瘦せ型")]
    #[case("1.5 100", "肥満")]
    fn 正_bmiから肥満度を判定する(#[case] input: &str, #[case] expected: &str) {
        let result = logic(input);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_diff!(result.as_str(), expected, "\n", 0);
    }

    #[rstest]
    #[case("1 error")]
    #[case("### 0")]
    fn 異_パースに失敗した際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        use std::num::ParseFloatError;
        assert!(error.downcast_ref::<ParseFloatError>().is_some());
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
    #[case("0 1")]
    #[case("0 0")]
    #[allow(non_snake_case)]
    fn 異_1つ目の入力が0ならエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        assert!(error.downcast_ref::<QuizSolveError>().is_some());
        let error = error.downcast::<QuizSolveError>().unwrap();
        assert_eq!(
            error,
            QuizSolveError::invalid_input_error(0, "the height value must be greater than zero")
        )
    }
}
