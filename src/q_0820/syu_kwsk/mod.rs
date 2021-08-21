#![allow(dead_code)]
use std::io;
use anyhow::Result;

use crate::utils::sagoj0_::io_util;

fn logic(input: &str) -> Result<f64> {
    let r = input.parse::<f64>()?;
    let pi: f64 = 3.14;

    let volume = 4.0 / 3.0 * pi * r * r * r;
    Ok(volume)
}

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("4", "267.94666666666666")]
    #[case("1", "4.1866666666666665")]
    fn test_correct_volume(#[case] input: &str, #[case] expected: &str) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), expected);
    }

    #[rstest]
    #[case("1", "1")]
    fn test_wrong_volume(#[case] input: &str, #[case] expected: &str) {
        let result = logic(input);

        assert!(result.is_ok());
        assert_ne!(result.unwrap().to_string(), expected);
    }
}
