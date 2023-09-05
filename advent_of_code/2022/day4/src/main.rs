use std::fs;

const PUZZLE_PART: u32 = 0;

fn main() {
    let input = get_input(PUZZLE_PART);
    println!("input:\n{}", input);
}

fn get_input(part: u32) -> String {
    let input_file_name = format!("src/input_part{}.txt", part);
    let message = format!("Should be getting input from a file: {}", input_file_name);
    let input = fs::read_to_string(input_file_name).expect(&message);
    input
}
