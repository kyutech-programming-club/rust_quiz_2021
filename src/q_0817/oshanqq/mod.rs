use std::num::ParseIntError;
use std::error::Error;

#[allow(dead_code)]
pub fn convert(input: &str) -> Result<i32, ParseIntError> {
  input.parse::<i32>()
}

#[allow(dead_code)]
pub fn add_one(input: Result<i32, ParseIntError>) -> Result<i32, Box<dyn Error>> {
    Ok(input.unwrap() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn added_nomally() {
        let number = "42";
        assert!(convert(&number).is_ok());
        assert_eq!(add_one(convert(&number)).unwrap(), 43);
    }

    #[test]
    fn added_abnomaly() {
        let number = "553";
        assert!(convert(&number).is_ok());
        assert_ne!(add_one(convert(&number)).unwrap(), 553);
    }

    #[test]
    fn parse_error() {
        let input = "ahiahi";
        assert!(convert(&input).is_err());
    }
}
