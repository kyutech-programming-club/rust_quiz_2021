#![allow(dead_code)]

use crate::utils::sagoj0_::error::QuizSolveError::LackOfInputOnParseError;
use crate::utils::sagoj0_::{io_util, parse_util};
use anyhow::{ensure, Result};
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<isize> {
    let len = input.len();
    ensure!(len > 0, LackOfInputOnParseError);

    // split("")は一文字ずつ&strに切り分けるが先頭と末尾に""が追加される
    let mut iter = input.split("").skip(1).take(len);

    (0..len).try_fold(0, |acc, _| {
        parse_util::parse_from_iter::<isize, _>(&mut iter).map(|x| acc + x)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sagoj0_::error::QuizSolveError;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("1234", 10)]
    #[case("15", 6)]
    #[case("123456789", 45)]
    fn 正_入力の各桁の総和を求める(#[case] input: &str, #[case] expected: isize) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("1 error")]
    #[case("### 0")]
    #[case("111 222")]
    #[case(" ")]
    fn 誤_パースに失敗した際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        use std::num::ParseIntError;
        assert!(error.downcast_ref::<ParseIntError>().is_some());
    }

    #[rstest]
    #[case("")]
    fn 誤_入力が足りなければエラーを返す(#[case] input: &str) {
        let result = logic(input);

        println!("{:?}", result);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert!(error.downcast_ref::<QuizSolveError>().is_some());
        let error = error.downcast::<QuizSolveError>().unwrap();
        assert_eq!(error, QuizSolveError::LackOfInputOnParseError)
    }
}
