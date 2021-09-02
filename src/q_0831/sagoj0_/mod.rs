#![allow(dead_code)]

use crate::utils::sagoj0_::error::QuizSolveError::LackOfInputOnParseError;
use crate::utils::sagoj0_::{io_util, parse_util::try_parse};
use anyhow::{ensure, Result};
use nalgebra::Vector3;
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<isize> {
    let x = input
        .split_whitespace()
        .take(3)
        .map(try_parse)
        .collect::<Result<Vec<isize>>>()?;
    ensure!(x.len() == 3, LackOfInputOnParseError); // takeは要素数が足りなくても失敗しない
    let x = Vector3::from_vec(x);

    let y = input
        .split_whitespace()
        .skip(3)
        .take(3)
        .map(try_parse)
        .collect::<Result<Vec<isize>>>()?;
    ensure!(y.len() == 3, LackOfInputOnParseError);
    let y = Vector3::from_vec(y);

    Ok(x.dot(&y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sagoj0_::error::QuizSolveError;
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
