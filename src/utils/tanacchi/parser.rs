use std::{error, str::FromStr, iter::Iterator};
use anyhow;
use crate::utils::tanacchi::error::Error as MyError;

pub fn parse_from_iter<'a, T, Iter>(iter: &mut Iter) -> anyhow::Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: Sync + Send + error::Error + 'static,
    Iter: Iterator<Item = &'a str>,
{
    if let Some(s) = iter.next() {
        Ok(s.parse::<T>()?)
    } else {
        anyhow::bail!(MyError::LackOfInputElemsError)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_integers_splited_by_whitespaces() {
        let mut input = "1 23\n456\t789".split_whitespace();
        let expected = vec![1_i32, 23_i32, 456_i32, 789_i32];
        for exp in expected {
            let val: anyhow::Result<i32> = parse_from_iter(&mut input);
            assert!(val.is_ok());
            assert_eq!(val.unwrap(), exp);
        }
    }

    #[test]
    fn raise_error_for_lack_of_input() {
        let mut input = "1.23".split_whitespace();

        let val: anyhow::Result<f64> = parse_from_iter(&mut input);
        assert!(val.is_ok());
        assert_eq!(val.unwrap(), 1.23);

        let err: anyhow::Result<i32> = parse_from_iter(&mut input);
        assert!(err.is_err());

        let err = err.unwrap_err();
        assert!(err.downcast_ref::<MyError>().is_some());

        let err = err.downcast_ref::<MyError>().unwrap();
        assert_eq!(err, &MyError::LackOfInputElemsError);
    }
}
