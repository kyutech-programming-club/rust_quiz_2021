#![allow(dead_code)]
use crate::utils::sagoj0_::parse_util::parse_from_iter;
use crate::utils::sagoj0_::{io_util::io_handler, parse_util::parse_to_vec};
use anyhow::Result;
use std::collections::HashSet;
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<&'static str> {
    let mut input = input.split_whitespace();
    let n: usize = parse_from_iter(&mut input)?;
    let a: Vec<usize> = parse_to_vec(&mut input, n)?;

    let &min = a.iter().min().unwrap();
    let &max = a.iter().max().unwrap();
    let len = a.into_iter().collect::<HashSet<usize>>().len();

    if min == 1 && max == n && len == n {
        Ok("Yes")
    } else {
        Ok("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("5 3 1 2 4 5", "Yes")]
    #[case("6 3 1 4 1 5 2", "No")]
    #[case("3 1 2 3", "Yes")]
    #[case("1 1", "Yes")]
    fn 正_atcoderの入出力例において成功する(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let result = logic(input);

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
