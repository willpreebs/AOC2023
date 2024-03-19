use std::{fs::File, io::{BufReader, BufRead}};


pub fn read_file_to_strings(filename: &str) -> Vec<String> {

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for expect_line in reader.lines() {
        let line = expect_line.unwrap();
        let line = line.as_str();
        lines.push(String::from(line));
    }

    return lines;
}