use std::slice;

fn sum_similar_letters(string: &str) -> u32 {
    // Split the string into two variables
    let (var1, var2) = string.split_at(string.len() / 2);

    // Create a set to store the common letters
    let mut common_letters = std::collections::HashSet::new();

    // Loop through the letters in var1 and check if they are also in var2
    for c in var1.chars() {
        if var2.contains(c) {
            if !common_letters.contains(&c) {
                common_letters.insert(c);
            }
        }
    }

    // Initialize the sum to 0
    let mut sum = 0;

    // Loop through the common letters and calculate their sum
    for c in common_letters {
        sum += letter_value(c);
    }

    // Return the sum of the common letters
    sum
}

fn detect_same_letters(vec: &Vec<&str>) {
    let mut common_letters: Vec<str> = Vec::new();

    let [string1, string2, string3] = vec;

    for letter1 in string1.chars() {
        if string2.contains(letter1) || string3.contains(letter1) {
            if !common_letters.contains(letter1){
                common_letters.push(letter1);
            }
        }
    }

    for letter2 in string2.chars() {
        if string1.contains(letter2) || string3.contains(letter2) {
            if !common_letters.contains(letter2){
                common_letters.push(letter2);
            }
        }
    }

    for letter3 in string3.chars() {
        if string1.contains(letter3) || string2.contains(letter3) {
            if !common_letters.contains(letter3){
                common_letters.push(letter3);
            }
        }
    }

    println!("{:?}", common_letters);
}

fn letter_value(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u8 - b'a' + 1) as u32,
        'A'..='Z' => (c as u8 - b'A' + 27) as u32,
        _ => 0,
    }
}

fn main() {
    let test = vec!["vJrwpWtwJgWrhcsFMMfFFhFp",
"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
"PmmdzqPrVvPwwTWBwg"];
    let rucksack: Vec<&str> = include_str!("../info.txt").split("\n").collect();
    let rucksack2: Vec<Vec<&str>> = rucksack.chunks(3).map(|chunk| chunk.to_vec()).collect();
    let mut final_response: Vec<u32> = Vec::new();
    let mut final_response2: Vec<u32> = Vec::new();
    for element in rucksack {
        final_response.push(sum_similar_letters(element));
    }
    // for element in rucksack2 {
    //     final_response2.push(detect_same_letters(&element));
    // }
    detect_same_letters(&test);
    let final_response: u32 = final_response.iter().sum();
    let final_response2: u32 = final_response2.iter().sum();
    print!("Response 1 :{:?}, Response 2 : {:?}", final_response, final_response2)
}