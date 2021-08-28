#![allow(dead_code)]

use crate::utils::sagoj0_::{io_util, parse_util};
use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<usize> {
    let _ = parse_util::try_parse::<usize>(input)?;
    let rev_str = input.chars().rev().collect::<String>();
    parse_util::try_parse::<usize>(rev_str.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sagoj0_::error::QuizSolveError::{self, *};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("42", 24)]
    #[case("17", 71)]
    #[case("1234", 4321)]
    #[case("303", 303)]
    #[case("1", 1)]
    #[case("60", 6)]
    #[case("500", 5)]
    fn 正_入力を反転する(#[case] input: &str, #[case] expected: usize) {
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
        assert!(result.unwrap_err().is::<std::num::ParseIntError>());
    }

    #[rstest]
    #[case("")]
    #[case(" ")]
    fn 誤_入力が足りなければエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert_eq!(result.is_err(), true);
        let error = result.unwrap_err();
        assert_error_match!(error, QuizSolveError, LackOfInputOnParseError);
    }
}
