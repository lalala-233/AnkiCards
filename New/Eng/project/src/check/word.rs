use super::prelude::*;
const VALID_WORD_SYMBOL: &[char] = &['\'', '-', ' ', '(', ')', ',', '.', '?', '!', '/', '%'];
const SPECIAL_HANDLE_COMBINATION: &[&str] = &[". ", " .", ".."];
const ALLOWED_COMBINATIONS: &[&str] = &[
    // before whitespace
    ") ", ", ", "' ", /* ones' something */
    // after whitespace
    " (", // special handle
    ". ", "..", " .",
];
pub fn check_word(word: &str) -> Result<(), String> {
    if word.chars().all(is_valid_english_word_char)
        && have_valid_combination(word, VALID_WORD_SYMBOL, ALLOWED_COMBINATIONS)
        && have_special_handle(word, SPECIAL_HANDLE_COMBINATION, "...")
        && is_starts_and_ends_with_valid_english_char(word)
    {
        return Ok(());
    }
    Err(word.to_string())
}
fn is_valid_english_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || VALID_WORD_SYMBOL.contains(&c)
}
