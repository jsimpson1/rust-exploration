extern crate utilities;

use std::collections::{HashMap};
use utilities::import;

const PATH: &str = "inputs/input0.txt";

fn main() {
    let input = import::get_input(PATH);
    println!("input:\n{}", input);
    let crates = crates_at_top_of_stacks(&input);
    println!("crates:\n{}", crates)
}
// #[derive(Debug)]
// struct StackAndMoves(str, str);

fn parse_stacks_and_moves(input: &str) -> HashMap<u32, Vec<char>> {
    let input_split = input.split("\n\n").collect::<Vec<&str>>();
    let initial_stacks = parse_stacks(input_split[0]);
    // println!("stacks={:?}", initial_stacks);
    let moves_input = input_split[1];
    println!("moves_input=\n{}", moves_input);
    let moves = moves_input.split("\n").collect::<Vec<_>>();
    let result = moves
        .iter()
        .fold(initial_stacks, |stack, &move_command| {
            println!("stacks={:?}", &stack);
            move_crates(stack, move_command)
        });
    println!("result={:?}", &result);
    return result
}

fn get_number_from_move_component(move_components: &Vec<&str>, index: u32) -> u32 {
    return move_components
        .get(index as usize)
        .map(|s|
            s.parse::<u32>().unwrap()
        ).unwrap();
}

fn parse_move_command(move_command: &str) -> [u32; 3]{
    let move_components = move_command.trim().split(" ").collect::<Vec<_>>();
    let number_of_crates_to_move = get_number_from_move_component(&move_components, 1);
    let move_from_index = get_number_from_move_component(&move_components, 3);
    let move_to_index = get_number_from_move_component(&move_components, 5);
    return [
        number_of_crates_to_move,
        move_from_index,
        move_to_index
    ]
}

fn move_crates(stack: HashMap<u32, Vec<char>>, move_command: &str) -> HashMap<u32, Vec<char>> {

    println!("move_command={}", move_command);

    let move_parts = parse_move_command(move_command);
    let num_of_crates_to_move = move_parts[0];
    let from_key = move_parts[1];
    let to_key = move_parts[2];

    let mut new_stack = stack.clone();

    match &stack.get(&from_key) {
        Some(source_stack) => {
            match &stack.get(&to_key) {
                Some(target_stack) => {
                    let last_n_index = source_stack.len() - num_of_crates_to_move as usize;
                    let source_stack_split = source_stack.split_at(last_n_index);
                    let mut source_crates_to_move = source_stack_split.1.to_vec();
                    source_crates_to_move.reverse();
                    let new_target_stack = [target_stack.to_vec(), source_crates_to_move].concat();
                    new_stack.insert(to_key, new_target_stack);
                    // remaining source
                    let new_source_stack = source_stack_split.0.to_vec();
                    new_stack.insert(from_key, new_source_stack)
                },
                None => {
                    new_stack.insert(to_key, source_stack.to_vec())
                }
            }
        },
        None => {
            // nothing to move
            None
        }
    };

    return new_stack
}

fn parse_stacks(stack: &str) -> HashMap<u32, Vec<char>> {
    let mut stack_vec = stack.split("\n").collect::<Vec<&str>>();
    // println!("stack_vec=\n{:?}", stack_vec);
    stack_vec.reverse();
    let keys = &stack_vec[0]
        .split_whitespace()
        .collect::<Vec<&str>>();

    let keys = keys.into_iter().map(|v| v.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
    println!("keys={:?}", keys);
    let crate_lines = &stack_vec[1..stack_vec.len()];
    println!("crate_lines={:?}", &crate_lines);
    let mut stacks: HashMap<u32, Vec<char>> = HashMap::new();

    for &line in crate_lines {
        let chars = line
            .chars()
            .collect::<Vec<_>>();
        let values = chars
            .chunks(4)
            .into_iter()
            .collect::<Vec<_>>();
        println!("values={:?}", &values);
        let crate_chars = values.iter().map(|chars| chars[1] ).collect::<Vec<_>>();
        println!("crate_chars={:?}", &crate_chars);
        for key in &keys {
            let index = usize::try_from(key.clone()).unwrap() - 1;
            let crate_char = crate_chars[index];
            match crate_char {
                ' ' => {
                    // do not push anything
                }
                non_empty_crate_char => {
                    match stacks.get_mut(key) {
                        Some(crates) => {
                            crates.push(non_empty_crate_char);
                        }
                        None => {
                            let new_vec = [non_empty_crate_char].to_vec();
                            stacks.insert(key.clone(), new_vec);
                        }
                    }
                }
            };
        }
    }
    return stacks;
}

fn crates_at_top_of_stacks(input: &str) -> String {
    let stacks_after_moves = parse_stacks_and_moves(input);
    let keys = (1..stacks_after_moves.len()+1)
        .map(|v| v as u32)
        .collect::<Vec<u32>>();

    let result_str = keys
        .iter()
        .fold(String::new(), |mut str_value, index| {
            match stacks_after_moves.get(index) {
                Some(crates) => {
                    match crates.last() {
                        Some(last_crate) => {
                            let crate_char = *last_crate;
                            str_value.push(crate_char);
                            str_value
                        },
                        None => {
                            str_value
                        }
                    }

                },
                None => {
                    str_value
                }
            }
        });
    return result_str
}
