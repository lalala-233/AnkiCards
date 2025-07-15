mod check;
fn main() -> std::io::Result<()> {
    // Open the input file (test.txt)
    let file_name = impure::read_env();
    let binding = impure::read_to_string(&file_name)?;
    let (_, content) = split_header(&binding);
    let error_info = check::check(content);
    impure::copy_error(error_info, binding.lines().count());
    Ok(())
}

fn split_header(file: &str) -> (&str, &str) {
    let index = file.match_indices('\n').nth(5).unwrap().0;
    file.split_at(index + '\n'.len_utf8())
}
pub enum Error {}

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
