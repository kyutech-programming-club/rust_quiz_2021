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
    use crate::utils::tanacchi::stream::apply;

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
}
