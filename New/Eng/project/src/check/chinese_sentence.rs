use crate::check::prelude::have_valid_combination;

const VALID_CHINESE_SENTENCE_CHAR: &[char] = &[
    '，', '。', '：', '！', '？', '《', '》', '…', ' ', '—', '、', '「', '」', '·', '.',
];
const VALID_CHINESE_START_CHAR: &[char] = &['《', '「'];
const VALID_CHINESE_END_CHAR: &[char] = &['。', '？', '！', '…', '」'];
const ALLOWED_CHINESE_COMBINATION: &[&str] = &["……", "——", "」。", "。」", ".」"];
pub fn check_chinese_sentence(sentence: &str) -> Result<(), String> {
    if sentence.chars().all(is_valid_sentence_char)
        && is_valid_str(sentence)
        && is_starts_and_ends_with_valid_char(sentence)
    {
        return Ok(());
    }
    Err(sentence.to_string())
}
fn is_starts_and_ends_with_valid_char(checked_str: &str) -> bool {
    let start = checked_str.chars().next().unwrap();
    let end = checked_str.chars().last().unwrap();
    is_valid_start_char(start) && is_valid_end_char(end)
}
fn is_valid_end_char(c: char) -> bool {
    VALID_CHINESE_END_CHAR.contains(&c)
}
fn is_valid_start_char(c: char) -> bool {
    c.is_alphanumeric() || VALID_CHINESE_START_CHAR.contains(&c)
}
fn is_valid_sentence_char(c: char) -> bool {
    c.is_alphanumeric() || VALID_CHINESE_SENTENCE_CHAR.contains(&c)
}
fn is_valid_str(checked_str: &str) -> bool {
    have_valid_combination(
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
        ];
        for sentence in VALID {
            assert_eq!(check_chinese_sentence(sentence), Ok(()));
        }
        for sentence in INVALID {
            assert_eq!(
                check_chinese_sentence(sentence),
                Err((*sentence).to_string())
            );
        }
    }
}
