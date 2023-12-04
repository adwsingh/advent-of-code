use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let limits: HashMap<&str, u32> = HashMap::<&str, u32>::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    let file_path = env::args().nth(1).expect("Expected a file path to input");
    let output: u32 = BufReader::new(File::open(file_path).expect("Cannot open file."))
        .lines()
        .map(Result::unwrap)
        .map(|line| { process(line, &limits) })
        .sum();
    println!("{}", output)
}

fn process(line: String, limits: &HashMap<&str, u32>) -> u32 {
    //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let mut tmp = line.split(": ");
    let game_id =  str::parse::<u32>(tmp.nth(0)
        .unwrap()
        .split("Game ")
        .nth(1).unwrap())
        .unwrap();
    let mut counts = tmp.nth(0).unwrap().split(";");
    for count in counts {
        for color_count in count.split(",") {
            let color_count = color_count.trim();
            let mut tmp = color_count.split(" ");
            let count = str::parse::<u32>(tmp.nth(0).unwrap()).unwrap();
            let color = tmp.nth(0).unwrap();
            if count > limits.get(color).copied().unwrap_or(0) {
                return 0;
            }
        }
    }
    game_id
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
        assert_eq!(1, process(input, &limits))
    }
}