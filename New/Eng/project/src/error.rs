use crate::SEPARATOR_NUMBER;
use thiserror::Error;
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("have {0}/{SEPARATOR_NUMBER} separator `|`")]
    WrongSeparatorNumber(usize),
    #[error("{0}")]
    EnglishSentence(String),
    #[error("{0}")]
    ChineseSentence(String),
    #[error("{0}")]
    Pronunciation(String),
    #[error("{0}")]
    Word(String),
}
