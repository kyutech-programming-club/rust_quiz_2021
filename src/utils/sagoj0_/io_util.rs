use anyhow::Result;
use std::fmt::Display;
use std::io::{Read, Write};

pub fn io_handler<F, T>(src: &mut impl Read, dst: &mut impl Write, logic: F) -> Result<()>
where
    F: FnOnce(&str) -> Result<T>,
    T: Display,
{
    let mut buf = String::new();
    src.read_to_string(&mut buf)?;

    let logic_result = logic(&buf)?;

    writeln!(dst, "{}", logic_result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("42", "0\n")]
    #[case("-1", "1\n")]
    fn 正_quiz0818の計算を行う関数で正常通りに動作する(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let expected = expected.as_bytes();

        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];

        fn test_logic(input: &str) -> Result<isize> {
            let num = input.parse::<isize>()?;
            Ok((num % 2).abs())
        }

        let result = io_handler(&mut stdin_mock, &mut stdout_mock, test_logic);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }
    #[rstest]
    #[case("42", "43\n")]
    #[case("-1", "0\n")]
    fn 正_quiz0817の計算を行う関数で正常通りに動作する(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let expected = expected.as_bytes();

        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];

        fn test_logic(input: &str) -> Result<isize> {
            let num = input.parse::<isize>()?;
            Ok(num + 1)
        }

        let result = io_handler(&mut stdin_mock, &mut stdout_mock, test_logic);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }

    use anyhow::anyhow;
    use std::io::{self, ErrorKind::*};
    #[rstest]
    #[case(anyhow!(io::Error::new(NotFound, "ないです")))]
    #[case(anyhow!(io::Error::new(PermissionDenied, "ダメです")))]
    fn 誤_logicがエラーを返した際はそのエラーを返す(#[case] err: anyhow::Error) {
        use anyhow::bail;

        let mut stdin_mock = "".as_bytes();
        let mut stdout_mock = vec![];

        let expected = err.to_string();

        let test_logic = |_: &str| -> Result<isize> {
            bail!(err);
        };

        let result = io_handler(&mut stdin_mock, &mut stdout_mock, test_logic);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), expected);
    }
}
