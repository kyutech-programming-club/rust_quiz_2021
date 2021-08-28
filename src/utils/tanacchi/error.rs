//! エラー周りの定義
use thiserror::Error;
use std::num::ParseIntError;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// 入力の数が不正のときのエラー
    #[error("lack of number of input elems")]
    LackOfInputElemsError,

    /// 整数にパースできなかったときのエラー
    #[error("connot parse to integer")]
    ParseIntError(#[source] ParseIntError),
}
