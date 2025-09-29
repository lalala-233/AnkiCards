mod chinese_sentence;
mod english_sentence;
mod prelude;
mod word;
pub const MAX_LENGTH_OF_ENGLISH_SENTENCE: usize = 80;
pub const MIN_LENGTH_OF_ENGLISH_SENTENCE: usize = 20;
pub fn check(checked_str: &str) -> Option<(usize, String)> {
    checked_str
        .lines()
        .enumerate()
        .find_map(|(count_of_line, line)| check_line(line).err().map(|s| (count_of_line, s)))
}
pub fn count_english_len(checked_str: &str) -> Vec<usize> {
    checked_str
        .lines()
        .filter_map(|line| extract_line(line))
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
fn check_line(line: &str) -> Result<(), String> {
    let Some(
        [
            _deck,
            word,
            _pronunciation,
            _chinese,
            english_sentence,
            chinese_sentence,
            _tag,
            _notetype,
        ],
    ) = extract_line(line)
    else {
        return Err('|'.to_string());
    };
    word::check_word(word)?;
    english_sentence::check_english_sentence(english_sentence)?;
    chinese_sentence::check_chinese_sentence(chinese_sentence)?;
    Ok(())
}
fn extract_line(line: &str) -> Option<[&str; 8]> {
    line.split('|').collect::<Vec<_>>().try_into().ok()
}
