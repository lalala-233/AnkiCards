pub use crate::error::Error;
const ENGLISH_CHARS_FOLLOWED_BY_SPACE: &[char] = &[';'];
const ENGLISH_CHARS_FOLLOWED_BY_SPACE_OR_NUMBER: &[char] = &[
    ',', // $5,000
    ':', // 5:00
];
pub fn find_invalid_symbol_combination(
    checked_str: &str,
    symbols: &[char],
    valid_combinations: &[&str],
) -> Option<String> {
    cartesian_product(symbols, symbols)
        .into_iter()
        .filter(|s| !valid_combinations.contains(&s.as_str()))
        .find(|invalid| checked_str.contains(invalid))
}
pub fn have_valid_ellipsis_if_present(checked_str: &str) -> bool {
    checked_str.matches("..").count() == checked_str.matches("...").count()
}

fn cartesian_product(chars_1: &[char], chars_2: &[char]) -> Vec<String> {
    chars_1
        .iter()
        .flat_map(|c1| chars_2.iter().map(move |c2| format!("{c1}{c2}")))
        .collect()
}

pub fn check_symbol_followed_by_space_or_number(checked_str: &str) -> bool {
    let left_chars = checked_str.chars();
    let right_chars = checked_str.chars().skip(1);
    left_chars
        .zip(right_chars)
        .all(symbol_followed_by_space_or_number)
}
fn symbol_followed_by_space_or_number(left_and_right: (char, char)) -> bool {
    is_followed_by_space_or_number(left_and_right) && is_followed_by_space(left_and_right)
}
fn is_followed_by_space_or_number((left, right): (char, char)) -> bool {
    if ENGLISH_CHARS_FOLLOWED_BY_SPACE_OR_NUMBER.contains(&left) {
        right == ' ' || right.is_numeric()
    } else {
        true
    }
}
fn is_followed_by_space((left, right): (char, char)) -> bool {
    if ENGLISH_CHARS_FOLLOWED_BY_SPACE.contains(&left) {
        right == ' '
    } else {
        true
    }
}
