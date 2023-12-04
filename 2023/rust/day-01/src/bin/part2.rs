use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = env::args().nth(1).expect("Expected a file path to input");
    let mut sum = 0;
    for line in BufReader::new(File::open(file_path).unwrap()).lines() {
        match line {
            Ok(value) => {
                let num = convert_to_num(&value);
                println!("{} : {}", value, num);
                sum += num;
            }
            Err(_) => { panic!("Error occurred") }
        }
    }
    println!("{}", sum)
}

fn convert_to_num(value: &String) -> i32 {
    let first_digit = extract_digit(&value, false);
    let last_digit = extract_digit(&(value.chars().rev().collect()), true);
    first_digit * 10 + last_digit
}

fn extract_digit(value: &String, is_reverse: bool) -> i32 {
    let mut cur_string: String = String::new();
    for c in value.chars() {
        if c.is_ascii_digit() {
            let digit = c.to_digit(10).unwrap() as i32;
            return digit;
        } else {
            if is_reverse {
                cur_string.insert(0, c);
            } else { cur_string.push(c) };
            if cur_string.contains("one") {
                return 1;
            } else if cur_string.contains("two") {
                return 2;
            } else if cur_string.contains("three") {
                return 3;
            } else if cur_string.contains("four") {
                return 4;
            } else if cur_string.contains("five") {
                return 5;
            } else if cur_string.contains("six") {
                return 6;
            } else if cur_string.contains("seven") {
                return 7;
            } else if cur_string.contains("eight") {
                return 8;
            } else if cur_string.contains("nine") {
                return 9;
            }
        }
    }
    return 0
}

#[cfg(test)]
mod tests {
    use crate::convert_to_num;

    #[test]
    fn it_works() {
        let input = "lstwone347nine".to_string();
        let input = "twone".to_string();
        assert_eq!(convert_to_num(&input), 21);
    }
}