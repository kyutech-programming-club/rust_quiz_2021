use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Hash)]
pub enum QuizSolveError {
    #[error("no input error")]
    LackOfInputOnParseError,
}
