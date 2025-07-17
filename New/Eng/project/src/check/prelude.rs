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
