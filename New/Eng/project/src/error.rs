use crate::{SEPARATOR, SEPARATOR_NUMBER};
use thiserror::Error;
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("have {0}/{SEPARATOR_NUMBER} separator `{SEPARATOR}`")]
    WrongSeparatorNumber(usize),
    #[error("{0}")]
    EnglishSentence(String),
    #[error("{sentence}")]
    ChineseSentence { sentence: String, error: String },
    #[error("{0}")]
    Pronunciation(String),
    #[error("{0}")]
    Word(String),
}
impl Error {
    #[must_use]
    pub fn inner(&self) -> &str {
        match self {
            Self::WrongSeparatorNumber(_) => SEPARATOR,
            Self::EnglishSentence(_)
            | Self::ChineseSentence { .. }
            | Self::Pronunciation(_)
            | Self::Word(_) => todo!(),
        }
    }
}
