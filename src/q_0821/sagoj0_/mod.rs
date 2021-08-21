#![allow(dead_code)]

use crate::utils::sagoj0_::io_util;
use crate::utils::sagoj0_::parse_util;
use anyhow::{ensure, Result};
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)?;
    Ok(())
}

fn logic(input: &str) -> Result<String> {
    let mut iter = input.split_whitespace();

    let height: f64 = parse_util::parse(&mut iter)?;
    let width: f64 = parse_util::parse(&mut iter)?;

    ensure!(height > 0., "the height value must be greater than zero");

    Ok(match width / height / height {
        bmi if bmi < 18.5 => "瘦せ型".to_owned(),
        bmi if bmi < 25. => "普通".to_owned(),
        _ => "肥満".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1.6 60", "普通")]
    #[case("1.7 50", "瘦せ型")]
    #[case("1.5 100", "肥満")]
    fn 正_bmiから肥満度を判定する(#[case] input: &str, #[case] expected: &str) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("1 error")]
    #[case("### 0")]
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

    #[rstest]
    #[case("1  ")]
    #[case("0")]
    fn 誤_入力が足りなければエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert_eq!(error.to_string().as_str(), "no input error")
    }

    #[rstest]
    #[case("0 1")]
    #[case("0 0")]
    #[allow(non_snake_case)]
    fn 誤_1つ目の入力が0ならエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error.to_string().as_str(),
            "the height value must be greater than zero"
        )
    }
}
