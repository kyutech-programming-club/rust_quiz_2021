#![allow(dead_code)]

use anyhow::Result;
use std::io::Read;

fn input_plus1<T: Read>(input: &mut T) -> Result<isize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    let num = buf.parse::<isize>()?;

    Ok(num + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 正_読み込んだ値に1足した数値が返る() {
        let input = "42".to_owned();
        let mut stdin_mock = input.as_bytes();

        let result = input_plus1(&mut stdin_mock);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42 + 1);
    }

    #[test]
    fn 異_入力足す1以外の値は返らない() {
        let input = "42".to_owned();
        let mut stdin_mock = input.as_bytes();
        let result = input_plus1(&mut stdin_mock);

        assert!(result.is_ok());
        assert_ne!(result.unwrap(), 42);
    }

    #[test]
    #[allow(non_snake_case)]
    fn 異_パースに失敗した際はParseIntErrorを返す() {
        use matches::assert_matches;
        use std::num::ParseIntError;

        let input = "aa".to_owned();
        let mut stdin_mock = input.as_bytes();

        let result = input_plus1(&mut stdin_mock);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert_matches!(error.root_cause().downcast_ref::<ParseIntError>(), Some(_));
    }
}
