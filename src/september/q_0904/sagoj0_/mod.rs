#![allow(dead_code)]

use crate::utils::sagoj0_::io_util;
use crate::utils::sagoj0_::parse_util::parse_from_iter;
use anyhow::Result;
use rand::{thread_rng, Rng, RngCore};
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<usize> {
    let mut iter = input.split_whitespace();
    let n: usize = parse_from_iter(&mut iter)?;

    Ok(my_rand(n))
}

pub fn my_rand_interface(n: usize, rng: &mut impl RngCore) -> usize {
    rng.gen::<usize>() % n
}

fn my_rand(n: usize) -> usize {
    let mut rng = thread_rng();
    my_rand_interface(n, &mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sagoj0_::error::QuizSolveError::{self, LackOfInputOnParseError};
    use pretty_assertions::assert_eq;
    use rand::rngs::mock::StepRng;
    use rstest::rstest;

    #[rstest]
    #[case(100, 10, 10)]
    #[case(2, 3, 1)]
    #[case(100, 150, 50)]
    fn 正_乱数を生成する(#[case] n: usize, #[case] random: u64, #[case] expected: usize) {
        let mut rng = StepRng::new(random, 1);
        let result = my_rand_interface(n, &mut rng);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("")]
    #[case(" ")]
    fn 異_入力が足りなければエラーを返す(#[case] input: &str) {
        let result = logic(input);

        assert_eq!(result.is_err(), true);
        let error = result.unwrap_err();
        assert_error_match!(error, QuizSolveError, LackOfInputOnParseError);
    }
}
