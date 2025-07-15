use crate::check::prelude::have_valid_combination;

const VALID_CHINESE_SENTENCE_CHARS: &[char] = &[
    '，', '。', '：', '！', '？', '《', '》', '…', ' ', '—', '、', '「', '」',
];
const ALLOWED_CHINESE_COMBINATION: &[&str] = &["……", "——", "」。", "。」"];
pub fn check_chinese_sentence(sentence: &str) -> Result<(), String> {
    if sentence.chars().all(is_invalid_sentence_char) && is_valid_str(sentence) {
        return Ok(());
    }
    Err(sentence.to_string())
}
fn is_invalid_sentence_char(c: char) -> bool {
    c.is_alphanumeric() || VALID_CHINESE_SENTENCE_CHARS.contains(&c)
}
fn is_valid_str(checked_str: &str) -> bool {
    have_valid_combination(
        checked_str,
        VALID_CHINESE_SENTENCE_CHARS,
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
        ];
        const INVALID: &[&str] = &[
            "人行道/路面上不许停车。",                // for `/``
            "演讲者向主席道了谢，（会议）就结束了。", // for `（` and `）`
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
