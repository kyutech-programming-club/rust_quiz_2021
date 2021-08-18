#![allow(dead_code)]

use anyhow::{anyhow, Result};
use std::io::{Read, Write};

fn parse(iter: &mut std::str::SplitWhitespace) -> Result<isize> {
    if let Some(s) = iter.next() {
        Ok(s.parse::<isize>()?)
    } else {
        eprintln!("数値が足りません");
        Err(anyhow!("Invalid number of input"))
    }
}

fn main<R: Read, W: Write>(src: &mut R, dst: &mut W) -> Result<()> {
    let mut buf = String::new();
    src.read_to_string(&mut buf)?;

    let mut iter = buf.split_whitespace();

    let x = parse(&mut iter)?;
    let y = parse(&mut iter)?;

    writeln!(dst, "{}", if x % y == 0 { "Y" } else { "N" })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("4 2", "Y\n")]
    #[case("100 1", "Y\n")]
    #[case("57 3", "Y\n")]
    #[case("-4 -2", "Y\n")]
    #[case("0 1", "Y\n")]
    #[case("0\n1", "Y\n")]
    fn 正_割り切れた際に_yが出力される(#[case] input: &str, #[case] expected: &str) {
        let expected = expected.as_bytes();

        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }

    #[rstest]
    #[case("1 3", "N\n")]
    #[case("-5 4", "N\n")]
    fn 正_割り切れなかった際に_nが出力される(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let expected = expected.as_bytes();

        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn 誤_パースに失敗した際はParseIntErrorを返す() {
        use matches::assert_matches;
        use std::num::ParseIntError;

        let mut stdin_mock = "-aaa".as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert_matches!(error.root_cause().downcast_ref::<ParseIntError>(), Some(_));
    }

    #[test]
    #[allow(non_snake_case)]
    fn 誤_入力が足りなければエラーを返す() {
        let mut stdin_mock = "1".as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert_eq!(error.to_string().as_str(), "Invalid number of input")
    }
}
