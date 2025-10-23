use crate::prelude::*;
use crate::{MAX_LENGTH_OF_ENGLISH_SENTENCE, MIN_LENGTH_OF_ENGLISH_SENTENCE};
const MAX_WORDS_IN_SENTENCE: usize = 15;
const MIN_WORDS_IN_SENTENCE: usize = 5;
const VALID_SYMBOLS: &[char] = &[
    '\"', '\'', '-', ' ', '(', ')', ',', '.', '?', '!', '/', '%', ':', ';', '$',
];
const VALID_END_CHAR: &[char] = &['.', '?', '!'];
const VALID_COMBINATIONS: &[&str] = &[
    // before whitespace
    ": ", "; ", "% ", "! ", "? ", ") ", ", ", // placeholder for cargo fmt
    "' ", // as in `teachers' day`
    // after whitespace
    " $", "\" ", " (", // placeholder for cargo fmt
    // special check `...`
    ". ", "..", " .", // placeholder for cargo fmt
    // `"` should be matched
    "\".", ".\"", " \"",
];
pub fn check_english_sentence(sentence: &str) -> Result<(), String> {
    if sentence.is_empty() {
        return Err("Empty sentence".to_string());
    }
    if sentence.contains("  ") {
        return Err("Contains multiple spaces".to_string());
    }
    if !have_valid_ellipsis_if_present(sentence) {
        return Err("Invalid ellipsis `..` number".to_string());
    }
    if !have_valid_quotation_mark(sentence) {
        return Err("Invalid question mark `\"` number".to_string());
    }
    if !have_appropriate_length(sentence) {
        return Err(format!("Invalid length {}", sentence.len()));
    }

    find_invalid_start_char(sentence)?;
    find_invalid_end_char(sentence)?;
    find_invalid_sentence_char(sentence)?;
    find_invalid_symbol(sentence)?;
    find_alphabetic_adjacent_to_ascii_alphanumeric(sentence)?;
    find_alphabetic_adjacent_to_left_parenthesis(sentence)?;
    find_specific_symbol_not_followed_by_space_or_number(sentence)?;

    Ok(())
}
fn find_invalid_symbol(sentence: &str) -> Result<(), String> {
    find_invalid_symbol_combination(sentence, VALID_SYMBOLS, VALID_COMBINATIONS)
        .map(|s| format!("Invalid symbol: {s}"))
        .map_or(Ok(()), Err)
}
fn find_invalid_sentence_char(sentence: &str) -> Result<(), String> {
    sentence
        .chars()
        .find(|&c| !is_valid_english_sentence_char(c))
        .map(|c| format!("Invalid char: {c}"))
        .map_or(Ok(()), Err)
}
fn have_appropriate_length(checked_str: &str) -> bool {
    let length = checked_str.len();
    let word_count = checked_str.split(' ').count();
    (MIN_LENGTH_OF_ENGLISH_SENTENCE..=MAX_LENGTH_OF_ENGLISH_SENTENCE).contains(&length)
        || (MIN_WORDS_IN_SENTENCE..=MAX_WORDS_IN_SENTENCE).contains(&word_count)
}
fn have_valid_quotation_mark(checked_str: &str) -> bool {
    checked_str.matches('\"').count().is_multiple_of(2) && checked_str.matches("\"\"").count() == 0
}
fn is_valid_english_sentence_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || VALID_SYMBOLS.contains(&c)
}

fn find_invalid_end_char(sentence: &str) -> Result<(), String> {
    if sentence.ends_with(VALID_END_CHAR) {
        Ok(())
    } else {
        Err("End with invalid char".to_string())
    }
}
fn find_invalid_start_char(sentence: &str) -> Result<(), String> {
    if sentence.starts_with(|c: char| c.is_ascii_alphanumeric()) {
        Ok(())
    } else {
        Err("Start with invalid char".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        const VALID: &[&str] = &[
            "The 6:30 train for Berlin has been canceled.",
            "She toured the U.S. when she was fourteen.",
            "How do you do? I do not think we have met.",
            "Ouch! You hit my crazy bone.",
            "A 10% service charge is added on to the bill.",
            "He always keeps his promises.",
            "Don't laugh at him; he's very sensitive.",
            "There is no so-called \"recipe for success\".",
            "I value this necklace at $5,000.",
            "English has five main vowel letters: A, E, I, O, U.",
            "You can abbreviate \"Example\" to \"e.g.\" in formal writing.",
            "Let me see... where did I leave my hat?",
            "The company is committed to producing high-quality yet affordable smartphones for the global market.", // long but words count is valid
        ];
        const INVALID: &[&str] = &[
            "If I had the time, I 'd make something better.", // for ` '`
            "\"Feather is very light, so that we say \"as light as a feather\".", // for extra `"`
            "Go on doing sth., Go on to do sth.",             // for `.,`
            "Allocate rations for a week - long camping trip.", // for `-`
            "These are my books.",                            // too short
            "If a horse refuses a jump,penalty points are added to the score.", // for `,` without following space
            "Some of them--alas--will never return.",                           // for `--`
            "Late last year an allied Taliban faction tried to seize large tracts of the Swat valley in the North-West Frontier Province.", // too long
            "Let me see.. where did I leave my hat?", // for `..`
        ];
        for sentence in VALID {
            assert_eq!(check_english_sentence(sentence), Ok(()));
        }
        for &sentence in INVALID {
            assert!(
                check_english_sentence(sentence).is_err(),
                "sentence: {sentence}"
            );
        }
    }
}
