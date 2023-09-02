use std::cmp::Ordering;
use std::fs;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSOR: u32 = 3;

fn main() {

//     let input =
// "A Y
// B X
// C Z";

    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let result = solve_puzzle(input.trim());
    println!("result={result}")
}

fn opponent_shape_score(choice: char) -> u32 {
    match choice {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSOR,
        _ => 0
    }
}

fn self_shape_score(choice: char) -> u32 {
    match choice {
        'X' => ROCK,
        'Y' => PAPER,
        'Z' => SCISSOR,
        _ => 0
    }
}

const O_ROCK: char = 'A';
const O_PAPER: char = 'B';
const O_SCISSOR: char = 'C';

const S_ROCK: char = 'X';
const S_PAPER: char = 'Y';
const S_SCISSOR: char = 'Z';

fn outcome_score(self_choice: char, opponent_choice: char) -> u32 {

    match (self_choice, opponent_choice) {
        (S_ROCK, O_ROCK) => 3,
        (S_ROCK, O_PAPER) => 0,
        (S_ROCK, O_SCISSOR) => 6,
        (S_PAPER, O_PAPER) => 3,
        (S_PAPER, O_ROCK) => 6,
        (S_PAPER, O_SCISSOR) => 0,
        (S_SCISSOR, O_SCISSOR) => 3,
        (S_SCISSOR, O_ROCK) => 0,
        (S_SCISSOR, O_PAPER) => 6,
        _ => 0
    }
}

fn round_score(
    line: &str
) -> u32 {
    // println!("line={line}");
    let character: Vec<char> = line.trim().chars().collect();
    let opponent_choice = character[0];
    let self_choice = character[2];
    // println!("opponent_choice={opponent_choice}, self_choice={self_choice}");
    let s_shape_score = self_shape_score(self_choice);
    // println!("o_shape_score={o_shape_score}, s_shape_score={s_shape_score}");
    let round_outcome = outcome_score(self_choice, opponent_choice);
    // println!("round_outcome={round_outcome}");
    let total_score = round_outcome + s_shape_score;
    // println!("total_score={total_score}");

    total_score
}

fn solve_puzzle(input: &str) -> u32 {

    let lines = input
        .split("\n")
        .collect::<Vec<&str>>();

    let total_score = lines
        .iter()
        .fold(
            0,
            |acc, &line| acc + round_score(line)
        );

    return total_score
}