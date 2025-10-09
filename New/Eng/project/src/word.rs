use super::prelude::*;
const VALID_SYMBOLS: &[char] = &['\'', '-', ' ', '(', ')', ',', '.', '?', '!', '/', '%'];
const VALID_END_WORD_CHAR: &[char] = &[')', '.', '?', '!'];
const VALID_COMBINATIONS: &[&str] = &[
    // before whitespace
    ") ", ", ", "' ", /* ones' something */
    // after whitespace
    " (", // special handle
    ". ", "..", " .",
];
pub fn check_word(word: &str) -> Result<(), String> {
    if word.is_empty() {
        return Err("Empty word".to_string());
    }
    if word.contains("  ") {
        return Err("Contains multiple spaces".to_string());
    }
    if !have_valid_ellipsis_if_present(word) {
        return Err("Invalid ellipsis `..` number".to_string());
    }
    if !check_symbol_followed_by_space_or_number(word) {
        return Err("Some symbol not followed by space or number".to_string());
    }
    find_invalid_word_char(word)?;
    find_invalid_start_char(word)?;
    find_invalid_end_char(word)?;
    find_invalid_symbol(word)?;
    find_alphabetic_adjacent_to_ascii_alphanumeric(word)?;
    Ok(())
}
fn find_invalid_symbol(word: &str) -> Result<(), String> {
    find_invalid_symbol_combination(word, VALID_SYMBOLS, VALID_COMBINATIONS)
        .map(|s| format!("Invalid symbol: {s}"))
        .map_or(Ok(()), Err)
}

fn find_invalid_word_char(word: &str) -> Result<(), String> {
    word.chars()
        .find(|&c| !is_valid_english_word_char(c))
        .map(|c| format!("Invalid char: {c}"))
        .map_or(Ok(()), Err)
}
fn is_valid_english_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || VALID_SYMBOLS.contains(&c)
}
fn find_invalid_start_char(word: &str) -> Result<(), String> {
    if word.starts_with(|c: char| c.is_ascii_alphanumeric()) {
        Ok(())
    } else {
        Err("Start with invalid char".to_string())
    }
}
fn find_invalid_end_char(word: &str) -> Result<(), String> {
    if word.ends_with(VALID_END_WORD_CHAR) || word.ends_with(|c: char| c.is_ascii_alphabetic()) {
        Ok(())
    } else {
        Err("End with invalid char".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        const VALID: &[&str] = &[
            "matter (2)", // ok because we support this
            "Here you are.",
        ];
        const INVALID: &[&str] = &[];
        for word in VALID {
            assert_eq!(check_word(word), Ok(()));
        }
        for &word in INVALID {
            assert!(check_word(word).is_err(), "sentence: {word}");
        }
    }
}
