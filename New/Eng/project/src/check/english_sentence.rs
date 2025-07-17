use super::prelude::*;
const VALID_SYMBOL: &[char] = &[
    '\"', '\'', '-', ' ', '(', ')', ',', '.', '?', '!', '/', '%', ':', ';', '$',
];
const VALID_END_WORD_CHAR: &[char] = &['.', '?', '!'];
const ALLOWED_COMBINATIONS: &[&str] = &[
    // before whitespace
    ": ", "; ", "% ", "! ", "? ", ") ", ", ", "' ", /* ones' something */
    // after whitespace
    " $", "\" ", " (", // special handle
    ". ", "..", " .",  // something else
    "\".", // something else check
    " \"",
];
const SPECIAL_HANDLE_COMBINATION: &[&str] = &[" .", ".."];
pub fn check_english_sentence(sentence: &str) -> Result<(), String> {
    if sentence.chars().all(is_valid_english_sentence_char)
        && have_valid_combination(sentence, VALID_SYMBOL, ALLOWED_COMBINATIONS)
        && have_special_handle(sentence, SPECIAL_HANDLE_COMBINATION, "...")
        && have_valid_quotation_mark(sentence)
        && have_appropriate_length(sentence)
        && is_starts_and_ends_with_valid_char(sentence)
        && basic_english_checks(sentence)
    {
        return Ok(());
    }
    Err(sentence.to_string())
}
const fn have_appropriate_length(checked_str: &str) -> bool {
    checked_str.len() >= 20 && checked_str.len() <= 80
}
fn have_valid_quotation_mark(checked_str: &str) -> bool {
    checked_str.matches('\"').count() % 2 == 0 && checked_str.matches("\"\"").count() == 0
}
fn is_valid_english_sentence_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || VALID_SYMBOL.contains(&c)
}
fn is_starts_and_ends_with_valid_char(checked_str: &str) -> bool {
    let start = checked_str.chars().next().unwrap();
    let end = checked_str.chars().last().unwrap();
    is_valid_sentence_start_char(start) && is_valid_sentence_end_char(end)
}

const fn is_valid_sentence_start_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
}
fn is_valid_sentence_end_char(c: char) -> bool {
    VALID_END_WORD_CHAR.contains(&c)
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
        ];
        for sentence in VALID {
            assert_eq!(check_english_sentence(sentence), Ok(()));
        }
        for sentence in INVALID {
            assert_eq!(
                check_english_sentence(sentence),
                Err((*sentence).to_string())
            );
        }
    }
}
