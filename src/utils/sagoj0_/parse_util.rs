use crate::utils::sagoj0_::error::QuizSolveError;
use anyhow::{bail, Result};
use std::iter::Iterator;
use std::{error, str::FromStr};

pub fn parse<'a, T, I>(iter: &mut I) -> Result<T>
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
