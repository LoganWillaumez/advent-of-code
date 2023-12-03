use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./src/data.txt") {
        let mut final_result_one = Vec::new();
        let mut final_result_two = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
               let (is_possible, index) = treat_game(&ip);
               let result_two = treat_game_power(&ip);
               final_result_two.push(result_two);
               if is_possible {
                final_result_one.push(index);
               }
            }
        }
        println!("First star is : {}", final_result_one.iter().sum::<i32>());
        println!("Second star is : {}", final_result_two.iter().sum::<i32>());
    }
}

fn treat_game(text: &str) -> (bool, i32){
    let mut text_to_treat = separate_text(text, ':');
    let index_to_get = separate_text(text_to_treat[0], ' ')[1].parse::<i32>().unwrap();
    let mut final_result = true;
    text_to_treat = separate_text(text_to_treat[1], ';');
    for text in text_to_treat{

        let part_color = separate_text(text.trim(), ',');
        for element in part_color {
            let final_check = separate_text(element.trim(), ' ');

            let (number, color): (i32, &str) = (final_check[0].parse().unwrap(), final_check[1]);
            let new_result = limit_from_color(color, number);
            if new_result == false {
                final_result = false;
            }
        }
    }
    (final_result, index_to_get)
}

fn update_color_count<'a>(color_counts: &mut HashMap<&'a str, i32>, color: &'a str, new_count: i32) {
    let count = color_counts.entry(color).or_insert(0);
    if *count < new_count {
        *count = new_count;
    }
}

fn treat_game_power(text: &str) -> i32 {
    let mut color_counts = HashMap::new();

    color_counts.insert("green", 0);
    color_counts.insert("red", 0);
    color_counts.insert("blue", 0);

    let mut text_to_treat = separate_text(text, ':');
    text_to_treat = separate_text(text_to_treat[1], ';');
    for text in text_to_treat{

        let part_color = separate_text(text.trim(), ',');
        for element in part_color {
            let final_check = separate_text(element.trim(), ' ');

            let (number, color): (i32, &str) = (final_check[0].parse().unwrap(), final_check[1]);
            update_color_count(&mut color_counts, color, number );
        }
    }
    let values: Vec<i32> = color_counts.values().cloned().collect();
    let mut product = 1;
    for value in values {
        product *= value;
    }
    product
}



fn separate_text(text: &str, pattern: char) -> Vec<&str>{
    let parts: Vec<&str> = text.split(pattern).collect();
    parts
}

fn limit_from_color(color: &str, number: i32) -> bool {
    match color {
        "red" => number <= 12,
        "green" => number <= 13,
        "blue" => number <= 14,
        _ => false,
    }
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
    fn test_treat_game() {
        assert_eq!(treat_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), (true, 1));
        assert_eq!(treat_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), (false, 4));
        assert_eq!(treat_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), (true, 5));
        assert_eq!(treat_game("Game 99: 3 red, 4 blue; 7 red, 5 blue, 3 green; 2 green, 1 blue, 1 red; 4 blue, 2 green, 1 red; 1 green, 1 red, 2 blue; 1 green, 6 blue, 7 red"), (true, 99));
    }
}