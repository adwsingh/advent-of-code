use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = env::args().nth(1).expect("Expected a file path to input");
    let mut sum = 0;
    for line in BufReader::new(File::open(file_path).unwrap()).lines() {
        match line {
            Ok(value) => {
                let mut first_digit: i32 = -1;
                let mut last_digit = 0;
                for c in value.chars() {
                    if c.is_ascii_digit() {
                        let digit = c.to_digit(10).unwrap() as i32;
                        if (first_digit == -1) {
                            first_digit = digit
                        }
                        last_digit = digit
                    }
                }
                sum += first_digit * 10 + last_digit;
            }
            Err(_) => { panic!("Error occurred") }
        }
    }
    println!("{}", sum)
}