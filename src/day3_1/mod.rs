#[path = "../read_file.rs"]
mod read_file;
use regex::Regex;

const REGEXP: &str = r"mul\((-?\d+),(-?\d+)\)";

pub fn main() {
    let input = read_file::read_file("./src/day3_1/input.txt".to_owned());
    let re = Regex::new(REGEXP).unwrap();

    let mut sum: i64 = 0;

    for cap in re.captures_iter(&input) {
        sum += (&cap[1]).parse::<i64>().unwrap() * (&cap[2]).parse::<i64>().unwrap()
    }

    println!("{sum}")
}
