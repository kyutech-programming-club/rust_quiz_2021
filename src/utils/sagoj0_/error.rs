use std::fmt;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum QuizSolveError {
    #[error("no input error")]
    LackOfInputOnParseError,
    #[error("Invalid input of {value:?}: {err_msg}")]
    InvalidInputError {
        value: QuizSolveErrorValue,
        err_msg: String,
    },
}

pub struct QuizSolveErrorValue {
    pub value: Box<dyn Debug + Send + Sync>,
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
