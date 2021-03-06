#![allow(dead_code)]
use crate::utils::sagoj0_::io_util::io_handler;
use crate::utils::sagoj0_::parse_util::{parse_from_iter, parse_to_vec};
use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<usize> {
    let mut input = input.split_whitespace();
    let h: usize = parse_from_iter(&mut input)?;
    let w: usize = parse_from_iter(&mut input)?;

    let a: Vec<usize> = parse_to_vec(&mut input, h * w)?;
    let &min = a.iter().min().unwrap();
    Ok(a.into_iter().map(|x| x - min).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("2 3 2 2 3 3 2 2", 2)]
    #[case("3 3 99 99 99 99 0 99 99 99 99", 792)]
    #[case("3 2 4 4 4 4 4 4", 0)]
    fn 正_atcoderの入出力例において成功する(
        #[case] input: &str,
        #[case] expected: usize,
    ) {
        let result = logic(input);

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
