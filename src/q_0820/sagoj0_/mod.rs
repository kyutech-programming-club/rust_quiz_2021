#![allow(dead_code)]

use crate::utils::sagoj0_::io_util;
use anyhow::{ensure, Result};
use std::f64::consts::PI;
use std::io;

fn logic(input: &str) -> Result<f64> {
    let r = input.parse::<f64>()?;

    ensure!(r >= 0 as f64, "0以上を入力してください");
    let answer = 3. * PI * r * r * r / 4.;
    Ok(answer)
}

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn volume(r: f64) -> f64 {
        3. * PI * r * r * r / 4.
    }

    #[rstest]
    #[case("3", volume(3 as f64))]
    #[case("1.2", volume(1.2 as f64))]
    #[case("1000", volume(1000 as f64))]
    #[case("0", volume(0 as f64))]
    fn 正_球の体積を計算する(#[case] input: &str, #[case] expected: f64) {
        let result = logic(input);

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, expected);
    }

    #[rstest]
    #[case("-1")]
    #[case("-5.7")]
    fn 誤_負の数の際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        assert_eq!(error.to_string().as_str(), "0以上を入力してください");
    }

    #[rstest]
    #[case("error")]
    #[case("###")]
    fn 誤_パースに失敗した際はエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        use matches::assert_matches;
        use std::num::ParseFloatError;
        assert_matches!(
            error.root_cause().downcast_ref::<ParseFloatError>(),
            Some(_)
        );
    }
}
