use std::fs;
use std::fs::read_to_string;

pub fn read_file(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
