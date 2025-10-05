use crate::prelude::*;

const VALID_CHINESE_SENTENCE_CHAR: &[char] = &[
    '，', '。', '：', '！', '？', '《', '》', '…', ' ', '—', '、', '「', '」', '·', '.',
];
const VALID_CHINESE_START_CHAR: &[char] = &['《', '「'];
const VALID_CHINESE_END_CHAR: &[char] = &['。', '？', '！', '…', '」'];
const ALLOWED_CHINESE_COMBINATION: &[&str] = &["……", "——", "」。", "。」", ".」"];
pub fn check_chinese_sentence(sentence: &str) -> Result<(), String> {
    find_invalid_sentence_char(sentence)?;

    find_invalid_symbol(sentence)
        .map(|s| format!("Invalid symbol: {s}"))
        .map_or(Ok(()), Err)?;

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
fn find_invalid_end_char(checked_str: &str) -> Result<(), String> {
    let end = checked_str.chars().last().unwrap();

    if VALID_CHINESE_END_CHAR.contains(&end) {
        Ok(())
    } else {
        Err(end.to_string())
    }
}
fn find_invalid_start_char(checked_str: &str) -> Result<(), String> {
    let start = checked_str.chars().next().unwrap();

    if start.is_alphanumeric() || VALID_CHINESE_START_CHAR.contains(&start) {
        Ok(())
    } else {
        Err(start.to_string())
    }
}
fn is_valid_sentence_char(c: char) -> bool {
    c.is_alphanumeric() || VALID_CHINESE_SENTENCE_CHAR.contains(&c)
}
fn find_invalid_symbol(checked_str: &str) -> Option<String> {
    have_valid_symbol_combination(
        checked_str,
        VALID_CHINESE_SENTENCE_CHAR,
        ALLOWED_CHINESE_COMBINATION,
    )
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
            assert!(check_chinese_sentence(sentence).is_err());
            //TODO: check error message
        }
    }
}
