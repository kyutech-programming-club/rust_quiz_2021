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

pub fn try_parse<T>(s: &str) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: Sync + Send + error::Error + 'static,
{
    let s = s.trim_start().trim_end();
    ensure!(s != "", QuizSolveError::LackOfInputOnParseError);
    s.parse::<T>().map_err(|e| anyhow!(e))
}
