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
mod test {
    use super::*;

    #[test]
    fn name() {
        let mut stdin_mock = "42 3".as_bytes();
        let expected = "Y\n".as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }
}
