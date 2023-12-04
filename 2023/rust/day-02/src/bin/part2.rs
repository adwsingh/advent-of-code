use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let file_path = env::args().nth(1).expect("Expected a file path to input");
    let output: u32 = BufReader::new(File::open(file_path).expect("Cannot open file."))
        .lines()
        .map(Result::unwrap)
        .map(|line| { process(line) })
        .sum();
    println!("{}", output)
}

fn process(line: String) -> u32 {
    //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let counts = line.split(": ").nth(1).unwrap().split(";");
    let mut limits = HashMap::new();
    for count in counts {
        for color_count in count.split(",") {
            let color_count = color_count.trim();
            let mut tmp = color_count.split(" ");
            let count = str::parse::<u32>(tmp.nth(0).unwrap()).unwrap();
            let color = tmp.nth(0).unwrap();
            if count > limits.get(color).copied().unwrap_or(u32::MIN) {
                limits.insert(color, count);
            }
        }
    }
    limits.get("red").copied().unwrap_or(0) * limits.get("green").copied().unwrap_or(0) * limits.get("blue").copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::process;

    #[test]
    fn test_parsing() {
        let input = String::from("Game 1: 5 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let limits: HashMap<&str, u32> = HashMap::<&str, u32>::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14)
        ]);
        assert_eq!(48, process(input))
    }
}