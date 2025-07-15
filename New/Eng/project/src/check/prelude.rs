const VALID_END_WORD_CHAR: &[char] = &[',', '.', '?', '!'];
pub fn have_valid_combination(
    checked_str: &str,
    valid_symbol: &[char],
    allowed_combination: &[&str],
) -> bool {
    cartesian_product(valid_symbol, valid_symbol)
        .iter()
        .filter(|s| !allowed_combination.contains(&s.as_str()))
        .all(|invalid| !checked_str.contains(invalid))
}
pub fn have_special_handle(checked_str: &str, special_handle: &[&str], contains: &str) -> bool {
    if special_handle.iter().any(|s| checked_str.contains(s)) {
        return checked_str.contains(contains);
    }
    true
}
pub fn is_starts_and_ends_with_valid_english_char(checked_str: &str) -> bool {
    let start = checked_str.chars().next().unwrap();
    let end = checked_str.chars().last().unwrap();
    is_valid_english_start_char(start) && is_valid_english_end_char(end)
}
fn cartesian_product(chars_1: &[char], chars_2: &[char]) -> Vec<String> {
    chars_1
        .iter()
        .flat_map(|c1| chars_2.iter().map(move |c2| format!("{c1}{c2}")))
        .collect()
}
const fn is_valid_english_start_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
}
fn is_valid_english_end_char(c: char) -> bool {
    c.is_ascii_alphabetic() || VALID_END_WORD_CHAR.contains(&c)
}
