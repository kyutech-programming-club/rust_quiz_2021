#![allow(dead_code)]

use crate::utils::sagoj0_::error::QuizSolveError;
use crate::utils::sagoj0_::io_util;
use anyhow::{ensure, Result};
use std::f64::consts::PI;
use std::io;

fn logic(input: &str) -> Result<f64> {
    let r = input.parse::<f64>()?;

    ensure!(
        r >= 0 as f64,
        QuizSolveError::invalid_input_error(0, "0以上を入力してください")
    );
    let answer = 4. * PI * r * r * r / 3.;
    Ok(answer)
}

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("1", 4. * PI / 3.)]
    #[case("1.2", 4. * PI * 1.2 * 1.2 * 1.2 / 3.)]
    #[case("1000", 4. * PI * 1000. * 1000. * 1000. / 3.)]
    #[case("0", 0 as f64)]
    fn 正_球の体積を計算する(#[case] input: &str, #[case] expected: f64) {
        let result = logic(input);

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_relative_eq!(value, expected);
    }

    #[rstest]
    #[case("-1")]
    #[case("-5.7")]
    fn 誤_負の数の際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        assert!(error.downcast_ref::<QuizSolveError>().is_some());
        let error = error.downcast::<QuizSolveError>().unwrap();
        assert_eq!(
            error,
            QuizSolveError::invalid_input_error(0, "0以上を入力してください")
        );
    }

    #[rstest]
    #[case("error")]
    #[case("###")]
    fn 誤_パースに失敗した際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        use std::num::ParseFloatError;
        assert!(error.downcast_ref::<ParseFloatError>().is_some());
    }
}
