use std::io;
use std::f64::consts::PI;
use anyhow::{ensure, Result};
use crate::utils::tanacchi::{error::Error as MyError, stream};

fn calc_volume(input: &str) -> Result<f64> {
    let r = input.parse::<f64>()?;
    ensure!(
        r >= 0.0,
        MyError::InvalidInputError("Radian must be greater than 0.")
    );
    Ok(4.0 * PI * r.powi(3) / 3.0)
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    stream::apply(&mut stdin, &mut stdout, calc_volume)
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::f64::consts::PI;
    use std::num::ParseFloatError;

    #[rstest]
    #[case("0", 0.0_f64)]
    #[case("1", 4.0 * PI / 3.0)]
    #[case("23.4", 4.0 * PI * 23.4_f64.powi(3) / 3.0)]
    fn should_calc_valid_float_number(#[case] input: &str, #[case] expected: f64) {
        let result = calc_volume(input);
        assert!(result.is_ok());
        assert_relative_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("-1")]
    #[case("-3.1")]
    fn raise_error_for_nagative_radian(#[case] input: &str) {
        let result = calc_volume(input);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.downcast_ref::<MyError>().is_some());

        let err = err.downcast::<MyError>().unwrap();
        assert_eq!(
            err,
            MyError::InvalidInputError("Radian must be greater than 0.")
        );
    }

    #[rstest]
    #[case("ahiahi")]
    #[case("float")]
    fn raise_error_when_input_isnt_float(#[case] input: &str) {
        let result = calc_volume(input);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.downcast_ref::<ParseFloatError>().is_some());
    }
}
