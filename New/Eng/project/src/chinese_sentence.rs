use crate::prelude::*;

const VALID_SYMBOLS: &[char] = &[
    '，', '。', '：', '！', '？', '《', '》', '…', ' ', '—', '、', '「', '」', '·', '.',
];
const VALID_CHINESE_START_CHAR: &[char] = &['《', '「'];
const VALID_CHINESE_END_CHAR: &[char] = &['。', '？', '！', '…', '」'];
const VALID_COMBINATIONS: &[&str] = &["……", "——", "」。", "。」", ".」"];
pub fn check_chinese_sentence(sentence: &str) -> Result<(), String> {
    if sentence.is_empty() {
        return Err("Empty sentence".to_string());
    }
    if sentence.contains("  ") {
        return Err("Contains multiple spaces".to_string());
    }
    find_invalid_sentence_char(sentence)?;
    find_invalid_symbol(sentence)?;
    find_invalid_start_char(sentence)?;
    find_invalid_end_char(sentence)?;

    Ok(())
}
fn find_invalid_sentence_char(sentence: &str) -> Result<(), String> {
    sentence
        .chars()
        .find(|&c| !is_valid_sentence_char(c))
        .map(|c| format!("Invalid char: {c}"))
        .map_or(Ok(()), Err)
}
fn find_invalid_end_char(sentence: &str) -> Result<(), String> {
    let end = sentence.chars().last().unwrap();

    if VALID_CHINESE_END_CHAR.contains(&end) {
        Ok(())
    } else {
        Err(end.to_string())
    }
}
fn find_invalid_start_char(sentence: &str) -> Result<(), String> {
    if sentence.starts_with(VALID_CHINESE_START_CHAR)
        || sentence.starts_with(|c: char| c.is_alphanumeric())
    {
        Ok(())
    } else {
        Err("End with invalid char".to_string())
    }
}
fn is_valid_sentence_char(c: char) -> bool {
    c.is_alphanumeric() || VALID_SYMBOLS.contains(&c)
}
fn find_invalid_symbol(sentence: &str) -> Result<(), String> {
    find_invalid_symbol_combination(
        sentence,
        VALID_SYMBOLS,
        VALID_COMBINATIONS,
    )
    .map(|s| format!("Invalid symbol: {s}"))
    .map_or(Ok(()), Err)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        const VALID: &[&str] = &[
            "所谓「成功的诀窍」是不存在的。",
            "羽毛很轻，因此我们说「轻如鸿毛」。",
            "亚伯拉罕·林肯在美国废除了奴隶制。",
            "《钢琴家》是第 4 部获得最佳影片提名的电影。",
            "在正式写作中，你可以将「Example」缩写为「e.g.」。",
        ];
        const INVALID: &[&str] = &[
            "人行道/路面上不许停车。",                // for `/``
            "演讲者向主席道了谢，（会议）就结束了。", // for `（` and `）`
            "火车正快速驶向下一个车站。 ",            // for ` `
            " 我正要离开，就在这时电话响了。",        // for ` `
            "人民大会堂于 1959 年 9 月对外开放，",    // for `，`
            "他们 35 年的婚姻生活一直保持着浪漫色彩", // for missing `。`
            #[allow(clippy::invisible_characters)]
            "我们对宇宙了解得越多，产生的问题也就越多。​", // for invisible character \u{200B}
        ];
        for sentence in VALID {
            assert_eq!(check_chinese_sentence(sentence), Ok(()));
        }
        for &sentence in INVALID {
            assert!(
                check_chinese_sentence(sentence).is_err(),
                "sentence: {sentence}"
            );
            //TODO: check error message
        }
    }
}
