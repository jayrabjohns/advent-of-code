use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use super::Question;

#[derive(Debug)]
pub struct QuestionError {
    pub message: String,
}

impl QuestionError {
    pub fn new(message: &str) -> Box<QuestionError> {
        Box::new(QuestionError {
            message: String::from(message),
        })
    }
}

impl Error for QuestionError {}

impl Display for QuestionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "QuestionError: {}", self.message)
    }
}
