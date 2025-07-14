fn main() -> std::io::Result<()> {
    // Open the input file (test.txt)
    let file_name = impure::read_env();
    let binding = impure::read_to_string(&file_name)?;
    let (_, content) = split_header(&binding);
    let error_info = todo::check(content);
    impure::copy_error(error_info, binding.lines().count());
    Ok(())
}

fn split_header(file: &str) -> (&str, &str) {
    let index = file.match_indices('\n').nth(5).unwrap().0;
    file.split_at(index + '\n'.len_utf8())
}
pub enum Error {}
mod todo {
    pub fn check(s: &str) -> Option<(usize, String)> {
        s.lines()
            .enumerate()
            .find_map(|(count_of_line, line)| check_line(line).err().map(|s| (count_of_line, s)))
    }
    fn check_line(line: &str) -> Result<(), String> {
        let Some([
            _deck,
            word,
            _pronunciation,
            _chinese,
            _english_sentence,
            _chinese_sentence,
            _tag,
            _notetype,
        ]) = extract_line(line)
        else {
            return Err('|'.to_string());
        };
        check_word(word)?;
        // check_english_sentence(english_sentence)?;
        Ok(())
    }
    fn check_word(word: &str) -> Result<(), String> {
        if word.chars().all(is_valid_word_char) && is_valid_word_str(word) {
            return Ok(());
        }
        Err(word.to_string())
    }
    fn is_valid_word_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || VALID_WORD_CHAR.contains(&c)
    }
    const VALID_WORD_CHAR: &[char] = &['\'', '-', ' ', '(', ')', ',', '.', '?', '!','/'];
    const ALLOWED_COMBINATIONS: &[&str] = &[
        " (", ") ", ", ", "' ", /* ones' something */
        ".."," .",". " /* special handing */
    ];
    fn cartesian_product(chars_1: &[char], chars_2: &[char]) -> Vec<String> {
        chars_1
            .iter()
            .flat_map(|c1| chars_2.iter().map(move |c2| format!("{c1}{c2}")))
            .collect()
    }
    fn is_valid_word_str(s: &str) -> bool {
        is_combination_valid(s) && is_extra_combination_valid(s)
    }
    fn is_extra_combination_valid(s: &str) -> bool {
        if s.contains("..") || s.contains(" .")|| s.contains(". "){
            s.contains("...")
        } else {
            true
        }
    }
    fn is_combination_valid(s: &str) -> bool {
        cartesian_product(VALID_WORD_CHAR, VALID_WORD_CHAR)
            .iter()
            .filter(|s| !ALLOWED_COMBINATIONS.contains(&s.as_str()))
            .all(|invalid| !s.contains(invalid))
    }
    fn _check_english_sentence(sentence: &str) -> Result<(), String> {
        let error_list = ["："];
        if error_list.iter().any(|error| sentence.contains(error)) {
            return Err(sentence.to_string());
        }
        // for error in error_list {
        //     match_then_err(sentence, error)?;
        // }

        Ok(())
    }
    fn extract_line(line: &str) -> Option<[&str;8]> {
        let mut iter = line.split('|');
        let result = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];
        if iter.next().is_some() {
            None
        } else {
            Some(result)
        }
    }
}
mod impure {
    use std::{fs::File, io::Write, thread::sleep, time::Duration};
    pub fn read_env() -> String {
        std::env::args().nth(1).expect("Please provide a file name")
    }
    pub fn read_to_string(file_name: &str) -> std::io::Result<String> {
        std::fs::read_to_string(file_name)
    }
    pub fn _write_to_file(file_name: &str, header: &str, content: &str) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        write!(file, "{header}{content}")
    }
    pub fn copy_error(error_info: Option<(usize, String)>, len_of_line: usize) {
        if let Some((line, s)) = error_info {
            let mut board = arboard::Clipboard::new().unwrap();
            board.set_text(&s).unwrap();
            sleep(Duration::from_millis(25));
            panic!("Invalid `{s}` at {line}/{len_of_line}",);
        }
    }
}
