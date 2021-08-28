//! エラー周りの定義
use thiserror::Error;
use std::num::ParseIntError;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// 入力の数が不正のときのエラー
    #[error("number of input elems must be {0}")]
    InvalidNumOfInputElems(isize),

    /// 整数にパースできなかったときのエラー
    #[error("connot parse to integer")]
    ParseIntError(#[source] ParseIntError),
}
