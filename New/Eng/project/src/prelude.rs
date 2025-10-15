const ENGLISH_CHARS_FOLLOWED_BY_SPACE: &[char] = &[';'];
const SHOULD_BE_FOLLOWED_BY_SPACE_OR_NUMBER: &[char] = &[
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
pub fn find_specific_symbol_not_followed_by_space_or_number(
    checked_str: &str,
) -> Result<(), String> {
    find_from_adjacent_text(checked_str, specific_symbol_not_followed_by_space_or_number)
        .map(|(left, right)| {
            format!(
                "Some symbol not followed by space or number (left: `{left}`, right: `{right}`)"
            )
        })
        .map_or(Ok(()), Err)
}
pub fn find_alphabetic_adjacent_to_ascii_alphanumeric(sentence: &str) -> Result<(), String> {
    find_from_adjacent_text(sentence, alphabetic_adjacent_to_ascii_alphanumeric).map(|(left,right)|format!("Invalid alphabetic adjacent to ascii alphanumeric (left: `{left}`, right: `{right}`)"))
        .map_or(Ok(()),  Err)
}
pub fn find_alphabetic_adjacent_to_left_parenthesis(sentence: &str) -> Result<(), String> {
    find_from_adjacent_text(sentence, alphabetic_adjacent_to_left_parenthesis)
        .map(|(left, right)| {
            format!("Must have space between alphabetic and left parenthesis at `{left}{right}`")
        })
        .map_or(Ok(()), Err)
}
fn alphabetic_adjacent_to_ascii_alphanumeric(left: char, right: char) -> bool {
    let is_left_only_alphabetic = left.is_alphabetic() && !left.is_ascii_alphanumeric();
    let is_right_only_alphabetic = right.is_alphabetic() && !right.is_ascii_alphanumeric();
    (is_left_only_alphabetic && right.is_ascii_alphanumeric())
        || (is_right_only_alphabetic && left.is_ascii_alphanumeric())
}
fn alphabetic_adjacent_to_left_parenthesis(left: char, right: char) -> bool {
    left.is_alphabetic() && right == '('
}
fn is_specific_symbol_followed_by_space_or_number(left: char, right: char) -> bool {
    if SHOULD_BE_FOLLOWED_BY_SPACE_OR_NUMBER.contains(&left) {
        right == ' ' || right.is_numeric()
    } else {
        true
    }
}
fn is_specific_symbol_followed_by_space(left: char, right: char) -> bool {
    if ENGLISH_CHARS_FOLLOWED_BY_SPACE.contains(&left) {
        right == ' '
    } else {
        true
    }
}
fn specific_symbol_not_followed_by_space_or_number(left: char, right: char) -> bool {
    !is_specific_symbol_followed_by_space_or_number(left, right)
        || !is_specific_symbol_followed_by_space(left, right)
}
fn find_from_adjacent_text(
    sentence: &str,
    predicate: impl Fn(char, char) -> bool,
) -> Option<(char, char)> {
    let left_chars = sentence.chars();
    let right_chars = sentence.chars().skip(1);
    left_chars
        .zip(right_chars)
        .find(|&(left, right)| predicate(left, right))
}
