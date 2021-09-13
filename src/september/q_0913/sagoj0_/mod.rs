#![allow(dead_code)]
use crate::utils::sagoj0_::io_util::io_handler;
use crate::utils::sagoj0_::parse_util::{parse_from_iter, parse_to_tuple_vec};
use anyhow::{anyhow, Result};
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<isize> {
    let mut input = input.split_whitespace();
    let n: usize = parse_from_iter(&mut input)?;
    let x: isize = parse_from_iter(&mut input)?;
    let x = x * 100;

    parse_to_tuple_vec::<isize, isize, _>(&mut input, n)?
        .into_iter()
        .scan(0, |acc, (v, p)| {
            *acc += v * p;
            Some(*acc)
        })
        .enumerate()
        .find(|(_, sum)| *sum > x)
        .map_or_else(|| Some(-1), |(i, _)| Some(i as isize + 1_isize))
        .ok_or_else(|| anyhow!("unreachable"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("2 15 200 5 350 3", 2)]
    #[case("2 10 200 5 350 3", 2)]
    #[case("3 1000000 1000 100 1000 100 1000 100", -1)]
    fn 正_atcoderの入出力例において成功する(
        #[case] input: &str,
        #[case] expected: isize,
    ) {
        let result = logic(input);

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
