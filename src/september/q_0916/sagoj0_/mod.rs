#![allow(dead_code)]
use crate::utils::sagoj0_::io_util::io_handler;
use crate::utils::sagoj0_::parse_util::parse_from_iter;
use anyhow::Result;
use std::io;
use num::integer::gcd;

 fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<usize> {
    let mut input = input.split_whitespace();
    let k: usize = parse_from_iter(&mut input)?;

    let mut sum = 0;

    for i in 1..=k {
        for j in i..=k {
            for k in j..=k {
                sum += match gcd(i, gcd(j, k)) {
                    n if i == j && j == k => n,
                    n if i == j || j == k || k == i => n * 3,
                    n  => n * 6,
                }

            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    use rstest::rstest;

    #[rstest]
    #[case("2", 9)]
    #[case("200", 10813692)]
    fn 正_atcoderの入出力例において成功する(
        #[case] input: &str,
        #[case] expected: usize,
    ) {
        let result = logic(input);

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
