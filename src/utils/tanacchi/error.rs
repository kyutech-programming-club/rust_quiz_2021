//! エラー周りの定義
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// 入力の数が不正のときのエラー
    #[error("number of input elems must be {0}")]
    InvalidNumOfInputElems(isize),

    /// 入力が不正のとき（メッセージ付き）
    #[error("number of input elems must be {0}")]
    InvalidInputError(&'static str),
}
