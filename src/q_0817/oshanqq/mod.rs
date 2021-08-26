use anyhow::Result;
use std::io::Read;

#[allow(dead_code)]
fn plus_one<T: Read>(input: &mut T) -> Result<isize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let num = buf.parse::<isize>()?;

    Ok(num + 1)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn add_nomally() {
        let input = "42".to_owned();
        let mut stdin_mock = input.as_bytes();

        let result = plus_one(&mut stdin_mock);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42 + 1);
    }

    #[test]
    fn failed_addition() {
        let input = "42".to_owned();
        let mut stdin_mock = input.as_bytes();

        let result = plus_one(&mut stdin_mock);

        assert!(result.is_ok());
        assert_ne!(result.unwrap(), 42);
    }

    #[test]
    fn type_error() {
        use matches::assert_matches;
        use std::num::ParseIntError;

        let input = "aa".to_owned();
        let mut stdin_mock = input.as_bytes();

        let result = plus_one(&mut stdin_mock);

        assert!(result.is_err());
        let error = result.unwrap_err();

        assert_matches!(error.root_cause().downcast_ref::<ParseIntError>(), Some(_));
    }
}

