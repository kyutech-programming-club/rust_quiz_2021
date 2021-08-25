#![allow(dead_code)]

use crate::utils::sagoj0_::io_util;
use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<isize> {
    let num = input.parse::<isize>()?;
    Ok(num + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::rstest;

    #[rstest]
    #[case("42", 43)]
    fn 正_読み込んだ値に1足した数値が返る(
        #[case] input: &str,
        #[case] expected: isize,
    ) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("42", 42)]
    fn 誤_入力足す1以外の値は返らない(#[case] input: &str, #[case] expected: isize) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_ne!(result.unwrap(), expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn 誤_パースに失敗した際はParseIntErrorを返す() {
        use std::num::ParseIntError;

        let result = logic("aa");

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert!(error.downcast_ref::<ParseIntError>().is_some());
    }
}
