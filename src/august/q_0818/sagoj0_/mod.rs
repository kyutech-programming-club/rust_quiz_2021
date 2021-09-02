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
    Ok((num % 2).abs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("42", 0)]
    #[case("-8", 0)]
    #[case("0", 0)]
    fn 正_偶数の際に0が出力される(#[case] input: &str, #[case] expected: isize) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("3", 1)]
    #[case("-5", 1)]
    fn 正_奇数の際に1が出力される(#[case] input: &str, #[case] expected: isize) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn 異_パースに失敗した際はParseIntErrorを返す() {
        use std::num::ParseIntError;
        let result = logic("aa");

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert!(error.downcast_ref::<ParseIntError>().is_some());
    }
}
