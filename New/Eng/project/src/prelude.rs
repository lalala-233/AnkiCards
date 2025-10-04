const ENGLISH_CHARS_FOLLOWED_BY_SPACE: &[char] = &[';'];
const ENGLISH_CHARS_FOLLOWED_BY_SPACE_OR_NUMBER: &[char] = &[',', ':'];

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

fn cartesian_product(chars_1: &[char], chars_2: &[char]) -> Vec<String> {
    chars_1
        .iter()
        .flat_map(|c1| chars_2.iter().map(move |c2| format!("{c1}{c2}")))
        .collect()
}

pub fn basic_english_checks(checked_str: &str) -> bool {
    let left_chars = checked_str.chars();
    let right_chars = checked_str.chars().skip(1);
    left_chars.zip(right_chars).all(basic_english_check)
}
fn basic_english_check(left_and_right: (char, char)) -> bool {
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
