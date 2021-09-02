//! 文字列のパースに関するユーティリティを提供する．

use crate::utils::sagoj0_::error::QuizSolveError;
use anyhow::{anyhow, bail, ensure, Result};
use std::iter::Iterator;
use std::{error, str::FromStr};

/// &strのイテレータを受け取り，それを消費して任意の値にパースする．
pub fn parse_from_iter<'a, T, I>(iter: &mut I) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: Sync + Send + error::Error + 'static,
    I: Iterator<Item = &'a str>,
{
    if let Some(s) = iter.next() {
        Ok(s.parse::<T>()?)
    } else {
        bail!(QuizSolveError::LackOfInputOnParseError)
    }
}

/// parseを行うもしくはparseが行えるかどうか判定を行う．
/// 空入力の際にLackOfInputOnParseErrorを返却する他，
/// 各Parse時のエラーも適宜返却する．
pub fn try_parse<T>(s: &str) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: Sync + Send + error::Error + 'static,
{
    let s = s.trim_start().trim_end();
    ensure!(!s.is_empty(), QuizSolveError::LackOfInputOnParseError);
    s.parse::<T>().map_err(|e| anyhow!(e))
}

/// 指定した長さ`len`分だけイテレータを消費し
/// その長さのvecを返す
pub fn parse_to_vec<'a, T, I>(iter: &mut I, len: usize) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Sync + Send + error::Error + 'static,
    I: Iterator<Item = &'a str>,
{
    (0..len)
        .map(|_| parse_from_iter(iter))
        .collect::<Result<Vec<T>>>()
}
