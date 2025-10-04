mod chinese_sentence;
mod english_sentence;
pub mod error;
mod prelude;
mod pronunciation;
mod word;
use error::Error;
pub const SEPARATOR_NUMBER: usize = 8;
pub const MAX_LENGTH_OF_ENGLISH_SENTENCE: usize = 80;
pub const MIN_LENGTH_OF_ENGLISH_SENTENCE: usize = 20;
#[must_use]
pub fn check(checked_str: &str) -> Option<(usize, Error)> {
    checked_str
        .lines()
        .enumerate()
        .find_map(|(count_of_line, line)| check_line(line).err().map(|s| (count_of_line, s)))
}
#[must_use]
pub fn count_english_len(checked_str: &str) -> Vec<usize> {
    checked_str
        .lines()
        .flat_map(|line| extract_line(line))
        .map(
            |[
                _deck,
                _word,
                _pronunciation,
                _chinese,
                english_sentence,
                _chinese_sentence,
                _tag,
                _notetype,
            ]| { english_sentence.len() },
        )
        .collect()
}
fn check_line(line: &str) -> Result<(), Error> {
    let result = extract_line(line);
    let Ok(
        [
            _deck,
            word,
            pronunciation,
            _chinese,
            english_sentence,
            chinese_sentence,
            _tag,
            _notetype,
        ],
    ) = result
    else {
        return Err(Error::WrongSeparatorNumber(result.unwrap_err().len()));
    };
    word::check_word(word)?;
    pronunciation::check_pronunciation(pronunciation)?;
    english_sentence::check_english_sentence(english_sentence)?;
    chinese_sentence::check_chinese_sentence(chinese_sentence)?;
    Ok(())
}
fn extract_line(line: &str) -> Result<[&str; SEPARATOR_NUMBER], Vec<&str>> {
    line.split('|').collect::<Vec<_>>().try_into()
}
