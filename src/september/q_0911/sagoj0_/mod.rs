#![allow(dead_code)]
use crate::utils::sagoj0_::io_util::io_handler;
use crate::utils::sagoj0_::parse_util::parse_from_iter;
use anyhow::{anyhow, Result};
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<isize> {
    let mut input = input.split_whitespace();
    let a: isize = parse_from_iter(&mut input)?;
    let b: isize = parse_from_iter(&mut input)?;
    let c: isize = parse_from_iter(&mut input)?;
    let d: isize = parse_from_iter(&mut input)?;

    (0..=a)
        .find(|i| a <= ((c * d) - b) * i)
        .or(Some(-1))
        .ok_or_else(|| anyhow!("unreachable"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("5 2 3 2", 2)]
    #[case("6 9 2 3", -1)]
    fn 正_atcoderの入出力例において成功する(
        #[case] input: &str,
        #[case] expected: isize,
    ) {
        let result = logic(input);

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
