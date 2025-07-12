use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    // Open the input file (test.txt)
    let file_name = "初中.txt";
    let template = "英语单词模板";
    let input_file = File::open(file_name)?;
    let reader = BufReader::new(input_file);
    // Create or open the output file (result.txt)
    let mut output_file = File::create(format!("{file_name}-result.txt"))?;
    impure::write_header(&mut output_file)?;
    let mut counts = 0.0;
    // Process each line from the input file
    for line in reader.lines().map_while(Result::ok) {
        {
            // Split the line into parts using the "|" delimiter

            counts += 1.0;
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 7 {
                eprintln!("{parts:?}");
                impure::copy_error("不是 7 个", parts[4], counts);
            }

            // Extract the English sentence and Chinese translation
            let english_sentence: Vec<&str> = parts[4].split("<br>").collect();
            let chinese_translation: Vec<&str> = parts[5].split("<br>").collect();

            let mut english = english_sentence[0].to_owned();
            let mut chinese = String::new();
            if english_sentence.len() != chinese_translation.len() {
                eprintln!("{english} | {chinese}");
                impure::copy_error("对不上", parts[4], counts);
            }
            if english.is_ascii() {
                chinese_translation[0].clone_into(&mut chinese);
            } else {
                let space_index = english.find(|c: char| !c.is_ascii()).unwrap();

                // Split the string into two parts
                english_sentence[0][0..space_index].clone_into(&mut english);
                english_sentence[0][space_index..]
                    .to_owned()
                    .clone_into(&mut chinese);
                eprintln!("{english} | {chinese}");
                impure::copy_error("已复制", parts[4], counts);
            }

            let processed_line = format!(
                "\"{}\"|\"{}\"|\"{}\"|\"{}\"|\"{}\"|\"{}\"|\"{}\"|\"{}\"\n",
                parts[0], parts[1], parts[2], parts[3], english, chinese, parts[6], template
            );
            // Write the processed line to the output file

            write!(output_file, "{processed_line}")?;
        }
    }

    println!("Processed lines are written to result.txt with {counts}");
    Ok(())
}
mod impure {
    use std::{fs::File, io::Write, thread::sleep, time::Duration};

    pub fn write_header(output_file: &mut File) -> Result<(), std::io::Error> {
        output_file.write_all(
            "#separator:|
#html:true
#notetype column:8
#deck column:1
#tags column:7
"
            .as_bytes(),
        )
    }
    pub fn copy_error(error_info: &str, copy_content: &str, counts_of_pass: f64) {
        let all_line = 19403.0;
        let mut board = arboard::Clipboard::new().unwrap();
        board.set_text(copy_content).unwrap();
        sleep(Duration::from_millis(25));
        panic!(
            "{}，{}/{}，{}",
            error_info,
            counts_of_pass,
            all_line,
            counts_of_pass / all_line
        );
    }
}
