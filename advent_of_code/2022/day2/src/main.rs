use std::fs;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSOR: u32 = 3;
const O_ROCK: char = 'A';
const O_PAPER: char = 'B';
const O_SCISSOR: char = 'C';

const S_ROCK: char = 'X';
const S_PAPER: char = 'Y';
const S_SCISSOR: char = 'Z';
fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let result = solve_puzzle(input.trim());
    println!("result={result}")
}
const LOSS_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;
fn self_shape_score(choice: char) -> u32 {
    match choice {
        'X' => LOSS_SCORE,
        'Y' => DRAW_SCORE,
        'Z' => WIN_SCORE,
        _ => 0
    }
}

const LOSS: char = 'X';
const DRAW: char = 'Y';
const WIN: char = 'Z';

fn outcome_score(self_choice: char, opponent_choice: char) -> u32 {
    let result: u32 = match (opponent_choice, self_choice ) {
        (O_ROCK, LOSS) => SCISSOR,
        (O_ROCK, DRAW) => ROCK,
        (O_ROCK, WIN) => PAPER,
        (O_PAPER, LOSS) => ROCK,
        (O_PAPER, DRAW) => PAPER,
        (O_PAPER, WIN) => SCISSOR,
        (O_SCISSOR, LOSS) => PAPER,
        (O_SCISSOR, DRAW) => SCISSOR,
        (O_SCISSOR, WIN) => ROCK,
        _ => {
            // println!("default");
            0
        }
    };
    // println!("opponent_choice={opponent_choice}, self_choice={self_choice}, result={result}");
    result

}

fn round_score(
    line: &str
) -> u32 {
    let character: Vec<char> = line.trim().chars().collect();
    let opponent_choice = character[0];
    let self_choice = character[2];

    let s_shape_score = self_shape_score(self_choice);

    let round_outcome = outcome_score(self_choice, opponent_choice);

    let total_score = round_outcome + s_shape_score;

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