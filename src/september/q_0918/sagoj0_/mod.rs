#![allow(dead_code)]
use crate::utils::sagoj0_::io_util::io_handler;
use crate::utils::sagoj0_::parse_util::{parse_from_iter, parse_to_tuple_vec};
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
    let n: usize = parse_from_iter(&mut input)?;
    let ab: Vec<(usize, usize)> = parse_to_tuple_vec(&mut input, n)?;


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    use rstest::rstest;

    #[rstest]
    #[case("20 3 5 10 15", 10)]
    #[case("20 3 0 5 15", 10)]
    fn 正_atcoderの入出力例において成功する(
        #[case] input: &str,
        #[case] expected: usize,
    ) {
        let result = logic(input);

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
