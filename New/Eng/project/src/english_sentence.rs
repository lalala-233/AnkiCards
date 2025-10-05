use crate::prelude::*;
use crate::{MAX_LENGTH_OF_ENGLISH_SENTENCE, MIN_LENGTH_OF_ENGLISH_SENTENCE};
const VALID_SYMBOL: &[char] = &[
    '\"', '\'', '-', ' ', '(', ')', ',', '.', '?', '!', '/', '%', ':', ';', '$',
];
const VALID_END_CHAR: &[char] = &['.', '?', '!'];
const ALLOWED_COMBINATIONS: &[&str] = &[
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
    if !have_valid_ellipsis_if_present(sentence) {
        return Err("Invalid ellipsis `..` number".to_string());
    }
    if !have_valid_quotation_mark(sentence) {
        return Err("Invalid question mark `\"` number".to_string());
    }
    if !have_appropriate_length(sentence) {
        return Err(format!("Invalid length {}", sentence.len()));
    }
    if !check_symbol_followed_by_space_or_number(sentence) {
        return Err("some symbol not followed by space or number".to_string());
    }
    find_invalid_sentence_char(sentence)?;
    find_invalid_symbol(sentence)?;
    find_invalid_start_char(sentence)?;
    find_invalid_end_char(sentence)?;

    Ok(())
}
fn find_invalid_symbol(sentence: &str) -> Result<(), String> {
    find_invalid_symbol_combination(sentence, VALID_SYMBOL, ALLOWED_COMBINATIONS)
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
const fn have_appropriate_length(checked_str: &str) -> bool {
    checked_str.len() >= MIN_LENGTH_OF_ENGLISH_SENTENCE
        && checked_str.len() <= MAX_LENGTH_OF_ENGLISH_SENTENCE
}
fn have_valid_quotation_mark(checked_str: &str) -> bool {
    checked_str.matches('\"').count().is_multiple_of(2) && checked_str.matches("\"\"").count() == 0
}
fn is_valid_english_sentence_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || VALID_SYMBOL.contains(&c)
}

fn find_invalid_end_char(sentence: &str) -> Result<(), String> {
    let end = sentence.chars().last().unwrap();

    if VALID_END_CHAR.contains(&end) {
        Ok(())
    } else {
        Err(end.to_string())
    }
}
fn find_invalid_start_char(sentence: &str) -> Result<(), String> {
    let start = sentence.chars().next().unwrap();

    if start.is_ascii_alphanumeric() {
        Ok(())
    } else {
        Err(start.to_string())
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
