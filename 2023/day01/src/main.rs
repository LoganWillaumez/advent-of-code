use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(Debug)]
enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Number {
    fn from_str(s: &str) -> Option<Number> {
        match s {
            "one" => Some(Number::One),
            "two" => Some(Number::Two),
            "three" => Some(Number::Three),
            "four" => Some(Number::Four),
            "five" => Some(Number::Five),
            "six" => Some(Number::Six),
            "seven" => Some(Number::Seven),
            "eight" => Some(Number::Eight),
            "nine" => Some(Number::Nine),
            _ => None,
        }
    }

    fn to_digit(&self) -> char {
        match self {
            Number::One => '1',
            Number::Two => '2',
            Number::Three => '3',
            Number::Four => '4',
            Number::Five => '5',
            Number::Six => '6',
            Number::Seven => '7',
            Number::Eight => '8',
            Number::Nine => '9',
        }
    }
}


fn main() {
    if let Ok(lines) = read_lines("./src/data.txt") {
        let mut final_result = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                let get_data = process_line(ip);
                final_result.push(get_data);
            }
        }
        println!("final result is: {}", final_result.iter().sum::<i32>());
    }
}

fn process_line(line: String) -> i32 {
    let target_words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut result = Vec::new();
    let mut i = 0;

    while i < line.len() {
        if let Some(char) = line[i..].chars().next() {
            if char.is_digit(10) {
                result.push(char);
                i += 1;
            } else {
                let mut found = false;
                for mot in &target_words {
                    if line[i..].starts_with(mot) {
                        if let Some(number) = Number::from_str(mot){
                            found = true;
                            result.push(number.to_digit());
                            i += mot.len() -1; 
                            break;
                        }
                    }
                }
                if !found {
                    i += 1;
                }
            }
        }
    }

    let result = vec![result[0], result[result.len() -1]].iter().collect::<String>();
    result.parse::<i32>().expect("Error while getting the i32")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line() {
        assert_eq!(process_line("abcone2threexyz".to_string()), 13);
        assert_eq!(process_line("4nineeightseven2".to_string()), 42);
        assert_eq!(process_line("7pqrstsixteen".to_string()), 76);
        assert_eq!(process_line("nrshscpxfivepqxccgpzfeight3zcvgvkvlcdmrmqmrqrj".to_string()), 53);
        assert_eq!(process_line("zoneight234".to_string()), 14);
        assert_eq!(process_line("xtwone3four".to_string()), 24);
    }
}