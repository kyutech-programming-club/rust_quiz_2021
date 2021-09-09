#![allow(dead_code)]

use crate::utils::sagoj0_::{io_util, parse_util::parse_from_iter};
use anyhow::Result;
use rand::seq::SliceRandom;
use rand::RngCore;
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<Vec<usize>> {
    let mut iter = input.split_whitespace();
    let n: usize = parse_from_iter(&mut iter)?;

    Ok(shuffle(n))
}

fn shuffle(n: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    shuffle_interface(n, &mut rng)
}

fn shuffle_interface(n: usize, rng: &mut impl RngCore) -> Vec<usize> {
    let mut elements = (0..n).collect::<Vec<usize>>();
    elements.shuffle(rng);
    elements
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rand::rngs::mock::StepRng;
    use rstest::rstest;

    #[rstest]
    #[case(100)]
    #[case(2)]
    #[case(50)]
    fn 正_乱数を生成する(#[case] n: usize) {
        let mut rng = StepRng::new(0, 1);
        let expected = (0..n).map(|x| (x + 1) % n).collect::<Vec<usize>>();
        let result = shuffle_interface(n, &mut rng);

        assert_eq!(result, expected);
    }
}
