#![allow(dead_code)]
use crate::utils::sagoj0_::io_util::io_handler;
use crate::utils::sagoj0_::parse_util::parse_from_iter;
use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<usize> {
    let mut input = input.split_whitespace();
    let _: usize = parse_from_iter(&mut input)?;
    let x: usize = parse_from_iter(&mut input)?;
    let s: String = parse_from_iter(&mut input)?;

    Ok(s.chars().fold(x, |point, c| match c {
        'o' => point + 1,
        _ if point > 0 => point - 1,
        _ => point,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("3 0 xox", 0)]
    #[case("20 199999 oooooooooxoooooooooo", 200017)]
    #[case("20 10 xxxxxxxxxxxxxxxxxxxx", 0)]
    fn 正_atcoderの入出力例において成功する(
        #[case] input: &str,
        #[case] expected: usize,
    ) {
        let result = logic(input);

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
