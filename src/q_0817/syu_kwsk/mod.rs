#![allow(dead_code)]

fn main<Input: std::io::Read>(input: &mut Input) -> anyhow::Result<isize> {
    let mut buf = String::new();

    input.read_to_string(&mut buf)?;
    let num = buf.parse::<isize>()?;

    Ok(num + 1)
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn test_increment() {
        let input = "1".to_owned();
        let mut stdin_mock = input.as_bytes();

        let result = main(&mut stdin_mock);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1 + 1);
    }

    #[test]
    fn test_increment_error() {
        let input = "1".to_owned();
        let mut stdin_mock = input.as_bytes();

        let result = main(&mut stdin_mock);

        assert!(result.is_ok());
        assert_ne!(result.unwrap(), 1);
    }

    #[test]
    fn test_parse_error() {
        use matches::assert_matches;
        use std::num::ParseIntError;

        let input = "abc".to_owned();
        let mut stdin_mock = input.as_bytes();

        let result = main(&mut stdin_mock);

        assert!(result.is_err());
        let error = result.unwrap_err();

        assert_matches!(error.root_cause().downcast_ref::<ParseIntError>(), Some(_));
    }
}
