use anyhow::Result;
use crate::utils::tanacchi::parser::parse_from_iter;


#[allow(dead_code)]
fn main(input: &str) -> Result<&'static str> {
    let mut input = input.split_whitespace();
    let height: f64 = parse_from_iter(&mut input)?;
    let weight: f64 = parse_from_iter(&mut input)?;
    let bmi = weight / height.powi(2);
    Ok(if bmi < 18.5 {
        "痩せ型"
    } else if bmi >= 25.0 {
        "肥満"
    } else {
        "普通"
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use crate::utils::tanacchi::{
        error::Error as MyError,
        stream::apply,
    };
    use std::num::ParseFloatError;

    #[rstest]
    #[case("1.70 50", "痩せ型\n")]
    #[case("1.70 70", "普通\n")]
    #[case("1.70 90", "肥満\n")]
    fn should_return_result_for_valid_examples(#[case] input: &str, #[case] expected: &str) {
        let mut stdin_mock = input.as_bytes();
        let expected = expected.as_bytes();
        let mut stdout_mock = vec![];
        let result = apply(&mut stdin_mock, &mut stdout_mock, main);
        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }

    #[rstest]
    #[case("50")]
    #[case("1.70")]
    fn raise_error_when_number_of_inputs_is_not_enough(#[case] input: &str) {
        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];
        let result = apply(&mut stdin_mock, &mut stdout_mock, main);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.downcast_ref::<MyError>().is_some());

        let err = err.downcast::<MyError>().unwrap();
        assert_eq!(err, MyError::LackOfInputElemsError);
    }

    #[rstest]
    #[case("ahi 50")]
    #[case("1.70 chani")]
    #[case("ahi ahi")]
    fn raise_error_when_input_cannot_be_parsed_to_float(#[case] input: &str) {
        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];
        let result = apply(&mut stdin_mock, &mut stdout_mock, main);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.downcast_ref::<ParseFloatError>().is_some());
    }
}
