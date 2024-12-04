#[path = "../read_file.rs"]
mod read_file;

const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";

pub fn main() {
    let inputs = read_file::read_lines("./src/day4_1/input.txt");
    let width = inputs[0].chars().count();
    let height = inputs.len();

    let mut sum: i64 = 0;
    for i in 0..=height - 1 {
        for j in 0..=width - 1 {
            sum += find_word(inputs.clone(), i, j);
        }
    }

    println!("{}", sum)
}

fn find_word(inputs: Vec<String>, i: usize, j: usize) -> i64 {
    let mut sum = 0;

    if find_word_horizontal(inputs.clone(), i, j, XMAS)
        || find_word_horizontal(inputs.clone(), i, j, SAMX)
    {
        sum += 1;
    }

    if find_word_vertical(inputs.clone(), i, j, XMAS)
        || find_word_vertical(inputs.clone(), i, j, SAMX)
    {
        sum += 1;
    }

    if find_word_diagonal(inputs.clone(), i, j, XMAS)
        || find_word_diagonal(inputs.clone(), i, j, SAMX)
    {
        sum += 1;
    }

    if find_word_revert_diagonal(inputs.clone(), i, j, XMAS)
        || find_word_revert_diagonal(inputs.clone(), i, j, SAMX)
    {
        sum += 1;
    }

    return sum;
}

fn find_word_horizontal(inputs: Vec<String>, i: usize, j: usize, word: &str) -> bool {
    let word_length = word.chars().count();
    let width = inputs[0].chars().count();

    let width_exceed = j + word_length > width;
    if width_exceed {
        return false;
    }

    let mut str: Vec<String> = Vec::new();
    for k in 0..=word_length - 1 {
        str.push(inputs[i].chars().nth(j + k).unwrap().to_string());
    }
    let joined = str.join("");

    joined == word
}

fn find_word_vertical(inputs: Vec<String>, i: usize, j: usize, word: &str) -> bool {
    let word_length = word.chars().count();
    let height = inputs.len();

    let height_exceed = i + word_length > height;
    if height_exceed {
        return false;
    }

    let mut str: Vec<String> = Vec::new();
    for k in 0..=word_length - 1 {
        str.push(inputs[i + k].chars().nth(j).unwrap().to_string());
    }
    let joined = str.join("");

    joined == word
}

fn find_word_diagonal(inputs: Vec<String>, i: usize, j: usize, word: &str) -> bool {
    let word_length = word.chars().count();
    let width = inputs[0].chars().count();
    let height = inputs.len();

    let width_exceed = j + word_length > width;
    let height_exceed = i + word_length > height;

    if width_exceed || height_exceed {
        return false;
    }

    let mut str: Vec<String> = Vec::new();
    for k in 0..=word_length - 1 {
        str.push(inputs[i + k].chars().nth(j + k).unwrap().to_string());
    }
    let joined = str.join("");

    joined == word
}

fn find_word_revert_diagonal(inputs: Vec<String>, i: usize, j: usize, word: &str) -> bool {
    let word_length = word.chars().count();
    let height = inputs.len();

    let width_exceed = (word_length - 1) > j;
    let height_exceed = i + word_length > height;

    if width_exceed || height_exceed {
        return false;
    }

    let mut str: Vec<String> = Vec::new();
    for k in 0..=word_length - 1 {
        str.push(inputs[i + k].chars().nth(j - k).unwrap().to_string());
    }
    let joined = str.join("");

    joined == word
}
