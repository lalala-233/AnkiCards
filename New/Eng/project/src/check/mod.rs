mod chinese_sentence;
mod english_sentence;
mod prelude;
mod word;
pub fn check(checked_str: &str) -> Option<(usize, String)> {
    checked_str.lines()
        .enumerate()
        .find_map(|(count_of_line, line)| check_line(line).err().map(|s| (count_of_line, s)))
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
    let mut iter = line.split('|');
    let result = [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ];
    if iter.next().is_some() {
        None
    } else {
        Some(result)
    }
}
