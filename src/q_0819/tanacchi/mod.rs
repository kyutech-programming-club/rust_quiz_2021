use anyhow::{bail, Result};
use std::io::{Read, Write};
use std::string::String;

#[allow(dead_code)]
fn parse(input: &mut std::str::SplitWhitespace) -> Result<i32> {
    if let Some(s) = input.next() {
        Ok(s.parse::<i32>()?)
    } else {
        eprintln!("Number of input must be more than 1.");
        bail!("Invalid number of input.")
    }
}

#[allow(dead_code)]
fn main(src: &mut impl Read, dst: &mut impl Write) -> Result<()> {
    let mut s = String::new();
    src.read_to_string(&mut s)?;

    let mut input = s.split_whitespace();
    let x = parse(&mut input)?;
    let y = parse(&mut input)?;

    if y == 0 {
        bail!("The second number must not be 0.");
    }
    writeln!(dst, "{}", if x % y == 0 { "Y" } else { "N" })?;
    Ok(())
}


#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("42 3", "Y\n")]
    #[case("-42 3", "Y\n")]
    #[case("2\n1", "Y\n")]
    #[case("0\t1", "Y\n")]
    fn should_write_Y_for_divisible_nums(#[case] input: &str, #[case] expected: &str) {
        let mut stdin_mock = input.as_bytes();
        let expected = expected.as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }

    #[rstest]
    #[case("41 3", "N\n")]
    #[case("-41 3", "N\n")]
    #[case("2\n3", "N\n")]
    #[case("1\t9", "N\n")]
    fn should_write_N_for_undivisible_nums(#[case] input: &str, #[case] expected: &str) {
        let mut stdin_mock = input.as_bytes();
        let expected = expected.as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }
}
