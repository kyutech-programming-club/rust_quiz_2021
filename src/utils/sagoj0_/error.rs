//! rust_quizの実装で用いるエラーを定義する.
//! enumの列挙子は適宜追加する.

use std::fmt::{self, Debug};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum QuizSolveError {
    /// 入力が足りない際のエラー
    #[error("no input error")]
    LackOfInputOnParseError,

    /// 任意の値vに対しQuizSolveError::invalid_input_error(v, "hogehoge")で
    /// 不正な入力値の際のエラーを生成する
    #[error("Invalid input of `{value:?}`: {err_msg}")]
    InvalidInputError {
        value: QuizSolveErrorValue,
        err_msg: String,
    },
}

impl QuizSolveError {
    pub fn invalid_input_error<T>(value: T, err_msg: &str) -> Self
    where
        T: Debug + Send + Sync + 'static,
    {
        Self::InvalidInputError {
            value: QuizSolveErrorValue::new(value),
            err_msg: err_msg.to_owned(),
        }
    }
}

pub struct QuizSolveErrorValue {
    value: Box<dyn Debug + Send + Sync>,
}

impl QuizSolveErrorValue {
    fn new<T>(value: T) -> Self
    where
        T: Debug + Send + Sync + 'static,
    {
        Self {
            value: Box::new(value),
        }
    }
}

impl Debug for QuizSolveErrorValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
impl PartialEq for QuizSolveErrorValue {
    fn eq(&self, other: &QuizSolveErrorValue) -> bool {
        format!("{:?}", self.value) == format!("{:?}", other.value)
    }
}
