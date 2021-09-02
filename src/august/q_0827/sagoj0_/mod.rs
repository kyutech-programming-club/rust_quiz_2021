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

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn logic(input: &str) -> Result<usize> {
    let mut iter = input.split_whitespace();
    let n: usize = parse_util::parse_from_iter(&mut iter)?;
    ensure!(
        n > 0,
        QuizSolveError::invalid_input_error(0, "the argument must be greater than zero")
    );
    Ok((2..=n).filter(|&x| is_prime(x)).count())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sagoj0_::error::QuizSolveError;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(1, false)]
    #[case(2, true)]
    #[case(3, true)]
    #[case(4, false)]
    #[case(13, true)]
    #[case(57, false)]
    #[case(61, true)]
    #[case(100, false)]
    #[case(999983, true)]
    #[case(1e9 as usize + 7, true)]
    fn 正_素数判定を行う(#[case] input: usize, #[case] expected: bool) {
        let result = is_prime(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("10", 4)]
    #[case("20", 8)]
    fn 正_入力以下の素数を数える(#[case] input: &str, #[case] expected: usize) {
        let result = logic(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("error")]
    #[case("###")]
    fn 異_パースに失敗した際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        use std::num::ParseIntError;
        assert!(error.downcast_ref::<ParseIntError>().is_some());
    }

    #[rstest]
    #[case("")]
    #[case(" ")]
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
    #[case("0")]
    fn 異_引数が0の際はエラーを返す(#[case] input: &str) {
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
