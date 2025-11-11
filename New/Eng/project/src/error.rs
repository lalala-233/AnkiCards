use crate::{SEPARATOR, SEPARATOR_NUMBER};
use thiserror::Error;
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("have {0}/{SEPARATOR_NUMBER} separator `{SEPARATOR}`")]
    WrongSeparatorNumber(usize),
    #[error("{sentence}")]
    EnglishSentence { sentence: String, error: String },
    #[error("{sentence}")]
    ChineseSentence { sentence: String, error: String },
    #[error("{pronunciation}")]
    Pronunciation {
        pronunciation: String,
        error: String,
    },
    #[error("{word}")]
    Word { word: String, error: String },
}
impl Error {
    #[must_use]
    pub fn error(&self) -> &str {
        match self {
            Self::WrongSeparatorNumber(_) => SEPARATOR,
            Self::EnglishSentence { error, .. }
            | Self::ChineseSentence { error, .. }
            | Self::Pronunciation { error, .. }
            | Self::Word { error, .. } => error,
        }
    }
}
