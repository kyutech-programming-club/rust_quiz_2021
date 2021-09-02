#![allow(dead_code)]

use crate::utils::sagoj0_::{io_util, parse_util::parse_to_vec};
use anyhow::Result;
use nalgebra::Vector3;
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<isize> {
    let mut iter = input.split_whitespace();
    let x = Vector3::from_vec(parse_to_vec(&mut iter, 3)?);
    let y = Vector3::from_vec(parse_to_vec(&mut iter, 3)?);

    Ok(x.dot(&y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sagoj0_::error::QuizSolveError::{self, LackOfInputOnParseError};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("1 2 3 3 2 1", 10)]
    fn 正_ベクトルの内積を求める(#[case] input: &str, #[case] expected: isize) {
        let result = logic(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("1 2 3 4 3 e")]
    #[case("2 # # 3 3 3")]
    fn 異_パースに失敗した際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        assert!(result.unwrap_err().is::<std::num::ParseIntError>());
    }

    #[rstest]
    #[case("")]
    #[case(" ")]
    #[case("1 2 3 4 5")]
    #[case("1 2 3 4")]
    #[case("1")]
    fn 異_入力が足りなければエラーを返す(#[case] input: &str) {
        let result = logic(input);
        assert_eq!(result.is_err(), true);
        let error = result.unwrap_err();
        assert_error_match!(error, QuizSolveError, LackOfInputOnParseError);
    }
}
