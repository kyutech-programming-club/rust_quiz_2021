#![allow(dead_code)]

use super::super::q_0902::sagoj0_::power;
use crate::utils::sagoj0_::error::QuizSolveError::OverflowError;
use crate::utils::sagoj0_::{io_util, parse_util::parse_from_iter};
use anyhow::{anyhow, Result};
use std::io;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io_util::io_handler(&mut stdin, &mut stdout, logic)
}

fn logic(input: &str) -> Result<bool> {
    let mut iter = input.split_whitespace();
    let n: usize = parse_from_iter(&mut iter)?;

    let result = powering_sum(n)?;
    let expected = fifth_power_sum_formula(n)?;
    Ok(result == expected as isize)
}

fn powering_sum(n: usize) -> Result<isize> {
    (1..=n)
        .map(|i| power(i as isize, 5))
        .collect::<Result<Vec<isize>>>()?
        .into_iter()
        .try_fold(0_isize, |acc, x| acc.checked_add(x))
        .ok_or(anyhow!(OverflowError("overflow at sum".to_owned())))
}

fn fifth_power_sum_formula(n: usize) -> Result<usize> {
    use crate::utils::sagoj0_::monad::{Applicative, Monad};

    (mdo! {
        square <= Some(n).and_then(|x| x.checked_pow(2)), // n^2
        add1_square <= Some(n + 1).bind(|x| x.checked_pow(2)), // (n+1)^2
        //  2n^2 + 2n - 1 => 2n(n+1) - 1
        n2_nadd1_sub1 <=Option::pure(n).bind(|x| x.checked_mul(2)) // 2n
                     .bind(|x| x.checked_mul(n+1)) // 2n(n+1)
                     .bind(|x| Some(x-1)), // 2n(n+1)-1
        sq_mul_add1sq <= square.checked_mul(add1_square), // n^2*(n+1)^2
        // n^2*(n+1)^2は4の倍数
        res <= n2_nadd1_sub1.checked_mul(sq_mul_add1sq / 4), // n^2*(n+1)^2/4*2n(n+1)-1
        Option::pure(res / 3)
    })
    .ok_or(anyhow!(OverflowError("overflow at formula".to_owned())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sagoj0_::error::QuizSolveError::{self, LackOfInputOnParseError};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("5")]
    #[case("30")]
    #[case("1824")]
    fn 正_5乗の和の公式の結果がq_0902のpower関数の結果と一致する(
        #[case] input: &str,
    ) {
        let result = logic(input);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), true);
    }

    // 最大値の少なくとも3倍を扱う関係上公式を利用した方が早くオーバーフローする
    #[rstest]
    #[case("1825", "overflow at formula")]
    #[case("3000", "overflow at sum")]
    #[case("20000000", "overflow at power")]
    fn 異_オーバーフローを検知する(#[case] input: &str, #[case] err_msg: &str) {
        let result = logic(input);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_error_match!(
            error,
            QuizSolveError,
            QuizSolveError::OverflowError(err_msg.to_owned())
        );
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
