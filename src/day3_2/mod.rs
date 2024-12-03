#[path = "../read_file.rs"]
mod read_file;
use regex::Regex;

const REGEXP: &str = r"(do\(\)|don't\(\)|mul\((-?\d+),(-?\d+)\))";
const REGEXP_MUL: &str = r"mul\((-?\d+),(-?\d+)\)";

pub fn main() {
    let input = read_file::read_file("./src/day3_2/input.txt".to_owned());
    let re = Regex::new(REGEXP).unwrap();
    let mul_re: Regex = Regex::new(REGEXP_MUL).unwrap();

    let mut sum: i64 = 0;
    let mut flag: bool = true;

    for cap in re.captures_iter(&input) {
        if &cap[0] == "do()".to_owned() {
            flag = true;
            continue;
        }

        if &cap[0] == "don't()".to_owned() {
            flag = false;
            continue;
        }

        if !flag {
            continue;
        }

        for mul_cap in mul_re.captures_iter(&cap[0]) {
            sum += (&mul_cap[1]).parse::<i64>().unwrap() * (&mul_cap[2]).parse::<i64>().unwrap()
        }
    }

    println!("{sum}")
}
