use crate::prelude::{find_empty, find_multiple_spaces};

/// 英语国际音标完整列表 (IPA)
pub const VALID_SYMBOL: &[&str] = &[
    // ========== 元音 Vowels ==========
    // 短元音 (Short vowels)
    "ɪ", // as in 'sit', 'bit'
    "ʊ", // as in 'put', 'book'
    "u", // as in 'do' (American short)
    "e", // as in 'bed', 'egg' (British)
    "ɛ", // as in 'bed', 'egg' (American)
    "ə", // as in 'about', 'sofa'
    "i", // as in 'happy', 'very'
    "æ", // as in 'cat', 'bad'
    "ʌ", // as in 'cup', 'luck'
    "ɒ", // as in 'hot', 'clock' (British)
    "ɑ", // as in 'hot', 'father' (American)
    // 长元音 (Long vowels)
    "iː", // as in 'see', 'tea'
    "uː", // as in 'food', 'do' (British)
    "ɑː", // as in 'father', 'car' (British)
    "ɔː", // as in 'saw', 'law'
    "ɜː", // as in 'bird', 'learn'
    // 双元音 (Diphthongs)
    "eɪ", // as in 'day', 'say'
    "aɪ", // as in 'eye', 'my'
    "ɔɪ", // as in 'boy', 'toy'
    "aʊ", // as in 'now', 'cow'
    "əʊ", // as in 'go', 'row' (British)
    "oʊ", // as in 'go', 'row' (American)
    "ɪə", // as in 'near', 'beer'
    "eə", // as in 'hair', 'care'
    "ʊə", // as in 'pure', 'tour',
    // ========== 辅音 Consonants ==========
    "p",  // as in 'pen', 'top'
    "b",  // as in 'big', 'rub'
    "t",  // as in 'tea', 'cat'
    "d",  // as in 'dog', 'bed'
    "k",  // as in 'cat', 'key'
    "ɡ",  // as in 'go', 'egg',
    "f",  // as in 'fish', 'off'
    "v",  // as in 'voice', 'have'
    "θ",  // as in 'think', 'bath'
    "ð",  // as in 'this', 'father'
    "s",  // as in 'see', 'miss'
    "z",  // as in 'zoo', 'buzz'
    "ʃ",  // as in 'she', 'wish'
    "ʒ",  // as in 'vision', 'measure'
    "h",  // as in 'hot', 'hat',
    "tʃ", // as in 'chip', 'watch'
    "dʒ", // as in 'job', 'badge',
    "m",  // as in 'man', 'ham'
    "n",  // as in 'now', 'ten'
    "ŋ",  // as in 'sing', 'long',
    "l",  // as in 'leg', 'ball'
    "r",  // as in 'red', 'car'
    "j",  // as in 'yes', 'you'
    "w",  // as in 'wet', 'window',
    // ========== 超音段特征 ==========
    "ˈ", // 主重音 (Primary stress)
    "ˌ", // 次重音 (Secondary stress)
    "ː", // 长音符号 (Length mark)
    ".", // 音节分隔 (Syllable break)
    // ========== 其他 Symbol ==========
    "英[", " 美[", "]", "(", ")", ";", " ",
];

const START_STR: &[&str] = &["英[", "美["];
const END_STR: &str = "]";

pub fn check_pronunciation(pronunciation: &str) -> Result<(), String> {
    if find_empty(pronunciation).is_err() {
        // Allow empty
        return Ok(());
    }
    find_multiple_spaces(pronunciation)?;
    if !have_balanced_symbol(pronunciation) {
        return Err("Unbalanced symbol".to_string());
    }
    find_invalid_start(pronunciation)?;
    find_invalid_end(pronunciation)?;
    find_invalid_symbol(pronunciation)?;
    // find_alphabetic_adjacent_to_ascii_alphanumeric(pronunciation)?; // cannot use because SYMBOLS are not ASCII

    Ok(())
}
fn find_invalid_start(pronunciation: &str) -> Result<(), String> {
    if START_STR
        .iter()
        .any(|prefix| pronunciation.starts_with(prefix))
    {
        Ok(())
    } else {
        Err("Invalid start".to_string())
    }
}
fn find_invalid_end(pronunciation: &str) -> Result<(), String> {
    if pronunciation.ends_with(END_STR) {
        Ok(())
    } else {
        Err("Invalid end".to_string())
    }
}

fn have_balanced_symbol(checked_str: &str) -> bool {
    checked_str.matches('[').count() == checked_str.matches(']').count()
        && (checked_str.matches('(').count() == checked_str.matches(')').count())
}
fn find_invalid_symbol(checked_str: &str) -> Result<(), String> {
    if checked_str.is_empty() {
        Ok(())
    } else {
        VALID_SYMBOL
            .iter()
            .find_map(|sym| checked_str.strip_prefix(sym))
            .map_or_else(|| Err(checked_str.to_string()), find_invalid_symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        const VALID: &[&str] = &[
            "",
            "英[dɪˈvaɪz] 美[dɪˈvaɪz]",
            "英[tiːz] 美[tiːz]",
            "英[fjuːm] 美[fjuːm]",
            "英[fəˈtiːɡ] 美[fəˈtiːɡ]",
            "英[snɪə(r)] 美[snɪr]",
            "英[əˈplaɪəns] 美[əˈplaɪəns]",
            "英[wɒt] 美[wɑːt]",
            "英[rəʊ] 美[roʊ]",
            "英[juː;jʊ] 美[juː;jʊ]",
            "英[eɡ] 美[ɛɡ]",
            "英[duː] 美[du]",
            "英[ˈveri] 美[ˈveri]",
            "英[æt skuːl] 美[æt skʊl]",
            "英[ˈkʌm ɒn]",
        ];
        const INVALID: &[&str] = &[
            "英[dɪ'vaɪz] 美[dɪ'vaɪz]",                // for invalid `'`
            "英[du: wel ɪn] 美[du: wel ɪn]",          // for invalid `:`
            "英[ˈkʌmˌɒn, -ˌɔːn] 美[ˈkʌmˌɑːnˌ -ˌɔːn]", // for invalid `-`
            "英[ˈtəukjəu] 美[ˈtokjo]",                // for invalid `o`
            "英[ɒˈstreɪlɪə] 美[ɔˈstreljə]",           // for invalid `ɔ`
            "英[ˈtiː,ʃɜːt] 美[ˈti,ʃɜːrt]",            // for invalid `,`
        ];
        for &pronunciation in VALID {
            assert_eq!(check_pronunciation(pronunciation), Ok(()));
        }
        for &pronunciation in INVALID {
            assert!(
                check_pronunciation(pronunciation).is_err(),
                "{pronunciation}"
            );
        }
    }
}
