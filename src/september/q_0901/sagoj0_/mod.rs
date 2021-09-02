#![allow(dead_code)]

use crate::utils::sagoj0_::{io_util, parse_util::parse_to_vec};
use anyhow::Result;
use nalgebra::{Matrix3x1, Vector3};
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<Matrix3x1<isize>> {
    let mut iter = input.split_whitespace();
    let x = Vector3::from_vec(parse_to_vec(&mut iter, 3)?);
    let y = Vector3::from_vec(parse_to_vec(&mut iter, 3)?);

    Ok(x.cross(&y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sagoj0_::error::QuizSolveError::{self, LackOfInputOnParseError};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("1 2 3 3 2 1", Vector3::new(-4, 8, -4))]
    #[case("5 -2 3 4 0 -4", Vector3::new(8, 32, 8))]
    fn 正_ベクトルの外積を求める(
        #[case] input: &str,
        #[case] expected: Matrix3x1<isize>,
    ) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
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
