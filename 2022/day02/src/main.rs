
fn main() {
    let GAMESET: Vec<&str> = include_str!("../info.txt").split("\n").collect();
    let mut result_one: Vec<u32> =Vec::new();
    let mut result_two: Vec<u32> =Vec::new();
    for game in GAMESET {
        match game {
            "A X" => {
                result_one.push(4);
                result_two.push(3);
            },
            "A Y" => {
                result_one.push(8);
                result_two.push(4);
            },
            "A Z" => {
                result_one.push(3);
                result_two.push(8);
            },
            "B X" => {
                result_one.push(1);
                result_two.push(1);
            },
            "B Y" => {
                result_one.push(5);
                result_two.push(5);
            },
            "B Z" => {
                result_one.push(9);
                result_two.push(9);
            },
            "C X" => {
                result_one.push(7);
                result_two.push(2);
            },
            "C Y" => {
                result_one.push(2);
                result_two.push(6);
            },
            "C Z" => {
                result_one.push(6);
                result_two.push(7);
            },
            _ => {}
        }
    }
    let result_one: u32 = result_one.iter().sum();
    let result_two: u32 = result_two.iter().sum();
    println!("Final result one : {} and two: {}", result_one, result_two);
}
